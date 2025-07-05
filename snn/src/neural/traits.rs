
/// Trait for mapping an input value to an output value.
pub trait Mapper {
    /// Maps the input value to an output value.
    fn transform(&self, input: u8) -> u8;
}

/// Trait for combining multiple input values into a single output value.
pub trait Combinator {
    /// Combines a slice of input values into a single output value.
    fn combine(&self, inputs: &[u8]) -> u8;

    fn identifier(&self) -> String;
}
