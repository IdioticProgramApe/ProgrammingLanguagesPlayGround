Brief Rust
==========

Preparation
-----------
| Name               | URL                                                                        |
| :-----------:      | :-----------                                                               |
| VS build c++ tools | https://visualstudio.microsoft.com/visual-cpp-build-tools/                 |
| rustup             | https://www.rust-lang.org/tools/install                                    |
| Vscode Config      | https://gist.github.com/IdioticProgramApe/91412c28c8b333767bb919d07e077335 |

Basics
------

| Topic      | File Path | Notes    |
| :-----------: | :----------- | :--------- |
| struct      | basics/src/main.rs      | `Debug` macro, `struct`, `impl` keywords    |
| enum   | basics/src/enums.rs       | `match` syntax, `Option` enum, `if let` keywords |
| project organization | basics/src/filesystem | `mod.rs`, `pub`, `mod` keywords |
| error handling | basics/src/error_handle.rs | `panic!` macro, `Result` enum, `?` operator, <br> `unwrap`, `expect`, `Err::kind()` method |
| generic type | basics/src/generics.rs | `<T>` |
| *trait* | basics/src/traits.rs | virtual function/interface, `impl Traits for Struct`, <br> `+` operator |
| *lifetime* | basics/src/lifetime.rs | `'a` (`'static`) |
| io | basics/src/file_io.rs | `std::fs::{Read, Write, OpenOptions}` <br> `std::io::{stdin, Read, Write}`  |
| container | basics/src/collections.rs | `std::collections::HashMap`, `Vec<T>`, `String` |
| oop | basics/src/oor.rs | `impl<T> SomeClass<T> where T: Debug {...}` |
| concurrency | basics/src/concurrency.rs | closure, `std::thread::{spawn, sleep}` <br> `std::sync::mpsc::channel` |

Caution
-------
- reference: `&`, `*`
- mutable: `mut`
- ownership: as `unique_ptr<T>` in c++ ?
- lifetime: `&'a str` ?
- string: `&str`, `str`, `String`, `&String` ?