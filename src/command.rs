use clap::Parser;
use serde::Deserialize;
use std::fmt;

enum Setter {
    DEFAULTS,
    ENVIRONMENT,
    ARGUMENTS,
}

impl fmt::Display for Setter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match self {
            Setter::DEFAULTS => "Default Value",
            Setter::ENVIRONMENT => "Environment Variable",
            Setter::ARGUMENTS => "Command Line Arguments",
        };
        write!(f, "{}", output)
    }
}

#[derive(Deserialize)]
struct EnvConfig {
    debug: Option<bool>,
    test: Option<i32>,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct ArgConfig {
    #[arg(short, long)]
    debug: Option<bool>,

    #[arg(short, long)]
    test: Option<i32>,
}

struct Value<T> {
    value: T,
    set_by: Setter,
}

impl<T> fmt::Display for Value<T> where T: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - ({})", self.value, self.set_by)
    }
}

fn wrap<T>(
    primary: Option<T>,
    secondary: Option<T>,
    default: T,
) -> Value<T> {
    match primary {
        Some(r) => Value {
            value: r,
            set_by: Setter::ARGUMENTS,
        },
        None => match secondary {
            Some(r) => Value {
                value: r,
                set_by: Setter::ENVIRONMENT,
            },
            None => Value {
                value: default,
                set_by: Setter::DEFAULTS,
            }
        }
    }
}

pub struct Config {
    debug: Value<bool>,
    test: Value<i32>,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "
                Debug: {}
                Test:  {}
               ",
               self.debug, self.test)
    }
}

pub fn init() -> Config {
    let env_conf = envy::prefixed("TETRS_")
        .from_env::<EnvConfig>().expect("what?");

    let arg_conf = ArgConfig::parse();

    Config {
        debug: wrap(arg_conf.debug, env_conf.debug, false),
        test: wrap(arg_conf.test, env_conf.test, 1),
    }
}
