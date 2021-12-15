use num::pow::Pow;
use num::rational::Ratio;
use num::One;
use num::Zero;
use std::fmt;
use std::ops::{Div, Mul};

const NUM_DIM: usize = 7usize;
const _DIMENSION_SYMBOLS: [&str; NUM_DIM] = ["L", "M", "T", "I", "Θ", "N", "J"];
const UNIT_SYMBOLS: [&str; NUM_DIM] = ["m", "kg", "s", "A", "K", "mol", "cd"];

type DimensionalExponent = Ratio<i8>;

#[derive(Debug, Clone, Copy, PartialEq)]
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
            s.push_str(&format!("{:.*e}", (d + n.log10().floor()) as usize, n));
            //s.push_str(&n.to_string());
            s.push('(');
            s.push_str(&u.round().to_string());
            s.push(')');
        } else {
            s.push_str(&n.to_string());
            s.push('(');
            s.push_str("...");
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
            * (self.uncertainty / self.number + other.uncertainty / other.number).sqrt();
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
            * (self.uncertainty / self.number + other.uncertainty / other.number).sqrt();
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

    #[allow(dead_code, non_snake_case)]
    fn W() -> QuantityValue {
        let mut q = QuantityValue::default();
        q.number = f64::one();
        q.dimension[0] = DimensionalExponent::one() * 2;
        q.dimension[1] = DimensionalExponent::one();
        q.dimension[2] = DimensionalExponent::one() * -3;
        q
    }

    #[allow(dead_code)]
    fn recip(&self) -> Self {
        let q = self.clone();
        q.pow(-1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let q = QuantityValue::default();
        assert_eq!(q.number, 0f64);
        assert_eq!(q.uncertainty, 0f64);
    }

    #[test]
    //#[should_panic]
    fn test_recip() {
        let q = QuantityValue::default();
        assert_eq!(q.number, 0f64);
        assert_eq!(q.uncertainty, 0f64);
        assert!(q.recip().number.is_infinite());
    }

    #[test]
    fn test_div_by_zero() {
        let r = QuantityValue::m();
        let rho = QuantityValue::kg() / r.pow(3.0);
        let mut f = QuantityValue::default();
        f.number = f64::one();
        f.dimension[0] = DimensionalExponent::one();
        f.dimension[1] = DimensionalExponent::one() * -1;
        assert_eq!(r.pow(-2.0) * rho.recip(), f);
    }
}
