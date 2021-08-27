/// A macro that creates files and directories based on the given input.
/// # Example
/// ```
/// # use tempfile::tempdir;
/// # use efes::gen_fs;
/// # use std::path::PathBuf;
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let root_dir = PathBuf::from("/home/user");
/// # let root_dir = tempdir()?.into_path();
/// gen_fs!(root_dir => (a: file1 file2) b c (another_directory: foo bar));
/// # Ok(())
/// # }
/// ```
/// would create a directory structure in `root_dir` equivalent to the following:
/// ```shell
/// .
/// ├── a
/// │   ├── file1
/// │   └── file2
/// ├── another_directory
/// │   ├── bar
/// │   └── foo
/// ├── b
/// └── c
/// ```
#[macro_export]
macro_rules! gen_fs {
        ($root:ident => $f:ident $($tail:tt)*) => {
            ::std::fs::File::create($root.join(stringify!($f))).unwrap();
            gen_fs!($root => $($tail)*);
        };
        ($root:ident => ($dir:ident : $($inner:tt)*)$($tail:tt)*) => {
            ::std::fs::create_dir($root.join(stringify!($dir))).unwrap();
            let $dir = $root.join(stringify!($dir));
            gen_fs!($dir => $($inner)*);
            gen_fs!($root => $($tail)*);
        };
        ($root:ident => ) => {};
}

/// A macro that creates a vector of [`PathBufs`](std::path::PathBuf) containing paths to all files specified in the input.
/// # Example
/// ```
/// # use tempfile::tempdir;
/// # use efes::gen_paths;
/// # use std::path::PathBuf;
/// # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
/// let root_dir = PathBuf::from("/home/user");
/// # let root_dir = tempdir()?.into_path();
/// let expected = gen_paths!(root_dir => (a: file1 file2) b c (another_directory: foo bar));
/// assert_eq!(expected, vec![
///     root_dir.join("a").join("file1"),
///     root_dir.join("a").join("file2"),
///     root_dir.join("b"),
///     root_dir.join("c"),
///     root_dir.join("another_directory").join("foo"),
///     root_dir.join("another_directory").join("bar"),
/// ]);
/// # Ok(())
/// # }
#[macro_export]
macro_rules! gen_paths {
        ($root:ident => $f:ident $($tail:tt)*) => {
            {
                #[allow(clippy::vec_init_then_push)]
                {
                    let mut z_macro = Vec::new();
                    ::efes::__gen_paths_private!(@$root | z_macro => $f $($tail)*);
                    z_macro
                }
            }
        };
        ($root:ident => ($dir:ident : $($inner:tt)*)$($tail:tt)*) => {
            {
                #[allow(clippy::vec_init_then_push)]
                {
                    let mut z_macro = Vec::new();
                    ::efes::__gen_paths_private!(@$root | z_macro => ($dir : $($inner)*) $($tail)*);
                    z_macro
                }
            }
        };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __gen_paths_private {
        (@$root:ident | $vec:ident => ) => {};
        (@$root:ident | $vec:ident => $f:ident $($tail:tt)*) => {
            $vec.push($root.join(stringify!($f)));
            ::efes::__gen_paths_private!(@$root | $vec => $($tail)*);
        };
        (@$root:ident | $vec:ident => ($dir:ident : $($inner:tt)*)$($tail:tt)*) => {
            let $dir = $root.join(stringify!($dir));
            ::efes::__gen_paths_private!(@$dir | $vec => $($inner)*);
            ::efes::__gen_paths_private!(@$root | $vec => $($tail)*);
        };
}
