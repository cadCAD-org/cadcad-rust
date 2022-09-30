use std::collections::HashMap;
use crate::dimension::Dimension;

pub struct Space(pub HashMap<&'static str, Dimension>);