//! `tool_call` 낮은 온도 프로파일을 요청에 합치는 자리
//!
//! 값 넣기는 요청 본문 조립(`llm/request_body.rs`)이 하므로 여기서는 검증과 합치기만 담당

use crate::config::{ModelIssue, SectionIssue, ValidationError};
use crate::config::{RESERVED_EXTRA_KEYS, is_valid_temperature};
use crate::llm::ChatRequest;

/// 프로파일 검증이 낸 설정 오류. 검증 규칙을 그대로 재사용하므로 그대로 넘김
pub type ProfileError = ValidationError;

/// `tool_call` 턴의 낮은 온도 프로파일. `temperature`만 받음
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ToolCallProfile {
    temperature: f64,
}

impl ToolCallProfile {
    /// 낮은 온도 값으로 프로파일 생성
    ///
    /// # 오류
    /// 범위를 벗어난 값은 `temperature` 항목 이름을 밝히고 거부
    pub fn new(temperature: f64) -> Result<Self, ProfileError> {
        if is_valid_temperature(&temperature) {
            Ok(Self { temperature })
        } else {
            Err(SectionIssue::SamplingOutOfRange {
                item: "temperature".to_owned(),
                model: "tool_call".to_owned(),
            }
            .into())
        }
    }

    /// 합치는 자리에서 덮으면 안 되는 요청 칸 목록(검사 기준, 검증 정본 재사용)
    #[must_use]
    pub const fn reserved_keys() -> [&'static str; 4] {
        RESERVED_EXTRA_KEYS
    }

    /// 요청 골격에 프로파일 온도를 합침
    ///
    /// 나머지 샘플링 항목은 골격 값 그대로 둠
    /// # 오류
    /// 예약 키를 덮으려는 자리면 `ExtraReservedKey`, 온도가 범위 밖이면 `SamplingOutOfRange`
    pub fn apply(&self, request: &mut ChatRequest) -> Result<(), ProfileError> {
        for key in Self::reserved_keys() {
            if request.extra.contains_key(key) {
                return Err(ModelIssue::ExtraReservedKey {
                    model: "tool_call".to_owned(),
                    key: (*key).to_owned(),
                }
                .into());
            }
        }
        request.sampling.temperature = Some(self.temperature);
        Ok(())
    }

    /// 프로파일의 온도 값
    #[must_use]
    pub const fn temperature(&self) -> f64 {
        self.temperature
    }
}
