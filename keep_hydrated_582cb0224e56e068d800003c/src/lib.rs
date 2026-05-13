fn _litres(time: f64) -> i32 {
    (time * 0.5) as i32
}

#[cfg(test)]
mod tests {
    use super::_litres;

    #[test]
    fn sample_tests() {
        assert_eq!(_litres(2.), 1);
        assert_eq!(_litres(1.4), 0);
        assert_eq!(_litres(12.3), 6);
        assert_eq!(_litres(0.82), 0);
        assert_eq!(_litres(11.8), 5);
        assert_eq!(_litres(1787.), 893);
        assert_eq!(_litres(0.), 0);
    }
}