use spline_generation::control_point::ControlPoint;
use spline_generation::way_point::WayPoint;

#[macro_use]
extern crate approx; // For the macro relative_eq!
extern crate nalgebra as na;
use na::{Vector3, Rotation3};

struct Spline {
    x_spline_spans: Vec<SplineSpan>,
    y_spline_spans: Vec<SplineSpan>,
    current_span_index: int32,
    current_way_point_index: int32,
    previous_t_value: float64,
    has_reached_end_of_spline: bool,
    current_spline_state: SplineStateMachine,
    way_points: Vec<WayPoint>,
}

enum SplineStateMachine {
    RUNNING,
    END_OF_SPLINE,
}

impl Spline {
    pub fn new(control_points: Vec<ControlPoint>, way_points: Vec<WayPoint>, start_velocity_vector: Vector2,
               end_velocity_vector: Vector2D) -> Spline {

        control_points.sort_by(|a, b| a.get_x_magnitude().cmp(&b.get_x_magnitude()).unwrap());
        way_points.sort_by(|a, b| a.get_x_magnitude().cmp(&b.get_x_magnitude()).unwrap());

        // Finding the spline spans
        let x_control_points = Vec::new();
        let y_control_points = Vec::new();

        for i in 0..control_points.len() {
            x_control_points.push(i, control_points.get(i).x_value);
            y_control_points.push(i, control_points.get(i).y_value);
        }

        let x_spline_spans = get_spline_spans(x_control_points);
        let y_spline_spans = get_spline_spans(y_control_points);


        Spline {
            x_spline_spans,
            y_spline_spans,
            current_span_index: 0,
            current_way_point_index: 0,
            previous_t_value: 0.0,
            has_reached_end_of_spline: false,
            current_spline_state: SplineStateMachine::RUNNING,
            way_points,
        }
    }

    /// Returns the velocity of the robot at that t-value
    pub fn get_velocity(&self, t_value: float64) -> Vector2D {
        if (t_value < self.previous_t_value) {
            panic!("t_value is less than the previous t_value");
        }

        match self.current_spline_state {
            SplineStateMachine::RUNNING => {
                let x_spline_span  = self.x_spline_spans[self.current_span_index];

                // The horizontal and vertical splines will have the same t-values
                if (t_value > x_spline_span.get_end_t_value()) {
                    self.current_span_index += 1;
                }

                // Safety check to make sure we don't go over the end of the spline (x and y spline spans are interchangeable)
                if (self.current_span_index >= self.x_spline_spans.len()) {
                    self.current_spline_state = SplineStateMachine::END_OF_SPLINE;
                    return 0.0;
                }

                x_spline_span = self.x_spline_spans[self.current_span_index];
                y_spline_span = self.y_spline_spans[self.current_span_index];

                return Vector2D::new(x_spline_span.get_velocity(t_value), y_spline_span.get_velocity(t_value));
            }
            SplineStateMachine::END_OF_SPLINE => {
                return Vector2D::new(0, 0);  // The robot should stop when it reaches the end of the spline
            }
        }
    }

    pub fn get_spline_spans(control_points: Vec<ControlPoint>, start_velocity: float64,
                            end_velocity: float64) -> Vec<SplineSpan> {

        let start_velocity_vector = Vector2::new(0, start_velocity);
        let end_velocity_vector = Vector2::new(0, end_velocity);

        let spline_spans = Vec::new();

        for i in 0..control_points.len() - 1 {
            let spline_span = SplineSpan::new(control_points.get(i), control_points.get(i + 1),
                                               start_velocity_vector, end_velocity_vector);

            spline_spans.push(spline_span);
            start_velocity_vector = spline_span.get_end_velocity_vector();

            // TODO figure out end_velocity_vector..........?
        }

    }

    /// Runs all the way points of the robot that the robot has passed through from the previous robot cycle to the
    /// current robot cycle
    pub fn run_all_way_points(&self) {
        while (self.current_way_point_index < self.way_points.len()
                && self.way_points[self.current_way_point_index].get_t_value() <= self.previous_t_value) {

            self.way_points[self.current_way_point_index].execute_command();
            self.current_way_point_index += 1;
        }
    }
}