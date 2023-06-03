use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;
use fallible_iterator::FallibleIterator;

use monkey::lexer::Lexer;

const PROMPT: &'static str = "mnky> ";

pub fn start<R, W>(stdin: &mut R, stdout: &mut W) -> Result<()>
where
    R: Read,
    W: Write,
{
    let mut buf_reader = BufReader::new(stdin);
    loop {
        let line = prompt(&mut buf_reader, stdout, PROMPT)?;
        if line.is_empty() {
            continue;
        }

        Lexer::new(&line).for_each(|token| {
            println!("{:?}", token);
            Ok(())
        })?;
    }
}

fn prompt<R, W>(stdin: &mut BufReader<&mut R>, stdout: &mut W, prompt: &str) -> Result<String>
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
