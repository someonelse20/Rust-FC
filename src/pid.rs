pub mod pid_mod {
    pub struct PID {
        p_gain: f32,
        i_gain: f32,
        d_gain: f32,

        proportional: f32,
        integral: f32,
        derivative: f32,
    }

    pub fn init_pid(p_gain: f32, i_gain: f32, d_gain: f32) -> PID {
        PID {
            p_gain: p_gain,
            i_gain: i_gain,
            d_gain: d_gain,

            proportional: 1.0,
            integral: 0.0,
            derivative: 0.0,
        }
    }

    pub fn update_pid() {}
}
