struct Coefficient {
    pub power: float64,
    pub coefficient: float64
}

impl Coefficient {
    pub fn new(power: float64, coefficient: float64) -> Coefficient {
        Coefficient {
            power,
            coefficient
        }
    }
}