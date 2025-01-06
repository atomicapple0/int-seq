use crate::sequence::Sequence;

/// An affine sequence is a sequence of the form `a*i + b`
pub struct AffineSeq {
    a: i128,
    b: i128,
}

impl Sequence for AffineSeq {
    fn infer(seq: &[i128]) -> Option<Self> {
        match seq.len() {
            0 => None,
            1 => Some(Self { a: 1, b: seq[0] }),
            _ => {
                let b = seq[0];
                let a = seq[1] - b;
                for (i, x) in seq.iter().enumerate() {
                    if *x != a * (i as i128) + b {
                        return None;
                    }
                }
                Some(Self { a, b })
            }
        }
    }

    fn generate(&self, seq: &[i128], end: i128) -> Vec<i128> {
        let start = seq[0];
        assert!(start <= end);
        assert!((start - self.b) % self.a == 0);
        let mut seq = Vec::new();
        let mut curr = start;
        while curr < end {
            seq.push(curr);
            curr += self.a;
        }
        seq
    }
}
