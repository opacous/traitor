pub trait CartesianProduct {
    type BaseSpace;

    fn dim(&self) -> usize;
    fn nth(&self) -> Option<&Self::BaseSpace>;
    fn nth_mut(&self) -> Option<&mut Self::BaseSpace>;
}
