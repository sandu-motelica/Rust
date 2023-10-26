enum MyError {
    IsNotAscii,
    IsNotDigit,
    IsNotHexDigit,
    IsNotLetter,
    IsNotPrintable,
}

impl MyError {
    fn get_str(&self) -> &str {
        match self {
            MyError::IsNotAscii => "Character is not ASCII",
            MyError::IsNotDigit => "Character is not a digit",
            MyError::IsNotHexDigit => "Character is not a base16 digit",
            MyError::IsNotLetter => "Character is not a letter",
            MyError::IsNotPrintable => "Character is not printable",
        }
    }
}

fn is_ascii(c: char) -> bool {
    if c as u32 <= 127 {
        return true;
    } else {
        return false;
    }
}

fn is_letter(c: char) -> bool {
    if (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') {
        return true;
    } else {
        return false;
    }
}

fn is_printable(c: char) -> bool {
    if is_ascii(c) && (c >= ' ' && c <= '~') {
        return true;
    } else {
        return false;
    }
}

fn is_digit(c: char) -> bool {
    if c >= '0' && c <= '9' {
        return true;
    } else {
        return false;
    }
}
fn is_hexdigit(c: char) -> bool {
    if (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F') {
        return true;
    } else {
        return false;
    }
}

fn to_uppercase(c: char) -> Result<char, MyError> {
    if is_letter(c) {
        Ok(if c >= 'a' && c <= 'z' {
            (c as u8 - 32) as char
        } else {
            c
        })
    } else {
        Err(MyError::IsNotLetter)
    }
}

fn to_lowercase(c: char) -> Result<char, MyError> {
    if is_letter(c) {
        Ok(if c >= 'A' && c <= 'Z' {
            (c as u8 + 32) as char
        } else {
            c
        })
    } else {
        Err(MyError::IsNotLetter)
    }
}

fn print_char(c: char) -> Result<(), MyError> {
    if is_printable(c) {
        Ok(())
    } else {
        Err(MyError::IsNotPrintable)
    }
}
fn char_to_number(c: char) -> Result<u32, MyError> {
    if is_ascii(c) && is_digit(c) {
        Ok(c as u32 - '0' as u32)
    } else {
        if !is_ascii(c) {
            Err(MyError::IsNotAscii)
        } else {
            Err(MyError::IsNotDigit)
        }
    }
}

fn char_to_number_hex(c: char) -> Result<u32, MyError> {
    if is_ascii(c) && is_hexdigit(c) {
        Ok(if c >= 'a' {
            c as u32 - 'a' as u32 + 10
        } else if c >= 'A' {
            c as u32 - 'A' as u32 + 10
        } else {
            c as u32 - '0' as u32
        })
    } else {
        if !is_ascii(c) {
            Err(MyError::IsNotAscii)
        } else {
            Err(MyError::IsNotHexDigit)
        }
    }
}
fn print_error(err: MyError) {
    println!("{}", err.get_str());
}

fn main() {
    let c1 = 'a';
    let c2 = '7';
    let c3 = 'C';

    match to_uppercase(c1) {
        Ok(result) => println!("Uppercase: {}", result),
        Err(err) => print_error(err),
    }

    match to_lowercase(c2) {
        Ok(result) => println!("Lowercase: {}", result),
        Err(err) => print_error(err),
    }

    match print_char(c3) {
        Ok(_) => println!("Character is printable."),
        Err(err) => print_error(err),
    }

    match char_to_number(c2) {
        Ok(result) => println!("Number: {}", result + 3),
        Err(err) => print_error(err),
    }

    match char_to_number_hex(c3) {
        Ok(result) => println!("Hex Number: {}", result),
        Err(err) => print_error(err),
    }

    match char_to_number(c3) {
        Ok(result) => println!("Number: {}", result),
        Err(err) => print_error(err),
    }
}
