//! 바이트 조각에서 완성 줄을 가르는 로직 자리. 줄 끝·UTF-8 변환·꼬리 추출만 봄.

use std::str::from_utf8;

use crate::llm::split_error::SplitError;

const CR: u8 = b'\r';
const LF: u8 = b'\n';

/// 완성 줄만 잘라 내고, 줄 끝이 없는 꼬리는 바이트로 되돌림.
pub fn split_complete_lines(source: &[u8]) -> (Vec<Vec<u8>>, Vec<u8>) {
    let mut lines = Vec::new();
    let mut start = 0usize;
    let mut index = 0usize;
    while index < source.len() {
        match source[index] {
            LF => {
                lines.push(source[start..index].to_vec());
                index += 1;
                start = index;
            }
            CR => {
                let after_cr = index + 1;
                let next_is_lf = source.get(after_cr) == Some(&LF);
                if !next_is_lf && after_cr == source.len() {
                    // 맨 뒤 CR은 다음 조각의 LF와 짝일 수 있어 이 줄을 통째로 미룸.
                    break;
                }
                lines.push(source[start..index].to_vec());
                index = if next_is_lf { after_cr + 1 } else { after_cr };
                start = index;
            }
            _ => index += 1,
        }
    }
    let tail = source[start..].to_vec();
    (lines, tail)
}

/// `data` 줄의 페이로드. `data`가 아닌 줄은 None.
/// WHATWG 규칙: 콜론 뒤 첫 칸 한 개만 제거. 앞뒤 그 밖의 공백은 그대로 둠.
pub fn data_payload(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("data:")?;
    Some(rest.strip_prefix(' ').unwrap_or(rest))
}

/// 완성 줄 바이트를 UTF-8 문자열로 옮김.
pub fn decode(bytes: &[u8]) -> Result<String, SplitError> {
    from_utf8(bytes)
        .map(|line| line.to_owned())
        .map_err(SplitError::from)
}

/// 스트림 꼬리에서 줄 하나를 뽑음. 맨 뒤 CR·CRLF는 줄 끝이라 버림.
pub fn tail_line(source: &[u8]) -> Option<&[u8]> {
    match source {
        [] => None,
        [LF] => None,
        [CR] => None,
        [rest @ .., LF] => Some(rest),
        [rest @ .., CR] => Some(rest),
        whole => Some(whole),
    }
}
