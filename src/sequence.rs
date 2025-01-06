use crate::{affine::AffineSeq, oeis::OeisSeq};

pub(crate) trait Sequence {
    fn infer(seq: &[i128]) -> Option<Self>
    where
        Self: Sized;
    fn generate(&self, seq: &[i128], end: i128) -> Vec<i128>;
}

pub(crate) fn infer_sequence(seq: &[i128]) -> Option<Box<dyn Sequence>> {
    if let Some(seq) = AffineSeq::infer(seq) {
        return Some(Box::new(seq));
    }
    if let Some(seq) = OeisSeq::infer(seq) {
        return Some(Box::new(seq));
    }
    None
}
