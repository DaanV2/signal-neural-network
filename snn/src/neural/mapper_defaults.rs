use crate::neural::mapper::MapperNode;

/// Creates a MapperNode that maps lowercase ASCII letters (a-z) to a fixed value (0b1111_1111).
/// This is useful for identifying lowercase letters in a neural network context.
pub fn create_mapper_for_lowercase() -> MapperNode {
    MapperNode::new().with_range(b'a'..=b'z', 0b1111_1111)
}

/// Creates a MapperNode that maps uppercase ASCII letters (A-Z) to a fixed value (0b1111_1111).
/// This is useful for identifying uppercase letters in a neural network context.
pub fn create_mapper_for_uppercase() -> MapperNode {
    MapperNode::new().with_range(b'A'..=b'Z', 0b1111_1111)
}

/// Creates a MapperNode that maps both lowercase (a-z) and uppercase (A-Z) ASCII letters to specific values.
/// Lowercase letters are mapped to 0b1111_1111 and uppercase letters to 0b1111_0000.
pub fn create_mapper_for_letters() -> MapperNode {
    MapperNode::new()
        .with_range(b'a'..=b'z', 0b1111_1111)
        .with_range(b'A'..=b'Z', 0b1111_1111)
}

/// Creates a MapperNode that maps lowercase ASCII letters (a-z) to 0b1111_1111 and uppercase letters (A-Z) to 0b1111_0000.
/// This is useful for identifying both lowercase and uppercase letters in a neural network context,
pub fn create_mapper_for_letters_cased() -> MapperNode {
    MapperNode::new()
        .with_range(b'a'..=b'z', 0b1111_1111)
        .with_range(b'A'..=b'Z', 0b1111_0000)
}

/// Creates a MapperNode that character to specific bit patterns.
/// - Digits (0-9) are mapped to 0b0000_0001.
/// - Lowercase letters (a-z) are mapped to 0b0000_0010.
/// - Uppercase letters (A-Z) are mapped to 0b0000_0100.
/// - Printable ASCII characters (excluding control characters) are mapped to 0b0000_1000.
/// - Control characters (space, tab, newline) are mapped to 0b0001_0000.
pub fn create_mapper_for_character() -> MapperNode {
    MapperNode::new()
        .with_range(b'0'..=b'9', 0b0000_0001)
        .with_range(b'a'..=b'z', 0b0000_0010)
        .with_range(b'A'..=b'Z', 0b0000_0100)
        .with_range(b'!'..=b'~', 0b0000_1000)
        .with_range(b' '..=b'\n', 0b0001_0000)
}

/// Creates a MapperNode that maps the outside numbers as low as possible. and middle numbers as high as possible.
pub fn create_mapper_pyramid() -> MapperNode {
    MapperNode::new_transformation(|x| {
        let y = x as u8;
        if y < 128 { return y } else { 255 - y }
    })
}

/// Creates a MapperNode that inverts the mapping of `create_mapper_pyramid`.
pub fn create_mapper_pyramid_inverted() -> MapperNode {
    MapperNode::new_transformation(|x| {
        let y = x as u8;
        if y < 128 { return 255 - y } else { y }
    })
}
