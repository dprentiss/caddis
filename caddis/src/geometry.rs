use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use std::f64;
use std::f64::consts;

use num_traits::identities::{One, Zero};

pub use alga::linear::{
    AffineSpace, EuclideanSpace, FiniteDimInnerSpace, FiniteDimVectorSpace, InnerSpace,
    NormedSpace, VectorSpace,
};

use alga::general::AbstractGroup;
use alga::general::AbstractGroupAbelian;
use alga::general::AbstractLoop;
use alga::general::AbstractMagma;
use alga::general::AbstractModule;
use alga::general::AbstractMonoid;
use alga::general::AbstractQuasigroup;
use alga::general::AbstractSemigroup;
use alga::general::Additive;
use alga::general::Identity;
use alga::general::Module;
use alga::general::TwoSidedInverse;

use rgsl::linear_algebra::SV_decomp;
use rgsl::types::matrix::MatrixF64;
use rgsl::types::vector::VectorF64;

type VectorType = VectorF64;

#[derive(Debug)]
pub struct Vector(VectorType);

type Scalar = f64;

impl Vector {
    const N: usize = 3;

    fn project(&self, other: &Self) -> Self {
        let mut v = other.0.clone().unwrap();
        v.scale(self.dot(other) / other.dot(other));
        Vector(v)
    }

    pub fn from_slice(s: &[Scalar]) -> Self {
        Vector(VectorType::from_slice(s).unwrap())
    }

    pub fn new() -> Self {
        Vector(VectorType::new(Self::N).unwrap())
    }
}

impl Clone for Vector {
    fn clone(&self) -> Self {
        Self(self.0.clone().unwrap())
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.0.equal(&other.0)
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut v = VectorF64::from_slice(self.0.as_slice().unwrap()).unwrap();
        v.add(&other.0);
        Self(v)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut v = VectorF64::from_slice(self.0.as_slice().unwrap()).unwrap();
        v.sub(&other.0);
        Self(v)
    }
}

impl Mul<<Vector as EuclideanSpace>::RealField> for Vector {
    type Output = Self;

    fn mul(self, s: Scalar) -> Self::Output {
        let mut v = VectorF64::from_slice(self.0.as_slice().unwrap()).unwrap();
        v.scale(s);
        Self(v)
    }
}

impl Div<<Vector as EuclideanSpace>::RealField> for Vector {
    type Output = Self;

    fn div(self, s: Scalar) -> Self::Output {
        let mut v = VectorF64::from_slice(self.0.as_slice().unwrap()).unwrap();
        v.scale(s.recip());
        Self(v)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut v = VectorF64::from_slice(self.0.as_slice().unwrap()).unwrap();
        v.scale(-1.0);
        Vector(v)
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        self.0.add(&other.0);
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Self) {
        self.0.sub(&other.0);
    }
}

impl MulAssign<<Vector as EuclideanSpace>::RealField> for Vector {
    fn mul_assign(&mut self, s: Scalar) {
        self.0.scale(s);
    }
}

impl DivAssign<<Vector as EuclideanSpace>::RealField> for Vector {
    fn div_assign(&mut self, s: Scalar) {
        self.0.scale(s.recip());
    }
}

impl EuclideanSpace for Vector {
    type Coordinates = Vector;
    type RealField = Scalar;

    fn origin() -> Self {
        Vector::new()
    }
}

impl AffineSpace for Vector {
    type Translation = <Self as EuclideanSpace>::Coordinates;
}

impl VectorSpace for Vector {
    type Field = <Self as EuclideanSpace>::RealField;
}

impl Module for Vector {
    type Ring = <Self as VectorSpace>::Field;
}

impl AbstractModule for Vector {
    type AbstractRing = <Self as Module>::Ring;

    fn multiply_by(&self, ring: Self::AbstractRing) -> Self {
        let mut v = self.0.clone().unwrap();
        v.scale(ring);
        Vector(v)
    }
}

impl AbstractGroupAbelian<Additive> for Vector {}

impl AbstractGroup<Additive> for Vector {}

impl AbstractLoop<Additive> for Vector {}

impl AbstractQuasigroup<Additive> for Vector {}

impl AbstractMagma<Additive> for Vector {
    fn operate(&self, other: &Self) -> Self {
        let mut v = self.0.clone().unwrap();
        v.add(&other.0);
        Vector(v)
    }
}

impl TwoSidedInverse<Additive> for Vector {
    fn two_sided_inverse(&self) -> Self {
        -Vector(self.0.clone().unwrap())
    }
}
impl Identity<Additive> for Vector {
    fn identity() -> Self {
        Vector(VectorType::new(Self::dimension()).unwrap())
    }
}

impl AbstractMonoid<Additive> for Vector {}

impl AbstractSemigroup<Additive> for Vector {}

impl Zero for Vector {
    fn zero() -> Self {
        Self::new()
    }

    fn is_zero(&self) -> bool {
        *self == Self::zero()
    }
}

