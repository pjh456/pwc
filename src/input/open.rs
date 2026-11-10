use crate::input::{Input, InputInner};

use std::fs::File;
use std::io::{self, BufReader};
use std::path::Path;

use memmap2::Mmap;

impl Input {
    pub fn open(path: Option<&Path>) -> io::Result<Input> {
        let is_stdin = match path {
            None => true,
            Some(p) => p.as_os_str() == "-",
        };

        if is_stdin {
            return Ok(Input {
                inner: InputInner::Stream(Box::new(io::stdin().lock())),
            });
        }

        let p = path.unwrap();
        let file = File::open(p)?;
        let len = file.metadata()?.len();

        if len == 0 {
            return Ok(Input {
                inner: InputInner::Empty,
            });
        }

        match unsafe { Mmap::map(&file) } {
            Ok(mmap) => Ok(Input {
                inner: InputInner::Mapped(mmap),
            }),
            Err(_) => Ok(Input {
                inner: InputInner::Stream(Box::new(BufReader::new(file))),
            }),
        }
    }
}
