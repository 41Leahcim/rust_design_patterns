use std::{path::PathBuf, time::Duration};

#[derive(Debug, Default, PartialEq)]
struct MyConfiguration {
    // Defaults to None
    output: Option<PathBuf>,

    // Defaults to an empty vector
    search_path: Vec<PathBuf>,

    // Defaults to 0 time
    timout: Duration,

    // Defaults to false
    check: bool,
}

fn main() {
    let mut conf = MyConfiguration::default();
    conf.check = true;
    println!("Conf = {conf:#?}");

    let conf1 = MyConfiguration {
        check: true,
        // Use the default values from MyConfiguration to initialize the rest
        ..Default::default()
    };
    assert_eq!(conf, conf1);
}
