#![allow(warnings, unsued)]
use rgsl::types::matrix::MatrixF64;
//use rgsl::types::matrix::MatrixF64View;
use rgsl::types::vector::VectorF64;
use rgsl::types::vector::VectorF64View;
use std::f64::consts::PI;

const NEWTONIAN_CONSTANT_OF_GRAVITATION: f64 = 6.674_30e-11;
const MOLAR_GAS_CONSTANT: f64 = 8.314_462_618;
const MEAN_MOLECULAR_WEIGHT: f64 = 0.5;

/// Henyey Matrix
pub struct Henyey {
    /// K - 1 shells (constant, integer)
    K: usize,
    /// Total mass (constant)
    M: f64,
    /// Total radius (constant)
    R: f64,

    /*
    /// Pressure (variable)
    P: f64,
    /// Temperature (variable)
    T: f64,
    /// Net energy per second passing outward through a sphere of radius r.
    l: f64,
    /// Mass (variable)
    m: f64,
    /// Radius (variable)
    r: f64,
    */

    /// Henyey matrix
    Y: VectorF64,
    pub H: MatrixF64,
}

impl Henyey {

    pub fn new(k: usize, m: f64, r: f64) -> Self {
        let h: MatrixF64 = MatrixF64::new(k*4-2, k*4-2).unwrap();
        let mut y: VectorF64 = VectorF64::new(k*4-2).unwrap();
        y.set_all(1.0);
        Henyey { K: k, M: m, R: r, Y: y, H: h }
    }


    pub fn C(&mut self) {
        let mut matrix
            = self.H.submatrix(3*self.K-2, 3*self.K-4, 4, 6);
        let y = &self.Y.as_slice().unwrap()[3*self.K-4..3*self.K];
        let derivative = |i, j| -> f64 {
            match (i, j) {
                _ => 7f64
            }
        };

        for i in 0..4 {
            for j in 0..6 {
                matrix.matrix_mut(|m| {m.unwrap().set(i,j,derivative(i,j));});
            }
        }
    }

    pub fn A(&mut self, k: usize) {
        let R = MOLAR_GAS_CONSTANT;
        let mu = MEAN_MOLECULAR_WEIGHT;
        let G = NEWTONIAN_CONSTANT_OF_GRAVITATION;
        let a = 1.0;
        let c = 1.0;
        let mass: [f64;4] = [5.0, 4.0, 1.0, 0.0];
        let kappa: [f64;4] = [5.0, 4.0, 1.0, 0.0];
        let mut matrix = self.H.submatrix(4*k-2, 4*(k-1), 4, 8);
        let y = &self.Y.as_slice().unwrap()[4*(k-1)..4*(k-1)+8];
        /*
        let r = || -> f64 {
            (mass[k] - mass[k+1]).recip() + 2f64 * R * (y[2] + y[6])
                / (PI * mu * (y[1] + y[5]) * (y[0] + y[4]).powi(3))
        };
        */

        let derivative = |i, j| -> f64 {
            match (i, j) {
                (0, 0) => {
                    (mass[k] - mass[k+1]).recip() + 2f64 * R * (y[2] + y[6])
                        / (PI * mu * (y[1] + y[5]) * (y[0] + y[4]).powi(3))
                },
                (0, 1) | (0, 5) => {
                    R * (y[2] + y[6])
                        / (PI * mu * (y[1] + y[5]).powi(2) * (y[0] + y[4]).powi(2))
                },
                (0, 2) | (0, 6) => {
                    -R * (y[2] + y[6])
                        / (PI * mu * (y[1] + y[5]) * (y[0] + y[4]).powi(2))
                },
                (0, 3) | (0, 7) => 0f64,
                (0, 4) => {
                    -(mass[k] - mass[k+1]).recip() + 2f64 * R * (y[2] + y[6])
                        / (PI * mu * (y[1] + y[5]) * (y[0] + y[4]).powi(3))
                },
                (1, 0) | (1, 4) => {
                    -G * (mass[k] + mass[k+1]) / (2.0 * PI * (y[0] + y[4]))
                },
                (1, 1) | (1, 5) => {
                    (mass[k] - mass[k+1]).recip()
                },
                (1, 2) | (1, 6) => 0f64,
                (1, 3) | (1, 7) => 0f64,
                (2, 0) | (2, 4) => {
                    -3.0 * (kappa[k] + kappa[k+1]) * (y[3] + y[7])
                        / (8.0 * PI.powi(2) * a * c
                           * (y[0] + y[4]).powi(5) * (y[2] + y[6]).powi(3))
                },
                (2, 1) | (2, 5) => {
                    3.0 * (y[3] + y[7])
                        / (2.0 * PI.powi(2) * a * c
                           * (y[0] + y[4]).powi(4) * (y[2] + y[6]).powi(3))
                },
                (2, 2) => {
                    (mass[k] - mass[k+1]).recip() - (kappa[k] + kappa[k+1])
                        * (y[3] + y[7])
                        / (2.0 * PI.powi(2) * a * c
                           * (y[0] + y[4]).powi(4) * (y[2] + y[6]).powi(4))
                },
                (2, 3) | (2, 7) => {
                    3.0 * (kappa[k] + kappa[k+1])
                        / (2.0 * PI.powi(2) * a * c
                           * (y[0] + y[4]).powi(4) * (y[2] + y[6]).powi(3))
                },
                (2, 6) => {
                    -(mass[k] - mass[k+1]).recip() - (kappa[k] + kappa[k+1])
                        * (y[3] + y[7])
                        / (2.0 * PI.powi(2) * a * c
                           * (y[0] + y[4]).powi(4) * (y[2] + y[6]).powi(4))
                },
                (3, 0) | (3, 4) => 0f64,
                (3, 1) | (3, 5) => 0f64,
                (3, 2) | (3, 6) => 0f64,
                (3, 3) => {
                    (mass[k] - mass[k+1]).recip()
                },
                (3, 7) => {
                    -(mass[k] - mass[k+1]).recip()
                },
                _ => 0.0,
            }
        };
        for i in 0..4 {
            for j in 0..8 {
                matrix.matrix_mut(|m| {m.unwrap().set(i,j,derivative(i,j));});
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let mut h = Henyey::new(4usize, 1.0, 1.0);
        h.A(4usize);
        //assert_eq!(h.H.get(0usize,0usize), 0f64);
        //assert_eq!(h.H.get(2usize,2usize), 1f64);
        //assert!(h.H.is_null());
    }
}
