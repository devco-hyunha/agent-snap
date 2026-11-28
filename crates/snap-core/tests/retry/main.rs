//! 02-4 재시도 판단 시험 묶음. 책임별 파일(대상 판별·`Retry-After` 해석·백오프·판단 조립)과 시험 재료 정본(`fixtures.rs`)만 봄.

mod after;
mod backoff;
mod classify;
mod decide;
mod fixtures;
mod random;
mod settings;
