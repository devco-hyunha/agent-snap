//! T3 모델 연산 시험 묶음. 책임을 파일별로 나눈다. 시험 재료는 엔드포인트 연산 시험과 같은 파일을 재사용.
mod add;
mod comment;
mod hidden;
mod keep;
#[path = "../endpoint_ops/materials.rs"]
mod materials;
mod notfound;
mod reject;
mod remove;
mod update;
