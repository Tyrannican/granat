use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum ExpiryState {
    Expired,
    Active(i64),
    NoExpiry
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum EntryValue {
    Integer(i64),
    Float(f64),
    String(String),
}

impl EntryValue {
    pub fn convert(value: impl AsRef<str>) -> Self {
        if let Ok(v) = value.as_ref().parse::<i64>() {
            return Self::Integer(v);
        } else if let Ok(v) = value.as_ref().parse::<f64>() {
            return Self::Float(v);
        } else {
            return Self::String(value.as_ref().to_string());
        }
    }
}

impl std::fmt::Display for EntryValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => return write!(f, "{}", i.to_string()),
            Self::Float(fl) => return write!(f, "{}", fl.to_string()),
            Self::String(s) => return write!(f, "{}", s.to_string()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub value: EntryValue,
    pub expiry: ExpiryState,
}

impl Entry {
    pub fn new(value: impl AsRef<str>) -> Self {
        return Self {
            value: EntryValue::convert(value),
            expiry: ExpiryState::NoExpiry,
        };
    }

    pub fn expires_in(mut self, expiry_time: i64) -> Self {
        let now = Utc::now().timestamp();
        self.expiry = ExpiryState::Active(now + expiry_time);
        return self;
    }

    pub fn ttl(&mut self) -> ExpiryState {
        match self.expiry {
            ExpiryState::Active(exp) => {
                let now = Utc::now().timestamp();
                if exp < now {
                    self.expiry = ExpiryState::Expired;
                }
            },
            _ => {}
        }

        return self.expiry.clone();
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.to_string())
    }
}

#[cfg(test)]
mod entry_tests {
    use super::*;

    #[test]
    fn create_entry_value() {
        let pve_number = EntryValue::convert("1234567");
        let pve_float = EntryValue::convert("123456.23");
        let nve_number = EntryValue::convert("-123376898355");
        let nve_float = EntryValue::convert("-49875286.87");
        let string = EntryValue::convert("i am a string value");

        assert_eq!(pve_number, EntryValue::Integer(1234567));
        assert_eq!(pve_float, EntryValue::Float(123456.23));
        assert_eq!(nve_number, EntryValue::Integer(-123376898355));
        assert_eq!(nve_float, EntryValue::Float(-49875286.87));
        assert_eq!(
            string,
            EntryValue::String("i am a string value".to_string())
        );
    }

    #[test]
    fn convert_entry_value_back_to_string() {
        let pve_number = EntryValue::Integer(1234567);
        let pve_float = EntryValue::Float(123456.78);
        let nve_number = EntryValue::Integer(-9876543);
        let nve_float = EntryValue::Float(-987654.32);
        let string = EntryValue::String("i should be a string".to_string());

        assert_eq!(pve_number.to_string(), "1234567".to_string());
        assert_eq!(pve_float.to_string(), "123456.78".to_string());
        assert_eq!(nve_number.to_string(), "-9876543".to_string());
        assert_eq!(nve_float.to_string(), "-987654.32".to_string());
        assert_eq!(string.to_string(), "i should be a string".to_string());
    }

    #[test]
    fn create_a_basic_entry() {
        let mut ev = Entry::new("1234567");
        assert_eq!(ev.value, EntryValue::Integer(1234567));
        assert!(ev.expiry == ExpiryState::NoExpiry);

        ev = Entry::new("123456.7");
        assert_eq!(ev.value, EntryValue::Float(123456.7));
        assert!(ev.expiry == ExpiryState::NoExpiry);

        ev = Entry::new("i am a string");
        assert_eq!(ev.value, EntryValue::String("i am a string".to_string()));
        assert!(ev.expiry == ExpiryState::NoExpiry);
    }

    #[test]
    fn create_entry_with_expiry() {
        let mut entry = Entry::new("five hundred").expires_in(2);
        assert_eq!(entry.ttl(), ExpiryState::Active(Utc::now().timestamp() + 2));

        while entry.ttl() != ExpiryState::Expired {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }

        assert_eq!(entry.ttl(), ExpiryState::Expired);
    }
}
