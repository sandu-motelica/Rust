fn prime(a: u32) -> bool {
    if a == 1 || a % 2 == 0 && a != 2 {
        return false;
    } else {
        let mut i: u32 = 3;
        while i * i <= a {
            if a % i == 0 {
                return false;
            }
            {
                i += 2;
            }
        }
    }

    return true;
}

fn next_prime(x: u16) -> Option<u16> {
    let mut y: u32 = x as u32 + 1;
    let max = std::u16::MAX as u32;
    while y < max {
        if !prime(y) {
            y = y + 1;
        } else {
            break;
        }
    }
    if y < max {
        return Some(y as u16);
    } else {
        return None;
    }
}

fn main() {
    let mut i: u32 = 65000;

    loop {
        let x: u16;
        match next_prime(i as u16) {
            Some(value) => {
                print!("{value}\n");
                x = value;
            }
            None => {
                print!("None\n");
                break;
            }
        }

        i = x as u32;
    }
}
