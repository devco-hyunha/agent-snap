//! 모의 서버 확인 묶음. 책임별 확인 파일(응답 조립·기록 저장·스크립트 규격·헤더 지연·본문 정지·연결 닫힘·요청 기록·연결 정리)만 봄.
mod close_after_body;
mod connection_close;
mod header_delay;
mod log_store;
mod request_record;
mod response_build;
mod script_spec;
mod stream_stop;
