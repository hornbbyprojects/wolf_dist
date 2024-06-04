use crate::resources::Resources;

#[derive(Debug, Clone)]
pub enum Need {
    Resources(Box<Resources>),
    IsFalse(bool),
    IsTrue(bool),
}

impl Need {
    pub fn as_is_true(&self) -> Option<bool> {
        match self {
            Need::IsTrue(x) => Some(*x),
            _ => None,
        }
    }
    pub fn as_is_false(&self) -> Option<bool> {
        match self {
            Need::IsFalse(x) => Some(*x),
            _ => None,
        }
    }
    pub fn is_satisfied(&self) -> bool {
        match self {
            Need::Resources(resources) => resources.is_empty(),
            Need::IsFalse(x) => !x,
            Need::IsTrue(x) => *x,
        }
    }
}
