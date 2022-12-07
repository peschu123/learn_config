use learn_config::config::Config;

fn main() {
    let config = Config::build();
    println!("{:?}", config);
}
