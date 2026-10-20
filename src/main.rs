use crate::pid::pid_mod::init_pid;

pub mod pid;

fn main() {
    init_pid(1.0, 1.0, 1.0);

    println!("Hello, world!");
}
