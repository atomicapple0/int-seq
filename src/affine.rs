/// An affine sequence is a sequence of the form `a*i + b`
pub struct Affine {
    a: i32,
    b: i32,
}

impl Affine {
    pub fn infer_from(seq: &[i32]) -> Option<Self> {
        match seq.len() {
            0 => return None,
            1 => return Some(Self { a: 1, b: seq[0] }),
            _ => {
                let b = seq[0];
                let a = seq[1] - b;
                for (i, x) in seq.iter().enumerate() {
                    if *x != a * (i as i32) + b {
                        return None;
                    }
                }
                Some(Self { a, b })
            }
        }
    }

    pub fn generate(&self, start: i32, end: i32) -> Vec<i32> {
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
