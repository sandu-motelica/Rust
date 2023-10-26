#[derive(Debug)]
enum MyError {
    Overflow,
}

fn checked_add(a: u32, b: u32) -> Result<u32, MyError> {
    if b > std::u32::MAX - a {
        Err(MyError::Overflow)
    } else {
        Ok(a + b)
    }
}

fn checked_mul(a: u32, b: u32) -> Result<u32, MyError> {
    if a == 0 || b == 0 {
        Ok(0)
    } else if b > std::u32::MAX / a {
        Err(MyError::Overflow)
    } else {
        Ok(a * b)
    }
}

fn checked_operations(a: u32, b: u32) -> Result<u32, MyError> {
    let suma = checked_add(a, b)?;
    let produs = checked_mul(suma, 2)?;
    Ok(produs)
}

fn main() {
    let x = 4294967295;
    let y = 2;

    match checked_operations(x, y) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => {
            eprintln!("Error: {:?}", err);
        }
    }
}
