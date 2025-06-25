use libc::{SIGINT, WEXITSTATUS, WIFEXITED, WIFSIGNALED, WTERMSIG, fork, raise, waitpid};

#[cfg(target_os = "linux")]
use libc::{SYS_wait4, rusage, syscall};

// Purpose/Initial Motivating Question: What's bash doing to get $? when a process was killed by a signal? `man 2 waitpid` says there's one thing it could be doing that'd be unreliable (using WEXITSTATUS when WIFEXITED was false).
// RESULT: bash (at least) just adds 128 to the number of the signal, obtained in some normal way (I haven't looked in the sources, but I bet it's `WIFSIGNALED` and `WIFEXITED`). This creates a collision with lots of otherwise valid (failure) return codes :(. Documented here: https://www.gnu.org/software/bash/manual/bash.html#Exit-Status
// I *bet* dash, zsh, and fish also do this. I haven't looked at their docs. POSIX does not insist that the exit status of a process terminated by a signal be 128+<signum>, but it must be greater than 128 (from [IEEE 1003.1-2024](https://ieeexplore.ieee.org/document/10555529), sign-in required, but freely accessible to UIC students).

fn main() {
    // NOTE: This technique for isolating the choice of syscall to use relies heavily on the fact that raw syscalls (like `SYS_wait4`) and glibc wrappers (like `waitpid`) both return negative numbers to indicate error. Thankfully, this is a pretty common convention.
    // What's less common is that we don't care at all about what the error condition was. That's what really makes this hack work.
    #[cfg(target_os = "linux")]
    {
        println!("Checking the raw syscall wait4:");
        println!();
        do_check_with_waiting_func(|child_pid, wait_status_addr| unsafe {
            syscall(
                SYS_wait4,
                child_pid,
                wait_status_addr,
                0,
                std::ptr::null_mut::<rusage>(),
            )
        });
    }
    #[cfg(not(target_os = "linux"))]
    {
        println!("This program wasn't compiled for Linux, so it doesn't check `SYS_wait4`.");
    }
    println!();

    println!("Checking the glibc wrapper `waitpid`:");
    println!();
    do_check_with_waiting_func(|child_pid, wait_status_addr| unsafe {
        waitpid(child_pid, wait_status_addr, 0) as i64
    });
}

fn do_check_with_waiting_func(waiting_func: impl Fn(libc::pid_t, *mut i32) -> i64) {
    // TODO: It would be interesting to iterate through the signals and actually compare this behavior to the behavior of any relevant Unix(ish? is that caveat necessary?) shells. e.g. by executing bash/dash/zsh/fish/nushell to run a command that terminates with a signal and comparing their actual exit statuses (obtained with `WEXITSTATUS`) to 128+<signum> for running those commands ourselves. That would require either finding a command that reliably kills itself with a signal or writing a program to cause a shell to invoke itself.
    // Both cool, but more than I want to do right now.
    let forkret = unsafe { fork() };
    if forkret == 0 {
        // Child
        assert_eq!(unsafe { raise(SIGINT) }, 0);
    }

    // Parent

    let child_pid = forkret;

    let mut wait_status = 0x7E7E7E; // Probably not the best, but it's uninitialized, and I know this pattern is sometimes used for that.
    assert!(waiting_func(child_pid, &raw mut wait_status) > 0);
    println!("The wait status was {:#010X}", wait_status); // I'm really not sure why this syntax works. I cribbed it from here: https://stackoverflow.com/questions/48972370/hexadecimal-formatting-with-padded-zeroes
    assert_eq!(WIFEXITED(wait_status), false);
    assert_eq!(WIFSIGNALED(wait_status), true);
    assert_eq!(WTERMSIG(wait_status), SIGINT);

    let expected_shell_exit_status = 128 + SIGINT;

    // According to [this statement on `man 2 wait`] (https://man7.org/linux/man-pages/man2/wait.2.html#RETURN_VALUE:~:text=This%20macro%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20should%20be%20employed%20only%20if%20WIFEXITED%20returned%20true.) `WEXITSTATUS` should not be employed unless `WIFEXITED` evaluated to true. As established above, it evaluated to false. When writing this test, I had naively thought `WEXITSTATUS` would evaluate to 130 in this situation, the same value that ends up in `$?` in bash/dash/zsh after you interrupt the last command with SIGINT (and that command doesn't handle SIGINT), but it doesn't! So that's good!
    println!(
        "The results of this macro should not be used: WEXITSTATUS(wait_status) = {:#010X}",
        WEXITSTATUS(wait_status)
    );
    assert_ne!(WEXITSTATUS(wait_status), expected_shell_exit_status); // Not guaranteed by any standard, but interesting if it breaks!
    assert_eq!(WEXITSTATUS(wait_status), 0); // Ditto above comment.

    assert_eq!(128 + WTERMSIG(wait_status), expected_shell_exit_status);
}
