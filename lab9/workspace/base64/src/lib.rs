/// Codifică un slice de octeți într-un șir de caractere base64.
///
/// # Exemple
///
/// ```
/// use base64::codifica_base64;
///
/// let rezultat = codifica_base64(b"Hello, World!");
/// assert_eq!(rezultat, "SGVsbG8sIFdvcmxkIQ=");
/// ```
pub fn codifica_base64(input: &[u8]) -> String {
    let mut rezultat = String::new();

    for bucata in input.chunks(3) {
        match bucata.len() {
            1 => codifica_bucata_1(bucata, &mut rezultat),
            2 => codifica_bucata_2(bucata, &mut rezultat),
            3 => codifica_bucata_3(bucata, &mut rezultat),
            _ => unreachable!(),
        }
    }

    rezultat
}

fn codifica_bucata_1(bucata: &[u8], rezultat: &mut String) {
    let v1 = (bucata[0] >> 2) & 0b111111;
    let v2 = (bucata[0] << 4) & 0b111111;

    let c1 = codifica_caracter_base64(v1);
    let c2 = codifica_caracter_base64(v2);

    rezultat.push(c1);
    rezultat.push(c2);
    rezultat.push('=');
}

fn codifica_bucata_2(bucata: &[u8], rezultat: &mut String) {
    let v1 = (bucata[0] >> 2) & 0b111111;
    let v2 = ((bucata[0] << 4) | (bucata[1] >> 4)) & 0b111111;
    let v3 = (bucata[1] << 2) & 0b111111;

    let c1 = codifica_caracter_base64(v1);
    let c2 = codifica_caracter_base64(v2);
    let c3 = codifica_caracter_base64(v3);

    rezultat.push(c1);
    rezultat.push(c2);
    rezultat.push(c3);
}

fn codifica_bucata_3(bucata: &[u8], rezultat: &mut String) {
    let v1 = (bucata[0] >> 2) & 0b111111;
    let v2 = ((bucata[0] << 4) | (bucata[1] >> 4)) & 0b111111;
    let v3 = ((bucata[1] << 2) | (bucata[2] >> 6)) & 0b111111;
    let v4 = bucata[2] & 0b111111;

    let c1 = codifica_caracter_base64(v1);
    let c2 = codifica_caracter_base64(v2);
    let c3 = codifica_caracter_base64(v3);
    let c4 = codifica_caracter_base64(v4);

    rezultat.push(c1);
    rezultat.push(c2);
    rezultat.push(c3);

    // Adaugă caracterele de padding doar dacă este necesar
    if bucata.len() == 3 {
        rezultat.push(c4);
    }
}

fn codifica_caracter_base64(valor: u8) -> char {
    if valor < 26 {
        (valor + b'A') as char
    } else if valor < 52 {
        ((valor - 26) + b'a') as char
    } else if valor < 62 {
        ((valor - 52) + b'0') as char
    } else if valor == 62 {
        '+'
    } else {
        '/'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_codifica_base64() {
        // șir de octeți care nu necesită padding
        assert_eq!(codifica_base64(b"Hello, World!"), "SGVsbG8sIFdvcmxkIQ=");

        // șir de octeți care necesită padding
        assert_eq!(codifica_base64(b"Test"), "VGVzdA=");

        // șir de octeți gol
        assert_eq!(codifica_base64(b""), "");

        // șir de octeți de lungime 1
        assert_eq!(codifica_base64(b"A"), "QQ=");
    }
}
