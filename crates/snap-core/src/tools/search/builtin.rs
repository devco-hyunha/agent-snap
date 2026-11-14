//! 내장 부분 문자열 검색 자리입니다(03-3 T2).
//!
//! ripgrep이 없을 때의 대체 경로이고, 부분 문자열 일치만 합니다(정규식은
//! ripgrep 몫이라 설명으로 알립니다). 숨김·비밀 거르기와 심볼릭 링크 비순회
//! (하위 폴더로 안 내려감)는 `walk`가 합니다.

use std::path::Path;

use super::walk::collect;

/// 내장 검색: 부분 문자열 일치, 형식 3종을 한 순회에서 만듭니다.
pub(crate) fn search_builtin(
    working_dir: &Path,
    start: &Path,
    pattern: &str,
    case_sensitive: bool,
    ext: Option<&str>,
    format: &str,
    include_hidden: bool,
) -> Vec<String> {
    let needle = if case_sensitive {
        pattern.to_owned()
    } else {
        pattern.to_lowercase()
    };
    let mut lines = Vec::new();
    for entry in collect(working_dir, start, true, include_hidden) {
        if entry.is_dir {
            continue;
        }
        if !ext_matches(&entry.path, ext) {
            continue;
        }
        let Ok(bytes) = std::fs::read(&entry.path) else {
            continue;
        };
        let Ok(text) = String::from_utf8(bytes) else {
            continue;
        };
        let mut hits = 0usize;
        for (index, raw) in text.split('\n').enumerate() {
            let line = raw.strip_suffix('\r').unwrap_or(raw);
            let matched = if case_sensitive {
                line.contains(&needle)
            } else {
                line.to_lowercase().contains(&needle)
            };
            if !matched {
                continue;
            }
            hits += 1;
            if format == "content" {
                lines.push(format!("{}:{}:{}", entry.rel, index + 1, line));
            }
        }
        if hits > 0 {
            if format == "files" {
                lines.push(entry.rel.clone());
            } else if format == "count" {
                lines.push(format!("{}:{}", entry.rel, hits));
            }
        }
    }
    lines
}

/// 확장자 필터 기준의 단일 정본입니다: `.rs`와 `rs`를 모두 받고, 비교는 대소문자를
/// 가리지 않습니다. ripgrep 경로도 같은 기준으로 맞춥니다(`ripgrep.rs`의 확장자 `--glob`
///에 `--glob-case-insensitive`를 붙여 이 판정과 같은 결과가 나오게 합니다).
pub(crate) fn ext_matches(path: &Path, ext: Option<&str>) -> bool {
    let Some(want) = ext else {
        return true;
    };
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let Some(got) = name.rsplit_once('.').map(|(_, ext)| ext) else {
        return false;
    };
    got.eq_ignore_ascii_case(want)
}
