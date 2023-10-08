use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub up_key: char,
    pub down_key: char,
    pub search_key: char,
}

impl Default for Config {
    fn default() -> Self {
        Self { up_key: 'j', down_key: 'k', search_key: 's' }
    }
}
