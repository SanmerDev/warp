use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Json: Sized {
    fn to_string(&self) -> String;
    fn to_string_pretty(&self) -> String;
}

impl<T: DeserializeOwned + Serialize> Json for T {
    #[inline]
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    #[inline]
    fn to_string_pretty(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
