use std::{fs::File, os::fd::FromRawFd, process::ExitCode};

fn main() -> ExitCode {
    let mut stat_buf = [0u8; 0x1000]; // If this isn't big enough I'll lose my mind.
    if unsafe { libc::fstat(3, &raw mut stat_buf as *mut libc::stat) } == -1 {
        eprintln!("This program is intended to be started with a regular file opened at file descriptor 0.\
                   No file was opened at file descriptor 0.\
                   As of now, this program does not check the type of the file.");
        return ExitCode::FAILURE;
    }

    let file_data = unsafe { memmap2::Mmap::map(&File::from_raw_fd(3)) }.unwrap();

    unsafe {
        libc::write(1, file_data.as_ptr() as *const libc::c_void, file_data.len());
    }
    return ExitCode::SUCCESS;
}
