use crate::{Point3, Vec3};

pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Ray {
        Ray { orig, dir }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ray_general() {
        let ray = Ray::new(Point3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
        let orig = ray.origin();
        assert_eq!([orig[0], orig[1], orig[2]], [1.0, 2.0, 3.0]);
        let dir = ray.direction();
        assert_eq!([dir[0], dir[1], dir[2]], [4.0, 5.0, 6.0]);

        let point = ray.at(0.0);
        assert_eq!([point[0], point[1], point[2]], [1.0, 2.0, 3.0]);
        let point = ray.at(1.0);
        assert_eq!([point[0], point[1], point[2]], [5.0, 7.0, 9.0]);
        let point = ray.at(-1.0);
        assert_eq!([point[0], point[1], point[2]], [-3.0, -3.0, -3.0]);
        let point = ray.at(20.0);
        assert_eq!([point[0], point[1], point[2]], [81.0, 102.0, 123.0]);
    }
}
