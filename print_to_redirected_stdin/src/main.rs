use std::os::fd::BorrowedFd;
use std::process::ExitCode;

fn main() -> ExitCode {
    const MSG: &'static [u8] = b"I sure hope this can't get printed to something I opened with input redirection in a reasonable shell like bash.\n";
    let Err(errno) = nix::unistd::write(unsafe { BorrowedFd::borrow_raw(0) }, MSG) else {
        eprintln!("Either something is quite weird about your shell, or you invoked me without input redirection, cause I just successfully wrote a string to standard in.");
        return ExitCode::FAILURE;
    };
    assert_eq!(errno, nix::errno::Errno::EBADF);
    eprintln!("All is hunky-dory! I failed to write to the file with errno: {errno}.");
    return ExitCode::SUCCESS;
}

