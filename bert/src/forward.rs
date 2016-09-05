// This module provide a special macro which takes names of method
// separated by comma. For each specified method will be returned an error
// in according to an interface.


#[macro_export]
macro_rules! forward_deserialize {
    ($($name:ident($($arg:ident: $ty:ty,)*);)*) => {
        $(#[inline]
        fn $name<V: Visitor>(&mut self, $($arg: $ty,)* visitor: V) -> Result<V::Value> {
            self.deserialize(visitor)
        })*
    }
}
