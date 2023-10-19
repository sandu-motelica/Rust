fn add_spaces(s: &mut String, mut nr: u32) {
    while nr > 0 {
        s.push(' ');
        nr -= 1;
    }
}

fn add_str(s: &mut String, add: &str) {
    s.push_str(add);
}

fn add_integer(s: &mut String, mut nr: u32) {
    let mut reverse = 0;
    let mut i = 0;
    while nr > 0 {
        reverse = reverse * 10 + nr % 10;
        nr /= 10;
    }

    while reverse > 0 {
        let c = ((reverse % 10) as u8 + '0' as u8) as char;
        s.push(c);
        reverse /= 10;
        i += 1;
        if i % 3 == 0 && reverse > 0 {
            s.push('_');
        }
    }
}

fn add_float(s: &mut String, nr: f32, mut precision: u32) {
    let mut integer = nr as i32;
    let mut fractional = nr - integer as f32;

    if integer == 0 {
        s.push('0');
    } else {
        let mut reverse = 0;
        let mut i = 0;
        while integer > 0 {
            reverse = reverse * 10 + integer % 10;
            integer /= 10;
        }

        while reverse > 0 {
            let c = ((reverse % 10) as u8 + '0' as u8) as char;
            s.push(c);
            reverse /= 10;
            i += 1;
            if i % 3 == 0 && reverse > 0 {
                s.push('_');
            }
        }
    }
    s.push('.');

    while precision > 0 {
        fractional *= 10.0;
        s.push((fractional as u8 + '0' as u8) as char);
        fractional -= fractional as i32 as f32;
        precision -= 1;
    }
}

fn main() {
    let mut s = String::from("");
    add_spaces(&mut s, 40);
    add_str(&mut s, "I 💚");
    s.push_str("\n");
    add_spaces(&mut s, 40);
    add_str(&mut s, "RUST.");
    s.push_str("\n\n");
    add_spaces(&mut s, 4);
    add_str(&mut s, "Most");
    add_spaces(&mut s, 12);
    add_str(&mut s, "crate");
    add_spaces(&mut s, 6);
    add_integer(&mut s, 306437968);
    add_spaces(&mut s, 11);
    add_str(&mut s, "and");
    add_spaces(&mut s, 5);
    add_str(&mut s, "lastest");
    add_spaces(&mut s, 9);
    add_str(&mut s, "is");
    s.push_str("\n");
    add_spaces(&mut s, 9);
    add_str(&mut s, "downloaded");
    add_spaces(&mut s, 8);
    add_str(&mut s, "has");
    add_spaces(&mut s, 13);
    add_str(&mut s, "downloads");
    add_spaces(&mut s, 5);
    add_str(&mut s, "the");
    add_spaces(&mut s, 9);
    add_str(&mut s, "version");
    add_spaces(&mut s, 4);
    add_float(&mut s, 2.038, 3);
    s.push('.');
    s.push_str("\n");
    add_spaces(&mut s, 20); // am adaugat spatiile exact ca in exemplu
    println!("{s}");
}
