use std::{collections::HashMap, ops::{Mul, Add}};
use counter::Counter;
use num_traits::pow::Pow;
use serde::Serialize;
use serde_json::{json, to_string_pretty};
use crate::dimension::Dimension;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Space {
    name: String,
    dims: HashMap<String, Dimension>
}

impl Space {
    pub fn new(name: String, dims: HashMap<String, Dimension>) -> Self {
        Self { name, dims }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn dims(&self) -> &HashMap<String, Dimension> {
        &self.dims
    }

    pub fn is_empty(&self) -> bool {
        self.dims.len() == 0
    }

    /// If any of the keys in `map` are not in `self.dims`, an error is thrown.
    pub fn rename_dims(&mut self, map: HashMap<String, String>) {
        let mut dims = self.dims.clone();
        for (old, new) in map.into_iter() {
            let d = dims.remove(&old).unwrap();
            dims.insert(new, d);
        }
        self.dims = dims;
    }

    fn update_dim(&mut self, name: String, new: Dimension) {
        self.dims.entry(name).and_modify(|old| { old.update(new); });
    }

    pub fn update_dims(&mut self, dims: HashMap<String, Dimension>) {
        for (name, dim) in dims.into_iter() {
            self.update_dim(name, dim);
        }
    }

    pub fn unroll_schema(&self) -> String {
        to_string_pretty(&json!(self.dims().clone())).expect("Could not unroll schema")
    }
}

impl Mul for Space {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let name = format!("{} * {}", &self.name, &rhs.name);
        let mut name_lhs = self.name.clone();
        let mut name_rhs = rhs.name.clone();
        if name_lhs == name_rhs {
            name_lhs.push_str("_0");
            name_rhs.push_str("_1");
        }
        Space {
            name,
            dims: HashMap::from([
                (name_lhs, Dimension::Space(self)),
                (name_rhs, Dimension::Space(rhs))
            ])
        }
    }
}

impl Add for Space {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let dims_lhs = &self.dims;
        let dims_rhs = &rhs.dims;
        let counts = dims_lhs.iter().chain(dims_rhs.iter()).map(|(name, _dim)| name).collect::<Counter<_>>();
        let mut new = HashMap::<String, Dimension>::new();
        for (old, suffix) in [dims_lhs, dims_rhs].iter().zip(["_0", "_1"]) {
            for (name, dim) in old.iter() {
                let mut name = name.clone();
                if counts[&name] == 2 {
                    name.push_str(suffix)
                }
                new.insert(name, dim.clone());
            }
        }
        Space { name: format!("{} + {}", &self.name, &rhs.name), dims: new }
    }
}

impl Pow<u8> for Space {
    type Output = Self;

    fn pow(self, rhs: u8) -> Self::Output {
        if rhs > 1 {
            let mut name = self.name.clone();
            let mut dims = HashMap::from([
                (format!("{}_0", name), Dimension::Space(self.clone()))
            ]);
            for i in 1..rhs {
                name.push_str(format!(" * {}", &self.name).as_str());
                dims.insert(format!("{}_{}", &self.name, i), Dimension::Space(self.clone()));
            }
            return Space { name, dims }
        }
        self
    }
}