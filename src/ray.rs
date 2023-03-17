/* Allow rust to know where to find the Vec3 class */
use crate::vec3::Vec3;

/* Aliasing */
type Point = Vec3;

/* The Ray class */
#[derive(PartialEq, Debug, Clone, Copy)]
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
        self.origin + self.direction * t
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ray_construction() {
        assert_eq!(Ray::new(Vec3::new(42.0, 21.4, 4.5), Vec3::new(52.0, 24.4, 4.53)), Ray {origin: Vec3::new(42.0, 21.4, 4.5), direction: Vec3::new(52.0, 24.4, 4.53)});
    }

    #[test]
    fn test_ray_at() {
        assert_eq!(Ray::new(Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0)).at(0.5), Vec3::new(1.5, 1.5, 1.5));
    }

    #[test]
    fn test_ray_getter_origin() {
        assert_eq!(Ray::new(Vec3::new(6.0, 1.530, 1.12), Vec3::new(1.45, 22.0, 1.7)).origin(), Vec3::new(6.0, 1.530, 1.12));
    }

    #[test]
    fn test_ray_getter_direction() {
        assert_eq!(Ray::new(Vec3::new(6.0, 1.530, 1.12), Vec3::new(1.45, 22.0, 1.7)).dir(),  Vec3::new(1.45, 22.0, 1.7));
    }

}