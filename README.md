# hello_rusty_worlds

A hello world crate with automated testing, documentation, continuous integration, tested example code, implemented as a library with a command line tool. Supports Sol and all its planets, not just 'Hello (unspecified) world!'

[![npm](https://img.shields.io/npm/l/express.svg)]()
[![Build Status](https://app.travis-ci.com/Densaugeo/hello_rusty_worlds.svg?branch=master)](https://app.travis-ci.com/github/Densaugeo/hello_rusty_worlds)

## How to use as a library

Add a dependency to the Cargo.toml dependencies section:

~~~
[dependencies]
hello_rusty_worlds = "0.1.*"
~~~

Then call from your code (such as /src/main.rs) using `extern crate`:

~~~
extern crate hello_rusty_worlds;

fn main() {
  println!("{}", hello_rusty_worlds::hello_world(3).unwrap()); // Prints "Hello Earth!"
}
~~~

## How to install as a binary

Cargo can add new commands using the `cargo install` subcommand:

~~~
[lunariel@morpheus ~]$ cargo install hello_rusty_worlds
Updating registry `https://github.com/rust-lang/crates.io-index`
Compiling hello_rusty_worlds v0.1.0
Installing /home/lunariel/.cargo/bin/hello-world
be sure to add `/home/lunariel/.cargo/bin` to your PATH to be able to run the installed binaries
~~~

`Cargo install` compiles a crate's [[bin]] targets (specified in its Cargo.toml) and puts them in cargo's /bin folder. As the message notes, you may need to add cargo's /bin folder to your $PATH. This will enable all commands installed through `cargo install`. On linux:

~~~
[lunariel@morpheus ~]$ export PATH=$PATH:/home/lunariel/.cargo/bin
~~~

The binary is installed under the name `hello-world`, and takes one argument, a planet's number:

~~~
[lunariel@morpheus ~]$ hello-world 1
Hello Mercury!
~~~

## Documentation

Documentation is hosted at https://densaugeo.github.io/hello_rusty_worlds/. This documentation is automatically generated from comments by the rustdoc tool, by running `cargo doc`.

## License

MIT
