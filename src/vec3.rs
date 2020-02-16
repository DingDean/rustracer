use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Return an vector with identical parts
    pub fn all(x: f64) -> Vec3 {
        Vec3 { x, y: x, z: x }
    }

    pub fn zeros() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn make_unit_vector(&self) -> Vec3 {
        let k = 1.0 / self.length();
        Vec3 {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Vec3 {
            x: other * self.x,
            y: other * self.y,
            z: other * self.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        *self = Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self = Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        -1.0 * self
    }
}

#[cfg(test)]
mod test {
    use super::Vec3;

    #[test]
    fn add() {
        let lhs = Vec3::new(1.0, 2.0, 3.0);
        let rhs = Vec3::new(1.0, 3.0, 2.0);
        let rst = Vec3::new(2.0, 5.0, 5.0);
        assert_eq!(lhs + rhs, rst);
    }

    #[test]
    fn add_assign() {
        let mut lhs = Vec3::new(1.0, 2.0, 3.0);
        lhs += Vec3::new(1.0, 3.0, 2.0);
        let rst = Vec3::new(2.0, 5.0, 5.0);
        assert_eq!(lhs, rst);
    }

    #[test]
    fn sub() {
        let lhs = Vec3::new(1.0, 2.0, 3.0);
        let rhs = Vec3::new(1.0, 3.0, 2.0);
        let rst = Vec3::new(0.0, -1.0, 1.0);
        assert_eq!(lhs - rhs, rst);
    }

    #[test]
    fn sub_assign() {
        let mut lhs = Vec3::new(1.0, 2.0, 3.0);
        lhs -= Vec3::new(1.0, 3.0, 2.0);
        let rst = Vec3::new(0.0, -1.0, 1.0);
        assert_eq!(lhs, rst);
    }

    #[test]
    fn mul_vec3() {
        let lhs = Vec3::new(1.0, 2.0, 3.0);
        let rhs = Vec3::new(1.0, 3.0, 2.0);
        let rst = Vec3::new(1.0, 6.0, 6.0);
        assert_eq!(lhs * rhs, rst);
    }

    #[test]
    /// 标量 * 向量
    fn mul_f64() {
        let lhs = 3.0;
        let rhs = Vec3::new(1.0, 2.0, 3.0);
        let rst = Vec3::new(3.0, 6.0, 9.0);
        assert_eq!(lhs * rhs, rst);
    }

    #[test]
    fn mul_assign_vec3() {
        let mut lhs = Vec3::new(1.0, 2.0, 3.0);
        lhs *= Vec3::new(1.0, 3.0, 2.0);
        let rst = Vec3::new(1.0, 6.0, 6.0);
        assert_eq!(lhs, rst);
    }

    #[test]
    fn mul_assign_f64_test() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        v *= 3.0;
        let rst = Vec3::new(3.0, 6.0, 9.0);
        assert_eq!(v, rst)
    }

    #[test]
    fn div_vec3_test() {
        let lhs = Vec3::new(6.0, 4.0, 4.0);
        let rhs = Vec3::new(3.0, 2.0, 2.0);
        let rst = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(lhs / rhs, rst);
    }

    #[test]
    fn div_f64_test() {
        let lhs = Vec3::new(2.0, 4.0, 6.0);
        let rhs = 2.0;
        let rst = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(lhs / rhs, rst);
    }

    #[test]
    fn div_assign_vec3_test() {
        let mut lhs = Vec3::new(6.0, 4.0, 4.0);
        let rhs = Vec3::new(3.0, 2.0, 2.0);
        let rst = Vec3::new(2.0, 2.0, 2.0);
        lhs /= rhs;
        assert_eq!(lhs, rst);
    }

    #[test]
    fn div_assign_f64_test() {
        let mut lhs = Vec3::new(6.0, 4.0, 4.0);
        let rhs = 2.0;
        let rst = Vec3::new(3.0, 2.0, 2.0);
        lhs /= rhs;
        assert_eq!(lhs, rst);
    }

    #[test]
    fn length() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(v.length(), 3.0);
    }

    #[test]
    fn squared_length() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(v.squared_length(), 9.0);
    }

    #[test]
    fn dot_test() {
        let lhs = Vec3::new(1.0, -1.0, 0.0);
        let rhs = Vec3::new(1.0, 1.0, 0.0);
        let rst = 0.0;
        assert_eq!(lhs.dot(rhs), rst);
    }

    #[test]
    fn cross_test() {
        let lhs = Vec3::new(1.0, 0.0, 0.0);
        let rhs = Vec3::new(0.0, 1.0, 0.0);
        let rst = Vec3::new(0.0, 0.0, 1.0);
        assert_eq!(lhs.cross(rhs), rst);
    }

    #[test]
    fn neg_test() {
        let lhs = Vec3::all(1.0);
        let rst = Vec3::all(-1.0);
        assert_eq!(-lhs, rst);
    }

    #[test]
    fn dot_cross_test() {
        let v = Vec3::new(2.0, 3.0, 1.0);
        let u = Vec3::new(1.0, 2.0, 2.0);
        let t = v.cross(u);
        assert_eq!(t.dot(u), 0.0);
    }
}
