use super::*;

pub trait CloneBoxAny: std::fmt::Debug + 'static {
    fn clone_box_any(self: &Self) -> Box<dyn CloneBoxAny>;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn as_any(&self) -> &dyn Any;
}

impl<T: Clone + std::fmt::Debug + 'static> CloneBoxAny for T {
    fn clone_box_any(self: &Self) -> Box<dyn CloneBoxAny> {
        Box::new(self.clone())
    }
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

pub fn downcast_clone_box_any<T: 'static>(clone_box_any: &Box<dyn CloneBoxAny>) -> Option<&T> {
    CloneBoxAny::as_any(&**clone_box_any).downcast_ref()
}
pub fn downcast_clone_box_any_mut<T: 'static>(
    clone_box_any: &mut Box<dyn CloneBoxAny>,
) -> Option<&mut T> {
    CloneBoxAny::as_any_mut(&mut **clone_box_any).downcast_mut()
}
