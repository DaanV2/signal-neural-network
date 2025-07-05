/// Mapper module for efficient byte-to-byte mapping and transformation.
use std::u8;

use crate::neural::traits::Mapper;

/// The maximum value for a single byte (u8).
const MAX: usize = std::u8::MAX as usize;
/// The length of the mapping array (256 for all possible u8 values).
const MAX_LENGTH: usize = MAX + 1;

/// A struct that provides fast mapping from u8 to u8 using a lookup table.
pub struct MapperNode {
    /// The transformation table: maps each u8 value to another u8 value.
    tf: [u8; MAX_LENGTH],
}

impl MapperNode {
    /// Creates a new `Mapper` with all values initialized to 0.
    pub const fn new() -> MapperNode {
        MapperNode {
            tf: [0 as u8; MAX_LENGTH],
        }
    }

    /// Creates a new `Mapper` from a provided mapping array.
    ///
    /// # Arguments
    /// * `data` - An array of 256 u8 values representing the mapping.
    pub const fn new_from(data: [u8; MAX_LENGTH]) -> MapperNode {
        return MapperNode { tf: data };
    }

    /// Creates a new `Mapper` using a transformation function.
    ///
    /// # Arguments
    /// * `transfn` - A function that takes an index (usize) and returns a u8 value for that index.
    pub fn new_transformation(transfn: impl Fn(usize) -> u8) -> MapperNode {
        let mut data = [0 as u8; MAX_LENGTH];
        for i in 0..MAX_LENGTH {
            data[i] = transfn(i);
        }
        MapperNode::new_from(data)
    }

    /// Transforms an input byte using the mapping table.
    ///
    /// # Arguments
    /// * `input` - The input byte to transform.
    ///
    /// # Returns
    /// The mapped output byte.
    pub const fn tranform(&self, input: u8) -> u8 {
        return self.tf[input as usize];
    }

    /// Returns the length of the mapping table (always 256).
    pub const fn len(&self) -> usize {
        return self.tf.len();
    }

    pub fn with_fill(&self, value: u8) -> MapperNode {
        let data = [value; MAX_LENGTH];
        MapperNode::new_from(data)
    }

    /// Returns a new `Mapper` with the given key-value pairs updated in the mapping table.
    ///
    /// # Arguments
    /// * `mapdata` - An iterator of (u8, u8) pairs to update in the mapping.
    pub fn with_mapdata<I>(&self, mapdata: I) -> MapperNode
    where
        I: IntoIterator<Item = (u8, u8)>,
    {
        let mut data = self.tf;
        for (k, v) in mapdata {
            data[k as usize] = v;
        }
        MapperNode::new_from(data)
    }

    /// Returns a new `Mapper` with the given key-value pairs from a HashMap updated in the mapping table.
    ///
    /// # Arguments
    /// * `mapdata` - A reference to a HashMap of (u8, u8) pairs to update in the mapping.
    pub fn with_mapdata_hashmap(&self, mapdata: &std::collections::HashMap<u8, u8>) -> MapperNode {
        let mut data = self.tf;
        for (&k, &v) in mapdata.iter() {
            data[k as usize] = v;
        }
        MapperNode::new_from(data)
    }

    /// Returns a new `Mapper` with all values in the given range set to the specified value.
    ///
    /// # Arguments
    /// * `range` - An iterator of u8 values to update in the mapping.
    /// * `value` - The value to set for each key in the range.
    pub fn with_range<R>(&self, range: R, value: u8) -> MapperNode
    where
        R: IntoIterator<Item = u8>,
    {
        let mut data = self.tf;
        for c in range {
            data[c as usize] = value;
        }
        MapperNode::new_from(data)
    }

    /// Returns a new `Mapper` with a modification function applied to each value in the mapping table.
    pub fn with_modification(&self, modification: impl Fn(usize, u8) -> u8) -> MapperNode {
        let mut data = self.tf;
        for i in 0..MAX_LENGTH {
            data[i] = modification(i, data[i]);
        }
        MapperNode::new_from(data)
    }


    /// Returns a new `Mapper` with each value inverted (i.e., `MAX - value`).
    ///
    /// # Returns
    /// A new `MapperNode` where each value is replaced by its inverse with respect to `MAX`.
    pub fn invert(&self) -> MapperNode {
        self.with_modification(|_, d| (MAX as u8 - d))
    }


    /// Returns a new `Mapper` with each value bitwise negated (NOT operation).
    ///
    /// # Returns
    /// A new `MapperNode` where each value is replaced by its bitwise NOT.
    pub fn not(&self) -> MapperNode {
        self.with_modification(|_, d| !d)
    }


    /// Returns a new `Mapper` with each value bitwise ANDed with the corresponding value from another `MapperNode`.
    ///
    /// # Arguments
    /// * `other` - Another `MapperNode` to AND with.
    ///
    /// # Returns
    /// A new `MapperNode` where each value is the result of `self & other`.
    pub fn and_node(&self, other: &MapperNode) -> MapperNode {
        self.with_modification(|i, d| d & other.tf[i])
    }


