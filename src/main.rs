mod ast;
mod lexer;
mod repl;
mod token;

use anyhow::Result;

fn main() -> Result<()> {
    repl::start(&mut std::io::stdin(), &mut std::io::stdout())
}
