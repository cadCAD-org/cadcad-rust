#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use cadcad_rust::space::Space;
    use cadcad_rust::dimension::Dimension;

    #[test]
    fn test_new_space() {
        // Child space with Integer and Real dimensions.
        let child = Space(
            HashMap::from([
                ("foo", Dimension::Integer(1)),
                ("bar", Dimension::Real(1.)),
            ])
        );
        // Child space with Integer, Real, and Space dimensions.
        Space(
            HashMap::from([
                ("foo", Dimension::Integer(2)),
                ("bar", Dimension::Real(2.)),
                ("baz", Dimension::Space(child)),
            ])
        );
    }
}