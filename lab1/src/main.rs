fn prime(a: i32) -> bool {
    if a == 1 || a % 2 == 0 && a != 2 {
        return false;
    } else {
        let mut i = 3;
        while i * i <= a {
            if a % i == 0 {
                return false;
            }
            {
                i += 2;
            }
        }
    }

    return true;
}

fn prime_numbers() {
    println!("Numere prime");
    let mut i: i32 = 0;
    while i <= 100 {
        if prime(i) == true {
            print!("{i} ");
        }
        i += 1;
    }
    println!("")
}

fn coprime(mut a: i32, mut b: i32) -> bool {
    if a == 0 || b == 0 {
        return false;
    }

    if a == 1 || b == 1 {
        return true;
    }

    while a != b {
        if b > a {
            b -= a;
        } else {
            a -= b;
        }
    }

    if a == 1 {
        return true;
    } else {
        return false;
    }
}

fn coprime_numbers() {
    let mut i: i32 = 0;
    let mut j: i32;

    println!("Numere coprime");
    while i <= 100 {
        j = 0;
        while j <= 100 {
            if coprime(i, j) {
                println!("a:{i} b:{j}");
            }
            j += 1;
        }
        i += 1;
    }
}

fn bottles_of_beer() {
    let mut nr = 99;
    while nr > 0 {
        if nr > 2 {
            println!(
                "{} bottles of beer on the wall,
{} bottles of beer.
Take one down, pass it around,
{} bottles of beer on the wall.",
                nr,
                nr,
                nr - 1
            );
        } else if nr == 2 {
            println!(
                "2 bottles of beer on the wall,
2 bottles of beer.
Take one down, pass it around,
1 bottle of beer on the wall."
            );
        } else {
            println!(
                "1 bottle of beer on the wall,
1 bottle of beer.
Take one down, pass it around,
No bottle of beer on the wall."
            );
        }
        nr -= 1;
    }
}

fn main() {
    prime_numbers();
    coprime_numbers();
    bottles_of_beer();
}
