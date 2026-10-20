use core::f32;

pub struct Pid {
    pub kp: f32,
    pub ki: f32,
    pub kd: f32,

    pub i_max: f32,
    pub rate_max:f32,

    i: f32,
    prev_error: f32,
    output: f32,
}

impl Pid {
    pub fn new(kp: f32, ki: f32, kd: f32, i_max: f32, rate_max: f32) -> Self {
        Pid {
            kp,
            ki,
            kd,
            i_max,
            rate_max,
            i: 0.0,
            prev_error: 0.0,
            output: 0.0
        }
    }

    pub fn update(&mut self, set_point: f32, value: f32) -> f32 {
        let error = set_point - value;

        let p = self.kp * error;

        self.i += self.ki * error;
        self.i = clamp(self.i, -self.i_max, self.i_max);

        let d = self.kd * (error - self.prev_error);
        self.prev_error = error;

        self.output = p + self.i + d;
        self.output = clamp(self.output, -self.rate_max, self.rate_max);

        self.output
    }

    pub fn output(&mut self) -> f32 {
        self.output
    }

    pub fn reset(&mut self) {
        self.i = 0.0;
        self.prev_error = 0.0;
        self.output = 0.0;
    }
}

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        return min
    } else if value > max {
        return max
    }
    return value
}

