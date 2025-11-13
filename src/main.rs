use std::fmt::Debug;
use std::io::{self, Read};
use std::str::FromStr;

pub struct Reader {
    buf: String,
    pos: usize,
}

impl Reader {
    pub fn new() -> Self {
        let mut s = String::new();
        io::stdin()
            .read_to_string(&mut s)
            .expect("failed to read stdin");
        Self { buf: s, pos: 0 }
    }

    pub fn from_string(s: String) -> Self {
        Self { buf: s, pos: 0 }
    }

    pub fn read<T>(&mut self) -> T
    where
        T: FromStr,
        T::Err: Debug,
    {
        let token = self.next_token().expect("no more tokens");
        token.parse::<T>().expect("failed to parse token")
    }

    pub fn read_vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: FromStr,
        T::Err: Debug,
    {
        let mut vec = Vec::<T>::with_capacity(n);
        for _ in 0..n {
            vec.push(self.read::<T>())
        }
        vec
    }

    fn next_token(&mut self) -> Option<&str> {
        let bytes = self.buf.as_bytes();
        let len = bytes.len();
        let mut i = self.pos;

        while i < len && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= len {
            self.pos = i;
            return None;
        }

        let start = i;
        while i < len && !bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        let end = i;
        self.pos = end;
        Some(&self.buf[start..end])
    }
}

fn main() {
    let mut reader = Reader::new();
    let n: usize = reader.read();
    let nums: Vec<i32> = reader.read_vec(n);
    println!("{:?}", nums)
}
