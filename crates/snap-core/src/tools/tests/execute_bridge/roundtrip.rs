//! execute 브리지 왕복(읽기→가공→집계) 시험.

use super::super::support::TempWorkdir;
use super::shared::run_execute;

/// `read_file` 뒤 `grep` 호출로 결과를 가공하는 흐름(읽기 → 검색 → 집계) 왕복 확인.
#[tokio::test]
async fn readonly_script_roundtrip_returns_summary_and_log_line() {
    let workdir = TempWorkdir::new("bridge-roundtrip");
    workdir.write("notes.txt", "hello bridge");
    let text = run_execute(
        &workdir,
        "const t = await read_file({path: 'notes.txt'}); const body = t.replace(/^\\s*\\d+:\\s*/, '').trim(); const m = `words=${body.split(/\\s+/).length}`; m;",
    )
    .await;
    assert!(text.contains("words=2"), "요약: {text}");
    assert!(
        text.contains("기록: [execute][1/1] read_file("),
        "기록 줄: {text}"
    );
    assert_eq!(text.lines().count(), 2, "요약 줄과 기록 줄만: {text}");
}

/// `read_file` → `grep` 가공 흐름 왕복. grep 결과의 개수를 세어 요약에 올림.
#[tokio::test]
async fn readonly_script_grep_roundtrip_counts_from_grep_result() {
    let workdir = TempWorkdir::new("bridge-grep-roundtrip");
    workdir.write("notes.txt", "hello bridge\nhello again\n");
    let text = run_execute(
        &workdir,
        "const t = await read_file({path: 'notes.txt'}); const g = await grep({pattern: 'hello', format: 'count'}); const hits = g.split('\\n').map((l) => Number(l.split(':').pop())).reduce((a, b) => a + b, 0); `lines=${t.split('\\n').filter((l) => l.trim()).length} hits=${hits}`;",
    )
    .await;
    assert!(
        text.contains("lines=2 hits=2"),
        "read_file과 grep 결과로 센 값: {text}"
    );
    assert!(
        text.contains("[execute][1/1] read_file(") && text.contains("[execute][2/2] grep("),
        "기록 줄 2개: {text}"
    );
    assert_eq!(text.lines().count(), 3, "요약 줄과 기록 줄 2개: {text}");
}

/// 스크립트 안 중간 변수 값은 컨텍스트에 오르지 않고 요약(마지막 표현식)만 오름.
#[tokio::test]
async fn script_middle_values_stay_out_of_summary() {
    let workdir = TempWorkdir::new("bridge-summary-only");
    workdir.write("notes.txt", "abc def");
    let text = run_execute(
        &workdir,
        "const hidden = 'middle-secret'; const list = ['one','two']; const sum = hidden.length + list.length; sum;",
    )
    .await;
    assert_eq!(text.split('\n').count(), 1, "요약만: {text}");
    assert_eq!(
        text.split('\n').next().unwrap_or_default(),
        "15",
        "요약은 마지막 문장값: {text}"
    );
    assert!(!text.contains("middle-secret"), "중간 변수 제외: {text}");
}

/// 문자열이 아닌 값의 요약은 JSON 글자로만 나오고 엔진 표시가 걸리지 않음.
#[tokio::test]
async fn non_string_values_sum_as_json_text_only() {
    let workdir = TempWorkdir::new("bridge-json-text");
    let number = run_execute(&workdir, "42;").await;
    assert_eq!(
        number.split('\n').next().unwrap_or_default(),
        "42",
        "숫자 요약은 JSON 글자: {number}"
    );
    let object = run_execute(&workdir, "({count: 3});").await;
    assert_eq!(
        object.split('\n').next().unwrap_or_default(),
        "{\"count\":3}",
        "객체 요약은 JSON 글자: {object}"
    );
    let function = run_execute(&workdir, "() => {};").await;
    let function_line = function.split('\n').next().unwrap_or_default().to_owned();
    assert!(
        function_line.starts_with('"') && function_line.ends_with('"'),
        "함수 요약도 JSON 글자(큰따옴표로 감싼 글자): {function_line}"
    );
    assert!(
        !function_line.contains("Value(") && !function_line.contains("Object("),
        "엔진 표시 제외: {function_line}"
    );
}

/// 1차 허용 목록 이름 4개가 JS 배열(`SNAP_TOOLS`)로 그대로 노출됨.
#[tokio::test]
async fn readonly_tool_names_are_exposed() {
    let workdir = TempWorkdir::new("bridge-names");
    let text = run_execute(&workdir, "SNAP_TOOLS.join('|');").await;
    assert_eq!(
        text.split('\n').next().unwrap_or_default(),
        "read_file|grep|glob|list_dir",
        "노출 목록: {text}"
    );
}
