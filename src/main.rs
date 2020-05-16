use backlight::Brightness;
use clap::{load_yaml, App};
use std::process;

const MIN_PERC_ALLOWED: i32 = 2;

fn set_brightness(action: &str, delta: i32) {
    let control = Brightness::default();
    let current = control.get_percent().unwrap();
    let new_perc: i32;

    if action == "inc" {
        print!("inc");
        new_perc = current + delta;
    } else {
        new_perc = current - delta;
    }

    if new_perc > MIN_PERC_ALLOWED {
        control.set_percent(new_perc).map_err(|err| println!("{:?}", err)).ok();
    } else {
        process::exit(1);
    }
}

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let opts = App::from(yaml).get_matches();

    if opts.is_present("inc") {
        let delta: i32 = opts.value_of("inc").unwrap().parse::<i32>().unwrap();
        set_brightness("inc", delta)
    } else {
        let delta: i32 = opts.value_of("dec").unwrap().parse::<i32>().unwrap();
        set_brightness("dec", delta)
    }
}
