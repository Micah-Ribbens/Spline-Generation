struct ControlPoint {
    pub t_value: int32,
    pub x_value: float64,
    pub y_value: float64,
}

/// A point that defines the path of the robot. The t_value is the 0-indexed point number of the control point
impl ControlPoint {
    pub fn new(t_value: int32, x_value: float64, y_value: float64) -> ControlPoint {
        ControlPoint {
            t_value,
            x_value,
            y_value,
        }
    }


}