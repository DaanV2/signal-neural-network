use std::u8;

const MAX: usize = std::u8::MAX as usize;
const MAX_LENGTH: usize = MAX + 1;

pub struct Mapper {
    tf: [u8; MAX_LENGTH],
}

impl Mapper {
    pub const fn new() -> Mapper {
        Mapper {
            tf: [0 as u8; MAX_LENGTH],
        }
    }

    pub const fn new_from(data: [u8; MAX_LENGTH]) -> Mapper {
        return Mapper { tf: data };
    }

    pub fn new_transformation(transfn: impl Fn(usize) -> u8) -> Mapper {
        let mut data = [0 as u8; MAX_LENGTH];
        for i in 0..MAX_LENGTH {
            data[i] = transfn(i);
        }
        Mapper::new_from(data)
    }


    pub const fn tranform(&self, input: u8) -> u8 {
        return self.tf[input as usize];
    }

    pub const fn len(&self) -> usize {
        return self.tf.len();
    }

    pub fn with_mapdata<I>(mapdata: I) -> Mapper
    where
        I: IntoIterator<Item = (u8, u8)>,
    {
        let mut data = [0u8; MAX_LENGTH];
        for (k, v) in mapdata {
            data[k as usize] = v;
        }
        Mapper::new_from(data)
    }

    pub fn with_mapdata_hashmap(mapdata: &std::collections::HashMap<u8, u8>) -> Mapper {
        let mut data = [0u8; MAX_LENGTH];
        for (&k, &v) in mapdata.iter() {
            data[k as usize] = v;
        }
        Mapper::new_from(data)
    }

    pub fn with_range<R>(range: R, value: u8) -> Mapper
    where
        R: IntoIterator<Item = u8>,
    {
        let mut data = [0u8; MAX_LENGTH];
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
        let mut data = [0u8; MAX_LENGTH];
        data[65] = 42;
        let mapper = Mapper::new_from(data);
        assert_eq!(mapper.tranform(65), 42);
        assert_eq!(mapper.tranform(0), 0);
    }

    #[test]
    fn test_new_transformation_identity() {
        let mapper = Mapper::new_transformation(|x| x as u8);
        for i in 0u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), i);
        }
    }

    #[test]
    fn test_new_transformation_add_one() {
        let mapper = Mapper::new_transformation(|x| x.wrapping_add(1) as u8);
        for i in 0u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), i.wrapping_add(1));
        }
    }

    #[test]
    fn test_with_mapdata_array() {
        let mapper = Mapper::with_mapdata([(1, 42), (255, 99)]);
        assert_eq!(mapper.tranform(1), 42);
        assert_eq!(mapper.tranform(255), 99);
        assert_eq!(mapper.tranform(0), 0);
    }

    #[test]
    fn test_with_mapdata_hashmap() {
        let mut map = HashMap::new();
        map.insert(1, 42);
        map.insert(255, 99);
        let mapper = Mapper::with_mapdata_hashmap(&map);
        assert_eq!(mapper.tranform(1), 42);
        assert_eq!(mapper.tranform(255), 99);
        assert_eq!(mapper.tranform(0), 0);
    }

    #[test]
    fn test_data_recognition_letter_with_range() {
        // Use with_range for lowercase
        let lower = Mapper::with_range(b'a'..=b'z', 1);
        for c in b'a'..=b'z' {
            assert_eq!(lower.tranform(c), 1);
        }
        assert_eq!(lower.tranform(b'A'), 0);
        assert_eq!(lower.tranform(b'0'), 0);

        // Use with_range for uppercase
        let upper = Mapper::with_range(b'A'..=b'Z', 2);
        for c in b'A'..=b'Z' {
            assert_eq!(upper.tranform(c), 2);
        }
        assert_eq!(upper.tranform(b'a'), 0);
        assert_eq!(upper.tranform(b'0'), 0);

        // Combine both
        let mut data = [0u8; MAX_LENGTH];
        for c in b'A'..=b'Z' {
            data[c as usize] = 1;
        }
        for c in b'a'..=b'z' {
            data[c as usize] = 1;
        }
        let both = Mapper::new_from(data);
        assert_eq!(both.tranform(b'A'), 1);
        assert_eq!(both.tranform(b'z'), 1);
        assert_eq!(both.tranform(b'0'), 0);
        assert_eq!(both.tranform(b'!'), 0);
    }

    #[test]
    fn test_data_recognition_properties_with_range() {
        let mut data = [0u8; MAX_LENGTH];
        // Use with_range for each property
        for (range, val) in [
            (b'a'..=b'z', 0b00000001),
            (b'A'..=b'Z', 0b00000010),
            (b'0'..=b'9', 0b00000100),
        ] {
            for c in range {
                data[c as usize] = val;
            }
        }
        for &c in b"!@#" {
            data[c as usize] = 0b00001000;
        }
        for &c in b" \t\n" {
            data[c as usize] = 0b00010000;
        }
        let mapper = Mapper::new_from(data);
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
        let mapper = Mapper::new_transformation(|x| if x > 5 { 1 } else { 0 });
        for i in 0u8..=5 {
            assert_eq!(mapper.tranform(i), 0);
        }
        for i in 6u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), 1);
        }
    }

    #[test]
    fn test_if_statement_emulation_less_than_5() {
        let mapper = Mapper::new_transformation(|x| if x < 5 { 1 } else { 0 });
        for i in 0u8..=4 {
            assert_eq!(mapper.tranform(i), 1);
        }
        for i in 5u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), 0);
        }
    }

    #[test]
    fn test_mathematical_operation_add_one() {
        let mapper = Mapper::new_transformation(|x| x.wrapping_add(1) as u8);
        for i in 0u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), i.wrapping_add(1));
        }
    }

    #[test]
    fn test_mathematical_operation_multiply_by_2() {
        let mapper = Mapper::new_transformation(|x| ((x as u8).wrapping_mul(2)));
        for i in 0u8..=u8::MAX {
            assert_eq!(mapper.tranform(i), i.wrapping_mul(2));
        }
    }

    #[test]
    fn test_mathematical_operation_square() {
        let mapper = Mapper::new_transformation(|x| {
            let y = x as u16;
            ((y * y) % 256) as u8
        });
        for i in 0u8..=u8::MAX {
            let expected = ((i as u16 * i as u16) % 256) as u8;
            assert_eq!(mapper.tranform(i), expected);
        }
    }
}
