//! 03-8 execute 브리지: 03-8 기록 줄을 쌓는 창구만 둡니다(AC 3: `[execute][i/total]` 형식).
//!
//! 결론: 03-2 도구 호출 기록(`logs`) 자리에 붙이는 03-8 전용 줄이고, `execute` 결과 문자열의
//! "기록" 칸에 담아 기록에 남깁니다. 엔진이 쓰는 별도 자리에서 기록을 채워도 되게
//! 내부를 스레드-safe하게 묶었습니다. 줄 글자 규칙은 `text_parts.rs`를 씁니다.

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use super::text_parts::one_line;
use super::text_parts::truncate_chars;

/// 03-8 기록 창구입니다(AC 3: `[execute][i/total]` 형식).
#[derive(Clone, Default)]
pub(in crate::tools) struct ScriptLog {
    shared: Arc<Inner>,
}

/// 기록 창구 안의 값입니다.
#[derive(Default)]
struct Inner {
    lines: Mutex<Vec<String>>,
    index: AtomicUsize,
}

impl ScriptLog {
    /// 03-8 기록 한 줄을 만들어 쌓습니다(spec AC 3).
    ///
    /// 기록 칸 `[i/total]`의 `total`은 그 자리까지의 누적 개수입니다(총 횟수를 실행 전에 알 수 없는 자리).
    /// 인자 요약은 앞 100자, 결과 요약은 앞 200자, 한 줄 규칙에 맞게 줄바꿈은 공백으로 고칩니다.
    /// 같은 규칙의 줄이 이어지며 총을 보여 줍니다.
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

    /// 쌓인 기록 줄을 한 줄씩 이어 붙입니다.
    pub(in crate::tools) fn text(&self) -> String {
        let lines = self
            .shared
            .lines
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        lines.join("\n")
    }

    /// 기록 줄이 하나라도 있는지 확인합니다(AC 3: 없으면 기록 칸을 생략).
    pub(in crate::tools) fn is_empty(&self) -> bool {
        let lines = self
            .shared
            .lines
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        lines.is_empty()
    }
}
