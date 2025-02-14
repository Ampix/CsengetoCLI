use crate::config::load_config;
use crate::MAIN_CONFIG;
use notify::{recommended_watcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;

pub fn watch_config_file() {
    let config_path = "config.json";
    let (tx, rx) = channel();
    let mut watcher = recommended_watcher(tx).unwrap();

    watcher
        .watch(Path::new(config_path), RecursiveMode::NonRecursive)
        .unwrap();

    thread::spawn(move || loop {
        match rx.recv() {
            Ok(_) => {
                let new_config = load_config();
                let mut config = MAIN_CONFIG.write().unwrap();
                *config = new_config;
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    });
}
