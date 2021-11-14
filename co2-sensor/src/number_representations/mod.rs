pub enum Unit {
    Fahrenheit,
    Celcius,
    Kelvin,
}

impl Unit {
    pub fn convert_temperature(&self, temperature: &f32) -> f32 {
        match self {
            Unit::Fahrenheit => {
                let temperature = temperature * 1.8 + 32.0;
                temperature
            },

            Unit::Kelvin => {
                let temperature = temperature + 273.15;
                temperature
            },

            Unit::Celcius => {
                *temperature
            },
        }
    }
}