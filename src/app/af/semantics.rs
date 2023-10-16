use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum Acceptability {
    IN,
    OUT,
    UNDEC,
}

#[derive(Debug)]
pub struct Labelling(pub Vec<Acceptability>);

pub trait Semantics {
    fn complete(&self) -> Vec<Labelling>;
    fn stable(&self) -> Vec<Labelling>;
    fn preferred(&self) -> Vec<Labelling>;
}
