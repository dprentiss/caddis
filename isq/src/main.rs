use std::fmt;
use fraction::{GenericFraction, Zero, One};
use std::ops::{Deref, DerefMut, Add, Sub, Mul, Div, Neg};

#[derive(Clone, Copy)]
struct Fraction(GenericFraction<u8>);

impl Mul<u8> for Fraction {
    type Output = Fraction;

    fn mul(self, other: u8) -> Self::Output {
        self
    }
}

impl Deref for Fraction {
    type Target = GenericFraction<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Fraction {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[allow(dead_code)]
enum BaseQuantity {
    Length,
    Mass,
    Time,
    ElectricCurrent,
    ThermodynamicTemperature,
    AmountOfASubstance,
    LuminousIntensity,
}

impl BaseQuantity {
    fn symbol(&self) -> &'static str {
        match *self {
            Self::Length => "L",
            Self::Mass => "M",
            Self::Time => "T",
            Self::ElectricCurrent => "I",
            Self::ThermodynamicTemperature => "Θ",
            Self::AmountOfASubstance => "N",
            Self::LuminousIntensity => "J",
        }
    }
}

#[derive(Clone,Copy)]
struct QuantityDimension {
    exponents: [Fraction;7],
}

impl QuantityDimension {
    const BASE_QUANTITIES: [BaseQuantity;7] = [
        BaseQuantity::Length,
        BaseQuantity::Mass,
        BaseQuantity::Time,
        BaseQuantity::ElectricCurrent,
        BaseQuantity::ThermodynamicTemperature,
        BaseQuantity::AmountOfASubstance,
        BaseQuantity::LuminousIntensity,
    ];

    fn pow(self, exp: u8) -> Self {
        let mut exponents: [Fraction;7] = [Fraction::zero();7];
        let iter = self.exponents.iter().zip(exponents.iter_mut());
        for (base, exponent) in iter {
            //*exponent = base * &exp;
        }
        QuantityDimension {exponents: exponents}
    }
}

impl Add for QuantityDimension {
    type Output = Self;

    fn add(self, _:Self) -> Self::Output {
        self
    }
}

impl Sub for QuantityDimension {
    type Output = Self;

    fn sub(self, _:Self) -> Self {
        self
    }
}

impl Neg for QuantityDimension {
    type Output = Self;

    fn neg(self) -> Self {
        self
    }
}

impl Mul for QuantityDimension {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut exponents: [Fraction;7] = [Fraction::zero();7];
        let iter = self.exponents.iter().zip(other.exponents.iter()).zip(exponents.iter_mut());
        for ((lhs, rhs), exponent) in iter {
            *exponent = *lhs + *rhs;
        }
        QuantityDimension {exponents: exponents}
    }
}

impl Div for QuantityDimension {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let mut exponents: [Fraction;7] = [Fraction::zero();7];
        let iter = self.exponents.iter().zip(other.exponents.iter()).zip(exponents.iter_mut());
        for ((lhs, rhs), exponent) in iter {
            *exponent = lhs - rhs;
        }
        QuantityDimension {exponents: exponents}
    }
}

impl fmt::Display for QuantityDimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for (quantity, &exponent) in Self::BASE_QUANTITIES.iter().zip(self.exponents.iter()) {
            if exponent.is_normal() {
                s.push_str(quantity.symbol());
                if exponent != Fraction::one() {
                    s.push('^');
                    s.push_str(&exponent.to_string());
                }
                s.push(' ');
            }
        }
        write!(f, "{}", s)
    }
}

#[allow(dead_code)]
struct QuantityValue {
    number: f32,
    reference: String,
}

#[allow(dead_code)]
struct Quantity {
    dimension: QuantityDimension,
    value: QuantityValue,
}

fn main() {

    let zero = Fraction::zero();
    let one = Fraction::one();
    let one_half = GenericFraction::new(1u8, 2u8);

    let exponents = [one, one, one_half, zero, one, zero, zero];
    let p = QuantityDimension { exponents: exponents };
    //let q = p.clone();

    println!("{}", p);
    println!("{}", p * p);
    println!("{}", p.pow(2u8));
    println!("{}", Fraction::one()+1u8);
}
