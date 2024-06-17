use serde::{Deserialize, Serialize};

pub trait Json<'a> {
    fn from_string(s: &'a str) -> Self;
    fn to_string(&self) -> String;
    fn to_string_pretty(&self) -> String;
}

impl<'a, T: Deserialize<'a> + Serialize> Json<'a> for T {
    fn from_string(s: &'a str) -> Self {
        serde_json::from_str(s).unwrap()
    }

    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
