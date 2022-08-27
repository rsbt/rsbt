use std::any::Any;

pub type AnyResult = Box<dyn Any + Send + Sync>;
