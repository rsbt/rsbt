use std::any::Any;

pub(crate) type AnyResult = Box<dyn Any + Send>;
