use std::ops::{Add, Sub, Mul, Div, Neg, DivAssign, MulAssign, AddAssign, SubAssign,
               Index, IndexMut};
use num_traits::identities::Zero;
use alga::linear::EuclideanSpace;
use alga::linear::AffineSpace;
use alga::linear::FiniteDimInnerSpace;
use alga::linear::FiniteDimVectorSpace;
use alga::linear::InnerSpace;
use alga::linear::VectorSpace;
use alga::linear::NormedSpace;
use alga::general::Module;
use alga::general::AbstractModule;
use alga::general::AbstractGroupAbelian;
use alga::general::AbstractGroup;
use alga::general::AbstractLoop;
use alga::general::AbstractMagma;
use alga::general::AbstractMonoid;
use alga::general::AbstractSemigroup;
use alga::general::AbstractQuasigroup;
use alga::general::Additive;
use alga::general::Identity;
use alga::general::TwoSidedInverse;

use rgsl::linear_algebra::SV_decomp;
use rgsl::types::matrix::MatrixF64;
use rgsl::types::vector::VectorF64;

//#[macro_use]
//extern crate alga_derive;
// use alga::general::Real;

#[derive(Debug, Copy, Clone, PartialEq/*, Alga*/)]
//#[alga_traits(GroupAbelian(Additive))]
struct Point(f64, f64, f64);

impl Point {

    fn project(&self, other: &Self) -> Self {
        *other * self.dot(other) / other.dot(other)
    }
}

impl DivAssign<f64> for Point {
    fn div_assign(&mut self, real: f64) {
        self.0 /= real;
        self.1 /= real;
        self.2 /= real;
    }
}

impl MulAssign<f64> for Point {
    fn mul_assign(&mut self, real: f64) {
        self.0 *= real;
        self.1 *= real;
        self.2 *= real;
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, real: f64) -> Self::Output {
        Self(self.0/real, self.1/real, self.2/real)
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, real: f64) -> Self::Output {
        Self(self.0*real, self.1*real, self.2*real)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0-other.0, self.1-other.1, self.2-other.2)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self(self.0+other.0, self.1+other.1, self.2+other.2)
    }
}

