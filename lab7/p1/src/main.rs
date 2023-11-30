use std::convert::From;
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Complex {
    rl: f64,
    im: f64,
}

impl Complex {
    fn new<T: Into<f64>, U: Into<f64>>(rl: T, im: U) -> Self {
        Self {
            rl: rl.into(),
            im: im.into(),
        }
    }

    fn conjugate(&self) -> Self {
        Self {
            rl: self.rl,
            im: -self.im,
        }
    }
}

impl<T: Into<Complex> + Clone> Add<T> for Complex {
    type Output = Self;

    fn add(self, other: T) -> Self {
        let other = other.clone().into();
        Self {
            rl: self.rl + other.rl,
            im: self.im + other.im,
        }
    }
}

impl<T: Into<Complex> + Clone> Sub<T> for Complex {
    type Output = Self;

    fn sub(self, other: T) -> Self {
        let other = other.clone().into();
        Self {
            rl: self.rl - other.rl,
            im: self.im - other.im,
        }
    }
}

impl<T: Into<Complex> + Clone> Mul<T> for Complex {
    type Output = Self;

    fn mul(self, other: T) -> Self {
        let other = other.clone().into();
        Self {
            rl: self.rl * other.rl - self.im * other.im,
            im: self.rl * other.im + self.im * other.rl,
        }
    }
}

impl From<i32> for Complex {
    fn from(num: i32) -> Self {
        Self {
            rl: num as f64,
            im: 0.0,
        }
    }
}

impl From<f64> for Complex {
    fn from(num: f64) -> Self {
        Self { rl: num, im: 0.0 }
    }
}

impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            rl: -self.rl,
            im: -self.im,
        }
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im < 0.0 {
            write!(f, "{}{}i", self.rl, self.im)
        } else if self.im == 0.0 {
            write!(f, "{}", self.rl)
        } else if self.rl == 0.0 {
            write!(f, "{}i", self.im)
        } else {
            write!(f, "{}+{}i", self.rl, self.im)
        }
    }
}
fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.rl, 1);
    assert_eq_rel!(a.im, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.rl, 3);
    assert_eq_rel!(c.im, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.im, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    // Note: .to_string() uses Display to format the type
    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.rl, -6);

    let j = -i + i;
    assert_eq_rel!(j.rl, 0);
    assert_eq_rel!(j.im, 0);

    println!("ok!");
}
