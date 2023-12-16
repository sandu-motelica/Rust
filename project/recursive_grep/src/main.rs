use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn cautare_fisier(path: &Path, sir_cautat: &str, count: bool, ignore: bool) -> io::Result<bool> {
    let mut file = File::open(path)?;
    let mut continut_fisier = String::new();
    file.read_to_string(&mut continut_fisier)?;

    // Folosesc alg Boyer-Moore pentru cautare
    let mut counter = 0;
    let copie_continut = continut_fisier.clone(); // fac o copie pt a afisa continult original in cazul in care ignore == true
    let sir;
    if ignore {
        sir = sir_cautat.to_lowercase();
        continut_fisier = continut_fisier.to_lowercase();
    } else {
        sir = sir_cautat.to_string();
    }

    let mut raspuns = false;
    let m = sir.len();
    let mut skip = vec![m; 256];

    let n = continut_fisier.len();

    let mut i = 0;
    while i < m - 1 {
        let caracter = sir.as_bytes()[i];
        skip[caracter as usize] = m - 1 - i;
        i += 1;
    }

    let mut i = m - 1;
    while i < n {
        let mut j = m - 1;
        let mut k = i;

        while j > 0 && continut_fisier.as_bytes()[k] == sir.as_bytes()[j] {
            j -= 1;
            k -= 1;
        }

        if continut_fisier.as_bytes()[k] == sir.as_bytes()[j] && j == 0 {
            if !raspuns {
                // println!("Fisierul: {:?}", path.file_name().unwrap_or_default());
                println!("{}", path.display());
            }
            if count {
                counter += 1;
            } else {
                let linie = continut_fisier[..k].matches('\n').count() + 1;

                if let Some(continut_linie) = copie_continut.lines().nth(linie - 1) {
                    println!("{}: {}", linie, continut_linie);
                } else {
                    println!("Indexul liniei depășește numărul total de linii.");
                }
            }

            raspuns = true;
        }

        i += skip[continut_fisier.as_bytes()[i] as usize];
    }
    if counter > 0 {
        println!("{} coincidente", counter);
    }
    Ok(raspuns)
}

fn parcurgere_recursiva(
    director: &Path,
    sir: &str,
    rezultat: &mut bool,
    count: bool,
    ignore: bool,
) {
    if let Ok(intrari) = fs::read_dir(director) {
        for e in intrari.flatten() {
            let path = e.path();
            if !path.is_dir() {
                // Dacă este un fișier, efectuăm căutarea în fișier
                match cautare_fisier(&path, sir, count, ignore) {
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
                parcurgere_recursiva(&path, sir, rezultat, count, ignore);
            }
        }
    }
}

fn main() {
    let argumente: Vec<String> = env::args().collect();
    if argumente.len() < 3 {
        eprintln!("Format: {} <adresa_directot> <sir_cautat> [-count] [-ignore] [-max:numarul maxim de linii analizate]", argumente[0]);
        std::process::exit(1);
    }
    let mut rezultat = false;

    let director = Path::new(&argumente[1]);
    let sir_cautat = &argumente[2];
    let mut count = false;
    let mut ignore = false;

    let optiuni: Vec<String> = (argumente[3..]).to_vec();
    // println!("Adresa: {}, sirul {}", director.display(), sir_cautat);
    for optiune in optiuni {
        match optiune.as_str() {
            "-count" => count = true,
            "-ignore" => ignore = true,
            _ => {
                eprintln!("Format: {} <adresa_directot> <sir_cautat> [-count] [-ignore] [-max:numarul maxim de linii analizate]", argumente[0]);
                std::process::exit(1);
            }
        }
    }

    parcurgere_recursiva(director, sir_cautat, &mut rezultat, count, ignore);
    if !rezultat {
        println!("Nu s-a gasit!");
    }
}
