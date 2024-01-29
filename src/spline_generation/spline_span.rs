use std::{Vec};

struct SplineSpan {
    start_control_point: Vec<ControlPoint>,
    end_control_point: Vec<ControlPoint>,
    start_velocity_vector: Vector2D,
    end_velocity_vector: Vector2D,
    coefficients: Vec<Coefficient>,
    velocity_coefficients: Vec<Coefficient>
}

impl SplineSpan {
    fn new(start_control_point: ControlPoint, end_control_point: controlPoint, start_velocity_vector: Vector2D, end_velocity_vector: Vector2D) -> SplineSpan {
        let coefficients = get_coefficients();
        let velocity_coefficients = get_velocity_coefficients();
        SplineSpan {
            start_control_point,
            end_control_point,
            start_velocity_vector,
            end_velocity_vector,
            coefficients,
            velocity_coefficients
        }
    }

    /// Returns the coefficients of the spline span
    fn get_coefficients() -> Vec<Coefficient> {
        let mut coefficients = Vec::new();

        // TODO find the coefficients

        // Use Matrix.solve() to find the coefficients
    }


    /// Returns the coefficients of the derivative of the given coefficients
    fn get_derivative_coefficients(coefficients: Vec<Coefficient>) -> Vec<Coefficient> {
        let mut new_coefficients = Vec::new();

        for i in 0..coefficients.len() {
            let coefficient = coefficients[i].coefficient * coefficients[i].power;
            let power = coefficients[i].power - 1;

            if (coefficient != 0) {
                new_coefficients.push(Coefficient::new(power, coefficient));
            }
        }
    }


    /// Returns the velocity at the specific point
    pub fn get_velocity(t_value: float64) -> float64 {
        if (!is_in_range(t_value)) {
            panic!("t_value is not in the range of this spline span");
        }

        let mut velocity = 0.0;

        for i in 0..derivative_coefficients.len() {
            let coefficient = derivative_coefficients[i].coefficient;
            let power = derivative_coefficients[i].power;

            velocity += coefficient * powf(t_value, power);
        }

        // Keeps the velocity between the max and min velocity!
        let max_velocity = 10.0;
        let min_velocity = 2.0;

        if (velocity > max_velocity) {velocity = max_velocity;}
        if (velocity < min_velocity) {velocity = min_velocity;}

        return velocity;
    }

    /// Adds another row to the coefficients matrix. The first 4 terms are coefficients1, the next 4 are coefficients2,
    /// and the last term is the end value
    fn add_to_coefficients_matrix(&mut self, coefficients: Vec<f64>, coefficients1: Vec<f64>, coefficients2: Vec<f64>, end_value: float64) {
    }

    /// Returns whether the t_value is in the range of this spline span
    fn is_in_range(&self, t_value: float64) -> bool {
        return t_value >= self.start_time() && t_value <= self.end_time();
    }

    pub fn start_t_value(&self) -> float64 {
        return self.start_velocity_vector.get_x_magnitude();
    }

    pub fn end_t_value(&self) -> float64 {
        return self.end_velocity_vector.get_x_magnitude();
    }
}
