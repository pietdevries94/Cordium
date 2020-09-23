mod config;

fn main() {
    let config = config::read_config();
    if config.sites.len() > 0 {
        println!("{}", config.sites[0].name);
        println!("{}", config.sites[0].url);
    }
}
