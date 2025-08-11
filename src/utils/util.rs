use hex;
use rand::rngs::OsRng;
use rand::TryRngCore;

pub mod option_chrono_datetime_as_bson_datetime {
    use bson::Bson;
    use chrono::{DateTime, Utc};
    use serde::de::{Error as DeError, Unexpected};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => {
                bson::serde_helpers::chrono_datetime_as_bson_datetime::serialize(d, serializer)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize into generic BSON first
        let bson_value = Bson::deserialize(deserializer)?;

        match bson_value {
            Bson::Null => Ok(None),
            Bson::DateTime(dt) => Ok(Some(dt.to_chrono())),
            Bson::String(s) => DateTime::parse_from_rfc3339(&s)
                .map(|dt| Some(dt.with_timezone(&Utc)))
                .map_err(|_| {
                    D::Error::invalid_value(Unexpected::Str(&s), &"valid RFC 3339 datetime string")
                }),
            other => Err(D::Error::invalid_type(
                Unexpected::Other(&format!("{:?}", other)),
                &"a BSON datetime, RFC 3339 string, or null",
            )),
        }
    }
}

pub fn generate_api_key() -> String {
    let mut key = [0u8; 32]; // 256-bit key
    let mut rng = OsRng; // instantiate the RNG
    let _ = rng.try_fill_bytes(&mut key);
    hex::encode(key)
}
