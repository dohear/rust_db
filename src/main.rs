mod command;
mod input_buffer;

use command::Command;
use command::MetaCommand;
use command::Statement;
use input_buffer::InputBuffer;
use std::io::{self, Write};

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut input_buffer = InputBuffer::new();

    loop {
        print_prompt();
        input_buffer.read_input();

        match Command::parse(&input_buffer.buffer) {
            Command::MetaCommand(meta) => match meta {
                MetaCommand::Exit => {
                    std::process::exit(0);
                }
                MetaCommand::Unrecognized(cmd) => {
                    println!("Unrecognized command '{}'.", cmd);
                }
            },
            Command::Statement(stmt) => match stmt {
                Statement::Insert => match InsertStatement::new(&input_buffer.buffer) {
                    Ok(inserted) => println!("Successfully inserted '{:?}'.", inserted),
                    Err(e) => println!("{:?}", e.to_string()),
                },
                Statement::Select => {
                    println!("This is where we would Select");
                }
                Statement::Unrecognized(cmd) => {
                    println!("Unrecognized command '{}'.", cmd);
                }
            },
        }
    }
}

#[derive(Debug)]
struct InsertStatement {
    id: u32,
    username: String,
    email: String,
}

impl InsertStatement {
    pub fn new(input: &[String]) -> Result<InsertStatement, String> {
        if input.len() < 4 {
            return Err("Not enough arguments".to_string());
        }

        let id = input[1].parse::<u32>().map_err(|_| "Failed to parse id")?;

        Ok(InsertStatement {
            id,
            username: input[2].clone(),
            email: input[3].clone(),
        })
    }
}

struct Row {
    id: u32,
    username: String,
    email: String,
}
