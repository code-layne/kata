fn _bmi(weight: u32, height: f32) -> &'static str {
    match weight as f32 / height.powi(2) {
        x if x <= 18.5 => "Underweight",
        x if x <= 25.0 => "Normal",
        x if x <= 30.0 => "Overweight",
        _ => "Obese",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(_bmi(50, 1.80), "Underweight");
        assert_eq!(_bmi(80, 1.80), "Normal");
        assert_eq!(_bmi(90, 1.80), "Overweight");
        assert_eq!(_bmi(110, 1.80), "Obese");
    }
}