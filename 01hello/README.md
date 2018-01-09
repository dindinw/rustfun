### Install

```sh
curl https://sh.rustup.rs -sSf | sh
```

### Check

```sh
$ source $HOME/.cargo/env
$ cargo version
cargo 0.24.0 (45043115c 2017-12-05)
$ rustc --version
rustc 1.23.0 (766bd11c8 2018-01-01)
$ rustdoc --version
rustdoc 1.23.0 (766bd11c8 2018-01-01)
$ rustup doc --std
```

### Usage

```
$ cargo run
   Compiling hello v0.1.0 (file:///work/rust/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.34 secs
     Running `target/debug/hello`
Usage: gcd NUMBER ...
$ cargo run 1 2
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello 1 2`
The greatest common divisor of [1, 2] is 1
$ cargo test
   Compiling hello v0.1.0 (file:///work/rust/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.16 secs
     Running target/debug/deps/hello-6ba2d3e099a79f40

running 1 test
test test_gcd ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured
```
