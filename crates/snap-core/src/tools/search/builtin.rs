//! 내장 부분 문자열 검색 자리
//!
//! ripgrep이 없을 때의 대체 경로. 부분 문자열 일치만 사용

use std::path::Path;

use super::walk::collect;

/// 내장 검색: 부분 문자열 일치, 형식 3종을 한 순회에서 생성
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

/// 확장자 필터 기준의 단일 정본. `.rs`와 `rs`를 모두 받고, 비교는 대소문자 무시
/// ripgrep 경로도 같은 기준(`--glob`에 `--glob-case-insensitive` 병합)으로 맞춰 같은 결과가 나오게 함
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
