# the-rs-book-projects
This is a repository containing the code I have written
while reading *The Rust Programming Language* book,
specifically an experimental branch that supports
interactive features like quizzes (available online
at [https://rust-book.cs.brown.edu](https://rust-book.cs.brown.edu)).
Besides collecting all the crates under a single workspace,
running `cargo fmt` and making sure that everything passes
`cargo clippy -- -W clippy::pedantic`, the code is
presented as it was written, with the limited Rust
knowledge I had at the time. Note that, while I am
a beginner in Rust, I am not a complete newbie! I had read
[Rust by Example](https://doc.rust-lang.org/rust-by-example)
beforehand and I work as a professional C++ software developer.

## Why is this published?
Mainly, as something to look back to in the future and
remember where I started from. Also, I believe that
some projects are interesting to check out as they
either diverge significantly from what the book did
or they were assignments that the book wanted us to
do on our own. Such interesting projects are, in
alphabetical order:
* [company](./crates/company);
* [deadlock](./crates/deadlock);
* [fibonacci](./crates/fibonacci);
* [median-mode](./crates/median_mode);
* [pig-latin](./crates/pig_latin);
* [temp-conv](./crates/temp_conv);
* [twelve-days-of-christmas](./crates/twelve_days_of_christmas);
* [web-server](./crates/web_server);
* [web-server-third-party](/crates/web_server_third_party).

If you haven't read the book and completed the projects, give them a
try before looking at the code in here! The book is a nice
read and the projects are fun to build!

## How do I compile the code?
First, you need to install the Rust toolchain by following
the excellent instructions from
[the Rust website](https://www.rust-lang.org/tools/install).
Make sure that the Rust binaries are in your `PATH`! You can
check this by running `rustc --version` at the command line.
Then:
* for [hello-world](./crates/hello_world), you `cd` into the
[hello_world directory](./crates/hello_world) and run
`rustc main.rs`;
* for everything else, you `cd` into the [workspace directory](.)
(the one where this file is located) and run `cargo build`.
Alternatively, you can `cd` into a specific project and run
`cargo build` from there if you want to compile just that crate.

While developing the code, I used the
[Visual Studio Code](https://code.visualstudio.com) editor
(which can be installed from
[https://code.visualstudio.com/Download](https://code.visualstudio.com/Download))
alongside the
[rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
extension. *VS Code* should recommend you to install the extension when opening
the [workspace directory](.) (the one where this file is located) for the first time.

## How is this project licensed?
This project is licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](./LICENSE-APACHE) or
   [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
 * MIT license
   ([LICENSE-MIT](./LICENSE-MIT) or
   [http://opensource.org/licenses/MIT/](http://opensource.org/licenses/MIT))

at your option.

This project is heavily based on *The Rust Programming Language*
book, specifically an experimental branch that supports
interactive features like quizzes, available online at
[https://rust-book.cs.brown.edu](https://rust-book.cs.brown.edu).
The book is dual-licensed under both the MIT and the Apache-2.0 licenses.

The [guessing-game](./crates/guessing_game) crate uses the
[rand](https://crates.io/crates/rand) crate as a dependency.
The [rand](https://crates.io/crates/rand) crate is dual-licensed under
both the MIT and the Apache-2.0 licenses.

The [hello-macro-derive](./crates/hello_macro/hello_macro_derive)
crate uses the [syn](https://crates.io/crates/syn) and
[quote](https://crates.io/crates/quote) crates. The two crates are
dual-licensed under both the MIT and the Apache-2.0 licenses.

The [pig-latin](./crates/pig_latin) crate uses the
[icu](https://crates.io/crates/icu) and
[itertools](https://crates.io/crates/itertools)
crates as dependencies. The [icu](https://crates.io/crates/icu)
crate is licensed under the Unicode-3.0 license. The
[itertools](https://crates.io/crates/itertools) crate is dual-licensed
under both the MIT and the Apache-2.0 licenses.

The [web-server-third-party](./crates/web_server_third_party) crate
uses the [rayon](https://crates.io/crates/rayon) crate as a dependency.
The [rayon](https://crates.io/crates/rayon) crate is dual-licensed
under both the MIT and the Apache-2.0 licenses.