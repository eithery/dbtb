use std::str::FromStr;
use serde::Deserialize;
use crate::std::string::Normalize;


#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ConnectionType {
    #[serde(rename = ":default")]
    Default,

    #[serde(rename = ":auto")]
    Auto,

    #[serde(rename = ":sspi")]
    Sspi,

    #[serde(rename = ":windows")]
    Windows,

    #[serde(rename = ":username")]
    Username
}


impl FromStr for ConnectionType {
    type Err = ();

    fn from_str(str_value: &str) -> Result<Self, Self::Err> {
        match str_value.normalize().as_str() {
            ":default" => Ok(ConnectionType::Default),
            ":auto" => Ok(ConnectionType::Auto),
            ":sspi" => Ok(ConnectionType::Sspi),
            ":windows" => Ok(ConnectionType::Windows),
            ":username" => Ok(ConnectionType::Username),
            _ => Ok(ConnectionType::Default),
        }
    }
}
