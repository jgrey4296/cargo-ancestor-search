// https://doc.rust-lang.org/cargo/reference/build-scripts.html
use std::env;
////-- public fns

////-- end public fns

fn main() {
    let val = env::var("JG_TEST_VAR");
    println!("Submod build script: {:?}", val);
    if let Ok(x) = val {
        if x == "blah" {
            panic!("Is Blah");
        }
    }
}
