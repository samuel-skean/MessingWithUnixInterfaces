use std::time::Duration;

use libc::{fork, pid_t, setpgid};

pub enum DesiredPgrpState {
    NonLeaderMemberOf { pgid: pid_t },
    Leader,
}

impl DesiredPgrpState {
    pub fn non_leader_member_of(pgid: pid_t) -> Self {
        assert!(pgid > 0);
        Self::NonLeaderMemberOf { pgid: pgid }
    }
}

pub fn spawn_child_in_pgrp(
    pgrp_state: DesiredPgrpState,
    spawn_shortlived_grandchild: bool,
) -> pid_t {
    let pgid_arg = match pgrp_state {
        DesiredPgrpState::NonLeaderMemberOf { pgid } => pgid,
        DesiredPgrpState::Leader => 0,
    };
    let forkret = unsafe { fork() };

    if forkret == 0 {
        // Child
        unsafe { setpgid(0, pgid_arg) };

        if spawn_shortlived_grandchild {
            let forkret = unsafe { fork() };
            if forkret == 0 {
                // Grandchild

                // Grandchild exits fast with a failure code, not that it will be observed by the parent.
                std::process::exit(1);
            }
            assert!(forkret > 0);
        }
        std::thread::sleep(Duration::from_secs(2));
        std::process::exit(0);
    }
    // Parent

    assert!(forkret > 0);

    let child_pid = forkret;
    unsafe { setpgid(child_pid, pgid_arg) };

    child_pid
}
