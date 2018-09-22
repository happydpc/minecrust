use geometry::rectangle::Rectangle;
use types::{Matrix4f, Point3f, Vector3f};

pub struct Square {
    rect: Rectangle,
}

impl Square {
    pub fn new(side: f32) -> Square {
        Square {
            rect: Rectangle::new(side, side),
        }
    }

    pub fn new_with_transform(side: f32, transform: &Matrix4f) -> Square {
        let mut sq = Square::new(side);
        sq.transform(transform);
        sq
    }

    pub fn transform(&mut self, tr: &Matrix4f) {
        self.rect.transform(tr);
    }

    pub fn vtx_data(&self, transform: &Matrix4f) -> Vec<f32> {
        self.rect.vtx_data(transform)
    }

    pub fn transform_mat(&self) -> Matrix4f {
        self.rect.transform
    }

    pub fn vertices(&self) -> &[Point3f] {
        &self.rect.vertices
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use na::geometry::Isometry;
    use na::{Rotation3, Translation3};
    use utils;

    #[test]
    fn test_transform1() {
        let mut s = Square::new(1.0);
        let t = Translation3::from_vector(Vector3f::new(0.0, 2.0, 0.0));
        s.transform(&t.to_homogeneous());
        #[rustfmt_skip]
        assert!(utils::mat4f_almost_eq(
            &s.transform_mat(),
            &Matrix4f::new(
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 2.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            )
        ));
        assert!(utils::pt3f_almost_eq(
            &s.vertices()[0],
            &Point3f::new(-0.5, 2.0, -0.5)
        ));
        assert!(utils::pt3f_almost_eq(
            &s.vertices()[1],
            &Point3f::new(-0.5, 2.0, 0.5)
        ));
        assert!(utils::pt3f_almost_eq(
            &s.vertices()[2],
            &Point3f::new(0.5, 2.0, 0.5)
        ));
        assert!(utils::pt3f_almost_eq(
            &s.vertices()[3],
            &Point3f::new(0.5, 2.0, -0.5)
        ));
    }

    #[test]
    fn test_transform2() {
        let mut s = Square::new(1.0);
        let t = Isometry::from_parts(
            Translation3::from_vector(Vector3f::new(0.0, 0.0, 0.5)),
            Rotation3::from_axis_angle(&Vector3f::x_axis(), ::std::f32::consts::FRAC_PI_2),
        );
        s.transform(&t.to_homogeneous());
        assert!(utils::pt3f_almost_eq(
            &s.vertices()[0],
            &Point3f::new(-0.5, 0.5, 0.5)
        ));
        assert!(utils::pt3f_almost_eq(
            &s.vertices()[1],
            &Point3f::new(-0.5, -0.5, 0.5)
        ));
        assert!(utils::pt3f_almost_eq(
            &s.vertices()[2],
            &Point3f::new(0.5, -0.5, 0.5)
        ));
        assert!(utils::pt3f_almost_eq(
            &s.vertices()[3],
            &Point3f::new(0.5, 0.5, 0.5)
        ));
    }
}