impl FiniteDimInnerSpace for Vector {
    fn orthonormalize(vs: &mut [Self]) -> usize {
        let len = vs.len();
        let mut chosen = 0usize;
        let mut discarded = 0usize;
        //println!("{}, {}, {}, {:?}", len, chosen, discarded, vs);
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
                    for j in 0..chosen {
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
        //println!("{}, {}, {}, {:?}", len, chosen, discarded, vs);
        chosen
    }

    /// Applies the given closure to each element of the orthonormal basis of
    /// the subspace orthogonal to free family of vectors 'vs'. If vs is not a
    /// free family, the result is unspecified.

    fn orthonormal_subspace_basis<F: FnMut(&Self) -> bool>(vs: &[Self], _f: F) {
        let mut a = MatrixF64::new(vs.len(), Self::dimension()).unwrap();
        let mut r = 0;
        for i in vs {
            let mut v = VectorType::new(Self::dimension()).unwrap();
            for j in 0..v.len() {
                v.set(j, i[j]);
            }
            a.set_row(r, &mut v);
            r += 1;
        }
        println!("a: {:?}", a);
        let mut v = MatrixF64::new(Self::dimension(), Self::dimension()).unwrap();
        let mut s = VectorType::new(Self::dimension()).unwrap();
        let mut work = VectorType::new(Self::dimension()).unwrap();
        SV_decomp(&mut a, &mut v, &mut s, &mut work);
        println!("vs: {:?}", vs);
        println!("a: {:?}", a);
        println!("v: {:?}", v);
        println!("s: {:?}", s);
    }
}

impl InnerSpace for Vector {
    fn inner_product(&self, other: &Self) -> Self::ComplexField {
        let mut v = self.0.clone().unwrap();
        let mut p = Scalar::zero();
        v.mul(&other.0);
        for i in v.as_slice().unwrap() {
            p += i;
        }
        p
    }
}

impl NormedSpace for Vector {
    type RealField = <Self as EuclideanSpace>::RealField;
    type ComplexField = Self::RealField;

    fn norm_squared(&self) -> Self::RealField {
        self.dot(&self)
    }

    fn norm(&self) -> Self::RealField {
        self.dot(&self).sqrt()
    }

    fn normalize(&self) -> Self {
        let v = Vector(self.0.clone().unwrap());
        let n = v.norm();
        v / n
    }

    fn normalize_mut(&mut self) -> Self::RealField {
        let norm = self.norm();
        *self /= norm;
        norm
    }

    fn try_normalize(&self, eps: Self::RealField) -> Option<Self> {
        let norm = self.norm();
        if norm <= eps {
            return None;
        } else {
            Some(self.normalize())
        }
    }

    fn try_normalize_mut(&mut self, eps: Self::RealField) -> Option<Self::RealField> {
        let norm = self.norm();
        if norm <= eps {
            return None;
        } else {
            Some(self.normalize_mut())
        }
    }
}

impl FiniteDimVectorSpace for Vector {
    fn dimension() -> usize {
        3usize
    }

    fn canonical_basis_element(i: usize) -> Self {
        let mut v = VectorType::new(Vector::dimension()).unwrap();
        v.set(i, Scalar::one());
        Vector(v)
    }

    fn dot(&self, other: &Self) -> Self::Field {
        self.inner_product(other)
    }

    unsafe fn component_unchecked(&self, i: usize) -> &Self::Field {
        &self[i]
    }

    unsafe fn component_unchecked_mut(&mut self, i: usize) -> &mut Self::Field {
        &mut self[i]
    }
}

impl Index<usize> for Vector {
    type Output = <Self as EuclideanSpace>::RealField;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0.as_slice().unwrap()[index]
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0.as_slice_mut().unwrap()[index]
    }
}

pub struct Sphere {
    center: Option<Vector>,
    #[allow(dead_code)]
    nose: Option<Vector>,
    radius: Option<Scalar>,
}

impl Sphere {
    pub fn from_nose(center: &Vector, nose: &Vector) -> Self {
        Sphere {
            center: Some(center.clone()),
            nose: Some(nose.clone()),
            radius: Some(nose.norm()),
        }
    }

    pub fn from_radius(center: &Vector, radius: Scalar) -> Self {
        Sphere {
            center: Some(center.clone()),
            nose: None,
            radius: Some(radius),
        }
    }

    pub fn volume(&self) -> Scalar {
        self.radius.unwrap().powi(3) * consts::FRAC_PI_3 * 4.0
    }

    pub fn area(&self) -> Scalar {
        self.radius.unwrap().powi(2) * consts::PI * 4.0
    }

    pub fn contains(self, point: Vector) -> bool {
        (point - self.center.unwrap()).norm() <= self.radius.unwrap()
    }

    pub fn height(self, point: Vector) -> Scalar {
        (point - self.center.unwrap()).norm() - self.radius.unwrap()
    }
}

#[allow(dead_code)]
struct Circle {
    center: Vector,
    normal: Vector,
    radius: Scalar,
    area: Scalar,
}

#[cfg(test)]
mod tests {
}
