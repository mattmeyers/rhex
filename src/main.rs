use std::boxed::Box;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let mut f = match File::open(env::args().nth(1).expect("path required")) {
        Ok(f) => f,
        Err(e) => {
            return Err(Box::new(e));
        }
    };

    let mut buffer = Vec::new();

    f.read_to_end(&mut buffer)?;

    let mut s = std::io::stdout();

    for (i, chunk) in buffer.chunks(16).enumerate() {
        if let Err(e) = write!(
            s,
            "{:08x}:{}  {}\n",
            i * 16,
            build_hex_string(chunk),
            build_ascii_string(chunk)
        ) {
            if e.kind() != std::io::ErrorKind::BrokenPipe {
                return Err(Box::new(e));
            }
        }
    }

    Ok(())
}

fn build_hex_string(s: &[u8]) -> String {
    let mut st = String::new();

    for (i, j) in s.iter().enumerate() {
        if i % 2 == 0 {
            st.push(' ');
        }

        st.push_str(format!("{:02x}", j).as_ref());
    }

    format!("{:40}", st)
}

fn build_ascii_string(s: &[u8]) -> String {
    let mut st = String::with_capacity(s.len());

    for i in s.iter() {
        match i {
            v if *v <= 0x1F || *v >= 0x7F => st.push('.'),
            v => st.push(*v as char),
        }
    }

    st
}
