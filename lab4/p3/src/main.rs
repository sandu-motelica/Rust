use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let mut original_text = fs::read_to_string("input.txt")?;
    original_text = transform_text(&original_text, "dl", "domnul");
    original_text = transform_text(&original_text, "dna", "doamna");
    original_text = transform_text(&original_text, "pt", "pentru");
    original_text = transform_text(&original_text, "ptr", "pentru");

    println!("{original_text}");

    Ok(())
}

fn transform_text(initial_text: &str, short_form: &str, long_form: &str) -> String {
    return initial_text.replace(short_form, long_form);
}
