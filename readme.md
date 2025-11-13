# Rust Input Reader
Lightweight input reader in Rust for programming contests, such as on Codeforces

## Features
- Reads all input up to EOF (in the console: ctrl+z, enter)
- No memory leaks
- Does not distinguish between whitespace characters and their number if it is more than 1

## Usage

```rust
fn main() {
    let mut reader = Reader::new();
    let n: usize = reader.read();
    let nums: Vec<i32> = reader.read_vec(n);
    println!("{:?}", nums);
}
```
