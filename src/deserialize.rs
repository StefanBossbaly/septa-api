use serde::de;
use std::{collections::HashMap, fmt, str::FromStr};

pub fn deserialize_csv_encoded_string<'a, D: de::Deserializer<'a>>(
    deserializer: D,
) -> Result<Vec<i32>, D::Error> {
    deserializer.deserialize_str(CsvEncodedStringVisitor)
}

pub fn deserialize_string_enum<'a, D: de::Deserializer<'a>, T: FromStr + 'a>(
    deserializer: D,
) -> Result<T, D::Error> {
    deserializer.deserialize_str(StringEnumVisitor {
        _marker: std::marker::PhantomData,
    })
}

pub fn deserialize_api_error<'a, D: de::Deserializer<'a>>(
    deserializer: D,
) -> Result<String, D::Error> {
    deserializer.deserialize_seq(ApiErrorVisitor)
}

struct CsvEncodedStringVisitor;

impl<'a> de::Visitor<'a> for CsvEncodedStringVisitor {
    type Value = Vec<i32>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string of comma-separated integers")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        let mut result = Vec::new();

        for s in v.split(',') {
            if !s.is_empty() {
                result.push(s.parse::<i32>().map_err(|e| {
                    de::Error::custom(format!("invalid integer {}, caused error {}", s, e))
                })?);
            }
        }

        Ok(result)
    }
}

#[derive(Default)]
struct StringEnumVisitor<'a, T: FromStr> {
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T: FromStr> de::Visitor<'a> for StringEnumVisitor<'a, T> {
    type Value = T;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string representation an enum value")
    }

    fn visit_str<E: de::Error>(self, v: &str) -> Result<Self::Value, E> {
        let enum_value: T = v
            .parse()
            .map_err(|_| de::Error::custom(format!("invalid enum value {}", v)))?;

        Ok(enum_value)
    }
}

struct ApiErrorVisitor;

impl<'a> de::Visitor<'a> for ApiErrorVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "an SEPTA API error")
    }

    fn visit_seq<A: de::SeqAccess<'a>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let mut error: Option<String> = None;

        while let Some(element) = seq.next_element::<HashMap<String, String>>()? {
            if error.is_some() {
                return Err(de::Error::custom("expected only one error"));
            } else if element.contains_key("error") {
                error = element.get("error").map(|s| s.to_string());
            }
        }

        error.ok_or_else(|| de::Error::custom("expected an error"))
    }
}
