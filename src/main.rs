mod command;

fn main() {
    let config = command::init();

    println!("{}", config)
}
