pub mod geometry;
pub mod units;
pub mod quantity;

use geometry::FiniteDimInnerSpace;
use geometry::NormedSpace;
use geometry::Vector;
use units::QuantityValue;

fn main() {

    let a = QuantityValue {
        number: 10000f64,
        uncertainty: 0.0001,
        ..QuantityValue::m()
    };
    let b = QuantityValue {
        number: 10000f64,
        uncertainty: 0.001,
        ..QuantityValue::m()
    };
    let c = QuantityValue {
        number: 10000f64,
        uncertainty: 0.01,
        ..QuantityValue::m()
    };
    let d = QuantityValue {
        number: 10000f64,
        uncertainty: 0.1,
        ..QuantityValue::m()
    };
    let e = QuantityValue {
        number: 10000f64,
        uncertainty: 1.0,
        ..QuantityValue::m()
    };
    let f = QuantityValue {
        number: 10000f64,
        uncertainty: 10.0,
        ..QuantityValue::m()
    };
    let g = QuantityValue {
        number: 10000f64,
        uncertainty: 100.0,
        ..QuantityValue::m()
    };
    let h = QuantityValue {
        number: 10000f64,
        uncertainty: 1000.0,
        ..QuantityValue::m()
    };
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
    println!("e: {}", e);
    println!("f: {}", f);
    println!("g: {}", g);
    println!("h: {}", h);
    println!("{}", ' ');
    let a = QuantityValue {
        number: 10000f64,
        uncertainty: 0.0005,
        ..QuantityValue::m()
    };
    let b = QuantityValue {
        number: 10000f64,
        uncertainty: 0.005,
        ..QuantityValue::m()
    };
    let c = QuantityValue {
        number: 10000f64,
        uncertainty: 0.05,
        ..QuantityValue::m()
    };
    let d = QuantityValue {
        number: 10000f64,
        uncertainty: 0.5,
        ..QuantityValue::m()
    };
    let e = QuantityValue {
        number: 10000f64,
        uncertainty: 5.0,
        ..QuantityValue::m()
    };
    let f = QuantityValue {
        number: 10000f64,
        uncertainty: 50.0,
        ..QuantityValue::m()
    };
    let g = QuantityValue {
        number: 10000f64,
        uncertainty: 500.0,
        ..QuantityValue::m()
    };
    let h = QuantityValue {
        number: 10000f64,
        uncertainty: 5000.0,
        ..QuantityValue::m()
    };
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
    println!("e: {}", e);
    println!("f: {}", f);
    println!("g: {}", g);
    println!("h: {}", h);

    let z = QuantityValue {
        number: 10.3608,
        uncertainty: 0.0005,
        ..QuantityValue::default()
    };
    let o = QuantityValue {
        number: 50.1234567,
        uncertainty: 0.000543,
        ..QuantityValue::default()
    };
    //println!("{}", z);
    let u = z.uncertainty;
    let t = u.log10();
    let e = t.trunc();
    println!("{}", t);
    println!("{}", e);
    println!("z: {}", z);
    println!("o/z: {}", o / z);
    println!("o*z: {}", o * z);
    println!("o*z*z: {}", o * z * z);
    println!("o/z/z: {}", o / z / z);
    println!("{}", o / z / z / z);
    println!("{}", o * z * z * z);

    let u = Vector::new(&[2.0, 0.0, 0.0]);
    let v = Vector::new(&[3.0, 0.0, 0.0]);
    //let v = Vector(VectorType::from_slice(&[3.0, 0.0, 0.0]).unwrap());
    let w = Vector::new(&[5.0, 0.0, 0.0]);
    let x = Vector::new(&[0.0, 7.0, 0.0]);
    let z = &mut [u, v, w, x];
    println!("{:?}", z);
    println!("{:?}", z[3]);
    println!("{:?}", z[3].normalize());
    let _d = Vector::orthonormalize(z);
    println!("{:?}", z[1]);
    println!("{:?}", z[1].normalize());
    println!("{:?}", z[1].norm_squared());
    println!("{:?}", z[1].norm());
    println!("{:?}", z[1].normalize().norm());
    //println!("{:?}", Point::orthonormalize(z));
    println!("{:?}", z);
    println!(
        "{:?}",
        Vector::orthonormal_subspace_basis(z, |_s: &Vector| { true })
    );
}
