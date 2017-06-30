extern crate png;
extern crate cairo;
extern crate systray;
extern crate linux_raw_input_rs;
extern crate config;
extern crate i3ipc;
#[macro_use]
extern crate serde_derive;

mod settings;
mod icon;

use settings::Settings;
use systray::Application;
use std::process::Command;
use std::collections::HashMap;
use linux_raw_input_rs::{get_input_devices, InputReader};
use linux_raw_input_rs::keys::Keys;
use linux_raw_input_rs::input::EventType;
use i3ipc::I3Connection;

struct State{
    pub current_selection: [usize; 10],
    pub current_screen: usize,
    pub hotkeys: HashMap<Keys, bool>
}

fn main(){
    let mut connection = I3Connection::connect().ok().expect("error connecting to i3");

    let device_path : String = (&get_input_devices()[0]).to_string();
    let mut reader = InputReader::new(device_path);

    let cfg = Settings::load();
    let mut app = Application::new().expect("couldn't create app");

    let mut state ={
        let mut keys : HashMap<Keys, bool> = HashMap::with_capacity(cfg.hotkey.len());
        for i in 0..cfg.hotkeys().len(){
            keys.insert(cfg.hotkeys()[i], false);
        }
        let screen : usize = {
            if cfg.separate_workspaces{
                connection
                .get_workspaces()
                .ok().expect("error getting workspaces")
                .workspaces
                .iter().find(|e| e.focused)
                .expect("error finding focused workspace")
                .num
            }else{
                0
            }
        } as usize;
        State{
            current_selection: [0; 10],
            current_screen: screen,
            hotkeys: keys
        }
    };

    let paths : Vec<String> = cfg.layouts.iter().map(|e| {
        if let cairo::Status::Success = icon::create_for(e){
            "/tmp/i3langlayout/".to_string() + e + ".png"
        } else {
            panic!("Error creating {}.png", e);
        }
    }).collect();
    
    switch_language(&cfg.layouts[state.current_selection[0]]);
    switch_icon(&paths[state.current_selection[0]], &mut app);

    loop{
        if cfg.separate_workspaces{
            if connection
            .get_workspaces()
            .ok().expect("error getting workspaces")
            .workspaces
            .iter().fold(false, |acc, e| {
                if e.focused && state.current_screen != e.num as usize{
                    state.current_screen = e.num as usize;
                    return true;
                }
                acc
            }){
                switch_language(&cfg.layouts[state.current_selection[state.current_screen]]);
                switch_icon(&paths[state.current_selection[state.current_screen]], &mut app);
            }
        }

        let input = reader.current_state();

        if input.is_key_event(){
            for i in 0..cfg.hotkeys().len(){
                if input.get_key() == cfg.hotkeys()[i]{
                    state.hotkeys.insert(cfg.hotkeys()[i], input.event_type() == EventType::Push);
                }
            }
        }

        if state.hotkeys.values().all(|&e| e){
            for i in 0..cfg.hotkeys().len(){
                state.hotkeys.insert(cfg.hotkeys()[i], false);
            }
            state.current_selection[state.current_screen] = (state.current_selection[state.current_screen] + 1) % cfg.layouts.len();
            switch_language(&cfg.layouts[state.current_selection[state.current_screen]]);
            switch_icon(&paths[state.current_selection[state.current_screen]], &mut app);
        }
    }
}

fn switch_language(lang : &String){
    Command::new("sh").arg("-c").arg("setxkbmap ".to_string() + lang).spawn().expect("Error executing command");
}
fn switch_icon(file: &String, app: &mut Application){
    app.set_icon_from_file(file).expect("Error setting up icon");
}