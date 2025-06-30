use std::env::args;

fn main() {
    println!("This is a Rust program.");
    println!(
        "Here are my arguments as reported by std::env::args. {:?}",
        args()
    );
    println!(
        "On Linux, the first item printed is the first argument on the command line used to run me."
    )
}
