// https://serde.rs/deserialize-struct.html

use std::{collections::HashMap, fs, io, path::Path};

use convert_case::{Case, Casing};
use serde::{
    de::{self, MapAccess, VariantAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Value;

#[derive(Debug)]
pub enum Err {
    Nyi(String),
    Io(io::Error),
    Serde(serde_json::Error),
}

impl From<io::Error> for Err {
    fn from(value: io::Error) -> Self {
        Err::Io(value)
    }
}

impl From<serde_json::Error> for Err {
    fn from(value: serde_json::Error) -> Self {
        Err::Serde(value)
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct Notes(pub HashMap<String, Value>);

#[derive(Debug, Deserialize, Default)]
pub enum TyE {
    #[default]
    Unit,
    #[serde(rename = "cds.String")]
    Str,
    #[serde(rename = "cds.Integer")]
    Int,
}

#[derive(Debug, Default)]
pub enum Ty {
    #[default]
    Unit,
    Str,
    Int,
}

impl<'de> Deserialize<'de> for Ty {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const VARIANTS: &[&str] = &["cds.String", "cds.Integer"];

        struct Vis<T>(Option<T>);

        enum _Ty {
            Str,
            Int,
        }

        impl<'de> Deserialize<'de> for _Ty {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                impl<'de> Visitor<'de> for Vis<_Ty> {
                    type Value = _Ty;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("_Ty")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match v {
                            "cds.Integer" => Ok(_Ty::Int),
                            "cds.String" => Ok(_Ty::Str),
                            _ => Err(de::Error::unknown_variant(v, VARIANTS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(Vis::<_Ty>(None))
            }
        }

        impl<'de> Visitor<'de> for Vis<Ty> {
            type Value = Ty;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Ty")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: de::EnumAccess<'de>,
            {
                match data.variant()? {
                    (_Ty::Str, var) => {
                        var.unit_variant()?;
                        Ok(Ty::Str)
                    }
                    (_Ty::Int, var) => {
                        var.unit_variant()?;
                        Ok(Ty::Int)
                    }
                }
            }
        }

        deserializer.deserialize_enum("Ty", VARIANTS, Vis::<Ty>(None))
    }
}

#[derive(Debug, Default)]
pub struct Desc {
    pub is_key: bool,
    pub ty: Ty,
    pub notes: Notes,
}

impl<'de> Deserialize<'de> for Desc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Vis {}

        impl<'de> Visitor<'de> for Vis {
            type Value = Desc;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Desc")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut desc = Desc {
                    ..Default::default()
                };
                while let Some(key) = map.next_key()? {
                    match key {
                        k if k.starts_with('@') => {
                            desc.notes.0.insert(
                                key[1..].to_string().to_case(Case::Snake),
                                map.next_value()?,
                            );
                        }
                        "key" => {
                            desc.is_key = map.next_value()?;
                        }
                        "type" => {
                            desc.ty = map.next_value()?;
                        }
                        _ => {
                            map.next_value::<Value>()?;
                        }
                    }
                }
                Ok(desc)
            }
        }

        deserializer.deserialize_map(Vis {})
    }
}

impl<'de> Deserialize<'de> for Fields {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Vis {}

        impl<'de> Visitor<'de> for Vis {
            type Value = Fields;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Fields")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut hm = HashMap::new();
                while let Some(key) = map.next_key::<String>()? {
                    hm.insert(key.to_case(Case::Snake), map.next_value::<Desc>()?);
                }
                Ok(Fields(hm))
            }
        }

        deserializer.deserialize_map(Vis {})
    }
}

#[derive(Debug)]
pub struct Fields(pub HashMap<String, <Fields as HasDesc>::Desc>);

pub trait HasDesc {
    type Desc;
}

impl HasDesc for Fields {
    type Desc = Desc;
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind", content = "elements")]
pub enum Definition {
    #[serde(rename = "entity")]
    Entity(Fields),
}

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub creator: String,
    pub flavor: String,
}

#[derive(Debug, Deserialize)]
pub struct Model {
    pub definitions: HashMap<String, Definition>,
    pub meta: Meta,
    #[serde(rename = "$version")]
    pub version: String,
}

impl Model {
    pub fn new<P>(p: P) -> Result<Model, Err>
    where
        P: AsRef<Path>,
    {
        let content = fs::read_to_string(p)?;
        let mut deserializer = serde_json::Deserializer::from_str(&content);
        // let model = serde_json::from_str(&content)?;
        let model = Model::deserialize(&mut deserializer)?;
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_loads_model() -> Result<(), Err> {
        let model = Model::new("data/example.json");
        dbg!(model);
        Ok(())
    }
}