    /// Returns a new `Mapper` with each value bitwise ANDed with a constant value.
    ///
    /// # Arguments
    /// * `v` - The value to AND with each entry in the mapping table.
    ///
    /// # Returns
    /// A new `MapperNode` where each value is the result of `value & v`.
    pub fn and(&self, v: u8) -> MapperNode {
        self.with_modification(|_, d| d & v)
    }


    /// Returns a new `Mapper` with each value bitwise ORed with the corresponding value from another `MapperNode`.
    ///
    /// # Arguments
    /// * `other` - Another `MapperNode` to OR with.
    ///
    /// # Returns
    /// A new `MapperNode` where each value is the result of `self | other`.
    pub fn or_node(&self, other: &MapperNode) -> MapperNode {
        self.with_modification(|i, d| d | other.tf[i])
    }


    /// Returns a new `Mapper` with each value bitwise ORed with a constant value.
    ///
    /// # Arguments
    /// * `v` - The value to OR with each entry in the mapping table.
    ///
    /// # Returns
    /// A new `MapperNode` where each value is the result of `value | v`.
    pub fn or(&self, v: u8) -> MapperNode {
        self.with_modification(|_, d| d | v)
    }


    /// Returns a new `Mapper` with each value bitwise XORed with the corresponding value from another `MapperNode`.
    ///
    /// # Arguments
    /// * `other` - Another `MapperNode` to XOR with.
    ///
    /// # Returns
    /// A new `MapperNode` where each value is the result of `self ^ other`.
    pub fn xor_node(&self, other: &MapperNode) -> MapperNode {
        self.with_modification(|i, d| d ^ other.tf[i])
    }


    /// Returns a new `Mapper` with each value bitwise XORed with a constant value.
    ///
    /// # Arguments
    /// * `v` - The value to XOR with each entry in the mapping table.
    ///
    /// # Returns
    /// A new `MapperNode` where each value is the result of `value ^ v`.
    pub fn xor(&self, v: u8) -> MapperNode {
        self.with_modification(|_, d| d ^ v)
    }
}

