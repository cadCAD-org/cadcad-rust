#[cfg(test)]
mod test {
    use rstest::*;
    use std::collections::{HashMap, HashSet};

    use cadcad_rust::space::Space;
    use cadcad_rust::dimension::Dimension;

    #[fixture]
    fn space_1() -> Space {
        Space::new(
            String::from("space_1"),
            HashMap::from([
                (String::from("d_1"), Dimension::Integer { loc: 1 }),
                (String::from("d_2"), Dimension::Real { loc: 1. }),
            ])
        )
    }

    #[fixture]
    fn space_2() -> Space {
        Space::new(
            String::from("space_2"),
            HashMap::from([
                (String::from("d_3"), Dimension::Integer { loc: 2 } ),
                (String::from("d_4"), Dimension::Real { loc: 2. }),
            ])
        )
    }

    #[fixture]
    fn empty_space() -> Space {
        Space::new(
            String::from("space_2"),
            HashMap::new()
        )
    }

    #[rstest]
    fn test_nest_spaces(space_1: Space, space_2: Space) {
        Space::new(
            String::from("space_3"),
            HashMap::from([
                (String::from("space_1"), Dimension::Space(space_1)),
                (String::from("space_2"), Dimension::Space(space_2)),
            ])
        );
    }

    #[rstest]
    fn test_cartesian_product(space_1: Space, space_2: Space) {
        let space_3 = space_1.clone() * space_2.clone();  // Clone necessary due to borrow rules.
        let space_4 = space_2 * space_1;
        assert!(space_3.dims().eq(&space_4.dims()));
        assert_eq!(space_3.dims().len(), 2);
    }

    #[rstest]
    fn test_merge_product(space_1: Space, space_2: Space) {
        let space_3 = space_1.clone() + space_2.clone();  // Clone necessary due to borrow rules.
        let space_4 = space_2.clone() + space_1.clone();  // Clone necessary due to borrow rules.
        assert!(space_3.dims().eq(&space_4.dims()));

        let mut expected_merged_dims = HashMap::<String, Dimension>::new();
        expected_merged_dims.extend(space_1.dims().to_owned());
        expected_merged_dims.extend(space_2.dims().to_owned());
        assert!(space_3.dims().eq(&expected_merged_dims));
    }

    #[rstest]
    fn test_is_empty(space_1: Space, empty_space: Space) {
        assert!(!space_1.is_empty());
        assert!(empty_space.is_empty());
    }

    #[rstest]
    fn test_rename_dims(mut space_1: Space) {
        space_1.rename_dims(
            HashMap::from([
                (String::from("d_1"), String::from("new_name"))
            ])
        );
        let expected_new_dims = HashSet::from([String::from("new_name"), String::from("d_2")]);
        let new_dims = space_1.dims().keys().cloned().collect::<HashSet<String>>();
        assert_eq!(new_dims, expected_new_dims);
    }
}