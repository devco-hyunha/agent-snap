//! 02-5 시험 재료 정본. raw HTTP 1.1로 모의 서버에 요청을 보내고 응답 조각을 읽는 창구.
use std::net::SocketAddr;
use std::time::Duration;

use tokio::io::AsyncBufReadExt;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::io::BufReader;
use tokio::net::TcpStream;
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::time::timeout;

/// 응답 머리 부분. 상태 코드와 헤더.
#[derive(Debug)]
pub(crate) struct Head {
    /// 응답 상태 코드.
    pub(crate) status: u16,

    /// 받은 응답 헤더.
    pub(crate) headers: Vec<(String, String)>,
}

/// 응답 본문 읽기 결과.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ChunkRead {
    /// 조각을 받았다.
    Chunk(Vec<u8>),

    /// chunked 끝 표시.
    End,

    /// 지정한 대기 시간을 넘겼다.
    Timeout,
}

/// 모의 서버와 연결한 raw HTTP 1.1 창구. chunked 프래이밍을 풀어서 조각만 돌려준다.
pub(crate) struct RawClient {
    /// 쓰기 창구.
    writer: OwnedWriteHalf,

    /// 읽기 창구. 한 줄씩 읽기 위해 버퍼를 쓴다.
    reader: BufReader<OwnedReadHalf>,
}

impl RawClient {
    /// 서버에 연결한다.
    pub(crate) async fn connect(address: SocketAddr) -> std::io::Result<Self> {
        let stream = TcpStream::connect(address).await?;
        let (reader, writer) = stream.into_split();
        Ok(Self {
            writer,
            reader: BufReader::new(reader),
        })
    }

    /// 요청을 보내고 응답 머리 부분을 읽는다.
    pub(crate) async fn send(
        &mut self,
        method: &str,
        path: &str,
        headers: &[(&str, &str)],
        body: &str,
    ) -> std::io::Result<Head> {
        let lines = request_text(method, path, headers, body);
        self.writer.write_all(lines.as_bytes()).await?;
        self.writer.flush().await?;
        let status = read_status_line(&mut self.reader).await?;
        let headers = read_headers(&mut self.reader).await?;
        Ok(Head { status, headers })
    }

    /// 쓰기 창구를 닫아 연결을 끝냄. 서버 쪽 연결 정리 확인에 쓴다.
    pub(crate) async fn shutdown(&mut self) -> std::io::Result<()> {
        self.writer.shutdown().await
    }

    /// 응답 본문 조각을 하나 읽는다. 대기가 `wait`보다 길면 `Timeout`.
    pub(crate) async fn read_chunk(&mut self, wait: Duration) -> std::io::Result<ChunkRead> {
        match timeout(wait, read_frame(&mut self.reader)).await {
            Err(_) => Ok(ChunkRead::Timeout),
            Ok(Err(error)) => Err(error),
            Ok(Ok(frame)) => Ok(frame),
        }
    }
}

/// 요청 원문 조립. `Content-Length`를 항상 붙인다.
fn request_text(method: &str, path: &str, headers: &[(&str, &str)], body: &str) -> String {
    let mut lines = format!(
        "{method} {path} HTTP/1.1\r\ncontent-length: {}\r\n",
        body.len()
    );
    for (name, value) in headers {
        lines.push_str(&format!("{name}: {value}\r\n"));
    }
    lines.push_str("host: mock\r\n\r\n");
    lines.push_str(body);
    lines
}

/// chunked 프래이밍을 풀어서 데이터 조각만 돌려준다. 크기가 0이면 끝 표시.
async fn read_frame(reader: &mut BufReader<OwnedReadHalf>) -> std::io::Result<ChunkRead> {
    let size = read_hex_size(reader).await?;
    if size == 0 {
        return Ok(ChunkRead::End);
    }
    let mut bytes = vec![0u8; size];
    reader.read_exact(&mut bytes).await?;
    let mut trailer = [0u8; 2];
    reader.read_exact(&mut trailer).await?;
    Ok(ChunkRead::Chunk(bytes))
}

/// 상태 줄에서 상태 코드만 뽑는다.
async fn read_status_line(reader: &mut BufReader<OwnedReadHalf>) -> std::io::Result<u16> {
    let line = read_line(reader).await?;
    let code = line
        .split_whitespace()
        .nth(1)
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or_default();
    Ok(code)
}

/// 헤더 줄을 이름과 값으로 읽는다. 빈 줄에서 끝난다.
async fn read_headers(
    reader: &mut BufReader<OwnedReadHalf>,
) -> std::io::Result<Vec<(String, String)>> {
    let mut headers = Vec::new();
    loop {
        let line = read_line(reader).await?;
        if line.is_empty() {
            return Ok(headers);
        }
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        headers.push((name.trim().to_owned(), value.trim().to_owned()));
    }
}

/// chunked 프래이밍의 조각 크기를 16진수로 읽는다. 연결이 끝나면 0.
async fn read_hex_size(reader: &mut BufReader<OwnedReadHalf>) -> std::io::Result<usize> {
    let line = read_line(reader).await?;
    let size = usize::from_str_radix(line.trim(), 16).unwrap_or_default();
    Ok(size)
}

/// 개행으로 끝난 한 줄을 읽는다. 연결이 끝나면 빈 문자열.
async fn read_line(reader: &mut BufReader<OwnedReadHalf>) -> std::io::Result<String> {
    let mut line = String::new();
    reader.read_line(&mut line).await?;
    Ok(line.trim_end().to_owned())
}
