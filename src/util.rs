use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Json: Sized {
    fn from_string(s: &str) -> Self;
    fn to_string(&self) -> String;
    fn to_string_pretty(&self) -> String;
}

impl<T: DeserializeOwned + Serialize> Json for T {
    #[inline]
    fn from_string(s: &str) -> Self {
        serde_json::from_str(s).unwrap()
    }

    #[inline]
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    #[inline]
    fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
