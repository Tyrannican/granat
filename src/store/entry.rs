use std::fmt;

use anyhow::{anyhow, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum ExpiryState {
    Expired,
    Active(i64),
    NoExpiry,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub value: String,
    pub expiry: ExpiryState,
}

impl Entry {
    pub fn new(value: impl AsRef<str>) -> Self {
        return Self {
            value: value.as_ref().to_string(),
            expiry: ExpiryState::NoExpiry,
        };
    }

    pub fn from_obj<T: Serialize>(obj: &T) -> Result<Self> {
        match serde_json::to_string::<T>(obj) {
            Ok(obj_str) => {
                return Ok(Self {
                    value: obj_str,
                    expiry: ExpiryState::NoExpiry,
                })
            }
            Err(e) => return Err(anyhow!("unable to serialize object: {e}")),
        }
    }

    pub fn expires_in(mut self, expiry_time: i64) -> Self {
        let now = Utc::now().timestamp();
        self.expiry = ExpiryState::Active(now + expiry_time);
        return self;
    }

    pub fn to_obj<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        match serde_json::from_str::<T>(&self.value) {
            Ok(obj) => return Ok(obj),
            Err(e) => return Err(anyhow!("unable to deserialize object: {e}")),
        };
    }

    pub fn ttl(&mut self) -> ExpiryState {
        match self.expiry {
            ExpiryState::Active(exp) => {
                let now = Utc::now().timestamp();
                if exp < now {
                    self.expiry = ExpiryState::Expired;
                }
            }
            _ => {}
        }

        return self.expiry.clone();
    }
}

impl Clone for Entry {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            expiry: self.expiry.clone(),
        }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value.to_string())
    }
}

#[cfg(test)]
mod entry_tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct TestDataStruct {
        value: i32,
        message: String,
    }

    #[test]
    fn create_a_basic_entry() {
        let mut ev = Entry::new("1234567");
        assert_eq!(ev.value, "1234567".to_string());
        assert!(ev.expiry == ExpiryState::NoExpiry);

        ev = Entry::new("123456.7");
        assert_eq!(ev.value, "123456.7".to_string());
        assert!(ev.expiry == ExpiryState::NoExpiry);

        ev = Entry::new("i am a string");
        assert_eq!(ev.value, "i am a string".to_string());
        assert!(ev.expiry == ExpiryState::NoExpiry);
    }

    #[test]
    #[ignore]
    /// Takes ages to run
    fn create_entry_with_expiry() {
        let mut entry = Entry::new("five hundred").expires_in(5);
        assert_eq!(entry.ttl(), ExpiryState::Active(Utc::now().timestamp() + 5));

        while entry.ttl() != ExpiryState::Expired {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        assert_eq!(entry.ttl(), ExpiryState::Expired);
    }

    #[test]
    fn create_entry_value_with_object() {
        let td = TestDataStruct {
            value: 5,
            message: "some message".to_string(),
        };

        let entry = Entry::from_obj(&td);
        assert!(entry.is_ok());
        let inner = entry.unwrap().value;

        let expected = "{\"value\":5,\"message\":\"some message\"}".to_string();
        assert_eq!(inner, expected);
    }

    #[test]
    fn convert_entry_to_object() {
        let td = TestDataStruct {
            value: 5,
            message: "some message".to_string(),
        };

        let entry = Entry::from_obj(&td);
        assert!(entry.is_ok());

        let inner = entry.unwrap();
        let output = inner.to_obj::<TestDataStruct>();

        assert!(output.is_ok());

        let inner_from_output = output.unwrap();

        assert_eq!(inner_from_output.value, 5);
        assert_eq!(inner_from_output.message, "some message".to_string());
    }

    #[test]
    fn ensure_none_when_converting_none_object() {
        let entry = Entry::new("5");
        let result = entry.to_obj::<TestDataStruct>();

        assert!(result.is_err());
    }
}
