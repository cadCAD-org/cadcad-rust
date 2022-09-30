use crate::space::Space;

#[derive(Clone, Debug, PartialEq)]
pub enum Dimension {
    Integer(i64),
    Real(f64),
    Space(Space)
}