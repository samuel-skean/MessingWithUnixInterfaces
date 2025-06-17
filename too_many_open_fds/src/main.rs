use libc::{RLIMIT_NOFILE, getrlimit64, rlimit64};

fn get_open_file_limits() -> rlimit64 {
    let mut open_file_limits = rlimit64 {
        rlim_cur: 0,
        rlim_max: 0,
    };

    let getrlimit64_ret = unsafe { getrlimit64(RLIMIT_NOFILE, &raw mut open_file_limits) };
    assert_eq!(getrlimit64_ret, 0);

    open_file_limits
}
fn main() {
    let open_file_limits = get_open_file_limits();
    println!("Here's the limits on the number of open files I'm allowed to have open:");

    println!(
        "Soft limit (I can change this!): {}",
        open_file_limits.rlim_cur
    );
    println!(
        "Hard limit (I likely can't change this): {}",
        open_file_limits.rlim_max
    );

    println!("Check back later for me to test these :)!");
}
