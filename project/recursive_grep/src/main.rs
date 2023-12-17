use regex::Regex;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn cautare_fisier(
    path: &Path,
    sir_cautat: &str,
    count: bool,
    ignore: bool,
    max_linii: usize,
    regex: bool,
) -> io::Result<bool> {
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

    if regex {
        let sir_regex = match Regex::new(&sir) {
            Ok(regex) => regex,
            Err(err) => {
                eprintln!("Eroare la compilarea regex: {}", err);
                return Ok(false);
            }
        };
        for (linie, continut_linie) in continut_fisier.lines().enumerate() {
            if max_linii < linie + 1 {
                break;
            }
            if sir_regex.is_match(continut_linie) {
                if !raspuns {
                    println!("{}", path.display());
                    raspuns = true;
                }
                //println!("Gasit cu regex: {}", sir_regex);
                if count {
                    counter += 1;
                } else if ignore {
                    if let Some(continut_linie) = copie_continut.lines().nth(linie) {
                        println!("{}: {}", linie + 1, continut_linie);
                    } else {
                        println!("Indexul liniei depaseste numarul total de linii.");
                    }
                } else {
                    println!("{}: {}", linie + 1, continut_linie);
                }
            }
        }
    } else {
        let m = sir.len();
        let mut skip = vec![m; 256];

        let mut i = 0;
        while i < m - 1 {
            let caracter = sir.as_bytes()[i];
            skip[caracter as usize] = m - 1 - i;
            i += 1;
        }
        for (linie, continut_linie) in continut_fisier.lines().enumerate() {
            // let continut_linie = clinie?;
            if max_linii < linie + 1 {
                break;
            }
            let n = continut_linie.len();
            let mut i = m - 1;
            while i < n {
                let mut j = m - 1;
                let mut k = i;
                while j > 0 && continut_linie.as_bytes()[k] == sir.as_bytes()[j] {
                    j -= 1;
                    k -= 1;
                }

                if continut_linie.as_bytes()[k] == sir.as_bytes()[j] && j == 0 {
                    if !raspuns {
                        println!("{}", path.display());
                        raspuns = true;
                    }
                    if count {
                        counter += 1;
                    } else if ignore {
                        if let Some(continut_linie) = copie_continut.lines().nth(linie) {
                            println!("{}: {}", linie + 1, continut_linie);
                        } else {
                            println!("Indexul liniei depășește numărul total de linii.");
                        }
                    } else {
                        println!("{}: {}", linie + 1, continut_linie);
                    }
                }

                i += skip[continut_linie.as_bytes()[i] as usize];
            }
        }
    }
    if counter > 0 {
        println!("{}", counter);
    }
    Ok(raspuns)
}

fn parcurgere_recursiva(
    director: &Path,
    sir: &str,
    rezultat: &mut bool,
    count: bool,
    ignore: bool,
    max_linii: usize,
    regex: bool,
) {
    if let Ok(intrari) = fs::read_dir(director) {
        for e in intrari.flatten() {
            let path = e.path();
            if !path.is_dir() {
                // Dacă este un fișier, efectuăm căutarea în fișier
                match cautare_fisier(&path, sir, count, ignore, max_linii, regex) {
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
                parcurgere_recursiva(&path, sir, rezultat, count, ignore, max_linii, regex);
            }
        }
    }
}

fn main() {
    let argumente: Vec<String> = env::args().collect();
    if argumente.len() < 3 {
        eprintln!("Format acceptat: {} <adresa_director> <sir_cautat> [-count] [-ignore] [-max:numarul maxim de linii analizate] [-regex]", argumente[0]);
        std::process::exit(1);
    }
    let mut rezultat = false;

    let director = Path::new(&argumente[1]);
    let sir_cautat = &argumente[2];
    let mut max_linii = usize::MAX;
    let mut count = false;
    let mut ignore = false;
    let mut regex = false;

    // println!("Adresa: {}, sirul {}", director.display(), sir_cautat);
    let mut i = 3;
    while i < argumente.len() {
        let optiune = &argumente[i];
        match optiune.as_str() {
            "-count" => count = true,
            "-ignore" => ignore = true,
            "-regex" => regex = true,
            "-max" => {
                i += 1;
                if i < argumente.len() {
                    if let Ok(numar) = argumente[i].parse::<usize>() {
                        max_linii = numar;
                    } else {
                        eprintln!("Valoarea de dupa -max nu este un numar.\nFormat acceptat: {} <adresa_directot> <sir_cautat> [-count] [-ignore] [-max : numarul maxim de linii analizate] [-regex]",argumente[0]);
                        std::process::exit(2);
                    }
                } else {
                    eprintln!("Format necunoscut! Nu este precizat numarul de linii dupa argumentul '-max'.");
                    std::process::exit(3);
                }
            }
            _ => {
                eprintln!("Format acceptat: {} <adresa_director> <sir_cautat> [-count] [-ignore] [-max : numarul maxim de linii analizate] [-regex]", argumente[0]);
                std::process::exit(1);
            }
        }
        i += 1;
    }
    // println!("{max_linii}");
    parcurgere_recursiva(
        director,
        sir_cautat,
        &mut rezultat,
        count,
        ignore,
        max_linii,
        regex,
    );
    if !rezultat {
        println!("Nu s-a gasit!");
    }
}
