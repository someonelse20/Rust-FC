pub trait Pid {
    fn init(p: f32, i: f32, d: f32) -> Self;
    fn update(&mut self, target: f32, error: f32);
}

pub struct PID_s {
    p_gain: f32,
    i_gain: f32,
    d_gain: f32,

    proportional: f32,
    integral: f32,
    derivative: f32,
}

impl Pid for PID_s {
    fn init(p: f32, i: f32, d: f32) -> Self {
        PID_s {
            p_gain: p,
            i_gain: i,
            d_gain: d,
            proportional: 1.0,
            integral: 0.0,
            derivative: 0.0,
        }
    }

    fn update(&mut self, target: f32, error: f32) {

    }
}
