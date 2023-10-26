fn checked_add(a: u32, b: u32) -> u32 {
    if b > std::u32::MAX - a {
        panic!("Overflow occurred in u32 addition!");
    }
    return a + b;
}

fn checked_mul(a: u32, b: u32) -> u32 {
    if a == 0 || b == 0 {
        return 0;
    }

    if b > std::u32::MAX / a {
        panic!("Overflow occurred in u32 multiplication!");
    }

    return a * b;
}

fn main() {
    let res1 = checked_add(100, 200);
    let res2 = checked_add(4294967295, 999999);
    let res3 = checked_mul(std::u32::MAX, 2);
    print!("{res1}, {res2}, {res3}");
}
