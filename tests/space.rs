#[cfg(test)]
mod test {
    use rstest::*;
    use std::collections::{HashMap, HashSet};
    use num_traits::pow::Pow;

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
        assert_eq!(space_3.dims(), space_4.dims());

        let mut expect_dims = HashMap::<String, Dimension>::new();
        expect_dims.extend(space_1.dims().to_owned());
        expect_dims.extend(space_2.dims().to_owned());
        assert_eq!(space_3.dims(), &expect_dims);
    }

    #[rstest]
    fn test_merge_product_with_overlapping_dims() {
        let space_1 = Space::new(
            String::from("space_1"),
            HashMap::from([
                (String::from("d_1"), Dimension::Integer { loc: 1 }),  // Overlapping!
                (String::from("d_2"), Dimension::Real { loc: 1. }),
            ])
        );
        let space_2 = Space::new(
            String::from("space_1"),
            HashMap::from([
                (String::from("d_1"), Dimension::Integer { loc: 1 }),  // Overlapping!
                (String::from("d_3"), Dimension::Real { loc: 1. }),
            ])
        );
        let space_3 = space_1 + space_2;
        let actual_keys = space_3.dims().keys().map(|k| k.as_str()).collect::<HashSet<&str>>();
        let expect_keys = HashSet::from(["d_1_0", "d_1_1", "d_2", "d_3"]);
        assert_eq!(actual_keys, expect_keys);
    }

    #[rstest]
    fn test_repeated_merge_product(space_1: Space) {
        let space_2 = space_1.clone().pow(3);
        let actual_keys = space_2.dims().keys().map(|k| k.as_str()).collect::<HashSet<&str>>();
        let expect_keys = HashSet::from(["space_1_0", "space_1_1", "space_1_2"]);
        assert_eq!(space_2.name(), "space_1 * space_1 * space_1");
        assert_eq!(actual_keys, expect_keys);
        for key in expect_keys {
            assert_eq!(space_2.dims()[key], Dimension::Space(space_1.clone()));
        }
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