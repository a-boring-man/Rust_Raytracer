use std::ops;

/* The 3D vector class */
#[derive(Copy, Clone, Debug)]
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

/* Implement the / operator for Vec3 */
impl ops::Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3 { e: [ self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.z() ] }
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
        v.divi(v.length())
    }
    
    /* Instance Methode */

    pub fn mult(&self, t: f64) -> Vec3 {
        Vec3 { e: [self.x() * t, self.y() * t, self.z() * t] }
    }

    pub fn divi(&self, t: f64) -> Vec3 {
        Vec3 { e: [self.x() / t, self.y() / t, self.z() / t] }
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.x() * self.x() + self.y() * self.y() + self.z() * self.z())
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