use std::io::Write;

use crate::insert::InsertStatement;

pub const USERNAME_MAX: usize = 32;
pub const EMAIL_MAX: usize = 255;

// Total serialized row size: 4 bytes (u32 id) + username field + email field
const SERIALIZED_SIZE: usize = 4 + USERNAME_MAX + EMAIL_MAX;

// Page size matches typical OS memory page (4KB)
const PAGE_SIZE: usize = 4096;
const ROWS_PER_PAGE: usize = PAGE_SIZE / SERIALIZED_SIZE;

// Top-level table structure;
// owns a collection of fixed-size pages
#[derive(Debug)]
pub struct Table {
    pages: Vec<Page>,
    num_pages: usize,
}

// A single 4KB page holding a byte buffer and a row count
#[derive(Debug)]
pub struct Page {
    data: Box<[u8]>,
    num_rows: usize,
}

// A row packed into a flat byte array for insertion into the table
#[derive(Debug, Clone)]
pub struct SerializedRow {
    byte_arr: Vec<u8>,
}

// An in-memory, deserialized row with typed fields
#[derive(Debug)]
pub struct Row {
    id: u32,
    username: String,
    email: String,
}

impl SerializedRow {
    // Pack an `InsertStatement` into a fixed-width byte layout:
    // [0..4]               -> id (little-endian u32)
    // [4..4+USERNAME_MAX]  -> username, zero-padded
    // [4+USERNAME_MAX..]   -> email, zero-padded

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

        // first 4 bytes are the row id
        let id = u32::from_le_bytes(bytes[0..4].try_into().unwrap());

        // Next USERNAME_MAX bytes are the username field; trim at first null byte
        let username_bytes = &bytes[4..4 + USERNAME_MAX];
        let username_len = username_bytes
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(USERNAME_MAX);
        let username = String::from_utf8_lossy(&username_bytes[..username_len]).to_string();

        // Remaining bytes are the email field; trim at first null byte
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

impl Page {
    /// Allocate a zeroed 4KB page with no rows yet written
    fn new() -> Self {
        Page {
            data: vec![0u8; PAGE_SIZE].into_boxed_slice(),
            num_rows: 0usize,
        }
    }

    /// Copy a serialized row into the next available slot in this page's buffer
    fn append_row(&mut self, row: SerializedRow) -> Result<(), String> {
        if self.num_rows >= ROWS_PER_PAGE {
            return Err("page full, need new page".to_string());
        }
        let offset = SERIALIZED_SIZE * self.num_rows;
        self.data[offset..offset + SERIALIZED_SIZE].copy_from_slice(&row.byte_arr);

        self.num_rows = self.num_rows + 1;

        Ok(())
    }

    /// Deserialize and return the row at the given intra-page slot index
    fn read_row(&self, row_offset: usize) -> Result<Row, String> {
        if row_offset >= ROWS_PER_PAGE {
            return Err(format!("Row {} out of bounds", row_offset));
        }
        let offset = row_offset * SERIALIZED_SIZE;
        SerializedRow::deserialize(&self.data[offset..offset + SERIALIZED_SIZE])
    }

    fn is_full(&self) -> bool {
        self.num_rows >= ROWS_PER_PAGE
    }
}

impl Table {
    pub fn new() -> Self {
        Table {
            pages: Vec::new(),
            num_pages: 0,
        }
    }

    // Serialize data and append it to the last page, allocating a new page if needed
    pub fn insert(&mut self, data: InsertStatement) -> Result<(), String> {
        let serialized = SerializedRow::serialize(data)?;
        if self.num_pages == 0 {
            self.pages.push(Page::new());
            self.num_pages += 1;
        }
        if self.pages[self.num_pages - 1].is_full() {
            self.pages.push(Page::new());
            self.num_pages += 1;
        }
        self.pages[self.num_pages - 1].append_row(serialized)?;

        Ok(())
    }

    //
    pub fn select(&self) {
        for pg in &self.pages {
            for i in 0..pg.data.len() {
                println!("{:?}", pg.read_row(i).unwrap());
            }
        }
    }
}
