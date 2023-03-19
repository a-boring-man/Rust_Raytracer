use std::ops;

/* The 3D vector class */
#[derive(Clone, Debug, PartialEq)]
pub struct Vec3 {
    pub e: [f64;3]
}

pub fn sqr(n: f64) -> f64 {
    n * n
}

/**********************
 * Overload implementation section
 **********************/

/* Implement the + operator for Vec3 */
impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 { e: [ self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z() ] }
    }
}
impl<'a> ops::Add<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 { e: [ self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z() ] }
    }
}
impl ops::Add<& Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: & Vec3) -> Self::Output {
        Vec3 { e: [ self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z() ] }
    }
}
impl ops::Add<Vec3> for & Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 { e: [ self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z() ] }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.x();
        self.e[1] += rhs.y();
        self.e[2] += rhs.z();
    }
}
impl ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: &Vec3) {
        self.e[0] += rhs.x();
        self.e[1] += rhs.y();
        self.e[2] += rhs.z();
    }
}

/* Implement the - operator for Vec3 */
impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 { e: [ self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z() ] }
    }
}
impl<'a> ops::Sub<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 { e: [ self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z() ] }
    }
}
impl ops::Sub<& Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: & Vec3) -> Self::Output {
        Vec3 { e: [ self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z() ] }
    }
}
impl ops::Sub<Vec3> for & Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 { e: [ self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z() ] }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.e[0] -= rhs.x();
        self.e[1] -= rhs.y();
        self.e[2] -= rhs.z();
    }
}
impl ops::SubAssign<&Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: &Vec3) {
        self.e[0] -= rhs.x();
        self.e[1] -= rhs.y();
        self.e[2] -= rhs.z();
    }
}

/* Implement the * operator for Vec3 */
impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 { e: [ self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z() ] }
    }
}
impl<'a> ops::Mul<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 { e: [ self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z() ] }
    }
}
impl ops::Mul<& Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: & Vec3) -> Self::Output {
        Vec3 { e: [ self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z() ] }
    }
}
impl ops::Mul<Vec3> for & Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 { e: [ self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z() ] }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 { e: [self.x() * rhs, self.y() * rhs, self.z() * rhs] }
    }
}
impl ops::Mul<f64> for & Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 { e: [ self.x() * rhs, self.y() * rhs, self.z() * rhs ] }
    }
}
impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 { e: [ self * rhs.x(), self * rhs.y(), self * rhs.z() ] }
    }
}
impl ops::Mul<& Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: & Vec3) -> Self::Output {
        Vec3 { e: [ self * rhs.x(), self * rhs.y(), self * rhs.z() ] }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

/* Implement the / operator for Vec3 */
impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3 { e: [ self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z() ] }
    }
}
impl<'a> ops::Div<&'a Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3 { e: [ self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z() ] }
    }
}
impl ops::Div<& Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: & Vec3) -> Self::Output {
        Vec3 { e: [ self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z() ] }
    }
}
impl ops::Div<Vec3> for & Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 { e: [ self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z() ] }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let k: f64 = 1.0/rhs;
        Vec3 { e: [self.x() * k, self.y() * k, self.z() * k] }
    }
}
impl ops::Div<f64> for & Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        let k = 1.0 / rhs;
        Vec3 { e: [self.x() * k, self.y() * k, self.z() * k] }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        let l: f64 = 1.0 / rhs;
        self.e[0] *= l;
        self.e[1] *= l;
        self.e[2] *= l;
    }
}

/* Implement basic function for the vector struct */
impl Vec3 {

    /**********************
     * Creation section
     **********************/

    /* Create a new vector with initialized value */
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    /**********************
     * Methode section
     **********************/

    /* Static Methode */

    pub fn unit_vec(v: &Vec3) -> Vec3 {
        v / v.length()
    }
    
    /* Instance Methode */

    pub fn length2(&self) -> f64 {
        sqr(self.x()) + sqr(self.y()) + sqr(self.z())
    }

    pub fn length(&self) -> f64 {
        self.length2().sqrt()
    }

    pub fn normalize(&mut self) -> f64 {
        let l: f64 = self.length();
        *self /= l;
        l
    }

