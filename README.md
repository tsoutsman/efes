# efes

[![Crates.io](https://img.shields.io/crates/v/efes.svg)](https://crates.io/crates/efes)
[![Docs.rs](https://docs.rs/efes/badge.svg)](https://docs.rs/efes)
[![CI](https://github.com/tsoutsman/efes/workflows/CI/badge.svg)](https://github.com/tsoutsman/efes/actions)

A collection of macros to simplify testing involving filesystems. The crate exposes two macros: `gen_fs` and `gen_paths`. `gen_fs` is used to actually create files on the system in a given directory. `gen_paths` generates a vector of paths based on the provided input.

# Example

## `gen_fs`

```rust
let root_dir = PathBuf::from("/home/user");
gen_fs!(root_dir => (a: file1 file2) b c (another_directory: foo bar));
```
which would create a directory structure in `root_dir` equivalent to the following:
```shell
.
├── a
│   ├── file1
│   └── file2
├── another_directory
│   ├── bar
│   └── foo
├── b
└── c
```

## `gen_paths`

```rust
let root_dir = PathBuf::from("/home/user");
let expected = gen_paths!(root_dir => (a: file1 file2) b c (another_directory: foo bar));
assert_eq!(expected, vec![
    root_dir.join("a").join("file1"),
    root_dir.join("a").join("file2"),
    root_dir.join("b"),
    root_dir.join("c"),
    root_dir.join("another_directory").join("foo"),
    root_dir.join("another_directory").join("bar"),
]);

```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
