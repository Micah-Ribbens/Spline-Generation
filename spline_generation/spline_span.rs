struct SplineSpan {
    control_points: Vec<ControlPoint>,  // 3 control points
    start_velocity_vector: Vector2D,
    end_velocity_vector: Vector2D,
    coefficients: Vec<f64>
}

impl SplineSpan {
    fn new(control_points: Vec<ControlPoint>, start_velocity_vector: Vector2D, end_velocity_vector: Vector2D) -> SplineSpan {
        let coefficients = get_coefficients();
        SplineSpan {
            control_points,
            start_velocity_vector,
            end_velocity_vector,
            coefficients
        }
    }

    /// Returns the coefficients of the spline span
    fn get_coefficients() -> Vec<f64> {
        let mut coefficients = Vec::new();
        coefficients.push(sel);

        return coefficients;
    }

    /// Adds another row to the coefficients matrix. The first 4 terms are coefficients1, the next 4 are coefficients2,
    /// and the last term is the end value
    fn add_to_coefficients_matrix(&mut self, coefficients: Vec<f64>, coefficients1: Vec<f64>, coefficients2: Vec<f64>, end_value: float64) {
    }

    /// Returns whether the t_value is in the range of this spline span
    fn is_in_range(&self, t_value: float64) -> bool {
        return t_value >= self.start_time() && t_value <= self.end_time();
    }

    pub fn start_time(&self) -> float64 {
        return self.start_velocity_vector.get_x_magnitude();
    }

    pub fn end_time(&self) -> float64 {
        return self.end_velocity_vector.get_x_magnitude();
    }
}
