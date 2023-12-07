use std::{collections::HashMap, fs};

fn main() -> Result<(), std::io::Error> {
    let continut_fisier = fs::read_to_string("input.txt")?;

    let mut frecvente_cuvinte = HashMap::new();

    // numar frecventele cuvintelor
    continut_fisier
        .split(|c: char| !c.is_alphanumeric()) // furnizez cuvintele
        .filter(|cuvant| !cuvant.is_empty()) //
        .for_each(|cuvant| {
            *frecvente_cuvinte.entry(cuvant.to_lowercase()).or_insert(0) += 1;
        });

    //  sorteaz dupa frecventa apoi alfabetic
    let mut frecvente_cuvinte_sortate: Vec<_> = frecvente_cuvinte.into_iter().collect();
    frecvente_cuvinte_sortate.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

    // calculez lungimea celui mai mare cuvant pentru a afisa aliniat frecventele acestora
    let lungime_maxima_cuvant = frecvente_cuvinte_sortate
        .iter()
        .map(|(cuvant, _)| cuvant.len())
        .max()
        .unwrap_or(0);
    // afisez
    for (cuvant, frecventa) in frecvente_cuvinte_sortate {
        println!("{:<lungime_maxima_cuvant$} => {}", cuvant, frecventa);
    }

    Ok(())
}
