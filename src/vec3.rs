use std::ops;

/* The 3D vector class */
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    e: [f64;3]
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

/* Implement the - operator for Vec3 */
impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 { e: [ self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z() ] }
    }
}

/* Implement the * operator for Vec3 */
impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 { e: [ self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z() ] }
    }
}
impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 { e: [self.x() * rhs, self.y() * rhs, self.z() * rhs] }
    }
}

/* Implement the / operator for Vec3 */
impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
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

    pub fn unit_vec(v: Vec3) -> Vec3 {
        v / v.length()
    }
    
    /* Instance Methode */

    pub fn length(&self) -> f64 {
        (self.x() * self.x() + self.y() * self.y() + self.z() * self.z()).sqrt()
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
    fn test_vec_sub() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) - Vec3::new(4.0, 6.0, 8.0), Vec3::new(-3.0, -4.0, -5.0));
    }

    #[test]
    fn test_vec_mul1() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * Vec3::new(4.0, 6.0, 8.0), Vec3::new(4.0, 12.0, 24.0));
    }

    #[test]
    fn test_vec_mul2() {
        assert_eq!(Vec3::new(1.0, 2.0, 3.0) * 3.0, Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_vec_div1() {
        assert_eq!(Vec3::new(1.0, 6.0, 7.5) / Vec3::new(4.0, 2.0, 3.0), Vec3::new(0.25, 3.0, 2.5));
    }

    #[test]
    fn test_vec_div2() {
        assert_eq!(Vec3::new(42.0, 2.0, 3.0) / 2.0, Vec3::new(21.0, 1.0, 1.5));
    }

    #[test]
    fn test_vec_unit_vec() {
        assert_eq!(Vec3::unit_vec(Vec3::new(42.0, 42.0, -42.0)), Vec3::new(0.5773502691896257, 0.5773502691896257, -0.5773502691896257));
    }

    #[test]
    fn test_vec_lenght() {
        assert_eq!(Vec3::new(3.0, 4.0, 0.0).length(), 5.0);
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

}