    pub fn normalized(&self) -> Vec3 {
        let mut v: Vec3 = self.clone();
        v.normalize();
        v
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f64 {
        v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3::new(  v1.y() * v2.z() - v1.z() * v2.y(),
                    v1.z() * v2.x() - v1.x() * v2.z(),
                    v1.x() * v2.y() - v1.y() * v2.x())
    }

    pub fn invert(&self) -> Vec3 {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }

    /**********************
     * Getter section
     **********************/

    /* Return the first vector component */
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    /* Return the second vector component */
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    /* Return the third vector component */
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    /* Color version */

    /* Return the first vector component */
    pub fn r(&self) -> f64 {
        self.e[0]
    }
    /* Return the second vector component */
    pub fn g(&self) -> f64 {
        self.e[1]
    }
    /* Return the third vector component */
    pub fn b(&self) -> f64 {
        self.e[2]
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec_construction() {
        assert_eq!(Vec3::new(42.0, 21.4, 4.5), Vec3 {e: [42.0, 21.4, 4.5]});
    }

    #[test]
    fn test_vec_add() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 6.0, 8.7), Vec3::new(5.0, 8.0, 11.7));
    }

    #[test]
    fn test_vec_addrr() {
        assert_eq!(&Vec3::new(1.0, 2.0, 3.0) + &Vec3::new(4.0, 6.0, 8.7), Vec3::new(5.0, 8.0, 11.7));
    }

    #[test]
    fn test_vec_addnr() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) + &Vec3::new(4.0, 6.0, 8.7), Vec3::new(5.0, 8.0, 11.7));
    }

    #[test]
    fn test_vec_addrn() {
        assert_eq!(&Vec3::new(1.0, 2.0, 3.0) + Vec3::new(4.0, 6.0, 8.7), Vec3::new(5.0, 8.0, 11.7));
    }

    #[test]
    fn test_vec_addas() {
        let mut v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        v1 += Vec3::new(4.0, 6.0, 8.7);
        assert_eq!(v1, Vec3::new(5.0, 8.0, 11.7));
    }

    #[test]
    fn test_vec_addasr() {
        let mut v1: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3 = Vec3::new(4.0, 6.0, 8.7);
        v1 += &v2;
        assert_eq!(v1, Vec3::new(5.0, 8.0, 11.7));
        assert_eq!(v2, Vec3::new(4.0, 6.0, 8.7));
    }

    #[test]
    fn test_vec_sub() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) - Vec3::new(4.0, 6.0, 8.0), Vec3::new(-3.0, -4.0, -5.0));
    }

    #[test]
    fn test_vec_subrr() {
        assert_eq!(&Vec3::new(1.0, 2.0, 3.0) - &Vec3::new(4.0, 6.0, 8.0), Vec3::new(-3.0, -4.0, -5.0));
    }

    #[test]
    fn test_vec_subnr() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) - &Vec3::new(4.0, 6.0, 8.0), Vec3::new(-3.0, -4.0, -5.0));
    }

    #[test]
    fn test_vec_subrn() {
        assert_eq!(&Vec3::new(1.0, 2.0, 3.0) - Vec3::new(4.0, 6.0, 8.0), Vec3::new(-3.0, -4.0, -5.0));
    }

    #[test]
    fn test_vec_subas() {
        let mut v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3 = Vec3::new(4.0, 6.0, 8.0);
        v -= v2;
        assert_eq!(v, Vec3::new(-3.0, -4.0, -5.0));
    }

    #[test]
    fn test_vec_subasr() {
        let mut v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        let v2: Vec3 = Vec3::new(4.0, 6.0, 8.0);
        v -= &v2;
        assert_eq!(v, Vec3::new(-3.0, -4.0, -5.0));
        assert_eq!(v2, Vec3::new(4.0, 6.0, 8.0));
    }

    #[test]
    fn test_vec_mul() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * Vec3::new(4.0, 6.0, 8.0), Vec3::new(4.0, 12.0, 24.0));
    }

    #[test]
    fn test_vec_mulrr() {
        assert_eq!(&Vec3::new(1.0, 2.0, 3.0) * &Vec3::new(4.0, 6.0, 8.0), Vec3::new(4.0, 12.0, 24.0));
    }

    #[test]
    fn test_vec_mulnr() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * &Vec3::new(4.0, 6.0, 8.0), Vec3::new(4.0, 12.0, 24.0));
    }

    #[test]
    fn test_vec_mulrn() {
        assert_eq!(&Vec3::new(1.0, 2.0, 3.0) * Vec3::new(4.0, 6.0, 8.0), Vec3::new(4.0, 12.0, 24.0));
    }

    #[test]
    fn test_vec_mulvf() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * 3.0, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_vec_mulfv() {
        assert_eq!(3.0 * Vec3::new(1.0, 2.0, 3.0), Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_vec_mulrf() {
        assert_eq!(&Vec3::new(1.0, 2.0, 3.0) * 3.0, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_vec_mulfr() {
        assert_eq!(3.0 * &Vec3::new(1.0, 2.0, 3.0), Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_vec_mulsa() {
        let mut v: Vec3 = Vec3::new(1.0, 2.0, 3.0);
        v *= 3.0;
        assert_eq!(v, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_vec_div() {
        assert_eq!(Vec3::new(1.0, 6.0, 7.5) / Vec3::new(4.0, 2.0, 3.0), Vec3::new(0.25, 3.0, 2.5));
    }

    #[test]
    fn test_vec_divrr() {
        assert_eq!(&Vec3::new(1.0, 6.0, 7.5) / &Vec3::new(4.0, 2.0, 3.0), Vec3::new(0.25, 3.0, 2.5));
    }

    #[test]
    fn test_vec_divnr() {
        assert_eq!(Vec3::new(1.0, 6.0, 7.5) / &Vec3::new(4.0, 2.0, 3.0), Vec3::new(0.25, 3.0, 2.5));
    }

    #[test]
    fn test_vec_divrn() {
        assert_eq!(&Vec3::new(1.0, 6.0, 7.5) / Vec3::new(4.0, 2.0, 3.0), Vec3::new(0.25, 3.0, 2.5));
    }

    #[test]
    fn test_vec_divf() {
        assert_eq!(Vec3::new(42.0, 2.0, 3.0) / 2.0, Vec3::new(21.0, 1.0, 1.5));
    }

    #[test]
    fn test_vec_divrf() {
        assert_eq!(&Vec3::new(42.0, 2.0, 3.0) / 2.0, Vec3::new(21.0, 1.0, 1.5));
    }

    #[test]
    fn test_vec_divas() {
        let mut v: Vec3 = Vec3::new(42.0, 2.0, 3.0);
        v /= 2.0;
        assert_eq!(v, Vec3::new(21.0, 1.0, 1.5));
    }

    #[test]
    fn test_vec_unit_vec() {
        assert_eq!(Vec3::unit_vec(&Vec3::new(42.0, 42.0, -42.0)), Vec3::new(0.5773502691896257, 0.5773502691896257, -0.5773502691896257));
    }

    #[test]
    fn test_vec_normalize() {
        let mut salut: Vec3 = Vec3::new(42.0, 42.0, -42.0);
        salut.normalize();
        assert_eq!(salut, Vec3::new(0.5773502691896257, 0.5773502691896257, -0.5773502691896257));
    }

    #[test]
    fn test_vec_normalized() {
        let first_vector: Vec3 = Vec3::new(42.0, 42.0, -42.0);
        let second_vector = first_vector.normalized();
        assert_eq!(second_vector, Vec3::new(0.5773502691896257, 0.5773502691896257, -0.5773502691896257));
        assert_eq!(first_vector, Vec3::new(42.0, 42.0, -42.0));
    }

    #[test]
    fn test_vec_lenght() {
        assert_eq!(Vec3::new(3.0, 4.0, 0.0).length(), 5.0);
    }

    #[test]
    fn test_vec_lenght2() {
        assert_eq!(Vec3::new(3.0, 4.0, 0.0).length2(), 25.0);
    }

    #[test]
    fn test_vec_invert() {
        let v1: Vec3 = Vec3::new(3.0, 4.0, 0.0);
        let v2: Vec3 = v1.invert();
        assert_eq!(v1, Vec3::new(3.0, 4.0, 0.0));
        assert_eq!(v2, Vec3::new(-3.0, -4.0, 0.0));
    }

    #[test]
    fn test_vec_getter_x() {
        assert_eq!(Vec3::new(3.0, 4.0, 0.0).x(), 3.0);
    }

    #[test]
    fn test_vec_getter_y() {
        assert_eq!(Vec3::new(3.0, 4.0, 0.0).y(), 4.0);
    }

    #[test]
    fn test_vec_getter_z() {
        assert_eq!(Vec3::new(3.0, 4.0, 0.0).z(), 0.0);
    }

    #[test]
    fn test_vec_getter_r() {
        assert_eq!(Vec3::new(3.0, 4.0, 0.0).r(), 3.0);
    }

    #[test]
    fn test_vec_getter_g() {
        assert_eq!(Vec3::new(3.0, 4.0, 0.0).g(), 4.0);
    }

    #[test]
    fn test_vec_getter_b() {
        assert_eq!(Vec3::new(3.0, 4.0, 0.0).b(), 0.0);
    }

}