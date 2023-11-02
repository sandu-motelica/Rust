fn rotate(c: char, baza: char) -> char {
    return ((c as u8 - baza as u8 + 13) % 26 + baza as u8) as char;
}

fn main() {
    let input = "Lorem Ipsum is ⛔ simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s.".to_string();

    let mut output = String::new();

    for c in input.chars() {
        let encrypted_char = if c.is_ascii() {
            if c >= 'a' && c <= 'z' {
                rotate(c, 'a')
            } else if c >= 'A' && c <= 'Z' {
                rotate(c, 'A')
            } else {
                c
            }
        } else {
            eprintln!("Error: Non-ASCII character encountered: '{}'", c);
            return;
        };
        output.push(encrypted_char);
    }
    println!("{}", output);
}
