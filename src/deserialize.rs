use chrono::{NaiveDateTime, NaiveTime};
use serde::de;
use std::{collections::HashMap, fmt, str::FromStr};

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

pub fn deserialize_option_csv_encoded_string<'a, D: de::Deserializer<'a>>(
    deserializer: D,
) -> Result<Option<Vec<i32>>, D::Error> {
    deserializer.deserialize_option(OptionCsvEncodedStringVisitor)
}

struct OptionCsvEncodedStringVisitor;

impl<'a> de::Visitor<'a> for OptionCsvEncodedStringVisitor {
    type Value = Option<Vec<i32>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string of comma-separated integers or null")
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_some<D: de::Deserializer<'a>>(self, d: D) -> Result<Self::Value, D::Error> {
        Ok(Some(d.deserialize_str(CsvEncodedStringVisitor)?))
    }
}

pub fn deserialize_optional_string_enum<'a, D: de::Deserializer<'a>, T: FromStr + 'a>(
    deserializer: D,
) -> Result<Option<T>, D::Error> {
    deserializer.deserialize_option(OptionStringEnumVisitor {
        _marker: Default::default(),
    })
}

#[derive(Default)]
struct OptionStringEnumVisitor<'a, T: FromStr> {
    _marker: std::marker::PhantomData<&'a T>,
}

impl<'a, T: FromStr> de::Visitor<'a> for OptionStringEnumVisitor<'a, T> {
    type Value = Option<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "a string representation an enum value that can be null"
        )
    }

    // Can be called T can be a PhantomData Unit Struct
    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_some<D: de::Deserializer<'a>>(self, d: D) -> Result<Self::Value, D::Error> {
        Ok(Some(d.deserialize_str(StringEnumVisitor {
            _marker: Default::default(),
        })?))
    }
}

pub fn deserialize_string_enum<'a, D: de::Deserializer<'a>, T: FromStr + 'a>(
    deserializer: D,
) -> Result<T, D::Error> {
    deserializer.deserialize_str(StringEnumVisitor {
        _marker: Default::default(),
    })
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

pub fn deserialize_api_error<'a, D: de::Deserializer<'a>>(
    deserializer: D,
) -> Result<String, D::Error> {
    deserializer.deserialize_seq(ApiErrorVisitor)
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

pub fn deserialize_naive_date_time<'a, D: de::Deserializer<'a>>(
    deserializer: D,
) -> Result<NaiveDateTime, D::Error> {
    deserializer.deserialize_str(NaiveDateTimeVisitor)
}

const DATE_TIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S%.f";

struct NaiveDateTimeVisitor;

impl<'a> de::Visitor<'a> for NaiveDateTimeVisitor {
    type Value = NaiveDateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a trivially encoded date string")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        match NaiveDateTime::parse_from_str(value, DATE_TIME_FORMAT) {
            Ok(date) => Ok(date),
            Err(e) => Err(E::custom(format!(
                "Error {} parsing timestamp {}",
                e, value
            ))),
        }
    }
}

pub fn deserialize_naive_time<'a, D: de::Deserializer<'a>>(
    deserializer: D,
) -> Result<NaiveTime, D::Error> {
    deserializer.deserialize_str(NaiveTimeVisitor)
}

const TIME_FORMAT: &str = "%I:%M%p";

struct NaiveTimeVisitor;

impl<'a> de::Visitor<'a> for NaiveTimeVisitor {
    type Value = NaiveTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a trivially encoded time string")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        match NaiveTime::parse_from_str(value, TIME_FORMAT) {
            Ok(date) => Ok(date),
            Err(e) => Err(E::custom(format!("Error {} parsing time {}", e, value))),
        }
    }
}

pub fn deserialize_naive_time_with_space<'a, D: de::Deserializer<'a>>(
    deserializer: D,
) -> Result<NaiveTime, D::Error> {
    deserializer.deserialize_str(NaiveTimeWithSpaceVisitor)
}

const TIME_FORMAT_WITH_SPACE: &str = "%I:%M %p";

struct NaiveTimeWithSpaceVisitor;

impl<'a> de::Visitor<'a> for NaiveTimeWithSpaceVisitor {
    type Value = NaiveTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "an encoded time string with a space between the minutes and the AM/PM"
        )
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        match NaiveTime::parse_from_str(value, TIME_FORMAT_WITH_SPACE) {
            Ok(date) => Ok(date),
            Err(e) => Err(E::custom(format!("Error {} parsing time {}", e, value))),
        }
    }
}

pub fn deserialize_option_naive_time_with_space<'a, D: de::Deserializer<'a>>(
    deserializer: D,
) -> Result<Option<NaiveTime>, D::Error> {
    deserializer.deserialize_str(OptionNaiveTimeWithSpaceVisitor)
}

struct OptionNaiveTimeWithSpaceVisitor;

impl<'a> de::Visitor<'a> for OptionNaiveTimeWithSpaceVisitor {
    type Value = Option<NaiveTime>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "an encoded time string with a space between the minutes and the AM/PM or 'na'"
        )
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        if value == "na" {
            Ok(None)
        } else {
            match NaiveTime::parse_from_str(value, TIME_FORMAT_WITH_SPACE) {
                Ok(date) => Ok(Some(date)),
                Err(e) => Err(E::custom(format!("Error {} parsing time {}", e, value))),
            }
        }
    }
}

pub fn deserialize_bool<'a, D: de::Deserializer<'a>>(deserializer: D) -> Result<bool, D::Error> {
    deserializer.deserialize_str(BoolStringVisitor)
}

struct BoolStringVisitor;

impl<'a> de::Visitor<'a> for BoolStringVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a trivially encoded bool")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        match value.to_ascii_lowercase().as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(de::Error::unknown_variant(value, &["true", "false"])),
        }
    }
}

pub fn deserialize_f64<'a, D: de::Deserializer<'a>>(deserializer: D) -> Result<f64, D::Error> {
    deserializer.deserialize_str(F64StringVisitor)
}

struct F64StringVisitor;

impl<'a> de::Visitor<'a> for F64StringVisitor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string encoded f64")
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        value
            .parse::<f64>()
            .map_err(|e| de::Error::custom(format!("Error {} parsing f64 {}", e, value)))
    }
}

pub fn deserialize_optional_f64<'a, D: de::Deserializer<'a>>(
    deserializer: D,
) -> Result<Option<f64>, D::Error> {
    deserializer.deserialize_option(OptionF64StringVisitor)
}

struct OptionF64StringVisitor;

impl<'a> de::Visitor<'a> for OptionF64StringVisitor {
    type Value = Option<f64>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a string encoded f64 or null")
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_unit<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_some<D: de::Deserializer<'a>>(self, d: D) -> Result<Self::Value, D::Error> {
        Ok(Some(d.deserialize_str(F64StringVisitor)?))
    }
}
