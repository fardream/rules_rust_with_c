use rules_rust_with_c::rs_simple_sin;

#[tokio::main]
async fn main() {
    println!("{}", rs_simple_sin(9.0));
}
