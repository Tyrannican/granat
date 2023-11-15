use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum EntryValue {
    Integer(i64),
    Float(f64),
    String(String)
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

    pub fn as_string(&self) -> String {
        match self {
            Self::Integer(i) => return i.to_string(),
            Self::Float(f) => return f.to_string(),
            Self::String(s) => return s.to_string()
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entry {
    pub value: EntryValue,
    pub expiry: Option<u64>
}

impl Entry {
    pub fn new(value: String) ->  Self {
        return Self {
            value: EntryValue::convert(value),
            expiry: None
        };
    }

    pub fn expiry(mut self, expiry_time: u64) -> Self {
        self.expiry = Some(expiry_time);
        return self;
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
        assert_eq!(string, EntryValue::String("i am a string value".to_string()));
    }

    #[test]
    fn convert_entry_value_back_to_string() {
        let pve_number = EntryValue::Integer(1234567);
        let pve_float = EntryValue::Float(123456.78);
        let nve_number = EntryValue::Integer(-9876543);
        let nve_float = EntryValue::Float(-987654.32);
        let string = EntryValue::String("i should be a string".to_string());

        assert_eq!(pve_number.as_string(), "1234567".to_string());
        assert_eq!(pve_float.as_string(), "123456.78".to_string());
        assert_eq!(nve_number.as_string(), "-9876543".to_string());
        assert_eq!(nve_float.as_string(), "-987654.32".to_string());
        assert_eq!(string.as_string(), "i should be a string".to_string());
    }

    #[test]
    fn create_a_basic_entry() {
        let mut ev = Entry::new("1234567".to_string());
        assert_eq!(ev.value, EntryValue::Integer(1234567));
        assert!(ev.expiry.is_none());

        ev = Entry::new("123456.7".to_string());
        assert_eq!(ev.value, EntryValue::Float(123456.7));
        assert!(ev.expiry.is_none());

        ev = Entry::new("i am a string".to_string());
        assert_eq!(ev.value, EntryValue::String("i am a string".to_string()));
        assert!(ev.expiry.is_none());
    }

    #[test]
    fn create_entry_with_expiry() {
        // TODO: Fill me in
    }
}
