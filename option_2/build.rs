// https://doc.rust-lang.org/cargo/reference/build-scripts.html
use std::env;
////-- public fns

////-- end public fns

fn main() {
    println!("Main build script: {:?}", env!("JG_TEST_VAR"));
    if env!("JG_TEST_VAR") != "blah" {
        panic!("not blah")
    }
}
