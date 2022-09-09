use chrono::{FixedOffset, NaiveDateTime, TimeZone, Utc};


const BEIJING_TIME_OFFSET: i32 = 8*3600;

pub fn naive_date_time_now()->NaiveDateTime {
    Utc::now()
        .with_timezone(&FixedOffset::east(BEIJING_TIME_OFFSET))
        .naive_local()
}

pub fn naive_date_time_from_nanos_time(timestamp: u64) -> NaiveDateTime {
    FixedOffset::east(BEIJING_TIME_OFFSET)
        .timestamp_nanos(timestamp as i64)
        .naive_local()
}

pub mod u128_dec_format {
    use serde::de;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(num: &u128, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&num.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u128, D::Error>
        where
            D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

pub mod u64_dec_format {
    use serde::de;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(num: &u64, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&num.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
        where
            D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}