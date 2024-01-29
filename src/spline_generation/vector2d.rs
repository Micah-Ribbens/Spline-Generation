use std::f64;

/// A struct that represents a vector in 2D space
pub struct Vector2D {
    pub angle: f64,
    pub magnitude: f64,
    pub x_magnitude: f64,
    pub y_magnitude: f64,
}

impl Vector2D {
    /// Initializes a Vector2D with the specified angle and magnitude.
    pub fn new(angle: f64, magnitude: f64) -> Self {
        let mut vector = Vector2D {
            angle: 0.0,
            magnitude: 0.0,
            x_magnitude: 0.0,
            y_magnitude: 0.0,
        };
        vector.set_magnitude_and_angle(angle, magnitude);
        vector
    }

    /// Gets the x magnitude of the vector.
    pub fn get_x_magnitude(&self) -> f64 {
        self.x_magnitude
    }

    /// Gets the y magnitude of the vector.
    pub fn get_y_magnitude(&self) -> f64 {
        self.y_magnitude
    }

    /// Gets the magnitude of the vector.
    pub fn get_magnitude(&self) -> f64 {
        self.magnitude
    }

    /// Sets the x and y magnitudes of the vector.
    pub fn set_x_and_y_magnitude(&mut self, x_magnitude: f64, y_magnitude: f64) -> &mut Self {
        let magnitude = (x_magnitude.powf(2.0) + y_magnitude.powf(2.0)).sqrt(); // Pythagorean theorem
        let angle = y_magnitude.atan2(x_magnitude);
        self.set_magnitude_and_angle(angle, magnitude);
        self
    }

    /// Gets a new vector with the specified x and y magnitudes (does not modify the current vector).
    pub fn get_vector_with_x_and_y_magnitude(&self, x_magnitude: f64, y_magnitude: f64) -> Vector2D {
        self.get_copy().set_x_and_y_magnitude(x_magnitude, y_magnitude)
    }

    /// Sets the angle and magnitude of the vector.
    pub fn set_magnitude_and_angle(&mut self, angle: f64, magnitude: f64) -> &mut Self {
        self.angle = angle;
        self.magnitude = magnitude;
        self.x_magnitude = angle.cos() * magnitude;
        self.y_magnitude = angle.sin() * magnitude;
        self
    }

    /// Gets a new vector with the specified angle and magnitude (does not modify the current vector).
    pub fn get_vector_with_magnitude_and_angle(&self, angle: f64, magnitude: f64) -> Vector2D {
        self.get_copy().set_magnitude_and_angle(angle, magnitude)
    }

    /// Sets the angle of the vector.
    pub fn set_angle(&mut self, angle: f64) -> &mut Self {
        self.set_magnitude_and_angle(angle, self.magnitude)
    }

    /// Gets a new vector with the specified angle (does not modify the current vector).
    pub fn get_vector_with_angle(&self, angle: f64) -> Vector2D {
        self.get_copy().set_angle(angle)
    }

    /// Sets the magnitude of the vector.
    pub fn set_magnitude(&mut self, magnitude: f64) -> &mut Self {
        self.set_magnitude_and_angle(self.angle, magnitude)
    }

    /// Gets a new vector with the specified magnitude (does not modify the current vector).
    pub fn get_vector_with_magnitude(&self, magnitude: f64) -> Vector2D {
        self.get_copy().set_magnitude(magnitude)
    }

    /// Sets the x magnitude of the vector.
    pub fn set_x_magnitude(&mut self, x_magnitude: f64) -> &mut Self {
        self.set_x_and_y_magnitude(x_magnitude, self.y_magnitude)
    }

    /// Gets a new vector with the specified x magnitude (does not modify the current vector).
    pub fn get_vector_with_x_magnitude(&self, x_magnitude: f64) -> Vector2D {
        self.get_copy().set_x_magnitude(x_magnitude)
    }

    fn get_copy(&self) -> Vector2D {
        Vector2D {
            angle: self.angle,
            magnitude: self.magnitude,
            x_magnitude: self.x_magnitude,
            y_magnitude: self.y_magnitude,
        }
    }
    /// Sets the y magnitude of the vector.
    pub fn set_y_magnitude(&mut self, y_magnitude: f64) -> &mut Self {
        self.set_x_and_y_magnitude(self.x_magnitude, y_magnitude);
        self
    }

    /// Gets a new vector with the specified y magnitude (does not modify the current vector).
    pub fn get_vector_with_y_magnitude(&self, y_magnitude: f64) -> Vector2D {
        self.get_copy().set_y_magnitude(y_magnitude)
    }

