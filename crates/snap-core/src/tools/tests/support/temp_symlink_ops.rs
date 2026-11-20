//! 시험용 임시 폴더의 심볼릭 링크 생성 재료
//!
//! 임시 폴더 아래에 링크를 만드는 조작만 담음

use std::path::Path;

use super::symlink::make_symlink;
use super::temp_folder::TempFolder;

impl TempFolder {
    /// 임시 폴더 아래에 디렉터리 심볼릭 링크 생성. 실패하면 `false`(권한·미지원 환경)
    pub fn try_symlink_dir(&self, target: &Path, link_name: &str) -> bool {
        let link = self.path.join(link_name);
        make_symlink(target, &link, true)
    }

    /// 임시 폴더 아래에 파일 심볼릭 링크 생성. 실패하면 `false`
    pub fn try_symlink_file(&self, target: &Path, link_name: &str) -> bool {
        let link = self.path.join(link_name);
        make_symlink(target, &link, false)
    }
}
