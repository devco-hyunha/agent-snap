//! 시험용 임시 폴더의 파일 쓰기·읽기 재료
//!
//! 임시 폴더 아래 파일을 쓰거나 읽는 조작만 담음

use std::path::PathBuf;

use super::temp_folder::TempFolder;

impl TempFolder {
    /// 임시 폴더 아래에 텍스트 파일을 쓰고 그 경로 반환(부모 폴더도 함께 생성)
    pub fn write_file(&self, name: &str, text: &str) -> PathBuf {
        self.write_bytes(name, text.as_bytes())
    }

    /// 임시 폴더 아래에 바이트 그대로 파일을 쓰고 그 경로 반환
    pub fn write_bytes(&self, name: &str, bytes: &[u8]) -> PathBuf {
        let target = self.path.join(name);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).expect("임시 폴더 아래 폴더를 만들 수 있어야 합니다");
        }
        std::fs::write(&target, bytes).expect("임시 파일을 쓸 수 있어야 합니다");
        target
    }

    /// 임시 폴더 아래 파일을 읽어 반환
    pub fn read_file(&self, name: &str) -> String {
        std::fs::read_to_string(self.path.join(name))
            .unwrap_or_else(|error| panic!("임시 파일을 읽지 못했습니다: {error}"))
    }
}
