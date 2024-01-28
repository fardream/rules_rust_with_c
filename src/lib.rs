extern "C" {
    fn simple_sin(x: f64) -> f64;
}

pub fn rs_simple_sin(x: f64) -> f64 {
    unsafe { simple_sin(x) }
}

#[cfg(test)]
mod test {
    use super::rs_simple_sin;

    #[test]
    fn test_rs_simple_sin() {
        assert_eq!(rs_simple_sin(0.), 0.);
    }
}
