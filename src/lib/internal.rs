use serde::de::*;
use serde::*;

macro_rules! id_i64 {
    ($($name:ident;)*) => {
        $(
            impl $name {
                /// Immutably borrow inner Id.
                #[inline]
                pub fn as_u64(&self) -> &i64 {
                    &self.0
                }

                /// Mutably borrow inner Id.
                #[inline]
                pub fn as_mut_u64(&mut self) -> &mut i64 {
                    &mut self.0
                }
            }

            // This is a hack so functions can accept iterators that either:
            // 1. return the id itself (e.g: `MessageId`)
            // 2. return a reference to it (`&MessageId`).
            impl AsRef<$name> for $name {
                fn as_ref(&self) -> &Self {
                    self
                }
            }

            impl<'a> From<&'a $name> for $name {
                fn from(id: &'a $name) -> $name {
                    id.clone()
                }
            }

            impl From<i64> for $name {
                fn from(id_as_i64: i64) -> $name {
                    $name(id_as_i64)
                }
            }

            impl PartialEq<i64> for $name {
                fn eq(&self, u: &i64) -> bool {
                    self.0 == *u
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::Display::fmt(&self.0, f)
                }
            }

            impl<'de> Deserialize<'de> for $name {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    deserializer.deserialize_any(I64Visitor).map($name)
                }
            }

            impl Serialize for ScheduleId {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    serializer.serialize_u64(self.0 as u64)
                }
            }

            impl From<$name> for u64 {
                fn from(id: $name) -> u64 {
                    id.0 as u64
                }
            }

            impl From<$name> for i64 {
                fn from(id: $name) -> i64 {
                    id.0 as i64
                }
            }
        )*
    }
}

#[derive(Copy, Clone, Default, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ScheduleId(pub i64);

macro_rules! num_visitors {
    ($($visitor:ident: $type:ty),*) => {
        $(
            #[derive(Debug)]
            pub struct $visitor;

            impl<'de> Visitor<'de> for $visitor {
                type Value = $type;

                fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    formatter.write_str("identifier")
                }

                fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
                    v.parse::<$type>().map_err(|_| {
                        let mut s = String::with_capacity(32);
                        s.push_str("Unknown ");
                        s.push_str(stringify!($type));
                        s.push_str(" value: ");
                        s.push_str(v);

                        de::Error::custom(s)
                    })
                }

                fn visit_i64<E: de::Error>(self, v: i64) -> Result<Self::Value, E> { Ok(v as $type) }

                fn visit_u64<E: de::Error>(self, v: u64) -> Result<Self::Value, E> { Ok(v as $type) }

                // This is called when serde_json's `arbitrary_precision` feature is enabled.
                fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                    struct Id {
                        num: $type,
                    }

                    struct StrVisitor;

                    impl<'de> Visitor<'de> for StrVisitor {
                        type Value = $type;

                        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            formatter.write_str("string")
                        }

                        fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> { s.parse().map_err(E::custom) }
                        fn visit_string<E: de::Error>(self, s: String) -> Result<Self::Value, E> { s.parse().map_err(E::custom) }
                    }

                    impl<'de> Deserialize<'de> for Id {
                        fn deserialize<D: Deserializer<'de>>(des: D) -> Result<Self, D::Error> {
                            Ok(Id { num: des.deserialize_str(StrVisitor)? })
                        }
                    }

                    map.next_value::<Id>().map(|id| id.num)
                }
            }
        )*
    }
}

num_visitors!(I64Visitor: i64);

id_i64! {
    ScheduleId;
}
