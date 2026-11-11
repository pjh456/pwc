#[derive(Default)]
pub(crate) struct Utf8Carry {
    carry: [u8; 4],
    len: usize,
}

impl Utf8Carry {
    pub fn pending_len(&self) -> usize {
        self.len
    }

    pub fn for_each_char<F>(&mut self, chunk: &[u8], f: &mut F)
    where
        F: FnMut(char),
    {
        let mut rest = chunk;

        if self.len > 0 {
            let take = (4 - self.len).min(rest.len());
            self.concat_with(&rest[..take]);

            let total = self.len;
            let buf = self.carry;
            let consumed = Utf8Carry::decode(&buf[..total], f);

            // still incompleted
            if consumed < total {
                debug_assert_eq!(take, rest.len());
                self.update_carry_tail(&buf[consumed..total]);
                return;
            }

            self.len = 0;
            rest = &rest[take..];
        }

        let consumed = Utf8Carry::decode(rest, f);
        if consumed < rest.len() {
            self.update_carry_tail(&rest[consumed..]);
        }
    }

    fn concat_with(&mut self, buf: &[u8]) {
        let buf_len = buf.len();
        self.carry[self.len..self.len + buf_len].copy_from_slice(buf);
        self.len += buf_len;
    }

    fn update_carry_tail(&mut self, tail: &[u8]) {
        let tail_len = tail.len();
        debug_assert!(tail_len < 4);
        self.carry[..tail_len].copy_from_slice(tail);
        self.len = tail_len;
    }

    fn decode<F>(s: &[u8], f: &mut F) -> usize
    where
        F: FnMut(char),
    {
        let mut at = 0;
        loop {
            let cur = &s[at..];
            match std::str::from_utf8(cur) {
                Ok(valid) => {
                    Utf8Carry::emit_chars(valid, f);
                    return s.len();
                }
                Err(e) => {
                    let good = e.valid_up_to();
                    let valid = std::str::from_utf8(&cur[..good]).unwrap();
                    Utf8Carry::emit_chars(valid, f);

                    match e.error_len() {
                        Some(n) => {
                            // Invalid Byte is regarded as 1 count.
                            // Make GNU wc happy :)
                            for _ in 0..n {
                                f(char::REPLACEMENT_CHARACTER);
                            }
                            at += good + n;
                        }
                        None => return at + good,
                    }
                }
            }
        }
    }

    fn emit_chars<F>(s: &str, f: &mut F)
    where
        F: FnMut(char),
    {
        for c in s.chars() {
            f(c)
        }
    }
}
