use tokio::fs::File;
extern "C" {
    fn simple_sin(x: f64) -> f64;
}

pub fn rs_simple_sin(x: f64) -> f64 {
    unsafe { simple_sin(x) }
}

pub async fn good_fn() -> String {
    let f = File::open("foo").await;

    match f {
        Ok(_) => {
            format!("success")
        }
        Err(_) => {
            format!("failed")
        }
    }
}

#[cfg(test)]
mod test {
    use super::rs_simple_sin;

    #[test]
    fn test_rs_simple_sin() {
        assert_eq!(rs_simple_sin(0.), 0.);
    }
}
