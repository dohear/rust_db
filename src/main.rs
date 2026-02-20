mod command;
mod input_buffer;

use command::Command;
use command::MetaCommand;
use command::Statement;
use input_buffer::InputBuffer;
use std::io::{self, Write};

const USERNAME_MAX: usize = 32;
const EMAIL_MAX: usize = 255;

fn print_prompt() {
    print!("db > ");
    io::stdout().flush().unwrap();
}

fn main() {
    let mut input_buffer = InputBuffer::new();

    let mut db = table::Table::new();

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
                    Ok(insert_stmt) => {
                        match db.insert(insert_stmt) {
                            Ok(inserted) => println!("Successfully inserted '{:?}'.", inserted),
                            Err(e) => println!("{:?}", e.to_string()),
                        };
                    }
                    Err(e) => println!("{:?}", e.to_string()),
                },
                Statement::Select => {
                    println!("This is where we would Select");
                }
                Statement::Unrecognized(cmd) => {
                    println!("Unrecognized command '{:?}'.", cmd);
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
        if input[2].len() > USERNAME_MAX {
            return Err(format!(
                "Username too long, must be less than {}",
                USERNAME_MAX
            ));
        }
        if input[3].len() > EMAIL_MAX {
            return Err(format!("email too long, must be less than {}", EMAIL_MAX));
        }
        let id = input[1].parse::<u32>().map_err(|_| "Failed to parse id")?;

        Ok(InsertStatement {
            id,
            username: input[2].clone(),
            email: input[3].clone(),
        })
    }
}

mod table {

    use std::io::Write;

    use crate::InsertStatement;

    const USERNAME_MAX: usize = 32;
    const EMAIL_MAX: usize = 255;
    const SERIALIZED_SIZE: usize = 4 + USERNAME_MAX + EMAIL_MAX;

    const PAGE_SIZE: usize = 4096;
    const ROWS_PER_PAGE: usize = PAGE_SIZE / SERIALIZED_SIZE;

    #[derive(Debug)]
    struct Row {
        id: u32,
        username: String,
        email: String,
    }

    #[derive(Debug, Clone)]
    struct SerializedRow {
        byte_arr: Vec<u8>,
    }

    impl SerializedRow {
        fn serialize(data: InsertStatement) -> Result<SerializedRow, String> {
            let mut buf = Vec::with_capacity(SERIALIZED_SIZE);

            buf.write_all(&data.id.to_le_bytes());

            let username_bytes = data.username.as_bytes();
            buf.write_all(username_bytes);
            buf.resize(4 + USERNAME_MAX, 0);

            let email_bytes = data.email.as_bytes();
            buf.write_all(email_bytes);
            buf.resize(SERIALIZED_SIZE, 0);

            let row = SerializedRow { byte_arr: buf };

            return Ok(row);
        }

        fn deserialize(bytes: &[u8]) -> Result<Row, String> {
            if bytes.len() < SERIALIZED_SIZE {
                return Err(format!("Not enough bytes to deserialize row"));
            }

            let id = u32::from_le_bytes(bytes[0..4].try_into().unwrap());

            let username_bytes = &bytes[4..4 + USERNAME_MAX];
            let username_len = username_bytes
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(USERNAME_MAX);
            let username = String::from_utf8_lossy(&username_bytes[..username_len]).to_string();

            let email_bytes = &bytes[4 + USERNAME_MAX..self::SERIALIZED_SIZE];
            let email_len = email_bytes
                .iter()
                .position(|&b| b == 0)
                .unwrap_or(EMAIL_MAX);
            let email = String::from_utf8_lossy(&email_bytes[..email_len]).to_string();

            Ok(Row {
                id,
                username,
                email,
            })
        }
    }

    struct Page {
        data: Box<[u8]>,
        num_rows: usize,
    }

    impl Page {
        fn new() -> Self {
            Page {
                data: vec![0u8; PAGE_SIZE].into_boxed_slice(),
                num_rows: 0usize,
            }
        }

        fn append_row(&mut self, row: SerializedRow) -> Result<(), String> {
            if self.num_rows >= ROWS_PER_PAGE {
                return Err("page full, need new page".to_string());
            }
            let offset = SERIALIZED_SIZE * self.num_rows;
            self.data[offset..offset + SERIALIZED_SIZE].copy_from_slice(&row.byte_arr);

            self.num_rows = self.num_rows + 1;

            Ok(())
        }

        fn read_row(&self, row_offset: usize) -> Result<Row, String> {
            if row_offset >= ROWS_PER_PAGE {
                return Err(format!("Row {} out of bounds", row_offset));
            }
            let offset = row_offset * SERIALIZED_SIZE;
            SerializedRow::deserialize(&self.data[offset..offset + SERIALIZED_SIZE])
        }

        fn is_full(&self) -> bool {
            self.num_rows >= PAGE_SIZE
        }
    }

    pub struct Table {
        pages: Vec<Page>,
        num_pages: usize,
    }

    impl Table {
        pub fn new() -> Self {
            Table {
                pages: Vec::new(),
                num_pages: 0,
            }
        }

        pub fn insert(&mut self, data: InsertStatement) -> Result<(), String> {
            let serialized = SerializedRow::serialize(data)?;
            if self.num_pages == 0 {
                self.pages[0] = Page::new();
                self.num_pages = self.num_pages + 1;
            }
            if self.pages[self.num_pages - 1].is_full() {
                self.pages[self.num_pages] = Page::new();
                self.num_pages = self.num_pages + 1;
            }
            self.pages[self.num_pages - 1].append_row(serialized)?;

            Ok(())
        }

        pub fn select(&self, row_num: usize) -> Row {
            todo!()
        }
    }
}
