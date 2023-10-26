fn next_power_of_two(x: u32) -> Option<u32> {
    let mut y: u32 = x + 1;
    let max = std::u32::MAX;

    while y < max {
        if !is_power_of_two(y) {
            y = y + 1;
        } else {
            break;
        }
    }

    if y < max {
        Some(y)
    } else {
        None
    }
}

fn is_power_of_two(n: u32) -> bool {
    n != 0 && (n & (n - 1)) == 0 // puteri a lui doi sunt repr binar de forma 100...00 iar n & (n - 1) == 0 (pt ca n-1 ar fi 01111...11)
}

fn main() {
    let num = std::u32::MAX - 1;

    match next_power_of_two(num) {
        Some(power) => println!("The next power of 2 for {} is {}", num, power),
        None => println!("Cannot calculate the next power of 2 for the given input."),
    }
}
