use std::{fs, io};

fn main() -> Result<(), io::Error> {
    let s = fs::read_to_string("file.txt")?; // Use "?" to properly unwrap the Result

    let mut linie_biti = String::new();
    let mut linie_caractere = String::new();
    let mut max_nr_biti = 0;
    let mut max_nr_caractere = 0;

    for linie in s.lines() {
        let nr_biti = linie.len();
        let nr_caractere = linie.chars().count();

        if nr_caractere > max_nr_caractere {
            linie_caractere = linie.to_string();
            max_nr_caractere = nr_caractere;
        }

        if nr_biti > max_nr_biti {
            linie_biti = linie.to_string();
            max_nr_biti = nr_biti;
        }
    }

    println!(
        "Cea mai lunga linie ca caractere: {} : {}",
        max_nr_caractere, linie_caractere
    );
    println!(
        "Cea mai lunga linie ca biti: {} : {}",
        max_nr_biti, linie_biti
    );

    Ok(())
}
