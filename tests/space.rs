#[cfg(test)]
mod test {
    use rstest::*;
    use std::collections::HashMap;

    use cadcad_rust::space::Space;
    use cadcad_rust::dimension::Dimension;

    #[fixture]
    fn space_1() -> Space {
        Space {
            name: String::from("space_1"),
            dims: HashMap::from([
                (String::from("d_1"), Dimension::Integer(1)),
                (String::from("d_2"), Dimension::Real(1.)),
            ])
        }
    }

    #[fixture]
    fn space_2() -> Space {
        Space {
            name: String::from("space_2"),
            dims: HashMap::from([
                (String::from("d_3"), Dimension::Integer(2)),
                (String::from("d_4"), Dimension::Real(2.)),
            ])
        }
    }

    #[rstest]
    fn test_nest_spaces(space_1: Space, space_2: Space) {
        Space {
            name: String::from("space_3"),
            dims: HashMap::from([
                (String::from("space_1"), Dimension::Space(space_1)),
                (String::from("space_2"), Dimension::Space(space_2)),
            ])
        };
    }

    #[rstest]
    fn test_cartesian_product(space_1: Space, space_2: Space) {
        let space_3 = space_1.clone() * space_2.clone();  // Clone necessary due to borrow rules.
        let space_4 = space_2 * space_1;
        assert!(space_3.dims.eq(&space_4.dims));
        assert_eq!(space_3.dims.len() as u8, 2);
    }
}