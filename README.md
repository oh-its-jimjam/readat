# readat
`readat` is a Rust library for efficient file reading that is missing from Rust's standard library.

## How to use it?

Here's an example

```rust
let file: File = ...;
let pos = f.seek(SeekFrom::Start(0)).unwrap();
let mut buf: &mut [u8] = ...;
// read n bytes from file at 0th position
let n = f.read_at(buf, 0).unwrap();
```

## Why use it?

    TL;DR It exposes the POSIX equivalent of pread

The current Rust standard library does not offer efficient disk reads for low-level applications such as databases. The reason for the inefficiency is that it does not expose a file read without needing to set the file seek position. Thus Rust users must manually seek the file's seek position and then read.

Not only is it sub-optimal because it involves 2 system calls but also it is not thread-safe as the file's seek position is an invariant.

```rust

let file: File = ...;
let pos = f.seek(SeekFrom::Start(0)).unwrap();
let mut buf: &mut [u8] = ...;
let n = f.read(buf).unwrap();
```

It also has the benefit for immutable files as the file's seek position does not to be changed - hence the trait uses `&self`.
