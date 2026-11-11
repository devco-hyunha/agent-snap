//! `AGENTS.md` 읽기 상태와 조립 결과 시험(spec AC 1·AC 2·AC 3·AC 4).
//!
//! 임시 폴더는 도구 시험 지원부 `TempFolder`를 그대로 재사용하고(DRY), 읽기 상태 3종과
//! 상한 초과 표시, 조립 순서를 문자열로 비교합니다.

use snap_core::agent::{
    AGENTS_MD_LONG_NOTICE, AgentsMd, assemble_messages, read_agents_md, read_agents_md_default,
};

use super::support::TempFolder;

/// AC 1·AC 3: 파일이 있으면 원문을 그대로 읽고, 없는 상태·빈 파일·읽기 실패는 지시 없음으로 봅니다.
#[test]
fn read_states_cover_missing_and_empty_and_failure() {
    let folder = TempFolder::new("agents-md-states");
    // 파일이 없는 상태: 읽기 전에 폴더만 만들어 둡니다.
    assert_eq!(
        read_agents_md_default(folder.path()),
        AgentsMd::Missing,
        "파일이 없으면 지시 없는 상태입니다"
    );

    folder.write_file("AGENTS.md", "규칙 원문 첫째 줄\n둘째 줄");
    let loaded = read_agents_md_default(folder.path());
    assert!(
        matches!(&loaded, AgentsMd::Loaded { text, has_length_notice: false } if text == "규칙 원문 첫째 줄\n둘째 줄"),
        "읽은 상태는 원문 그대로여야 합니다"
    );

    folder.write_file("AGENTS.md", "   \n\t ");
    assert_eq!(read_agents_md_default(folder.path()), AgentsMd::Empty);

    // 읽기 실패: 지시 파일 이름을 폴더로 만들어 버리면 텍스트로 읽을 수 없습니다.
    std::fs::remove_file(folder.path().join("AGENTS.md")).expect("지시 파일을 지워야 합니다");
    std::fs::create_dir(folder.path().join("AGENTS.md")).expect("같은 이름 폴더를 만들어야 합니다");
    let failed = read_agents_md_default(folder.path());
    assert!(
        matches!(&failed, AgentsMd::ReadFailed { reason } if !reason.is_empty()),
        "읽기 실패는 이유를 남깁니다"
    );
}

/// AC 4: 상한을 넘으면 원문은 그대로 두고 끝에 표시 한 줄만 붙임을 문자열로 확인합니다.
#[test]
fn length_notice_appends_one_line_without_trimming() {
    let folder = TempFolder::new("agents-md-length");
    let text = "길게 ".repeat(64); // 4,096바이트를 넘는 원문입니다.
    folder.write_file("AGENTS.md", &text);
    let loaded = read_agents_md(folder.path(), 64);
    let directives = loaded.directives();
    assert!(
        directives.starts_with(&text),
        "원문을 잘라 넣지 않고 그대로 담아야 합니다"
    );
    assert!(
        directives.ends_with(AGENTS_MD_LONG_NOTICE),
        "표시 한 줄이 끝에 붙어야 합니다"
    );
    assert_eq!(
        directives.lines().count(),
        text.lines().count() + 1,
        "표시가 여러 줄을 만들지 않습니다"
    );

    // 기본 상한(32 KiB) 안에서는 표시가 붙지 않습니다.
    let under_limit = read_agents_md_default(folder.path());
    assert_eq!(under_limit.directives(), text, "표시 없이 원문만 남습니다");
}

/// AC 1: 조립 순서가 규범 앞머리 → AGENTS.md 원문으로 고정됨을 문자열 비교로 확인합니다.
#[test]
fn assemble_orders_norm_then_agents_md() {
    let folder = TempFolder::new("agents-md-assemble");
    folder.write_file("AGENTS.md", "규칙 원문");
    let directives = read_agents_md_default(folder.path()).directives();
    let messages = assemble_messages("규범 앞머리", &directives, "과제 한 장");
    assert_eq!(messages[0].role, "system");
    assert_eq!(messages[0].content, "규범 앞머리\n규칙 원문");
    assert_eq!(messages[1].role, "user");

    // AC 2·AC 3: 읽지 않은 상태는 지시 없이 앞머리만 조립됩니다(루프는 계속 돕니다).
    let missing_directives = AgentsMd::Missing.directives();
    let missing = assemble_messages("규범 앞머리", &missing_directives, "과제 한 장");
    assert_eq!(missing[0].content, "규범 앞머리");
    let failed_directives = AgentsMd::ReadFailed {
        reason: "권한 없음".to_owned(),
    }
    .directives();
    let failed = assemble_messages("규범 앞머리", &failed_directives, "과제 한 장");
    assert_eq!(failed[0].content, "규범 앞머리");
}
