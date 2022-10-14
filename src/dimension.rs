use serde::Serialize;

use crate::space::Space;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub enum Dimension {
    Integer { loc: i32 },
    Real { loc: f64 },
    Space(Space)
}

impl Dimension {
    pub fn update(&self, dim: Dimension) -> Dimension {
        match dim {
            Dimension::Integer { loc: x } => Dimension::Integer { loc: x },
            Dimension::Real { loc: x } => Dimension::Real { loc: x },
            Dimension::Space(spc) => Dimension::Space(spc)
        }
    }
}