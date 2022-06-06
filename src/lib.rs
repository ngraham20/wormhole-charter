pub trait Signature {
    fn id(&self) -> String;
    fn name(&self) -> String;
    fn lifetime(&self) -> usize;
}

#[macro_export]
macro_rules! sig {
    ($x:ident) => {
        impl Signature for $x {
            fn id(&self) -> String {
                self.id.clone()
            }
            fn name(&self) -> String {
                self.name.clone()
            }
            fn lifetime(&self) -> usize {
                self.lifetime
            }
        }
    }
}