pub mod pid;
use crate::pid::Pid;

fn main() {
    let mut pid = Pid::init(1.0, 1.0, 1.0);
    pid.update(1, 0.5);
    println!("Hello, world!");
}
