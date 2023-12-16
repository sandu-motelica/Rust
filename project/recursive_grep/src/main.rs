use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn cautare_subsir(continut_fisier: String, sir_cautat: &str) -> bool {
    // Folosesc alg Boyer-Moore pentru cautare
    let mut raspuns = false;
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
            raspuns = true;
        }

        i += skip[continut_fisier.as_bytes()[i] as usize];
    }
    raspuns
}

fn cautare_fisier(path: &Path, sir: &str) -> io::Result<bool> {
    let mut file = File::open(path)?;
    let mut continut = String::new();
    file.read_to_string(&mut continut)?;
    let rezultat = cautare_subsir(continut, sir);
    Ok(rezultat)
}

fn parcurgere_recursiva(director: &Path, sir: &str, rezultat: &mut bool) {
    if let Ok(intrari) = fs::read_dir(director) {
        for e in intrari.flatten() {
            let path = e.path();
            if !path.is_dir() {
                // Dacă este un fișier, efectuăm căutarea în fișier
                match cautare_fisier(&path, sir) {
                    Ok(true) => *rezultat = true,
                    Err(e) => eprintln!("Eroare în timpul citirii fișierului: {}", e),
                    _ => {}
                }
            } else {
                // Dacă este un director, apelăm recursiv funcția pe acel director
                parcurgere_recursiva(&path, sir, rezultat);
            }
        }
    }
}

fn main() {
    let director = Path::new("D:\\Exemplu");
    let sir_cautat = "Today";
    let mut rezultat = false;
    parcurgere_recursiva(director, sir_cautat, &mut rezultat);
    if !rezultat {
        println!("Nu s-a gasit!");
    }
}
