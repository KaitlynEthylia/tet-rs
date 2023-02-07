use std::path::PathBuf;
use std::fmt;
use std::fmt::{ Display, Result, Formatter };

use std::clone::Clone;
use std::marker::{ Send, Sync };

use clap::{ command, arg, value_parser, ValueEnum, ArgMatches, parser::ValueSource };
use clap::builder::{ ArgAction, FalseyValueParser };

use dirs::config_dir;

use serde::Deserialize;

mod file;

#[derive(Debug)]
pub enum Setter {
    Default,
    Environment,
    File(PathBuf),
    Command,
}

#[derive(Debug)]
pub struct Value<T> {
    value: T,
    set_by: Setter,
}

fn to_value<'a, T: Clone + Send + Sync + 'static>(args: &'a ArgMatches, value: &'a str) -> Value<&'a T> {
    Value {
        value: args.get_one::<T>(value).unwrap(),
        set_by: match args.value_source(value) {
            Some(ValueSource::CommandLine) => Setter::Command,
            Some(ValueSource::EnvVariable) => Setter::Environment,
            Some(ValueSource::DefaultValue) => Setter::Default,
            _ => panic!("Invaild Value Source")
        },
    }
}

#[derive(Debug)]
pub struct Config<'a> {
    config: Value<&'a PathBuf>,
    debug: Value<&'a u8>,
    colour: Value<&'a TetrsColour>,
    difficulty: Value<&'a TetrsDifficulty>,
    no_hold: Value<&'a bool>,
    limit: Value<&'a u16>,
    width: Value<&'a u8>,
    height: Value<&'a u8>,
}

pub fn init() {
    let args = command!()
        .arg(
            arg!(-C --config <FILE> "Path to a vaild config file.")
            .env("TETRS_CONFIG")
            .long_help("Path to a vaild TOML file to load configs from. Arguments passed in from the command line will override those in the config file.")
            .default_value(format!("{}/tetrs.toml", config_dir().expect("gay").to_string_lossy()))
            .value_parser(value_parser!(PathBuf))
        )
        .arg(
            arg!(-D --dubug "Show debugging information.")
            .id("debug")
            .env("TETRS_DEBUG")
            .long_help("Prints debugging information. Set twice to prevent the game from starting.")
            .action(ArgAction::Count)
        )
        .arg(
            arg!(-c --colour <COLOUR> "Set where to display colours")
            .env("TETRS_COLOUR")
            .value_parser(value_parser!(TetrsColour))
            .default_value("line")
            .action(ArgAction::Set)
        )
        .arg(
            arg!(-d --difficulty <DIFFICULTY> "Set the difficulty of the game")
            .env("TETRS_DIFFICULTY")
            .value_parser(value_parser!(TetrsDifficulty))
            .default_value("normal")
            .action(ArgAction::Set)
        )
        .arg(
            arg!(-n --"no-hold" "Disables the abilitly to hold tetrominoes")
            .env("TETRS_NO_HOLD")
            .value_parser(FalseyValueParser::new())
        )
        .arg(
            arg!(-l --"line-limit" <LIMIT> "Stop the game after cleary N")
            .env("TETRS_LINE_LIMIT")
            .value_parser(value_parser!(u16))
            .default_value("0")
            .action(ArgAction::Set)
        )
        .arg(
            arg!(-W --width <WIDTH> "How wide the tetris board should be in columns")
            .env("TETRS_WIDTH")
            .value_parser(value_parser!(u8))
            .default_value("10")
            .action(ArgAction::Set)
        )
        .arg(
            arg!(-H --height <HEIGHT> "How tall the tetris board should be in rows")
            .env("TETRS_HEIGHT")
            .value_parser(value_parser!(u8))
            .default_value("24")
            .action(ArgAction::Set)
        ).get_matches(); 

        
    let config = Config {
        config: to_value::<PathBuf>(&args, "config"),
        debug: to_value::<u8>(&args, "debug"),
        colour: to_value::<TetrsColour>(&args, "colour"),
        difficulty: to_value::<TetrsDifficulty>(&args, "difficulty"),
        no_hold: to_value::<bool>(&args, "no-hold"),
        limit: to_value::<u16>(&args, "line-limit"),
        width: to_value::<u8>(&args, "width"),
        height: to_value::<u8>(&args, "height"),
    };

    file::load_config(config);
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum TetrsColour {
    Never,
    Falling,
    Line,
    Always,
}

#[derive(Deserialize, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum TetrsDifficulty {
    Noob,
    Easy,
    Normal,
    Hard,
    Expert,
    Incremental,
}

impl<T: fmt::Debug> Display for Value<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let setter = match &self.set_by {
            Setter::Default => String::from("Default Value"),
            Setter::Environment => String::from("Environment Variables"),
            Setter::File(path) => format!("File '{}'", path.display()),
            Setter::Command => String::from("Command Arguments"),
        };

        write!(f, "{:?}\n             - {}\n", self.value, setter)
    }
}

impl Display for Config<'_> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "
Config:     {}
Debug:      {}
Colour:     {}
Difficulty: {}
No Hold:    {}
Line Limit: {}
Width:      {}
Height:     {}
        ",self.config, self.debug, self.colour, self.difficulty, self.no_hold, self.limit, self.width, self.height)
    }
}
