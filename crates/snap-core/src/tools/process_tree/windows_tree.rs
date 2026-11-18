//! Job Object 핸들: Windows에서 셸 명령과 그 자식·손자를 한 번에 끝내는 창구
//!
//! 셸 시작 전에 Job Object를 만들어 `KILL_ON_JOB_CLOSE` 조건을 걸고 셸을 묶음에 넣음 → 핸들만 닫아도 트리 종료

use std::io;
use std::ptr::null;

use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::JobObjects::{
    AssignProcessToJobObject, CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
    JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
    SetInformationJobObject, TerminateJobObject,
};
use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_SET_QUOTA, PROCESS_TERMINATE};

/// Job Object 핸들. 프로세스를 담고 닫으면 그 자식과 그 자식의 자식까지 한 번에 끝남
pub struct JobHandle(HANDLE);

// `unsafe impl Send`: Win32 핸들은 스레드를 넘어 옮길 수 있는 OS 자원
unsafe impl Send for JobHandle {}

impl JobHandle {
    /// 셸을 시작하기 전에 Job Object를 만들고 `KILL_ON_JOB_CLOSE` 종료 조건을 설정
    ///
    /// 핸들만 닫혀도 트리가 끝나게 하는 조건
    pub fn create() -> io::Result<Self> {
        let job = unsafe { CreateJobObjectW(null(), null()) };
        if job == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }
        let owner = Self(job);
        owner.enable_kill_on_close()?;
        Ok(owner)
    }

    /// 셸 pid를 묶음에 넣음
    ///
    /// 손자 상속은 셸을 만든 직후 이 함수를 부르는 순서가 조건
    pub fn assign(&self, pid: Option<u32>) -> io::Result<()> {
        // tokio가 pid를 주지 않는 OS에서는 묶음에 넣을 수 없음
        let Some(pid) = pid else {
            return Err(io::Error::other("셸 프로세스 번호가 없습니다"));
        };
        // `unsafe`(OpenProcess로 연 핸들을 AssignProcessToJobObject에 그대로 넘기는 자리)
        unsafe {
            let opened = OpenProcess(PROCESS_SET_QUOTA | PROCESS_TERMINATE, 0, pid);
            if opened == INVALID_HANDLE_VALUE {
                return Err(io::Error::last_os_error());
            }
            let ok = AssignProcessToJobObject(self.0, opened);
            CloseHandle(opened);
            if ok == 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        }
    }

    /// Job Object 트리를 한 번에 끝냄(deadline 만료·cancel·프로그램 종료)
    ///
    /// 트리가 끝나지 않은 경우만 OS 오류로 반환(종료 요청 자체의 실패)
    pub fn terminate(&self) -> io::Result<()> {
        let ok = unsafe { TerminateJobObject(self.0, 0) };
        if ok == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    /// Job Object에 `KILL_ON_JOB_CLOSE` 종료 조건을 설정
    fn enable_kill_on_close(&self) -> io::Result<()> {
        // `unsafe`(OS 구조 정보를 OS API에 넘기는 자리, zeroed 구조를 즉시 채워 사용)
        unsafe {
            let mut info: JOBOBJECT_EXTENDED_LIMIT_INFORMATION = std::mem::zeroed();
            info.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            let size = u32::try_from(std::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>())
                .map_err(|_| io::Error::other("Job Object 정보 크기가 작지 않습니다"))?;
            let ok = SetInformationJobObject(
                self.0,
                JobObjectExtendedLimitInformation,
                std::ptr::from_ref(&info).cast(),
                size,
            );
            if ok == 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        }
    }
}

impl Drop for JobHandle {
    /// 핸들을 닫는 자리(프로그램 종료)
    ///
    /// `KILL_ON_JOB_CLOSE` 때문에 셸과 트리가 함께 종료되고, Job Object가 남기지 않음
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0) };
    }
}
