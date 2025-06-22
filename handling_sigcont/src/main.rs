use libc::*;

// You can do it! It does resume your process after running your handler!

extern "C" fn handler(_signum: c_int, _siginfo: *const siginfo_t, _ucontext: *mut c_void) {
    unsafe {
        write(1, b"hi\n" as *const u8 as *const _, 3);
    }
}

fn main() {
    let sa_mask = unsafe { std::mem::zeroed() };

    let sa = sigaction {
        sa_sigaction: handler as usize,
        sa_mask,
        sa_flags: SA_SIGINFO,
        sa_restorer: unsafe { std::mem::zeroed() },
    };
    unsafe {
        sigaction(SIGCONT, &sa, std::ptr::null_mut());
    }
    loop {
        unsafe {
            write(1, b"bye\n" as *const u8 as *const _, 4);
            std::thread::park_timeout(std::time::Duration::from_millis(500));
        }
    }
}
