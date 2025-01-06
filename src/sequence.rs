pub(crate) trait Sequence {
    fn infer(seq: &[i128]) -> Option<Self>
    where
        Self: Sized;
    fn generate(&self, seq: &[i128], end: i128) -> Vec<i128>;
}
