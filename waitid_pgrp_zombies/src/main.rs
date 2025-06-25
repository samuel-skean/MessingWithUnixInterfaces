mod spawn_child_in_pgrp;

use libc::{
    __errno_location, CLD_EXITED, ECHILD, P_PGID, SIGCHLD, WEXITED, perror, siginfo_t, waitid,
};
use spawn_child_in_pgrp::{DesiredPgrpState, spawn_child_in_pgrp};

const NUM_CHILDREN: usize = 20;

// TODO: Does waiting on a pgroup wait on descendents or just children in that pgroup?
// I think if it waits on descendents, that can create hella races otherwise avoided by the existence of the zombie phase in a process's life cycle.
fn main() {
    let mut spawned_child_pids = [None; NUM_CHILDREN];

    let leader_child_pid = spawn_child_in_pgrp(DesiredPgrpState::Leader);
    spawned_child_pids[0] = Some(leader_child_pid);

    for child_pid in spawned_child_pids[1..].iter_mut() {
        *child_pid = Some(spawn_child_in_pgrp(DesiredPgrpState::non_leader_member_of(
            leader_child_pid,
        )));
    }

    let mut successfully_waited_child_pids = [None; NUM_CHILDREN];

    // COMPLAINT: Why is this while loop pattern so *tricky* for me? Every time I have to think a bunch or write duplicate code. The duplicate code unsurprisingly causes errors.
    let mut child_info_p_unfilled = std::mem::MaybeUninit::<siginfo_t>::zeroed();
    let mut waitidret = unsafe {
        waitid(
            P_PGID,
            leader_child_pid.try_into().unwrap(),
            child_info_p_unfilled.as_mut_ptr(),
            WEXITED,
        )
    };

    let mut child_waited_idx = 0;
    while waitidret == 0 && child_waited_idx < NUM_CHILDREN {
        // SAFETY: waitid did not error
        let child_info_p = unsafe { child_info_p_unfilled.assume_init() };

        unsafe {
            println!("Fields of siginfo_t written by waitid:");
            println!("  si_pid = {}", child_info_p.si_pid());
            println!("  si_uid = {}", child_info_p.si_uid());
            println!("  si_signo = {}", child_info_p.si_signo);
            assert_eq!(child_info_p.si_signo, SIGCHLD); // Always true.
            println!("  si_status = {}", child_info_p.si_status());
            println!("  si_code = {}", child_info_p.si_code);

            // All children should exit cleanly:
            assert_eq!(child_info_p.si_code, CLD_EXITED);
            assert_eq!(child_info_p.si_status(), 0);
        }

        successfully_waited_child_pids[child_waited_idx] = Some(unsafe { child_info_p.si_pid() });
        child_waited_idx += 1;

        child_info_p_unfilled = std::mem::MaybeUninit::<siginfo_t>::zeroed();

        waitidret = unsafe {
            waitid(
                libc::P_PGID,
                leader_child_pid.try_into().unwrap(),
                child_info_p_unfilled.as_mut_ptr(),
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

    assert_eq!(child_waited_idx, NUM_CHILDREN);

    spawned_child_pids.sort_unstable();
    successfully_waited_child_pids.sort_unstable();
    assert_eq!(spawned_child_pids, successfully_waited_child_pids);
}
