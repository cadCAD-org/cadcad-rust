use crate::space::Space;


pub enum Dimension {
    Integer(i64),
    Real(f64),
    Space(Space)
}