use rules_rust_with_c::{good_fn, rs_simple_sin};

#[tokio::main]
async fn main() {
    println!("{}", good_fn().await);
    println!("{}", rs_simple_sin(9.0));
}
