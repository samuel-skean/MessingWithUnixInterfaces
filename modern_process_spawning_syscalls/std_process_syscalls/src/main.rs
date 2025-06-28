fn main() {
    let mut p = std::process::Command::new("/usr/bin/true");
    p.spawn().unwrap();
}
