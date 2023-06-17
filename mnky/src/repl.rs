use std::io::{BufRead, BufReader, Read, Write};

use anyhow::Result;

use monkey::lexer::Lexable;

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

        for token in line.lex() {
            writeln!(stdout, "{0:?}\t{0}", token)?
        }
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
