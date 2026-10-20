pub mod pid;
use crate::pid::Pid;

struct Motor {
    pid: Pid,
    rpm: f32,
}

fn main() {
    // let mut motors: [Motor; 4] = [{Pid::new(1.0, 1.0, 1.0, 1000000.0, 100.0), 0.0}, 3];
    let mut motors = Motor {
        pid: Pid::new(1.0, 1.0, 1.0, 1000000.0, 100.0),
        rpm: 0.0,
    };

    loop {
    }
}
