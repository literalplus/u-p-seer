u-p-seer
========

This application is intended to be able to watch your internet
connection and report cable modem restarts. This is built to
work with my own UPC modem, hence the name. 

Building
--------

**Important:** This application only supports Linux-based operating systems.
While it *might* run on other systems, it might not work well or at all.

To build this application, you first need to check it out:

````bash
git clone https://github.com/xxyy/u-p-seer.git
````

To compile and run it, install [`just`](https://github.com/casey/just) and run: (you need to 
[set up Rust](https://doc.rust-lang.org/book/second-edition/ch01-01-installation.html)
for this)

````bash
just build run
````

This places the binaries in `target/debug/u-p-seer`
(`u-p-seer.exe` on Windows).

You can also build with `cargo` directly, but for the application to actually work,
it needs permission to use `ping`. The `just` script accomplishes this using
`setcap`.

You can run without compiling using:

````bash
just run
````

Contributing
------------

Any and all contributions (issues, pull requests, wiki edits, ...)
are welcome! Just try to stick to the style of the existing code
and the generally accepted standards for Rust. I'm always open
for suggestions.

License
-------

This project is licensed under a MIT License.
Check out the `LICENSE.txt` file for more information.