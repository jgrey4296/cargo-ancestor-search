# A Couple of Cargo Tests
There are two modules in each option: `chainsearch`, and `submod`.
`chainsearch` is a binary crate, that uses `submod::add` to print the result of adding two numbers.

In each option, an env var `$JG_TEST_VAR` is set in the root module `./.cargo/config.toml` to "blah"

The Root module `build.rs` panics if `$JG_TEST_VAR` is *not* "blah". 
The submod `build.rs` pancis if `$JG_TEST_VAR` *is* "blah".
(reminder, when building, you can see the output of `build.rs` by calling `cargo build -vv`)

## Failing
This is something approximating your current situation.
`submod` is an explicit dependency in `Cargo.toml`, and if you try to `cargo build`, the build script panics.


## Option 1
This doesn't add `submod` as a dependency in the `Cargo.toml` of the root module.
Instead, `./option_1/build.rs` has the line:
```rust
println!("cargo:rustc-link-search=crate=submod/target/debug");
```
Adding the build directory of submod to the rustc call.

Meanwhile `./option_1/src/main.rs`:
```rust
extern crate submod;
```

And `./option_1/submod/.cargo/config.toml` *must* override the bad root variable:
```toml
[env]
JG_TEST_VAR = "bloo"
```

So to compile `option_1`:
1) cd into `./option_1/submod` and run `cargo build --config ./.cargo/config.toml` 
2) return to `./option_1` and `cargo build`

To Clean, you have to do the same again.

## Option 2
In `./option_2/Cargo.toml`, `submod` is declared as a dependency,
and creates a workspace of the two modules:
```toml
[dependencies]
submod = { path = "./submod" }

[workspace]
resolver        = "2"
members         = ["./submod", "."]
default-members = ["."]
```


`./option_2/src/main.rs` can now use `use submod;`

And `./option_2/submod/.cargo/config.toml` again *must* override the bad root variable:
```toml
[env]
JG_TEST_VAR = "bloo"
```

To compile `option_2`:
You don't need to `cd` anywhere.
You call an alias cargo command: `cargo bsub` (defined in `./option_2/.cargo/config.toml`)
This builds the submod, specifying its `cargo.toml`.

Then you can `cargo build`, which will build the root module successfully.
However, when you run `cargo clean`, it will remove the built submod, so you have to call `cargo bsub` again.


## Option 3
Option 3 moves the main module into `./option_3/chainsearch`,
and creates a Virtual Workspace in `./option_3/Cargo.toml`:
```toml
[workspace]
members = ["./chainsearch", "./submod"]
resolver = "2"
default-members = ["./chainsearch", "./submod"]
```


The `Cargo.toml` continues to declare `submod` as a dependency.
Note the path is now `..`:
```toml
[dependencies]
submod = { path = "../submod" }
```

This time, you can compile each separately with `cargo bmain` and `cargo bsub`,
and `submod` does *not* need to have an override in it's `config.toml`.
`cargo clean` will clean both `chainsearch` and `submod`.

If you *only* clean `chainsearch` (with `cargo clean -p chainsearch`),
then you can just `cargo bmain` again, and it'll build.
