use serde::Deserialize;
use std::{ fs, process::exit, path::PathBuf };
use toml;
use crate::config::{ Config, Value, Setter, TetrsColour, TetrsDifficulty };

#[derive(Deserialize, Debug)]
struct FileConfigWrapper {
    tetrs: FileConfig,
}

#[derive(Deserialize, Debug)]
struct FileConfig {
    debug: Option<u8>,
    colour: Option<TetrsColour>,
    difficulty: Option<TetrsDifficulty>,
    no_hold: Option<bool>,
    limit: Option<u16>,
    width: Option<u8>,
    height: Option<u8>,
}

pub fn load_config(base: Config) {
    let path = base.config.value;
    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            println!("{}", base);
            return
        },
    };
    let data: FileConfigWrapper = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };
    
    let config = Config {
        config: base.config,
        debug: compare(base.debug, &data.tetrs.debug, &path),
        colour: compare(base.colour, &data.tetrs.colour, &path),
        difficulty: compare(base.difficulty, &data.tetrs.difficulty, &path),
        no_hold: compare(base.no_hold, &data.tetrs.no_hold, &path),
        limit: compare(base.limit, &data.tetrs.limit, &path),
        width: compare(base.width, &data.tetrs.width, &path),
        height: compare(base.height, &data.tetrs.height, &path),
    };

    println!("{}", config)
}

fn compare<'a, T>(base: Value<&'a T>, conf: &'a Option<T>, path: &'a PathBuf) -> Value<&'a T> {
    match conf {
        None => base,
        Some(value) => {
            match base.set_by {
                Setter::Command => base,
                _ => Value {
                    value,
                    set_by: Setter::File(path.to_path_buf()),
                }
            }
        }
    }
}

