use crate::neural::traits::Combinator;

pub fn combinator_from_str(name: &str) -> Option<Box<dyn Combinator>> {
    match name.to_lowercase().as_str() {
        AdditionCombinatorNode::IDENTIFIER | "add" | "+" => {
            Some(Box::new(AdditionCombinatorNode {}))
        }
        MultiplicationCombinatorNode::IDENTIFIER | "multiplication" | "*" => {
            Some(Box::new(MultiplicationCombinatorNode {}))
        }
        MaxCombinatorNode::IDENTIFIER => Some(Box::new(MaxCombinatorNode {})),
        MinCombinatorNode::IDENTIFIER => Some(Box::new(MinCombinatorNode {})),
        AverageCombinatorNode::IDENTIFIER | "avg" => Some(Box::new(AverageCombinatorNode {})),
        MedianCombinatorNode::IDENTIFIER => Some(Box::new(MedianCombinatorNode {})),
        ORCombinatorNode::IDENTIFIER => Some(Box::new(ORCombinatorNode {})),
        ANDCombinatorNode::IDENTIFIER => Some(Box::new(ANDCombinatorNode {})),
        XORCombinatorNode::IDENTIFIER => Some(Box::new(XORCombinatorNode {})),
        NANDCombinatorNode::IDENTIFIER => Some(Box::new(NANDCombinatorNode {})),
        NORCombinatorNode::IDENTIFIER => Some(Box::new(NORCombinatorNode {})),
        XNORCombinatorNode::IDENTIFIER => Some(Box::new(XNORCombinatorNode {})),
        _ => None,
    }
}

pub struct AdditionCombinatorNode {}

impl AdditionCombinatorNode {
    pub const IDENTIFIER: &'static str = "addition";
}

impl Combinator for AdditionCombinatorNode {
    fn combine(&self, inputs: &[u8]) -> u8 {
        inputs.iter().sum()
    }
    fn identifier(&self) -> String {
        Self::IDENTIFIER.to_string()
    }
}

pub struct MultiplicationCombinatorNode {}

impl MultiplicationCombinatorNode {
    pub const IDENTIFIER: &'static str = "multiply";
}

impl Combinator for MultiplicationCombinatorNode {
    fn combine(&self, inputs: &[u8]) -> u8 {
        inputs.iter().product()
    }
    fn identifier(&self) -> String {
        Self::IDENTIFIER.to_string()
    }
}

pub struct MaxCombinatorNode {}
impl MaxCombinatorNode {
    pub const IDENTIFIER: &'static str = "max";
}
impl Combinator for MaxCombinatorNode {
    fn combine(&self, inputs: &[u8]) -> u8 {
        *inputs.iter().max().unwrap_or(&0)
    }
    fn identifier(&self) -> String {
        Self::IDENTIFIER.to_string()
    }
}

pub struct MinCombinatorNode {}
impl MinCombinatorNode {
    pub const IDENTIFIER: &'static str = "min";
}
impl Combinator for MinCombinatorNode {
    fn combine(&self, inputs: &[u8]) -> u8 {
        *inputs.iter().min().unwrap_or(&0)
    }
    fn identifier(&self) -> String {
        Self::IDENTIFIER.to_string()
    }
}

pub struct AverageCombinatorNode {}
impl AverageCombinatorNode {
    pub const IDENTIFIER: &'static str = "average";
}
impl Combinator for AverageCombinatorNode {
    fn combine(&self, inputs: &[u8]) -> u8 {
        if inputs.is_empty() {
            return 0;
        }
        let sum: usize = inputs.iter().map(|&x| x as usize).sum();
        (sum / inputs.len()) as u8
    }
    fn identifier(&self) -> String {
        Self::IDENTIFIER.to_string()
    }
}

pub struct MedianCombinatorNode {}
impl MedianCombinatorNode {
    pub const IDENTIFIER: &'static str = "median";
}
impl Combinator for MedianCombinatorNode {
    fn combine(&self, inputs: &[u8]) -> u8 {
        if inputs.is_empty() {
            return 0;
        }
        let mut sorted = inputs.to_vec();
        sorted.sort_unstable();
        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            (sorted[mid - 1] + sorted[mid]) / 2
        } else {
            sorted[mid]
        }
    }
    fn identifier(&self) -> String {
        Self::IDENTIFIER.to_string()
    }
}

pub struct ORCombinatorNode {}
impl ORCombinatorNode {
    pub const IDENTIFIER: &'static str = "or";
}
impl Combinator for ORCombinatorNode {
    fn combine(&self, inputs: &[u8]) -> u8 {
        inputs.iter().fold(0, |acc, &x| acc | x)
    }
    fn identifier(&self) -> String {
        Self::IDENTIFIER.to_string()
    }
}
pub struct ANDCombinatorNode {}
impl ANDCombinatorNode {
    pub const IDENTIFIER: &'static str = "and";
}
impl Combinator for ANDCombinatorNode {
    fn combine(&self, inputs: &[u8]) -> u8 {
        inputs.iter().fold(0xFF, |acc, &x| acc & x)
    }
    fn identifier(&self) -> String {
        Self::IDENTIFIER.to_string()
    }
}

pub struct XORCombinatorNode {}
impl XORCombinatorNode {
    pub const IDENTIFIER: &'static str = "xor";
}
impl Combinator for XORCombinatorNode {
    fn combine(&self, inputs: &[u8]) -> u8 {
        inputs.iter().fold(0, |acc, &x| acc ^ x)
    }
    fn identifier(&self) -> String {
        Self::IDENTIFIER.to_string()
    }
}
pub struct NANDCombinatorNode {}
impl NANDCombinatorNode {
    pub const IDENTIFIER: &'static str = "nand";
}
impl Combinator for NANDCombinatorNode {
    fn combine(&self, inputs: &[u8]) -> u8 {
        let and_result = inputs.iter().fold(0xFF, |acc, &x| acc & x);
        !and_result
    }
    fn identifier(&self) -> String {
        Self::IDENTIFIER.to_string()
    }
}
pub struct NORCombinatorNode {}
impl NORCombinatorNode {
    pub const IDENTIFIER: &'static str = "nor";
}
impl Combinator for NORCombinatorNode {
    fn combine(&self, inputs: &[u8]) -> u8 {
        let or_result = inputs.iter().fold(0, |acc, &x| acc | x);
        !or_result
    }
    fn identifier(&self) -> String {
        Self::IDENTIFIER.to_string()
    }
}
pub struct XNORCombinatorNode {}
impl XNORCombinatorNode {
    pub const IDENTIFIER: &'static str = "xnor";
}
impl Combinator for XNORCombinatorNode {
    fn combine(&self, inputs: &[u8]) -> u8 {
        let xor_result = inputs.iter().fold(0, |acc, &x| acc ^ x);
        !xor_result
    }
    fn identifier(&self) -> String {
        Self::IDENTIFIER.to_string()
    }
}
