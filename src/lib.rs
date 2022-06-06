pub trait Signature {
    fn id(&self) -> String;
    fn name(&self) -> String;
    fn lifetime(&self) -> usize;
}