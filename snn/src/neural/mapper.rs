
/// Mapper module for efficient byte-to-byte mapping and transformation.
use std::u8;


/// The maximum value for a single byte (u8).
const MAX: usize = std::u8::MAX as usize;
/// The length of the mapping array (256 for all possible u8 values).
const MAX_LENGTH: usize = MAX + 1;


/// A struct that provides fast mapping from u8 to u8 using a lookup table.
pub struct Mapper {
    /// The transformation table: maps each u8 value to another u8 value.
    tf: [u8; MAX_LENGTH],
}

impl Mapper {
    /// Creates a new `Mapper` with all values initialized to 0.
    pub const fn new() -> Mapper {
        Mapper {
            tf: [0 as u8; MAX_LENGTH],
        }
    }

    /// Creates a new `Mapper` from a provided mapping array.
    ///
    /// # Arguments
    /// * `data` - An array of 256 u8 values representing the mapping.
    pub const fn new_from(data: [u8; MAX_LENGTH]) -> Mapper {
        return Mapper { tf: data };
    }

    /// Creates a new `Mapper` using a transformation function.
    ///
    /// # Arguments
    /// * `transfn` - A function that takes an index (usize) and returns a u8 value for that index.
    pub fn new_transformation(transfn: impl Fn(usize) -> u8) -> Mapper {
        let mut data = [0 as u8; MAX_LENGTH];
        for i in 0..MAX_LENGTH {
            data[i] = transfn(i);
        }
        Mapper::new_from(data)
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

    /// Returns a new `Mapper` with the given key-value pairs updated in the mapping table.
    ///
    /// # Arguments
    /// * `mapdata` - An iterator of (u8, u8) pairs to update in the mapping.
    pub fn with_mapdata<I>(&self, mapdata: I) -> Mapper
    where
        I: IntoIterator<Item = (u8, u8)>,
    {
        let mut data = self.tf;
        for (k, v) in mapdata {
            data[k as usize] = v;
        }
        Mapper::new_from(data)
    }

    /// Returns a new `Mapper` with the given key-value pairs from a HashMap updated in the mapping table.
    ///
    /// # Arguments
    /// * `mapdata` - A reference to a HashMap of (u8, u8) pairs to update in the mapping.
    pub fn with_mapdata_hashmap(&self, mapdata: &std::collections::HashMap<u8, u8>) -> Mapper {
        let mut data = self.tf;
        for (&k, &v) in mapdata.iter() {
            data[k as usize] = v;
        }
        Mapper::new_from(data)
    }

    /// Returns a new `Mapper` with all values in the given range set to the specified value.
    ///
    /// # Arguments
    /// * `range` - An iterator of u8 values to update in the mapping.
    /// * `value` - The value to set for each key in the range.
    pub fn with_range<R>(&self, range: R, value: u8) -> Mapper
    where
        R: IntoIterator<Item = u8>,
    {
        let mut data = self.tf;
        for c in range {
            data[c as usize] = value;
        }
        Mapper::new_from(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;


    #[test]
    fn test_new_and_len() {
        let mapper = Mapper::new();
        assert_eq!(mapper.len(), 256);
        assert!(mapper.tf.iter().all(|&v| v == 0));
    }


    #[test]
    fn test_new_from() {
        let mapper = Mapper::new().with_mapdata([(65, 42)]);
        assert_eq!(mapper.tranform(65), 42);
        assert_eq!(mapper.tranform(0), 0);
    }


    #[test]
    fn test_new_transformation_identity() {
        let mapper = Mapper::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, x)));
        for i in 0u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), i);
        }
    }


    #[test]
    fn test_new_transformation_add_one() {
        let mapper = Mapper::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, x.wrapping_add(1))));
        for i in 0u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), i.wrapping_add(1));
        }
    }


    #[test]
    fn test_with_mapdata_array() {
        let mapper = Mapper::new().with_mapdata([(1, 42), (255, 99)]);
        assert_eq!(mapper.tranform(1), 42);
        assert_eq!(mapper.tranform(255), 99);
        assert_eq!(mapper.tranform(0), 0);
    }


    #[test]
    fn test_with_mapdata_hashmap() {
        let mut map = HashMap::new();
        map.insert(1, 42);
        map.insert(255, 99);
        let mapper = Mapper::new().with_mapdata_hashmap(&map);
        assert_eq!(mapper.tranform(1), 42);
        assert_eq!(mapper.tranform(255), 99);
        assert_eq!(mapper.tranform(0), 0);
    }


    #[test]
    fn test_data_recognition_letter_with_range() {
        // Use with_range for lowercase
        let lower = Mapper::new().with_range(b'a'..=b'z', 1);
        for c in b'a'..=b'z' {
            assert_eq!(lower.tranform(c), 1);
        }
        assert_eq!(lower.tranform(b'A'), 0);
        assert_eq!(lower.tranform(b'0'), 0);

        // Use with_range for uppercase
        let upper = Mapper::new().with_range(b'A'..=b'Z', 2);
        for c in b'A'..=b'Z' {
            assert_eq!(upper.tranform(c), 2);
        }
        assert_eq!(upper.tranform(b'a'), 0);
        assert_eq!(upper.tranform(b'0'), 0);

        // Combine both
        let both = Mapper::new().with_mapdata(
            (b'A'..=b'Z').map(|c| (c, 1)).chain((b'a'..=b'z').map(|c| (c, 1)))
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
        let mapper = Mapper::new().with_mapdata(pairs);
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
        let mapper = Mapper::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, if x > 5 { 1 } else { 0 })));
        for i in 0u8..=5 {
            assert_eq!(mapper.tranform(i), 0);
        }
        for i in 6u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), 1);
        }
    }


    #[test]
    fn test_if_statement_emulation_less_than_5() {
        let mapper = Mapper::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, if x < 5 { 1 } else { 0 })));
        for i in 0u8..=4 {
            assert_eq!(mapper.tranform(i), 1);
        }
        for i in 5u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), 0);
        }
    }


    #[test]
    fn test_mathematical_operation_add_one() {
        let mapper = Mapper::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, x.wrapping_add(1))));
        for i in 0u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), i.wrapping_add(1));
        }
    }


    #[test]
    fn test_mathematical_operation_multiply_by_2() {
        let mapper = Mapper::new().with_mapdata((0u8..=u8::MAX).map(|x| (x, x.wrapping_mul(2))));
        for i in 0u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), i.wrapping_mul(2));
        }
    }


    #[test]
    fn test_mathematical_operation_square() {
        let mapper = Mapper::new().with_mapdata((0u8..=u8::MAX).map(|x| {
            let y = x as u16;
            (x, ((y * y) % 256) as u8)
        }));
        for i in 0u8..=u8::MAX {
            let expected = ((i as u16 * i as u16) % 256) as u8;
            assert_eq!(mapper.tranform(i), expected);
        }
    }
}
