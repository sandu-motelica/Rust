use std::{fs, io};

fn main() -> io::Result<()> {
    let host = fs::read_to_string("C:\\Windows\\System32\\drivers\\etc\\hosts")?;

    for each_line in host.lines() {
        let trimmed_each_line = each_line.trim();

        if !trimmed_each_line.starts_with("#") && !trimmed_each_line.is_empty() {
            let mut columns_iterator = trimmed_each_line.split_whitespace();

            if let Some(first_column) = columns_iterator.next() {
                if let Some(second_column) = columns_iterator.next() {
                    println!("{} => {}", first_column, second_column);
                }
            }
        }
    }

    Ok(())
}
