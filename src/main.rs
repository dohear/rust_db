mod command;
mod db;
mod input_buffer;
mod insert;

use command::Command;
use command::MetaCommand;
use command::Statement;
use input_buffer::InputBuffer;
use insert::InsertStatement;
use std::io::{self, Write};

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap(); // forces prompt to appear in stdout without newline
}

fn main() {
    let mut input_buffer = InputBuffer::new(); // (empty) vec of tokenized input

    let mut db = db::Table::new(); // Creates empty vec of pages

    // begin REPL
    loop {
        print_prompt();
        input_buffer.read_input(); // Populate buffer with tokenized user input

        // Right now we handle meta commands (i.e. `.exit`) seperately
        // as they are trivial to handle compared to a SQL query
        match Command::parse(&input_buffer.buffer) {
            // If first char is `.` the input is parsed into a specific variant of
            // the MetaCommand enum and exicuted as such
            Command::MetaCommand(meta) => match meta {
                MetaCommand::Exit => {
                    std::process::exit(0);
                }
                MetaCommand::Unrecognized(cmd) => {
                    println!("Unrecognized command '{}'.", cmd);
                }
            },
            // Otherwise we parse the input as such: Command->Statement->{statement type}...
            Command::Statement(stmt) => match stmt {
                Statement::Insert => {
                    match InsertStatement::new(&input_buffer.buffer).and_then(|s| db.insert(s)) {
                        Ok(inserted) => println!("Successfully inserted '{:?}'.", inserted),
                        Err(e) => println!("{}", e),
                    }
                }
                Statement::Select => db.select(),
                Statement::Unrecognized(cmd) => {
                    println!("Unrecognized command '{:?}'.", cmd);
                }
            },
        }
    }
}
