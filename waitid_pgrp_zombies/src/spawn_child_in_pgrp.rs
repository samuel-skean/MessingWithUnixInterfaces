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

pub fn spawn_child_in_pgrp(pgrp_state: DesiredPgrpState) -> pid_t {
    let pgid_arg = match pgrp_state {
        DesiredPgrpState::NonLeaderMemberOf { pgid } => pgid,
        DesiredPgrpState::Leader => 0,
    };
    let forkret = unsafe { fork() };

    if forkret == 0 {
        // Child 1
        unsafe { setpgid(0, pgid_arg) };

        std::process::exit(0);
    }
    // Parent

    let child_pid = forkret;
    unsafe { setpgid(child_pid, pgid_arg) };

    child_pid
}
