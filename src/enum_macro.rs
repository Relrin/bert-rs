// Special wrapper around the enum type, which provide an opportunity to
// serialize/deserialize enum variants as a string


#[macro_export]
macro_rules! enum_str {
    ($name:ident { $($variant:ident($str:expr), )* }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $($variant,)*
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
                where S: ::serde::Serializer,
            {
                serializer.serialize_str(match *self {
                    $( $name::$variant => $str, )*
                })
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
                where D: ::serde::Deserializer<'de>,
            {
                struct EnumVisitor;

                impl<'de> ::serde::de::Visitor<'de> for EnumVisitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        formatter.write_str(concat!("a string matching one of the ", stringify!($name), " variants"))
                    }

                    fn visit_str<E>(self, value: &str) -> ::std::result::Result<$name, E>
                        where E: ::serde::de::Error,
                    {
                        match value {
                            $( $str => Ok($name::$variant), )*
                            _ => Err(E::custom(format!("unknown {} variant: {}", stringify!($name), value))),
                        }
                    }
                }

                deserializer.deserialize_str(EnumVisitor)
            }
        }
    }
}
