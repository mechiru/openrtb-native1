#[doc(hidden)]
#[macro_export]
macro_rules! impl_enum_serde {
	  (
        #[exchange(ident = $exchange:ident, greater = $n:expr)]
        $ty:ty { $($var:ident = $val:expr),+, }
    ) => {
        impl serde::Serialize for $ty {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let v: i32 = match *self {
                    $(
                        Self::$var => $val,
                    )*
                    Self::$exchange(v) => v,
                };
                serializer.serialize_i32(v)
            }
        }

        impl<'de> serde::Deserialize<'de> for $ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                match i32::deserialize(deserializer)? {
                    $(
                        $val => Ok(Self::$var),
                    )*
                    v if v > $n => Ok(Self::ExchangeSpecific(v)),
                    v => Err(serde::de::Error::custom(format!(
                        concat!(
                            "invalid value: {}, expected one of ",
                            $( $val, ", ", )*
                            "or greater ",
                            $n,
                        ),
                        v
                    ))),
                }
            }
        }
	  };
}
