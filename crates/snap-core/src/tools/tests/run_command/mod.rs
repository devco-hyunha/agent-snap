//! `run_command` 도구 시험의 승인 표식·인자·cancel·상한·출력·비밀 이름 트리를 등록하는 자리
//!
//! 셸을 쓰는 시험은 이 운영체제에서 확인이 쉬운 명령만 쓰고, 상한은 짧게 주고 경과는 하한·상한으로만 봄

#[path = "approval_gate.rs"]
mod approval_gate;
#[path = "args.rs"]
mod args;
#[path = "cancel.rs"]
mod cancel;
#[path = "deadline.rs"]
mod deadline;
#[path = "output.rs"]
mod output;
#[path = "secret_names.rs"]
mod secret_names;
