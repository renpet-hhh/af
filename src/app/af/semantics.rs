use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum Acceptability {
    IN,
    OUT,
    UNDEC,
}

#[derive(Debug, Clone)]
pub struct Labelling(pub Vec<Acceptability>);

pub trait Semantics {
    fn complete(&self) -> Vec<Labelling>;
    fn stable(&self) -> Vec<Labelling>;
    fn preferred(&self) -> Vec<Labelling>;
    fn get_semantics(&self, criteria: SemanticsType) -> Vec<Labelling> {
        match criteria {
            SemanticsType::COMPLETE => self.complete(),
            SemanticsType::GROUNDED => vec![],
            SemanticsType::PREFERRED => self.preferred(),
            SemanticsType::STABLE => self.stable(),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SemanticsType {
    COMPLETE,
    GROUNDED,
    PREFERRED,
    STABLE,
}
const SEMANTICS_NAME: &'static [&'static str] = &["Complete", "Grounded", "Preferred", "Stable"];

impl Into<String> for SemanticsType {
    fn into(self) -> String {
        String::from(SEMANTICS_NAME[self as usize])
    }
}
impl From<String> for SemanticsType {
    fn from(value: String) -> Self {
        if value == SEMANTICS_NAME[0] {
            SemanticsType::COMPLETE
        } else if value == SEMANTICS_NAME[1] {
            SemanticsType::GROUNDED
        } else if value == SEMANTICS_NAME[2] {
            SemanticsType::PREFERRED
        } else {
            SemanticsType::STABLE
        }
    }
}
impl Into<usize> for SemanticsType {
    fn into(self) -> usize {
        let s: String = self.into();
        for i in 0..SEMANTICS_NAME.len() {
            if s == SEMANTICS_NAME[i] {
                return i;
            }
        }
        panic!("Unknown semantics");
    }
}
