use fraction::GenericFraction;
use std::fmt;
use std::ops::Add;

#[allow(dead_code)]
enum BaseQuantity {
    Length(GenericFraction<u8>),
    Mass(GenericFraction<u8>),
    Time(GenericFraction<u8>),
    ElectricCurrent(GenericFraction<u8>),
    ThermodynamicTemperature(GenericFraction<u8>),
    AmountOfASubstance(GenericFraction<u8>),
    LuminousIntensity(GenericFraction<u8>),
}

impl BaseQuantity {
    fn get_symbol(&self) -> &'static str {
        match *self {
            Self::Length(..) => "L",
            Self::Mass(..) => "M",
            Self::Time(..) => "T",
            Self::ElectricCurrent(..) => "I",
            Self::ThermodynamicTemperature(..) => "Θ",
            Self::AmountOfASubstance(..) => "N",
            Self::LuminousIntensity(..) => "J",
        }
    }

    fn get_exponent(&self) -> GenericFraction<u8> {
        match *self {
            Self::Length(f) => f,
            Self::Mass(f) => f,
            Self::Time(f) => f,
            Self::ElectricCurrent(f) => f,
            Self::ThermodynamicTemperature(f) => f,
            Self::AmountOfASubstance(f) => f,
            Self::LuminousIntensity(f) => f,
        }
    }
}

impl fmt::Display for BaseQuantity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}^{}", self.get_symbol(), self.get_exponent())
    }
}

struct QuantityDimension {
    dimension: [BaseQuantity; 7],
}

impl PartialEq for QuantityDimension {
    fn eq(&self, other: &Self) -> bool {
        for q in &self.dimension {
        }
    }
}

impl Add for QuantityDimension {
    type Output = Self;

    fn add(self, other:Self) -> Self {
        self
    }
}

impl fmt::Display for QuantityDimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for q in &self.dimension {
            if q.get_exponent().is_normal() {
                s.push_str(&q.to_string());
                s.push_str(" ");
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
    let q: BaseQuantity = BaseQuantity::Length(GenericFraction::new(2u8, 1u8));
    println!("{}", q.get_symbol());
    println!("{}", q.get_exponent());
    /*{println!("^({}/{})",
             q.get_exponent().numer().unwrap(),
             q.get_exponent().denom().unwrap());
    */

    let zero = GenericFraction::new(0u8, 1u8);
    let one = GenericFraction::new(1u8, 1u8);
    let one_half = GenericFraction::new(1u8, 2u8);
    let dim = QuantityDimension {
        dimension: [
            BaseQuantity::Length(one),
            BaseQuantity::Mass(one_half),
            BaseQuantity::Time(one),
            BaseQuantity::ElectricCurrent(zero),
            BaseQuantity::ThermodynamicTemperature(zero),
            BaseQuantity::AmountOfASubstance(zero),
            BaseQuantity::LuminousIntensity(zero),
        ],
    };
    println!("{}", dim);
    println!("{}", BaseQuantity::Length(one));
}
