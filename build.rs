fn main() {
    cc::Build::new()
        .include("src")
        .file("src/simple_sin.cc")
        .compile("simple_sin");
}
