use serde::de;
use std::fmt;

pub fn deserialize_csv_encoded_string<'a, D: de::Deserializer<'a>>(
    deserializer: D,
) -> Result<Vec<i32>, D::Error> {
    deserializer.deserialize_str(CsvEncodedStringVisitor)
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
                result.push(
                    s.parse::<i32>()
                        .map_err(|_| de::Error::custom(format!("invalid integer {}", s)))?,
                );
            }
        }

        Ok(result)
    }
}
