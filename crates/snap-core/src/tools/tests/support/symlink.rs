//! OS 관례로 심볼릭 링크를 만드는 한 자리(두 운영체제 동등)

use std::path::Path;

#[must_use]
pub fn make_symlink(target: &Path, link: &Path, is_dir: bool) -> bool {
    #[cfg(windows)]
    let made = {
        // Windows는 폴더·파일 링크를 따로 만들고 권한이 없으면 실패. `is_dir` 값으로 종류를 정함
        let old = std::fs::remove_dir_all(link).or_else(|_| std::fs::remove_file(link));
        let _ = old;
        let dir_result = std::os::windows::fs::symlink_dir(target, link);
        if is_dir && dir_result.is_ok() {
            return true;
        }
        let _ = std::fs::remove_file(link);
        std::os::windows::fs::symlink_file(target, link).is_ok()
    };
    #[cfg(unix)]
    let made = {
        let _ = std::fs::remove_file(link);
        let _ = is_dir;
        std::os::unix::fs::symlink(target, link).is_ok()
    };
    made
}
