//! 02-3 디코더 실패 후 재호출 계약(D28) 단위 테스트입니다.

use snap_core::llm::StreamDecoder;

// AC 1(D28): 디코더 인스턴스는 한 번 `Err`를 낸 뒤에도 성공으로 보일 `Ok`를 내지 않는다.
#[test]
fn decoder_gives_no_success_events_after_first_error() {
    let mut decoder = StreamDecoder::new();
    let first = decoder.push(b"data: {broken\n\n");
    assert!(first.is_err(), "첫 조각은 실패");
    // 같은 인스턴스에 정상 데이터를 다시 넣어도 이벤트가 나오면 안 됩니다(성공 오해 금지).
    let after = decoder
        .push("data: {\"choices\":[{\"delta\":{\"content\":\"뒤\"}}]}\n\n".as_bytes())
        .expect("둘째는 빈 성공만");
    assert!(after.is_empty(), "실패 뒤 이벤트가 없어야 합니다");
    let (events, _assembled) = decoder.finish().expect("마무리도 빈 성공");
    assert!(events.is_empty(), "마무리 후에도 이벤트가 없어야 합니다");
}