impl AddAssign for Point {

    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl SubAssign for Point {

    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl AffineSpace for Point {
    type Translation = <Self as EuclideanSpace>::Coordinates;
}

impl EuclideanSpace for Point {
    type Coordinates = Self;
    type RealField = f64;

    fn origin() -> Self {
        Self(0f64, 0f64, 0f64)
    }
}

impl VectorSpace for Point {
    type Field = <Self as EuclideanSpace>::RealField;
}

impl Module for Point {
    type Ring = <Self as VectorSpace>::Field;
}

impl AbstractModule for Point {
    type AbstractRing = <Self as Module>::Ring;

    fn multiply_by(&self, real: Self::AbstractRing) -> Self {
        self.clone() * real
    }
}

impl AbstractGroupAbelian<Additive> for Point {
}

impl AbstractGroup<Additive> for Point {
}

impl AbstractLoop<Additive> for Point {
}

impl AbstractQuasigroup<Additive> for Point {
}

impl TwoSidedInverse<Additive> for Point {
    fn two_sided_inverse(&self) -> Self {
        -self.clone()
    }
}

impl AbstractMonoid<Additive> for Point {
}

impl AbstractSemigroup<Additive> for Point {
}

impl AbstractMagma<Additive> for Point {
    fn operate(&self, other: &Self) -> Self {
        self.clone() + other.clone()
    }
}

impl Identity<Additive> for Point {
    fn identity() -> Self {
        Self(0f64, 0f64, 0f64)
    }
}

impl Zero for Point {
    fn zero() -> Self {
        Self::identity()
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl FiniteDimInnerSpace for Point {

    fn orthonormalize(vs: &mut [Self]) -> usize {
        let len = vs.len();
        let mut chosen = 0usize;
        let mut discarded = 0usize;
        println!("{}, {}, {}, {:?}", len, chosen, discarded, vs);
        while chosen < Self::dimension() && chosen + discarded < len {
            //println!("{}, {}, {}, {:?}", len, chosen, discarded, vs);
            for i in chosen..(len - discarded) {
                //println!("{}, {}, {}, {:?}", len, chosen, discarded, vs);
                if vs[i].is_zero() {
                    discarded += 1;
                    if chosen + discarded > 1 {
                        vs.swap(i, len - discarded);
                    }
                    break;
                } else {
                    for j in  0..chosen {
                        println!("{}, {}, {}, {}, {}, {:?}", len, chosen, discarded, i, j, vs);
                        vs[i] -= vs[i].project(&vs[j]);
                        println!("{}, {}, {}, {}, {}, {:?}", len, chosen, discarded, i, j, vs);
                    }
                    if vs[i].is_zero() {
                        discarded += 1;
                        if chosen + discarded > 1 {
                            vs.swap(i, len - discarded);
                        }
                        break;
                    } else {
                        chosen += 1;
                        vs[i].normalize_mut();
                        break;
                    }
                }
            }
            //println!("{}, {}, {}, {:?}", len, chosen, discarded, vs);
        }
        println!("{}, {}, {}, {:?}", len, chosen, discarded, vs);
        chosen
    }

    /// Applies the given closure to each element of the orthonormal basis of
    /// the subspace orthogonal to free family of vectors 'vs'. If vs is not a
    /// free family, the result is unspecified.

    fn orthonormal_subspace_basis<F: FnMut(&Self) -> bool>(vs: &[Self], _f:F) {
        let mut a = MatrixF64::new(vs.len(), Self::dimension()).unwrap();
        let mut r = 0;
        for i in vs {
            let mut v = VectorF64::new(Self::dimension()).unwrap();
            for j in 0..v.len() {
                v.set(j, i[j]);
            }
            a.set_row(r, &mut v);
            r+=1;
        }
        println!("a: {:?}", a);
        let mut v = MatrixF64::new(Self::dimension(), Self::dimension()).unwrap();
        let mut s = VectorF64::new(Self::dimension()).unwrap();
        let mut work = VectorF64::new(Self::dimension()).unwrap();
        SV_decomp(&mut a, &mut v, &mut s, &mut work);
        println!("vs: {:?}", vs);
        println!("a: {:?}", a);
        println!("v: {:?}", v);
        println!("s: {:?}", s);
    }
}

impl FiniteDimVectorSpace for Point {

    fn dimension() -> usize {
        3usize
    }

    fn canonical_basis_element(i: usize) -> Self {
        match i {
            0 => Self(1f64, 0f64, 0f64),
            1 => Self(0f64, 1f64, 0f64),
            2 => Self(0f64, 0f64, 1f64),
            _ => panic!()
        }
    }

    fn dot(&self, other: &Self) -> Self::Field {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    unsafe fn component_unchecked(&self, i: usize) -> &Self::Field {
        match i {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!()
        }
    }

    unsafe fn component_unchecked_mut(&mut self, i: usize) -> &mut Self::Field {
        match i {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!()
        }
    }
}

impl Index<usize> for Point {
    type Output = <Self as EuclideanSpace>::RealField;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!()
        }
    }
}

impl IndexMut<usize> for Point {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!()
        }
    }
}

impl InnerSpace for Point {
    fn inner_product(&self, other: &Self) -> Self::ComplexField {
        self.dot(other)
    }
}

impl NormedSpace for Point {
    type RealField = <Self as EuclideanSpace>::RealField;
    type ComplexField = Self::RealField;

    fn norm_squared(&self) -> Self::RealField {
        self.dot(&self)
    }

    fn norm(&self) -> Self::RealField {
        self.dot(&self).sqrt()
    }

    fn normalize(&self) -> Self {
        *self / self.norm()
    }

    fn normalize_mut(&mut self) -> Self::RealField {
        let norm = self.norm();
        *self /= norm;
        norm
    }

    fn try_normalize(&self, eps: Self::RealField) -> Option<Self> {
        let norm = self.norm();
        if norm <= eps {
            return None
        } else {
            Some(self.normalize())
        }
    }

    fn try_normalize_mut(&mut self, eps: Self::RealField) -> Option<Self::RealField> {
        let norm = self.norm();
        if norm <= eps {
            return None
        } else {
            Some(self.normalize_mut())
        }
    }
}

fn main() {
    let u = Point(2.0, 0.0, 0.0);
    let v = Point(3.0, 0.0, 9.0);
    let w = Point(5.0, 0.0, 0.0);
    let x = Point(0.0, 7.0, 0.0);
    let z = &mut [u, v, w, x];
    let _d = Point::orthonormalize(z);
    println!("{:?}", v.normalize());
    println!("{:?}", v.norm_squared());
    println!("{:?}", v.norm());
    println!("{:?}", v.normalize().norm());
    //println!("{:?}", Point::orthonormalize(z));
    println!("{:?}", z);
    println!("{:?}", Point::orthonormal_subspace_basis(z, |_s: &Point| {true}));
}
