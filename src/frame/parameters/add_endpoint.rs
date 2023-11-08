use crate::ezsp::Status;
use crate::types::ByteSizedVec;
use serde::de::{Error, Visitor};
use serde::ser::SerializeSeq;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt::{Formatter, Write};

pub const ID: u16 = 0x0002;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Command {
    endpoint: u8,
    profile_id: u16,
    device_id: u16,
    app_flags: u8,
    input_cluster_count: u8,
    output_cluster_count: u8,
    input_cluster_list: ByteSizedVec<u16>,
    output_cluster_list: ByteSizedVec<u16>,
}

impl Command {
    #[must_use]
    pub const fn new(
        endpoint: u8,
        profile_id: u16,
        device_id: u16,
        app_flags: u8,
        input_cluster_count: u8,
        output_cluster_count: u8,
        input_cluster_list: ByteSizedVec<u16>,
        output_cluster_list: ByteSizedVec<u16>,
    ) -> Self {
        Self {
            endpoint,
            profile_id,
            device_id,
            app_flags,
            input_cluster_count,
            output_cluster_count,
            input_cluster_list,
            output_cluster_list,
        }
    }

    #[must_use]
    pub const fn endpoint(&self) -> u8 {
        self.endpoint
    }

    #[must_use]
    pub const fn profile_id(&self) -> u16 {
        self.profile_id
    }

    #[must_use]
    pub const fn device_id(&self) -> u16 {
        self.device_id
    }

    #[must_use]
    pub const fn app_flags(&self) -> u8 {
        self.app_flags
    }

    #[must_use]
    pub const fn input_cluster_count(&self) -> u8 {
        self.input_cluster_count
    }

    #[must_use]
    pub const fn output_cluster_count(&self) -> u8 {
        self.output_cluster_count
    }

    #[must_use]
    pub const fn input_cluster_list(&self) -> &ByteSizedVec<u16> {
        &self.input_cluster_list
    }

    #[must_use]
    pub const fn output_cluster_list(&self) -> &ByteSizedVec<u16> {
        &self.output_cluster_list
    }
}

impl Serialize for Command {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.endpoint)?;
        serializer.serialize_u16(self.profile_id)?;
        serializer.serialize_16(self.device_id)?;
        serializer.serialize_u8(self.app_flags)?;

        serializer.serialize_u8(self.input_cluster_list.len() as u8)?;
        serializer.serialize_u8(self.output_cluster_list.len() as u8)?;

        let mut seq = serializer.serialize_seq(Some(self.input_cluster_list.len()))?;
        for element in self.input_cluster_list.iter() {
            seq.serialize_element(element)?;
        }

        let mut seq = serializer.serialize_seq(Some(self.output_cluster_list.len()))?;
        for element in self.output_cluster_list.iter() {
            seq.serialize_element(element)?;
        }

        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Response {
    status: Status,
}

impl Response {
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self { status }
    }

    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }
}
use std::fmt;

use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};

impl<'de> Deserialize<'de> for Command {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Endpoint,
            ProfileId,
            DeviceId,
            AppFlags,
            InputClusters,
            OutputClusters,
        }

        // This part could also be generated independently by:
        //
        //    #[derive(Deserialize)]
        //    #[serde(field_identifier, rename_all = "lowercase")]
        //    enum Field { Secs, Nanos }
        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`secs` or `nanos`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "endpoint" => Ok(Field::Endpoint),
                            "profile_id" => Ok(Field::ProfileId),
                            "device_id" => Ok(Field::DeviceId),
                            "app_flags" => Ok(Field::AppFlags),
                            "input_cluster_list" => Ok(Field::InputClusters),
                            "output_cluster_list" => Ok(Field::OutputClusters),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = Duration;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Duration, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let secs = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let nanos = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                Ok(Duration::new(secs, nanos))
            }

            fn visit_map<V>(self, mut map: V) -> Result<Duration, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut secs = None;
                let mut nanos = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Secs => {
                            if secs.is_some() {
                                return Err(de::Error::duplicate_field("secs"));
                            }
                            secs = Some(map.next_value()?);
                        }
                        Field::Nanos => {
                            if nanos.is_some() {
                                return Err(de::Error::duplicate_field("nanos"));
                            }
                            nanos = Some(map.next_value()?);
                        }
                    }
                }
                let secs = secs.ok_or_else(|| de::Error::missing_field("secs"))?;
                let nanos = nanos.ok_or_else(|| de::Error::missing_field("nanos"))?;
                Ok(Duration::new(secs, nanos))
            }
        }

        const FIELDS: &'static [&'static str] = &["secs", "nanos"];
        deserializer.deserialize_struct("Duration", FIELDS, DurationVisitor)
    }
}
