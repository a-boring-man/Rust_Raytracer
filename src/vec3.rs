/* The 3D vector class */
pub struct Vec3 {
    e: [f64;3]
}

/* Implement basic function for the vector struct */
impl Vec3 {
    /* Create a new vector with initialized value */
    pub fn new_v(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }
    /* Create a default [0, 0, 0] vector */
    pub fn new() -> Vec3 {
        Vec3 { e: [0f64, 0f64, 0f64] }
    }
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