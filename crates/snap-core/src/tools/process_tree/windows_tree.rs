//! Job Object 핸들: Windows에서 셸 명령과 그 자식·손자를 한 번에 끝내는 창구(03-7).
//!
//! 결론: Job Object를 만들어 닫으면 트리가 끝나게 하고(`KILL_ON_JOB_CLOSE`), 셸을
//! 시작하기 전에 Job Object를 만들어 두고 셸을 묶음에 넣는다.
//! 이 창구는 D22가 정한 "자식 정리"이고, AppContainer 같은 권한 격리(D9)와 목적이 다르다
//! (D22 2.절 35행·76행).
//!
//! MSDN: Job Object에 할당된 프로세스와 그 하위 트리는 Job Object가 종료될 때 함께 종료된다
//! (Windows 8 이상). 실측 조사의 근거로 이 문서를 사용한다.
//!
//! Job Object는 `CREATE_BREAKAWAY_FROM_JOB` 옵션으로 자식이 묶음에서 빠질 수 있다.
//! MSDN(Job Objects 개요) 기준 묶음에서 나가려면 Job에 `JOB_OBJECT_LIMIT_BREAKAWAY_OK`·
//! `JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK` 확장을 걸어야 한다. 우리 Job Object는 두 확장을
//! 걸지 않는다 → Windows에서는 묶음 밖 경로가 막힌다. 묶음 밖 경로 표시는 Unix(`setsid`)
//! 시험으로 한다(spec 동작 규칙 5).

use std::io;
use std::ptr::null;

use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::JobObjects::{
    AssignProcessToJobObject, CreateJobObjectW, JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE,
    JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JobObjectExtendedLimitInformation,
    SetInformationJobObject, TerminateJobObject,
};
use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_SET_QUOTA, PROCESS_TERMINATE};

/// Job Object 핸들. 프로세스를 담고 닫으면 그 자식과 그 자식의 자식까지 한 번에 끝난다.
pub struct JobHandle(HANDLE);

// `unsafe impl Send`: Win32 핸들(`*mut c_void`)은 스레드를 넘어 옮길 수 있는 OS 자원이면
// OS가 이동을 보장한다. 우리 쪽 접근은 이 파일 안의 `self.0`을 쓰는 함수로만 열려 있다
// (안전 창구 한 자리).
unsafe impl Send for JobHandle {}

impl JobHandle {
    /// 셸을 시작하기 전에 Job Object를 만들고 `KILL_ON_JOB_CLOSE` 종료 조건을 건다(AC 2).
    ///
    /// 핸들만 닫혀도 트리가 끝나게 하는 조건이라 종료 시점(AC 2)이 이 조건에서 나온다.
    pub fn create() -> io::Result<Self> {
        let job = unsafe { CreateJobObjectW(null(), null()) };
        if job == INVALID_HANDLE_VALUE {
            return Err(io::Error::last_os_error());
        }
        let owner = Self(job);
        owner.enable_kill_on_close()?;
        Ok(owner)
    }

    /// 셸 pid를 묶음에 넣는다.
    ///
    /// `unsafe`(OS 핸들을 직접 다루는 OS API 창구라 안전을 caller가 맞춰야 한다)이며,
    /// 셸을 만든 직후 이 함수를 부르는 순서가 손자 상속의 조건이다. 할당 뒤 셸이 만든 자식은
    /// Job Object를 상속한다(MSDN).
    pub fn assign(&self, pid: Option<u32>) -> io::Result<()> {
        // tokio가 pid를 주지 않는 OS에서는 묶음에 넣을 수 없으므로 오류로 먼저 돌려준다.
        let Some(pid) = pid else {
            return Err(io::Error::other("셸 프로세스 번호가 없습니다"));
        };
        // `unsafe`(OpenProcess로 연 핸들을 AssignProcessToJobObject에 그대로 넘기는 자리).
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

    /// Job Object 트리를 한 번에 끝낸다(deadline 만료·cancel·프로그램 종료, spec AC 1·AC 2).
    ///
    /// 트리가 끝나지 않은 경우만 OS 오류로 돌려준다(종료 요청 자체의 실패다).
    pub fn terminate(&self) -> io::Result<()> {
        let ok = unsafe { TerminateJobObject(self.0, 0) };
        if ok == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }

    /// Job Object에 `KILL_ON_JOB_CLOSE` 종료 조건을 건다(AC 2).
    fn enable_kill_on_close(&self) -> io::Result<()> {
        // `unsafe`(OS 구조 정보를 OS API에 넘기는 자리, zeroed 구조를 즉시 채워 쓴다).
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
    /// 핸들을 닫는 자리(프로그램 종료, spec AC 2, 동작 규칙 3).
    ///
    /// `KILL_ON_JOB_CLOSE` 때문에 셸과 트리가 함께 종료되고, Job Object가 남기지 않는다.
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0) };
    }
}
