use serde::{Deserialize, Serialize};

pub mod general;

pub type KVPair = (String, String);

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum EntryValue {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    F32(f32),
    String(String)
}

impl EntryValue {
    pub fn convert(value: impl AsRef<str>) -> Self {
        if let Ok(v) = value.as_ref().parse::<i8>() {
            return Self::I8(v);
        } else if let Ok(v) = value.as_ref().parse::<i16>() {
            return Self::I16(v);
        } else if let Ok(v) = value.as_ref().parse::<i32>() {
            return Self::I32(v);
        } else if let Ok(v) = value.as_ref().parse::<i64>() {
            return Self::I64(v);
        } else if let Ok(v) = value.as_ref().parse::<i128>() {
            return Self::I128(v);
        } else if let Ok(v) = value.as_ref().parse::<f32>() {
            if v <= f32::MAX {
                return Self::F32(v)
            } else {
                return Self::String(value.as_ref().to_string());
            }
        } else {
            return Self::String(value.as_ref().to_string());
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Self::I8(v) => {
                return v.to_string();
            }
            Self::I16(v) => {
                return v.to_string();
            }
            Self::I32(v) => {
                return v.to_string();
            }
            Self::I64(v) => {
                return v.to_string();
            }
            Self::I128(v) => {
                return v.to_string();
            }
            Self::F32(v) => {
                return v.to_string();
            }
            Self::String(v) => {
                return v.to_string();
            }
        }
    }
}

#[cfg(test)]
mod entry_value_tests {
    use super::*;

    #[test]
    fn create_entry_value() {
        let sm = "12".to_string();
        let md = "412".to_string();
        let nm = "100000".to_string();
        let lg = "3000000000".to_string();
        let hg = "170131183460469231731687303".to_string();
        let f_32 = "12.72".to_string();
        let s = "I am a string".to_string();

        let sm_v = EntryValue::convert(sm);
        assert_eq!(sm_v, EntryValue::I8(12));

        let md_v = EntryValue::convert(md);
        assert_eq!(md_v, EntryValue::I16(412));

        let nm_v = EntryValue::convert(nm);
        assert_eq!(nm_v, EntryValue::I32(100_000));

        let lg_v = EntryValue::convert(lg);
        assert_eq!(lg_v, EntryValue::I64(3000000000));

        let hg_v = EntryValue::convert(hg);
        assert_eq!(hg_v, EntryValue::I128(170131183460469231731687303));

        let f_32_v = EntryValue::convert(f_32);
        assert_eq!(f_32_v, EntryValue::F32(12.72));

        let s_v = EntryValue::convert(s);
        assert_eq!(s_v, EntryValue::String("I am a string".to_string()));
    }

    #[test]
    fn convert_entry_value_back_to_string() {
        let sm_v = EntryValue::I8(12);
        let md_v = EntryValue::I16(412);
        let nm_v = EntryValue::I32(100_000);
        let lg_v = EntryValue::I64(3000000000);
        let hg_v = EntryValue::I128(170131183460469231731687303);
        let f32_v = EntryValue::F32(12.72);
        let s_v = EntryValue::String("I am a string".to_string());

        let sm = sm_v.as_string();
        let md = md_v.as_string();
        let nm = nm_v.as_string();
        let lg = lg_v.as_string();
        let hg = hg_v.as_string();
        let f_32 = f32_v.as_string();
        let s = s_v.as_string();

        assert_eq!(sm, "12".to_string());
        assert_eq!(md, "412".to_string());
        assert_eq!(nm, "100000".to_string());
        assert_eq!(lg, "3000000000".to_string());
        assert_eq!(hg, "170131183460469231731687303".to_string());
        assert_eq!(f_32, "12.72".to_string());
        assert_eq!(s, "I am a string".to_string());
    }
}
