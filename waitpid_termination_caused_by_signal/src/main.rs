use libc::{WEXITSTATUS, WIFEXITED, WIFSIGNALED, execvp, fork, perror, waitpid};

// Note: This program is *very unfinished*. To use it to perform the experiment it's meant to perform, you must run it in the background, discover the pid of cat that it spawns (I use `pstree -p $$`), send a SIGINT to the process running cat that it spawns (`kill -INT <pid of cat>`), and then foreground it (`fg`).
// That sentence is a little sloppy with the antecedents of 'it' - sometimes 'it' is a program, sometimes 'it' is a job (pgroup?). I used bash for this experiment.

// Purpose + Outstanding Question: What's bash doing to get $? when a process was killed by a signal? `man 2 waitpid` says there's one thing it could be doing that'd be UB (well, not something defined to be reliable by the standard).
// Intermediate answer: I was totally wrong! Don't know right now how bash/dash/zsh get 130 when a process was killed by SIGINT. See comment by the final assert.
// TODONOW: Likely for a different file in this repo: Does waiting on a pgroup wait on descendents or just children in that pgroup?
//   I think if it waits on descendents, that can create hella races otherwise avoided by the existance of the zombie phase in a process's life cycle.
fn main() {
    let forkret = unsafe { fork() };
    if forkret == 0 {
        // Child
        let argv = [c"cat".as_ptr(), std::ptr::null_mut()];
        let execve_return = unsafe { execvp(c"cat".as_ptr(), &raw const argv as _) };
        assert_eq!(execve_return, -1); // If *this* fails I'll eat my hat.
        unsafe {
            perror(b"After execve?!" as _);
        }
        unreachable!();
    }

    let mut wait_status = 0x7E7E7E; // Probably not the best, but it's uninitialized, and I know this pattern is sometimes used for that.
    assert!(unsafe { waitpid(forkret, &raw mut wait_status, 0) } > 0);
    println!("The wait status was {:#010X}", wait_status); // I'm really not sure why this syntax works. I cribbed it from here: https://stackoverflow.com/questions/48972370/hexadecimal-formatting-with-padded-zeroes
    assert_eq!(WIFEXITED(wait_status), false);
    assert_eq!(WIFSIGNALED(wait_status), true);

    // According to [this statement on `man 2 wait`] (https://man7.org/linux/man-pages/man2/wait.2.html#RETURN_VALUE:~:text=This%20macro%0A%20%20%20%20%20%20%20%20%20%20%20%20%20%20should%20be%20employed%20only%20if%20WIFEXITED%20returned%20true.) `WEXITSTATUS` should not be employed unless `WIFEXITED` evaluated to true. As established above, it evaluated to false. When writing this test, I had naively thought `WEXITSTATUS` would evaluate to 130 in this situation, the same value that ends up in `$?` in bash/dash/zsh after you interrupt the last command with SIGINT (and that command doesn't handle SIGINT), but it doesn't! So that's good! TODO: Where's that 130 coming from?
    assert_ne!(WEXITSTATUS(wait_status), 130);
}
