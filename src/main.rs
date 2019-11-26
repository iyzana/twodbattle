extern crate twodbattle;

fn main() -> Result<(), anyhow::Error> {
    let yaml = clap::load_yaml!("cli.yml");
    let matches = clap::App::from_yaml(yaml).get_matches();
    twodbattle::run(&matches)
}
