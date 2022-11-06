use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;
use fallible_iterator::FallibleIterator;

use crate::lexer::Lexer;

const PROMPT: &str = ">> ";

pub fn start<R, W>(stdin: &mut R, stdout: &mut W) -> Result<()>
where
    R: Read,
    W: Write,
{
    let mut buf_reader = BufReader::new(stdin);
    loop {
        let line = prompt(PROMPT, &mut buf_reader, stdout)?;
        if line.is_empty() {
            continue;
        }

        let mut lexer = Lexer::new(&line);
        loop {
            match lexer.next() {
                Ok(Some(token)) => println!("{:?}", token),
                Ok(None) => break,
                Err(err) => {
                    println!("Error: {}", err);
                    break;
                }
            }
        }
    }
}

fn prompt<R, W>(prompt: &str, stdin: &mut BufReader<&mut R>, stdout: &mut W) -> Result<String>
where
    R: Read,
    W: Write,
{
    stdout.write_all(prompt.as_bytes())?;
    stdout.flush()?;
    let mut input = String::new();
    stdin.read_line(&mut input)?;
    Ok(input)
}
