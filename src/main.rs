pub mod pid;
use crate::pid::{Pid, PID_s};

fn main() {
    let mut pid: PID_s = Pid::init(1.0, 1.0, 1.0);
    pid.update(1.0, 0.5);
    println!("Hello, world!");
}
