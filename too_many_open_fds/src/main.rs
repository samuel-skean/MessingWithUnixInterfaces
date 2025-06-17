use libc::{RLIMIT_NOFILE, getrlimit, rlimit};

fn get_open_fds_limits() -> rlimit {
    let mut open_fds_limits = rlimit {
        rlim_cur: 0,
        rlim_max: 0,
    };

    let getrlimit_ret = unsafe { getrlimit(RLIMIT_NOFILE, &raw mut open_fds_limits) };
    assert_eq!(getrlimit_ret, 0);

    open_fds_limits
}

fn main() {
    let open_fds_limits = get_open_fds_limits();
    println!("Here's the limits on the number of open fdss I'm allowed to have open:");

    println!(
        "Soft limit (I can change this!): {}",
        open_fds_limits.rlim_cur
    );
    println!(
        "Hard limit (I likely can't change this): {}",
        open_fds_limits.rlim_max
    );

    println!("Check back later for me to test these :)!");
}