impl Mapper for MapperNode {
    fn transform(&self, input: u8) -> u8 {
        self.tranform(input)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_invert() {
        let mapper = MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, x)));
        let inverted = mapper.invert();
        for i in 0u8..=u8::MAX {
            assert_eq!(inverted.tranform(i), u8::MAX - i);
        }
    }

    #[test]
    fn test_not() {
        let mapper = MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, x)));
        let not = mapper.not();
        for i in 0u8..=u8::MAX {
            assert_eq!(not.tranform(i), !i);
        }
    }

    #[test]
    fn test_and_node() {
        let mapper1 = MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, x)));
        let mapper2 = MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, 0b10101010)));
        let anded = mapper1.and_node(&mapper2);
        for i in 0u8..=u8::MAX {
            assert_eq!(anded.tranform(i), i & 0b10101010);
        }
    }

    #[test]
    fn test_and() {
        let mapper = MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, x)));
        let anded = mapper.and(0b11001100);
        for i in 0u8..=u8::MAX {
            assert_eq!(anded.tranform(i), i & 0b11001100);
        }
    }

    #[test]
    fn test_or_node() {
        let mapper1 = MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, x)));
        let mapper2 = MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, 0b00001111)));
        let ored = mapper1.or_node(&mapper2);
        for i in 0u8..=u8::MAX {
            assert_eq!(ored.tranform(i), i | 0b00001111);
        }
    }

    #[test]
    fn test_or() {
        let mapper = MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, x)));
        let ored = mapper.or(0b11110000);
        for i in 0u8..=u8::MAX {
            assert_eq!(ored.tranform(i), i | 0b11110000);
        }
    }

    #[test]
    fn test_xor_node() {
        let mapper1 = MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, x)));
        let mapper2 = MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, 0b11111111)));
        let xored = mapper1.xor_node(&mapper2);
        for i in 0u8..=u8::MAX {
            assert_eq!(xored.tranform(i), i ^ 0b11111111);
        }
    }

    #[test]
    fn test_xor() {
        let mapper = MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, x)));
        let xored = mapper.xor(0b01010101);
        for i in 0u8..=u8::MAX {
            assert_eq!(xored.tranform(i), i ^ 0b01010101);
        }
    }
    use super::*;

    use std::collections::HashMap;

    #[test]
    fn test_new_and_len() {
        let mapper = MapperNode::new();
        assert_eq!(mapper.len(), 256);
        assert!(mapper.tf.iter().all(|&v| v == 0));
    }

    #[test]
    fn test_new_from() {
        let mapper = MapperNode::new().with_mapdata([(65, 42)]);
        assert_eq!(mapper.tranform(65), 42);
        assert_eq!(mapper.tranform(0), 0);
    }

    #[test]
    fn test_new_transformation_identity() {
        let mapper = MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, x)));
        for i in 0u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), i);
        }
    }

    #[test]
    fn test_new_transformation_add_one() {
        let mapper =
            MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, x.wrapping_add(1))));
        for i in 0u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), i.wrapping_add(1));
        }
    }

    #[test]
    fn test_with_mapdata_array() {
        let mapper = MapperNode::new().with_mapdata([(1, 42), (255, 99)]);
        assert_eq!(mapper.tranform(1), 42);
        assert_eq!(mapper.tranform(255), 99);
        assert_eq!(mapper.tranform(0), 0);
    }

    #[test]
    fn test_with_mapdata_hashmap() {
        let mut map = HashMap::new();
        map.insert(1, 42);
        map.insert(255, 99);
        let mapper = MapperNode::new().with_mapdata_hashmap(&map);
        assert_eq!(mapper.tranform(1), 42);
        assert_eq!(mapper.tranform(255), 99);
        assert_eq!(mapper.tranform(0), 0);
    }

    #[test]
    fn test_data_recognition_letter_with_range() {
        // Use with_range for lowercase
        let lower = MapperNode::new().with_range(b'a'..=b'z', 1);
        for c in b'a'..=b'z' {
            assert_eq!(lower.tranform(c), 1);
        }
        assert_eq!(lower.tranform(b'A'), 0);
        assert_eq!(lower.tranform(b'0'), 0);

        // Use with_range for uppercase
        let upper = MapperNode::new().with_range(b'A'..=b'Z', 2);
        for c in b'A'..=b'Z' {
            assert_eq!(upper.tranform(c), 2);
        }
        assert_eq!(upper.tranform(b'a'), 0);
        assert_eq!(upper.tranform(b'0'), 0);

        // Combine both
        let both = MapperNode::new().with_mapdata(
            (b'A'..=b'Z')
                .map(|c| (c, 1))
                .chain((b'a'..=b'z').map(|c| (c, 1))),
        );
        assert_eq!(both.tranform(b'A'), 1);
        assert_eq!(both.tranform(b'z'), 1);
        assert_eq!(both.tranform(b'0'), 0);
        assert_eq!(both.tranform(b'!'), 0);
    }

    #[test]
    fn test_data_recognition_properties_with_range() {
        let mut pairs = vec![];
        for c in b'a'..=b'z' {
            pairs.push((c, 0b00000001));
        }
        for c in b'A'..=b'Z' {
            pairs.push((c, 0b00000010));
        }
        for c in b'0'..=b'9' {
            pairs.push((c, 0b00000100));
        }
        for &c in b"!@#" {
            pairs.push((c, 0b00001000));
        }
        for &c in b" \t\n" {
            pairs.push((c, 0b00010000));
        }
        let mapper = MapperNode::new().with_mapdata(pairs);
        assert_eq!(mapper.tranform(b'a'), 0b00000001);
        assert_eq!(mapper.tranform(b'B'), 0b00000010);
        assert_eq!(mapper.tranform(b'3'), 0b00000100);
        assert_eq!(mapper.tranform(b'!'), 0b00001000);
        assert_eq!(mapper.tranform(b' '), 0b00010000);
        assert_eq!(mapper.tranform(b'\t'), 0b00010000);
        assert_eq!(mapper.tranform(b'\n'), 0b00010000);
        assert_eq!(mapper.tranform(b'$'), 0);
    }

    #[test]
    fn test_if_statement_emulation_greater_than_5() {
        let mapper =
            MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, if x > 5 { 1 } else { 0 })));
        for i in 0u8..=5 {
            assert_eq!(mapper.tranform(i), 0);
        }
        for i in 6u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), 1);
        }
    }

    #[test]
    fn test_if_statement_emulation_less_than_5() {
        let mapper =
            MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, if x < 5 { 1 } else { 0 })));
        for i in 0u8..=4 {
            assert_eq!(mapper.tranform(i), 1);
        }
        for i in 5u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), 0);
        }
    }

    #[test]
    fn test_mathematical_operation_add_one() {
        let mapper =
            MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, x.wrapping_add(1))));
        for i in 0u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), i.wrapping_add(1));
        }
    }

    #[test]
    fn test_mathematical_operation_multiply_by_2() {
        let mapper =
            MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, x.wrapping_mul(2))));
        for i in 0u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), i.wrapping_mul(2));
        }
    }

    #[test]
    fn test_mathematical_operation_square() {
        let mapper = MapperNode::new().with_mapdata((0u8..=u8::MAX).map(|x| {
            let y = x as u16;
            (x, ((y * y) % 256) as u8)
        }));
        for i in 0u8..=u8::MAX {
            let expected = ((i as u16 * i as u16) % 256) as u8;
            assert_eq!(mapper.tranform(i), expected);
        }
    }
}
