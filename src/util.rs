
pub mod fmt {
    use core::fmt::{Write, Result};

    // Multiplexer for fmt::Write.
    pub struct MultiWriter <A, B> {
        a: A,
        b: B,
    }

    impl<A, B> MultiWriter<A, B> {
        pub fn new(a: A, b: B) -> MultiWriter<A, B> {
            MultiWriter{a: a, b: b}
        }
    }

    impl<A, B> Write for MultiWriter<A, B> where A: Write, B: Write {
        fn write_str(&mut self, s: &str) -> Result {
            try!(self.a.write_str(s));
            self.b.write_str(s)
        }
    }
}
