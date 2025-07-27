// ? is this just a wrapper?
// * yes

use config_neko::ConfigNeko;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ConfigStruct {
    pub voice_model: String,
}

pub fn init() -> ConfigNeko<ConfigStruct> {
    let config = config_neko::init::<ConfigStruct>();

    config
}
