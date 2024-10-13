pub mod constant;

use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use getset::{CopyGetters, Getters};

use crate::FirmwareEdition::{Plus, Premium, Standard};
use crate::ParseFirmwareEditionError::ParseEditionError;
use crate::ParseFirmwareVersionError::{InvalidSubversionFormatError, UnmatchedSubversionError};

#[derive(Default, Debug, Clone)]
pub enum FirmwareEdition {
    #[default]
    Standard,
    Plus,
    Premium,
}

#[derive(Debug)]
pub enum ParseFirmwareVersionError {
    UnmatchedSubversionError,
    InvalidSubversionFormatError,
}

#[derive(Debug)]
pub enum ParseFirmwareEditionError {
    ParseEditionError,
}

#[derive(Default, Debug, Clone)]
pub struct FirmwareVersion {
    major: u8,
    minor: u8,
    patch: u8,
}

#[derive(Getters, CopyGetters, Clone, Default)]
pub struct Firmware {
    #[get = "pub"]
    serial_number: String,
    #[get = "pub"]
    size: u64,
    #[get = "pub"]
    compile_time: DateTime<Utc>,
    #[get = "pub"]
    edition: FirmwareEdition,
    #[get = "pub"]
    version: FirmwareVersion,
}

impl fmt::Display for Firmware {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Serial Number: {}  Size(KB): {}    Compile Time: {}    Version: {}    Edition: {}",
            self.serial_number,
            self.size / 1024,
            self.compile_time,
            self.version,
            self.edition
        )
    }
}

impl Firmware {
    pub fn from(
        serial_number: String,
        size: u64,
        compile_time: String,
        edition: String,
        version: String,
    ) -> Option<Self> {
        if serial_number.is_empty() {
            return None;
        }
        if size == 0 {
            return None;
        }
        let compile_time = DateTime::parse_from_rfc3339(compile_time.as_str());
        if compile_time.is_err() {
            return None;
        }
        let compile_time = compile_time.unwrap().to_utc();
        let edition = FirmwareEdition::from_str(edition.as_str());
        if edition.is_err() {
            return None;
        }
        let edition = edition.unwrap();
        let version = FirmwareVersion::from_str(version.as_str());
        if version.is_err() {
            return None;
        }
        let version = version.unwrap();
        Some(Self {
            serial_number,
            size,
            compile_time,
            edition,
            version,
        })
    }
}

impl FromStr for FirmwareEdition {
    type Err = ParseFirmwareEditionError;

    fn from_str(input: &str) -> Result<FirmwareEdition, Self::Err> {
        match input {
            "STANDARD" => Ok(Standard),
            "PLUS" => Ok(Plus),
            "PREMIUM" => Ok(Premium),
            "Standard" => Ok(Standard),
            "Plus" => Ok(Plus),
            "Premium" => Ok(Premium),
            _ => Err(ParseEditionError),
        }
    }
}

impl FromStr for FirmwareVersion {
    type Err = ParseFirmwareVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return FirmwareVersion::parse(s);
    }
}

impl fmt::Display for FirmwareEdition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let edition = match self {
            Standard => "STANDARD",
            Plus => "PLUS",
            Premium => "PREMIUM",
        };
        write!(f, "{}", edition)
    }
}

impl fmt::Display for FirmwareVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let version = format!("{}.{}.{}", self.major, self.minor, self.patch);
        write!(f, "{}", version)
    }
}

impl FirmwareVersion {
    fn parse(version: &str) -> Result<Self, ParseFirmwareVersionError> {
        let arguments: Vec<&str> = version.trim().split('.').collect();
        if arguments.len() != 3 {
            return Err(UnmatchedSubversionError);
        }
        let major = Self::parse_subversion(arguments[0]);
        let minor = Self::parse_subversion(arguments[1]);
        let patch = Self::parse_subversion(arguments[2]);
        if major.is_none() || minor.is_none() || patch.is_none() {
            return Err(InvalidSubversionFormatError);
        }
        return Ok(Self {
            major: major.unwrap(),
            minor: minor.unwrap(),
            patch: patch.unwrap(),
        });
    }

    fn parse_subversion(subversion: &str) -> Option<u8> {
        if subversion.len() == 1 {
            let result = subversion.parse::<u8>();
            if result.is_ok() {
                return Some(result.unwrap());
            }
        } else if subversion.len() == 2 {
            let subversion = subversion;
            if subversion.starts_with('0') {
                let char = subversion.chars().nth(1).unwrap();
                if char.is_ascii_digit() {
                    return Some(char as u8);
                }
            }
            let result = subversion.parse::<u8>();
            if result.is_ok() {
                return Some(result.unwrap());
            }
        }
        return None;
    }
}

#[derive(Getters, CopyGetters, Default, Debug, Clone, Eq, PartialEq)]
pub struct BinaryFirmware {
    #[get = "pub"]
    timestamp: i64,
    #[get = "pub"]
    serial_number: String,
}

impl Ord for BinaryFirmware {
    fn cmp(&self, other: &Self) -> Ordering {
        other.timestamp.cmp(&self.timestamp)
    }
}

impl PartialOrd for BinaryFirmware {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl BinaryFirmware {
    pub fn new(timestamp: i64, serial_number: String) -> Self {
        Self {
            timestamp,
            serial_number,
        }
    }
    pub fn from_firmware(firmware: &Firmware) -> Self {
        Self {
            timestamp: Utc::now().timestamp(),
            serial_number: firmware.serial_number().clone(),
        }
    }
}
