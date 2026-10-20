use core::f32;

/// PID Controller for flight control loops
///
/// Provides proportional-integral-derivative control with anti-windup protection
/// and configurable rate limiting for actuator saturation handling.
pub struct Pid {
    /// Proportional gain
    pub kp: f32,
    /// Integral gain
    pub ki: f32,
    /// Derivative gain
    pub kd: f32,

    /// Integral anti-windup clamp (positive/negative saturation limits)
    pub i_max: f32,

    /// Output rate limit
    pub max_rate: f32,

    /// Previous error for derivative calculation
    prev_error: f32,

    /// Accumulated integral term
    integral: f32,

    /// Control output
    output: f32,
}

impl Pid {
    /// Create a new PID controller with default settings
    pub fn new(kp: f32, ki: f32, kd: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            i_max: 100.0,
            max_rate: 100.0,
            prev_error: 0.0,
            integral: 0.0,
            output: 0.0,
        }
    }

    /// Create a new PID controller with anti-windup and rate limiting
    pub fn with_limits(kp: f32, ki: f32, kd: f32, i_max: f32, max_rate: f32) -> Self {
        Self {
            kp,
            ki,
            kd,
            i_max,
            max_rate,
            prev_error: 0.0,
            integral: 0.0,
            output: 0.0,
        }
    }

    /// Compute control output from setpoint and measurement
    ///
    /// # Arguments
    /// - `setpoint`: Desired value
    /// - `measurement`: Current measured value
    ///
    /// # Returns
    /// New control output value
    pub fn compute(&mut self, setpoint: f32, measurement: f32) -> f32 {
        let error = setpoint - measurement;

        // Proportional term
        let p = self.kp * error;

        // Integral term with anti-windup
        self.integral += self.ki * error;

        // Clamp integral to prevent windup
        if self.integral > self.i_max {
            self.integral = self.i_max;
        } else if self.integral < -self.i_max {
            self.integral = -self.i_max;
        }

        let i = self.integral;

        // Derivative term
        let d = self.kd * (error - self.prev_error);
        self.prev_error = error;

        let d = d * 0.1; // Limit derivative spike

        // Control output
        let output = p + i + d;

        // Rate limiting
        if output > self.output + self.max_rate {
            self.output = self.output + self.max_rate;
        } else if output < self.output - self.max_rate {
            self.output = self.output - self.max_rate;
        } else {
            self.output = output;
        }

        self.output
    }

    /// Reset controller state
    pub fn reset(&mut self) {
        self.integral = 0.0;
        self.prev_error = 0.0;
        self.output = 0.0;
    }

    /// Set anti-windup limit
    pub fn set_i_max(&mut self, limit: f32) {
        self.i_max = limit;
    }

    /// Set output rate limit
    pub fn set_max_rate(&mut self, rate: f32) {
        self.max_rate = rate;
    }

    /// Get current integral value
    pub fn integral(&self) -> f32 {
        self.integral
    }

    /// Get current output
    pub fn output(&self) -> f32 {
        self.output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pid_basic() {
        let mut pid = Pid::new(1.0, 0.1, 0.01);

        // Step response test
        let mut error = 0.0;
        for _ in 0..100 {
            error = pid.compute(100.0, 0.0);
        }

        assert!(error > 0.0);
    }

    #[test]
    fn test_pid_anti_windup() {
        let mut pid = Pid::new(1.0, 1.0, 0.0);
        pid.i_max = 10.0;

        // Drive integral past limit
        for _ in 0..200 {
            pid.compute(100.0, 0.0);
        }

        let output = pid.compute(0.0, 100.0);
        assert!(output <= 10.0);
    }
}
