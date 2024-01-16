use spline_generation::vector2d::Vector2D;
use spline_generation::control_point::ControlPoint;
use spline_generation::way_point::WayPoint;

struct Spline {
    spline_spans: Vec<SplineSpan>
}

impl Spline {
    pub fn new(control_points: Vec<ControlPoint>, way_points: Vec<WayPoint>, initial_velocity: Vector2D,
               end_velocity: Vector2D) -> Spline {

        control_points.sort_by(|a, b| a.get_x_magnitude().cmp(&b.get_x_magnitude()).unwrap());
        way_points.sort_by(|a, b| a.get_x_magnitude().cmp(&b.get_x_magnitude()).unwrap());

        for i in 0..control_points.len() {
            let spline_part =

        }
    }
}