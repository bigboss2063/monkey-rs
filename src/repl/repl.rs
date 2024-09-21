use std::io::{self, BufRead, Read, Write};

use crate::{lexer::Lexer, token::Token};

pub fn start<R: Read, W: Write>(input: R, output: W) {
    let mut reader = io::BufReader::new(input);
    let mut writer = io::BufWriter::new(output);

    loop {
        writer.write_all(">> ".as_bytes()).unwrap();
        writer.flush().unwrap();

        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line).unwrap();

        if bytes_read == 0 {
            break; // EOF
        }

        let mut lexer = Lexer::new(line);

        let mut token = lexer.next_token();

        while token != Token::EOF {
            writeln!(writer, "{:?}", token).unwrap();
            token = lexer.next_token()
        }

        writer.flush().unwrap();
    }
}
