use std::{fmt, str::FromStr};

use appbiotic_error::ValidationError;
use chrono::{DateTime, Utc};

/// A [chrono::DateTime<Utc>] newtype providing consistent serialization ad parsing.
#[cfg_attr(
    feature = "serde",
    derive(serde_with::DeserializeFromStr, serde_with::SerializeDisplay)
)]
#[derive(Clone, Debug)]
pub struct Timestamp(pub DateTime<Utc>);

impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true))
    }
}

impl From<DateTime<Utc>> for Timestamp {
    fn from(value: DateTime<Utc>) -> Self {
        Timestamp(value)
    }
}

impl From<Timestamp> for DateTime<Utc> {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}

impl FromStr for Timestamp {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse::<DateTime<Utc>>().map_err(|error| {
            ValidationError::InvalidFormat {
                message: error.to_string(),
            }
        })?))
    }
}

impl TryFrom<String> for Timestamp {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[cfg(feature = "prost")]
impl TryFrom<Timestamp> for prost_wkt_types::Timestamp {
    type Error = ValidationError;

    fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
        let nanos: i32 = value
            .0
            .timestamp_subsec_nanos()
            .try_into()
            .map_err(|error| ValidationError::InvalidFormat {
                message: format!("invalid timestamp nanos: {error:?}"),
            })?;

        Ok(Self {
            seconds: value.0.timestamp(),
            nanos,
        })
    }
}

#[cfg(feature = "prost")]
impl TryFrom<prost_wkt_types::Timestamp> for Timestamp {
    type Error = ValidationError;

    fn try_from(value: prost_wkt_types::Timestamp) -> Result<Self, Self::Error> {
        let nsec: u32 = value
            .nanos
            .try_into()
            .map_err(|error| ValidationError::InvalidFormat {
                message: format!("invalid timestamp nsec: {error:?}"),
            })?;
        let timestamp = DateTime::from_timestamp(value.seconds, nsec).ok_or_else(|| {
            ValidationError::new_invalid_format("invalid timestamp seconds and nanos")
        })?;
        Ok(Self(timestamp))
    }
}

#[cfg(test)]
mod test {
    use serde_json::{json, Value};

    use super::*;

    #[test]
    fn json_serialization_works() {
        let time_str = "2024-12-30T23:01:02.000000000Z";
        let time: Timestamp = time_str.parse().unwrap();
        let value = serde_json::to_value(&time).unwrap();
        assert_eq!(value, Value::String(time_str.to_owned()));
    }

    #[test]
    fn json_deserialization_works() {
        let time_str = "2024-12-30T23:01:02.000000000Z";
        let json = json!(time_str);
        let time: Timestamp = serde_json::from_value(json).unwrap();
        assert_eq!(time.to_string(), time_str);
    }
}
