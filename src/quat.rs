use std::ops::{Mul, Add};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Quat {
    w: f64,
    i: f64,
    j: f64,
    k: f64
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Quat {
    pub fn new<T: Into<f64>>(w: T, i: T, j: T, k: T) -> Self {
        Quat {
            w: w.into(),
            i: i.into(),
            j: j.into(),
            k: k.into()
        }
    }

    pub fn normalize(&self) -> Self {
        let norm = f64::sqrt(self.sum_square());

        Quat {
            w: self.w / norm,
            i: self.i / norm,
            j: self.j / norm,
            k: self.k / norm
        }
    }

    pub fn inverse(&self) -> Self {
        let factor = 1.0/self.sum_square();

        Quat {
            w: self.w * factor,
            i: -self.i * factor,
            j: -self.j * factor,
            k: -self.k * factor
        }
    }

    pub fn tranform(&self, tf: Quat) -> Quat {
        tf * *self * tf.inverse()
    }

    pub fn increment(&mut self, x: f64, y: f64, z: f64) {

        let cos = |v: f64| {(v * 0.5).cos()};
        let sin = |v: f64| {(v * 0.5).sin()};
        
        let cx = cos(x);
        let sx = sin(x);

        let cy = cos(y);
        let sy = sin(y);

        let cz = cos(z);
        let sz = sin(z);

        self.w = cx * cy * cz + sx * sy * sz;
        self.i = sx * cy * cz - cx * sy * sz;
        self.j = cx * sy * cz + sx * cy * sz;
        self.k = cx * cy * sz - sx * sy * cz;

        self.normalize();
    }

    pub fn point(&self) -> Point {
        Point {
            x: self.i,
            y: self.j,
            z: self.k
        }
    }

    fn sum_square(&self) -> f64 {
        self.w*self.w + self.i*self.i + self.j*self.j + self.k*self.k
    }
}

impl Mul for Quat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let w = self.w*rhs.w - self.i*rhs.i - self.j*rhs.j - self.k*rhs.k;
        let i = self.w*rhs.i + self.i*rhs.w + self.j*rhs.k - self.k*rhs.j;
        let j = self.w*rhs.j - self.i*rhs.k + self.j*rhs.w + self.k*rhs.i;
        let k = self.w*rhs.k + self.i*rhs.j - self.j*rhs.i + self.k*rhs.w;

        Quat {
            w,
            i,
            j,
            k
        }
    }
}

impl Add for Quat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Quat {
            w: self.w+rhs.w,
            i: self.i+rhs.i,
            j: self.j+rhs.j,
            k: self.k+rhs.k
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::quat::Quat;
    #[test]
    fn multiplication() {
        let q1 = Quat::new(10.0, 5.0, 4.0, 7.0);
        let q2 = Quat::new(9.0, 2.0, 33.0, 4.0);
        let q3 = q1 * q2;

        println!("{:?}", q3);
    }
}

