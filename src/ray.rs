/* Allow rust to know where to find the Vec3 class */
use crate::vec3::Vec3;

/* Aliasing */
type Point = Vec3;

/* The Ray class */
pub struct Ray {
    origin: Point,
    direction: Vec3,
}

/* Implement basic function for the ray struct */
impl Ray {

    /**********************
     * Creation section
     **********************/

    /* Create a new ray using  as the origin point and  as directionnal vector */
    pub fn new(start: Point, dir: Vec3) -> Ray {
        Ray { origin: start, direction: dir }
    }

    /**********************
     * Methode section
     **********************/

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction.mult(t)
    }

    /**********************
     * Getter section
     **********************/

    /* Return the origin of the ray */
    pub fn origin(&self) -> Point {
        self.origin
    }

    /* Return the direction of the ray */
    pub fn dir(&self) -> Vec3 {
        self.direction
    }

}