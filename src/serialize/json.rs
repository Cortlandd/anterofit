extern crate serde_json;

use mime::{Mime, TopLevel, SubLevel};

use std::io::{Read, Write};

use super::{Serialize, Deserialize};

use ::error::map_res;
use ::Result;

pub use self::serde_json::Error;

pub struct Serializer;

impl super::Serializer for Serializer {
    fn serialize<T: Serialize, W: Write>(&self, val: &T, write: &mut W) -> Result<()> {
        map_res(self::serde_json::to_writer(write, val))
    }

    fn content_type(&self) -> Option<Mime> {
        Some(mime::json())
    }
}

pub struct PrettySerializer;

impl super::Serializer for PrettySerializer {
    fn serialize<T: Serialize, W: Write>(&self, val: &T, write: &mut W) -> Result<()> {
        map_res(self::serde_json::to_writer_pretty(write, val))
    }

    fn content_type(&self) -> Option<Mime> {
        Some(mime::json())
    }
}

pub struct Deserializer;

impl super::Deserializer for Deserializer {
    fn deserialize<T: Deserialize, R: Read>(&self, read: &mut R) -> Result<T> {
        map_res(self::serde_json::from_reader(read))
    }
}