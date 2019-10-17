use crate::{Basis, Vector3};
use euclid::{Transform3D, Point3D};

/// 3D Transformation (3x4 matrix) Using basis + origin representation.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform {
    /// The basis is a matrix containing 3 Vector3 as its columns: X axis, Y axis, and Z axis.
    /// These vectors can be interpreted as the basis vectors of local coordinate system
    /// traveling with the object.
    pub basis: Basis,
    /// The translation offset of the transform.
    pub origin: Vector3,
}

// TODO: methods!
impl Transform {
    pub fn translate(origin: Vector3) -> Transform {
        Transform {
            basis: Basis::identity(),
            origin
        }
    }

    pub fn from_transform(transform: &Transform3D<f32>) -> Transform {
        Transform {
            basis: Basis::from_transform(transform),
            origin: transform.transform_point3d(&Point3D::origin())
                .unwrap_or(Point3D::origin()).to_vector(),
        }
    }
}
