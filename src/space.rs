use std::{collections::HashMap, ops::{Mul, Add}};
use crate::dimension::Dimension;

#[derive(Clone, Debug, PartialEq)]
pub struct Space {
    pub name: String,
    pub dims: HashMap<String, Dimension>
}

impl Space {
    pub fn is_empty(&self) -> bool {
        self.dims.len() == 0
    }
}

impl Mul for Space {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Space {
            name: format!("{} * {}", &self.name, &rhs.name),
            dims: HashMap::from([
                (String::from(&self.name), Dimension::Space(self)),
                (String::from(&rhs.name), Dimension::Space(rhs)),
            ])
        }
    }
}

impl Add for Space {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let name = format!("{} + {}", &self.name, &rhs.name);
        let mut dims = self.dims;
        dims.extend(rhs.dims);
        Space { name, dims }
    }
}