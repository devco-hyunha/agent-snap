//! execute 브리지: 호출 기록 줄을 쌓는 창구.
//!
//! 도구 호출 기록 자리에 붙이는 전용 줄이며, 엔진이 쓰는 자리에서 기록을 채워도 되게 내부를 스레드-safe하게 묶음. 줄 글자 규칙은 `text_parts.rs`를 사용.

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use super::text_parts::one_line;
use super::text_parts::truncate_chars;

/// 호출 기록 창구.
#[derive(Clone, Default)]
pub(in crate::tools) struct ScriptLog {
    shared: Arc<Inner>,
}

/// 기록 창구 안의 값.
#[derive(Default)]
struct Inner {
    lines: Mutex<Vec<String>>,
    index: AtomicUsize,
}

impl ScriptLog {
    /// 기록 한 줄을 만들어 쌓음.
    pub(in crate::tools) fn push(&self, name: &str, args_json: &str, outcome_text: &str) {
        let index = self.shared.index.fetch_add(1, Ordering::SeqCst) + 1;
        let head = format!(
            "[execute][{index}/{index}] {}({})",
            name,
            one_line(&truncate_chars(args_json, 100))
        );
        let tail = one_line(&truncate_chars(outcome_text, 200));
        let mut lines = self
            .shared
            .lines
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        lines.push(format!("{head} → {tail}"));
    }

    /// 쌓인 기록 줄을 한 줄씩 이어 붙임.
    pub(in crate::tools) fn text(&self) -> String {
        let lines = self
            .shared
            .lines
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        lines.join("\n")
    }

    /// 기록 줄이 하나라도 있는지 확인.
    pub(in crate::tools) fn is_empty(&self) -> bool {
        let lines = self
            .shared
            .lines
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        lines.is_empty()
    }
}
