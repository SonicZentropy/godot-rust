use crate::Vector3;

/// A 3x3 matrix.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Basis {
    pub elements: [Vector3; 3],
}

// TODO more methods!
// Feel free to get inspiration from godot-src\core\math\basis.cpp
impl Basis {
    pub fn identity() -> Basis {
        Basis { 
            elements: [
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
            ]
        }
    }
}
