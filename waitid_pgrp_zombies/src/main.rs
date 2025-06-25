use libc::{
    __errno_location, CLD_EXITED, ECHILD, P_PGID, SIGCHLD, WEXITED, fork, perror, setpgid,
    siginfo_t, waitid,
};

// TODO: Does waiting on a pgroup wait on descendents or just children in that pgroup?
// I think if it waits on descendents, that can create hella races otherwise avoided by the existence of the zombie phase in a process's life cycle.
fn main() {
    let forkret = unsafe { fork() };

    if forkret == 0 {
        // Child 1
        unsafe { setpgid(0, 0) };

        return;
    }
    // Parent

    let child_1_pid = forkret;
    unsafe { setpgid(child_1_pid, 0) };

    let mut child_info_p = std::mem::MaybeUninit::<siginfo_t>::zeroed();
    let mut waitidret = unsafe {
        waitid(
            // At this point, this works with any of `libc::P_ALL`, `libc::P_PGID`, and `libc::P_PID`, since there's only one child:
            P_PGID,
            child_1_pid.try_into().unwrap(),
            child_info_p.as_mut_ptr(),
            WEXITED,
        )
    };

    while waitidret == 0 {
        // SAFETY: waitid did not error
        let child_info_p = unsafe { child_info_p.assume_init() };

        unsafe {
            println!("Fields of siginfo_t written by waitid:");
            println!("  si_pid = {}", child_info_p.si_pid());
            // TODO: Make this more flexible so the code can handle multiple children.
            assert_eq!(child_info_p.si_pid(), child_1_pid);
            println!("  si_uid = {}", child_info_p.si_uid());
            println!("  si_signo = {}", child_info_p.si_signo);
            assert_eq!(child_info_p.si_signo, SIGCHLD); // Always true.
            println!("  si_status = {}", child_info_p.si_status());
            println!("  si_code = {}", child_info_p.si_code);

            assert_eq!(child_info_p.si_code, CLD_EXITED);
            assert_eq!(child_info_p.si_status(), 0);
        }

        let mut child_info_p = std::mem::MaybeUninit::<siginfo_t>::zeroed();

        waitidret = unsafe {
            waitid(
                libc::P_PID,
                child_1_pid.try_into().unwrap(),
                child_info_p.as_mut_ptr(),
                WEXITED,
            )
        };
    }

    let waitid_errno = unsafe { *__errno_location() };
    unsafe {
        perror(c"waitid errored".as_ptr());
    }
    println!("errno = {}", waitid_errno);
    assert_eq!(waitid_errno, ECHILD);
}
