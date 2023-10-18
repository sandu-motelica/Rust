fn add_chars_n(s: &mut String, ch: char, mut nr: u32) {
    while nr > 0 {
        s.push(ch);
        nr -= 1;
    }
}

fn main() {
    let mut s = String::from("");
    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        add_chars_n(&mut s, c, 26 - i);
        i += 1;
    }
    print!("{}", s);
}
