use linux_raw_input_rs::keys::Keys;
use config::{Config, File};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings{
    pub separate_workspaces : bool,
    pub hotkey: [String; 2],
    pub layouts : Vec<String>
}

impl Settings {
    pub fn load() -> Settings{
        let mut conf_buff = env::home_dir().expect("error finding home dir");
        conf_buff.push(".config");
        conf_buff.push("i3langlayout");
        conf_buff.push("config");
        let conf = conf_buff.as_path();

        let parsed = Config::new()
                             .merge(File::from(conf).required(true))
                             .expect("error finding config file in ~/.config/i3langlayout/config");

        parsed.deserialize().expect("error deserializing")
    }
    pub fn hotkeys(&self) -> [Keys; 2]{
        [Keys::from_code(self.hotkey[0].parse::<u16>().expect("error parsing key")), Keys::from_code(self.hotkey[1].parse::<u16>().expect("error parsing key"))]
    }
}