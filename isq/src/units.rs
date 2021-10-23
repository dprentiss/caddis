use num::rational::Ratio;
use num::Zero;
use num::One;
use num::pow::Pow;
use std::fmt;
use std::ops::{Mul, Div};

const NUM_DIM: usize = 7usize;
const _DIMENSION_SYMBOLS: [&str; NUM_DIM] = ["L", "M", "T", "I", "Θ", "N", "J",];
const UNIT_SYMBOLS: [&str; NUM_DIM] = ["m", "kg", "s", "A", "K", "mol", "cd",];

type DimensionalExponent = Ratio<i8>;

#[derive(Debug, Clone, Copy)]
pub struct QuantityValue {
    pub number: f64,
    pub dimension: [DimensionalExponent; NUM_DIM],
    pub uncertainty: f64,
}

impl Default for QuantityValue {

    fn default() -> QuantityValue {
        QuantityValue {
            number: 0f64,
            dimension: [DimensionalExponent::zero(); NUM_DIM],
            uncertainty: 0f64,
        }
    }
}

impl fmt::Display for QuantityValue {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = self.number;
        let mut s = String::new();
        if self.uncertainty != 0.0 {
            //let mut e = n.log10().floor();
            let mut u = self.uncertainty;
            let d = u.log10().floor() * -1.0 + 1.0;
            println!("d: {}", d);
            u *= 10f64.powf(d);
            s.push_str(&format!("{:.*e}", (d + n.log10().floor())  as usize, n));
            //s.push_str(&n.to_string());
            s.push('(');
            s.push_str(&u.round().to_string());
            s.push(')');
        }
        s.push(' ');
        for (symbol, exponent) in UNIT_SYMBOLS.iter().zip(self.dimension.iter()) {
            if !exponent.is_zero() {
                s.push_str(symbol);
                if !exponent.is_one() {
                    s.push('^');
                    s.push('(');
                    s.push_str(&exponent.to_string());
                    s.push(')');
                }
                s.push(' ');
            }
        }
        write!(f, "{}", s)
    }
}

impl Pow<f64> for QuantityValue {
    type Output = Self;

    fn pow(self, exponent: f64) -> Self {
        let mut q = Self::default();
        q.number = self.number.pow(exponent);
        for i in 0..NUM_DIM {
            let exp = DimensionalExponent::approximate_float::<f64>(exponent).unwrap();
            q.dimension[i] = self.dimension[i] * exp
        }
        q
    }
}

impl Mul<f64> for QuantityValue {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        let mut q = self.clone();
        q.number = self.number * other;
        q
    }
}

impl Mul for QuantityValue {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut q = Self::default();
        q.number = self.number * other.number;
        for i in 0..NUM_DIM {
            q.dimension[i] = self.dimension[i] + other.dimension[i];
        }
        q.uncertainty = q.number.abs()
            * (self.uncertainty/self.number + other.uncertainty/other.number).sqrt();
        q
    }
}

impl Div for QuantityValue {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let mut q = Self::default();
        q.number = self.number / other.number;
        for i in 0..NUM_DIM {
            q.dimension[i] = self.dimension[i] - other.dimension[i];
        }
        q.uncertainty = q.number.abs()
            * (self.uncertainty/self.number + other.uncertainty/other.number).sqrt();
        q
    }
}

macro_rules! base_unit(
    ( $( $symbol:ident ),* ; $( $index:expr ),* ) => ( $(
        #[allow(dead_code,non_snake_case)]
        pub fn $symbol() -> QuantityValue {
            let mut q = QuantityValue::default();
            q.number = f64::one();
            q.dimension[$index] = DimensionalExponent::one();
            q
        }
    )* )
);

impl QuantityValue {

    base_unit!(m, kg, s, A, K, mol, cd; 0, 1, 2, 3, 4, 5, 6);

    #[allow(dead_code,non_snake_case)]
    fn W() -> QuantityValue {
        let mut q = QuantityValue::default();
        q.number = f64::one();
        q.dimension[0] = DimensionalExponent::one() * 2;
        q.dimension[1] = DimensionalExponent::one();
        q.dimension[2] = DimensionalExponent::one() * -3;
        q
    }
}

/*
fn main() {

    let a = QuantityValue{number:10000f64, uncertainty: 0.0001, ..QuantityValue::m() };
    let b = QuantityValue{number:10000f64, uncertainty: 0.001, ..QuantityValue::m() };
    let c = QuantityValue{number:10000f64, uncertainty: 0.01, ..QuantityValue::m() };
    let d = QuantityValue{number:10000f64, uncertainty: 0.1, ..QuantityValue::m() };
    let e = QuantityValue{number:10000f64, uncertainty: 1.0, ..QuantityValue::m() };
    let f = QuantityValue{number:10000f64, uncertainty: 10.0, ..QuantityValue::m() };
    let g = QuantityValue{number:10000f64, uncertainty: 100.0, ..QuantityValue::m() };
    let h = QuantityValue{number:10000f64, uncertainty: 1000.0, ..QuantityValue::m() };
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
    println!("e: {}", e);
    println!("f: {}", f);
    println!("g: {}", g);
    println!("h: {}", h);
    println!("{}", ' ');
    let a = QuantityValue{number:10000f64, uncertainty: 0.0005, ..QuantityValue::m() };
    let b = QuantityValue{number:10000f64, uncertainty: 0.005, ..QuantityValue::m() };
    let c = QuantityValue{number:10000f64, uncertainty: 0.05, ..QuantityValue::m() };
    let d = QuantityValue{number:10000f64, uncertainty: 0.5, ..QuantityValue::m() };
    let e = QuantityValue{number:10000f64, uncertainty: 5.0, ..QuantityValue::m() };
    let f = QuantityValue{number:10000f64, uncertainty: 50.0, ..QuantityValue::m() };
    let g = QuantityValue{number:10000f64, uncertainty: 500.0, ..QuantityValue::m() };
    let h = QuantityValue{number:10000f64, uncertainty: 5000.0, ..QuantityValue::m() };
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
    println!("e: {}", e);
    println!("f: {}", f);
    println!("g: {}", g);
    println!("h: {}", h);

    let z = QuantityValue{number:10.3608, uncertainty: 0.0005, ..QuantityValue::default() };
    let o = QuantityValue{number:50.1234567, uncertainty: 0.000543, ..QuantityValue::default() };
    //println!("{}", z);
    let u = z.uncertainty;
    let t = u.log10();
    let e = t.trunc();
    println!("{}", t);
    println!("{}", e);
    println!("z: {}", z);
    println!("o/z: {}", o/z);
    println!("o*z: {}", o*z);
    println!("o*z*z: {}", o*z*z);
    println!("o/z/z: {}", o/z/z);
    println!("{}", o/z/z/z);
    println!("{}", o*z*z*z);
}
*/
