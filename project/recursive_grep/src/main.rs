use std::{fs, io};

fn cautare_subsir(continut_fisier: String, sir_cautat: &str) {
    // Folosesc alg Boyer-Moore pentru cautare
    let m = sir_cautat.len();
    let mut skip = vec![m; 256];

    let n = continut_fisier.len();

    let mut i = 0;
    while i < m - 1 {
        let caracter = sir_cautat.as_bytes()[i];
        skip[caracter as usize] = m - 1 - i;
        i += 1;
    }

    let mut i = m - 1;
    while i < n {
        let mut j = m - 1;
        let mut k = i;

        while j > 0 && continut_fisier.as_bytes()[k] == sir_cautat.as_bytes()[j] {
            j -= 1;
            k -= 1;
        }

        if continut_fisier.as_bytes()[k] == sir_cautat.as_bytes()[j] && j == 0 {
            let index = k - continut_fisier[..k].rfind('\n').unwrap_or(0);
            let linie = continut_fisier[..k].matches('\n').count() + 1;
            println!("Gasit la linia {}, indexul {}.", linie, index);
        }

        i += skip[continut_fisier.as_bytes()[i] as usize];
    }
}

fn main() -> Result<(), io::Error> {
    let s = fs::read_to_string("fisier.txt")?;
    let sir_cautat = "Today";
    cautare_subsir(s, sir_cautat);
    Ok(())
}
