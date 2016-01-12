# hello_rusty_worlds

Hello World for Rust, including automated testing and documentation

[![npm](https://img.shields.io/npm/l/express.svg)]()
[![Build Status](https://travis-ci.org/Densaugeo/hello_rusty_worlds.svg?branch=master)](https://travis-ci.org/Densaugeo/hello_rusty_worlds)

## Rustdoc

https://densaugeo.github.io/hello_rusty_worlds/

## Working lib example

Add to Cargo.toml dependencies section:

~~~
[dependencies]
hello_rusty_worlds = { git = "https://github.com/Densaugeo/hello_rusty_worlds" }
~~~

Then call from your code using `extern crate`:

~~~
extern crate hello_rusty_worlds;

fn main() {
  println!("{}", hello_rusty_worlds::hello_world(3).unwrap()); // Prints "Hello Earth!"
}
~~~

## Working bin example

Install the command line tool through `cargo install`:

~~~
[lunariel@morpheus ~]$ cargo install --git https://github.com/Densaugeo/hello_rusty_worlds
Updating git repository `https://github.com/Densaugeo/hello_rusty_worlds`
Compiling hello_rusty_worlds v0.1.0 (https://github.com/Densaugeo/hello_rusty_worlds#b20dd5c0)
Installing /home/lunariel/.cargo/bin/hello-world
be sure to add `/home/lunariel/.cargo/bin` to your PATH to be able to run the installed binaries
~~~

As the message notes, you may need to add cargo's bin folder to your $PATH. This will enable all command
line tools installed through cargo. On linux:

~~~
[lunariel@morpheus ~]$ export PATH=$PATH:/home/lunariel/.cargo/bin
~~~

The binary is installed under the name `hello-world`, and takes one argument, a planet's number:

~~~
[lunariel@morpheus ~]$ hello-world 1
Hello Mercury!
~~~