    /// Normalizes the vector.
    pub fn normalize(&mut self) -> &mut Self {
        let new_x_magnitude = self.x_magnitude / self.magnitude;
        let new_y_magnitude = self.y_magnitude / self.magnitude;
        self.set_x_and_y_magnitude(new_x_magnitude, new_y_magnitude);
        self
    }

    /// Gets a new vector that is the normalized version of this vector (does not modify the current vector).
    pub fn get_normalized(&self) -> Vector2D {
        self.get_copy().normalize()
    }

    /// Gets a copy of the current vector.
    pub fn get_copy(&self) -> Vector2D {
        Vector2D {
            angle: self.angle,
            magnitude: self.magnitude,
            x_magnitude: self.x_magnitude,
            y_magnitude: self.y_magnitude,
        }
    }

    /// Adds another vector to this current vector.
    pub fn add(&mut self, other_vector: &Vector2D) -> &mut Self {
        let new_x_magnitude = other_vector.get_x_magnitude() + self.x_magnitude;
        let new_y_magnitude = other_vector.get_y_magnitude() + self.y_magnitude;
        self.set_x_and_y_magnitude(new_x_magnitude, new_y_magnitude);
        self
    }

    /// Gets a new vector that is the result of adding another vector to this vector (does not modify the current vector).
    pub fn get_added_vector(&self, other_vector: &Vector2D) -> Vector2D {
        self.get_copy().add(other_vector)
    }

    /// Subtracts another vector from this current vector.
    pub fn subtract(&mut self, other_vector: &Vector2D) -> &mut Self {
        let new_x_magnitude = other_vector.get_x_magnitude() - self.x_magnitude;
        let new_y_magnitude = other_vector.get_y_magnitude() - self.y_magnitude;
        self.set_x_and_y_magnitude(new_x_magnitude, new_y_magnitude);
        self
    }

    /// Gets a new vector that is the result of subtracting another vector from this vector (does not modify the current vector).
    pub fn get_subtracted_vector(&self, other_vector: &Vector2D) -> Vector2D {
        self.get_copy().subtract(other_vector)
    }

    /// Multiplies the magnitude of the vector by a scalar.
    pub fn multiply_by_scalar(&mut self, scalar: f64) -> &mut Self {
        self.magnitude *= scalar;
        self.set_magnitude_and_angle(self.angle, self.magnitude);
        self
    }

    /// Gets a new vector that is the result of multiplying this vector by a scalar (does not modify the current vector).
    pub fn get_vector_multiplied_by_scalar(&self, scalar: f64) -> Vector2D {
        self.get_copy().multiply_by_scalar(scalar)
    }

    /// Performs a Hadamard product on this vector.
    pub fn hadamard_product(&mut self, other_vector: &Vector2D) -> &mut Self {
        let new_x_magnitude = other_vector.get_x_magnitude() * self.x_magnitude;
        let new_y_magnitude = other_vector.get_y_magnitude() * self.y_magnitude;
        self.set_x_and_y_magnitude(new_x_magnitude, new_y_magnitude);
        self
    }

    /// Gets a new vector that is the result of performing a Hadamard product on this vector (does not modify the current vector).
    pub fn get_hadamard_product(&self, other_vector: &Vector2D) -> Vector2D {
        self.get_copy().hadamard_product(other_vector)
    }

    /// Rotates the vector counter-clockwise by 'angle'.
    pub fn rotate_vector(&mut self, angle: f64) -> &mut Self {
        let new_angle = modified_mod(self.angle + angle, f64::consts::PI * 2.0);
        self.set_angle(new_angle);
        self
    }

    /// Gets a new vector that is the result of rotating the current vector counter-clockwise by 'angle' (does not modify the current vector).
    pub fn get_rotated_vector(&self, angle: f64) -> Vector2D {
        self.get_copy().rotate_vector(angle)
    }

    /// Inverts the direction of the vector.
    pub fn invert_vector_direction(&mut self) -> &mut Self {
        self.rotate_vector(f64::consts::PI);
        self
    }

    /// Gets a new vector that is the result of inverting the direction of the current vector (does not modify the current vector).
    pub fn get_inverted_direction_vector(&self) -> Vector2D {
        self.get_copy().invert_vector_direction()
    }

    /// Calculates the dot product of two vectors.
    pub fn dot_product(&self, other_vector: &Vector2D) -> f64 {
        other_vector.get_x_magnitude() * self.x_magnitude + other_vector.get_y_magnitude() * self.y_magnitude
    }

    /// Checks if the current vector equals another vector.
    pub fn equals(&self, other_vector: &Vector2D) -> bool {
        other_vector.get_angle() == self.angle && other_vector.get_magnitude() == self.magnitude
    }
}