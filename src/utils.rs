pub use std::any::type_name;

// Find out the type of an object
pub fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}