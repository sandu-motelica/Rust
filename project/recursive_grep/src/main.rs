use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn cautare_fisier(path: &Path, sir_cautat: &str) -> io::Result<bool> {
    let mut file = File::open(path)?;
    let mut continut_fisier = String::new();
    file.read_to_string(&mut continut_fisier)?;

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
            if !raspuns {
                // println!("Fisierul: {:?}", path.file_name().unwrap_or_default());
                println!("{}", path.display());
            }
            let linie = continut_fisier[..k].matches('\n').count() + 1;

            if let Some(continut_linie) = continut_fisier.lines().nth(linie - 1) {
                println!("{}: {}", linie, continut_linie);
            } else {
                println!("Indexul liniei depășește numărul total de linii.");
            }

            raspuns = true;
        }

        i += skip[continut_fisier.as_bytes()[i] as usize];
    }
    Ok(raspuns)
}

fn parcurgere_recursiva(director: &Path, sir: &str, rezultat: &mut bool) {
    if let Ok(intrari) = fs::read_dir(director) {
        for e in intrari.flatten() {
            let path = e.path();
            if !path.is_dir() {
                // Dacă este un fișier, efectuăm căutarea în fișier
                match cautare_fisier(&path, sir) {
                    Ok(true) => *rezultat = true,
                    Err(e) => {
                        if let Some(eroare) = e.source() {
                            if eroare.is::<std::str::Utf8Error>() {
                                // Ignorăm fișierele care nu sunt în format UTF-8 valid
                                continue;
                            } else {
                                eprintln!(
                                    "Eroare în timpul citirii fișierului {}: {}",
                                    path.display(),
                                    e
                                );
                            }
                        }
                    }
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
    let argumente: Vec<String> = env::args().collect();
    if argumente.len() < 3 {
        eprintln!("Format: {} <adresa_directot> <sir_cautat>", argumente[0]);
        std::process::exit(1);
    }
    let mut rezultat = false;

    let director = Path::new(&argumente[1]);
    let sir_cautat = &argumente[2];
    println!("Adresa: {}, sirul {}", director.display(), sir_cautat);
    parcurgere_recursiva(director, sir_cautat, &mut rezultat);
    if !rezultat {
        println!("Nu s-a gasit!");
    }
}
