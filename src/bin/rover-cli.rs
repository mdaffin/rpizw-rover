extern crate rpizw_rover;
#[macro_use]
extern crate clap;

use rpizw_rover::error::*;

const PWM_CHIP: u32 = 0;
const LEFT_PWM: u32 = 0;
const RIGHT_PWM: u32 = 1;

fn run() -> Result<()> {
    use clap::App;
    use rpizw_rover::Rover;

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    let rover = Rover::new(PWM_CHIP, LEFT_PWM, RIGHT_PWM)?;
    rover.export()?;

    if let Some(_) = matches.subcommand_matches("disable") {
        rover.enable(false)
    } else if let Some(_) = matches.subcommand_matches("enable") {
        rover.enable(true)
    } else if let Some(_) = matches.subcommand_matches("stop") {
        rover.stop()
    } else if let Some(matches) = matches.subcommand_matches("speed") {
        let left = matches.value_of("LEFT").unwrap();
        let right = matches.value_of("RIGHT").unwrap_or(left);
        let left: i8 = left.parse::<i8>().chain_err(|| "failed to parse left speed")?;
        let right: i8 = right.parse::<i8>().chain_err(|| "failed to parse right speed")?;

        rover.set_speed(left, right)?;
        if !matches.is_present("dont-enable") {
            rover.enable(true)?;
        }
        Ok(())

    } else if let Some(_) = matches.subcommand_matches("unexport") {
        rover.enable(false)?;
        rover.unexport()
    } else {
        println!("{}", matches.usage());
        Ok(())
    }
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}
