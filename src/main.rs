extern crate clap;
extern crate pzem004t;

mod hal_impl;
use hal_impl::*;

use clap::Arg;
use clap::SubCommand;

macro_rules! die {
    ($fmt:expr) => ({ eprintln!($fmt); std::process::exit(-1) });
    ($fmt:expr, $($arg:tt)*) => ({ eprintln!($fmt, $($arg)*); std::process::exit(-1) });
}

fn args() -> clap::ArgMatches<'static> {
    clap::App::new("pzem-cli")
        .version("0.1.0")
        .about("A demo for a PZEM004T sensor driver for the embedded-hal.")
        .arg(
            Arg::with_name("PORT")
                .required(true)
                .help("Serial port for communication"),
        )
        .arg(
            Arg::with_name("timeout")
                .short("t")
                .long("timeout")
                .help("Response awaiting timeout in milliseconds")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("addr")
                .short("a")
                .long("addr")
                .help("Slave address of the sensor in hexadecimal format")
                .takes_value(true),
        )
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("read").about("Read and display the measurement results"))
        .subcommand(
            SubCommand::with_name("addr")
                .about("Read/modify the slave address")
                .arg(
                    Arg::with_name("new value")
                        .required(false)
                        .help("New address value (if given)"),
                ),
        )
        .subcommand(
            SubCommand::with_name("threshold")
                .about("Read/modify the alarm threshold")
                .arg(
                    Arg::with_name("new value")
                        .required(false)
                        .help("New alarm threshold value"),
                ),
        )
        .subcommand(SubCommand::with_name("reset").about("Reset the internal energy counter"))
        .get_matches()
}

fn main() {
    let args = args();

    let timeout: u64 = args
        .value_of("timeout")
        .map(|v| {
            v.parse()
                .unwrap_or_else(|e| die!("Cannot parse the timeout value `{}`: {}", v, e))
        })
        .unwrap_or(500);

    let mut timer = Timer::new();
    let timeout = Some((&mut timer, std::time::Duration::from_millis(timeout)));

    let addr = args.value_of("addr").map(|v| {
        u8::from_str_radix(v, 16)
            .unwrap_or_else(|e| die!("Cannot parse the slave address `{}`: {}", v, e))
    });

    let uart = args.value_of("PORT").unwrap(); // Calling .unwrap() is safe here because "PORT" is required
    let uart = Uart {
        port: serialport::open(uart).unwrap_or_else(|e| die!("Cannot open `{}`: {}.", uart, e)),
    };

    let mut pzem =
        pzem004t::Pzem::new(uart, addr).unwrap_or_else(|e| die!("PZEM004T error: {}", e));

    match args.subcommand() {
        ("read", _) => {
            let mut m = pzem004t::Measurement::default();
            match pzem.read(&mut m, timeout) {
                Err(e) => die!("Could not read PZEM004T: {}", e),
                Ok(()) => {
                    println!("Voltage: {:.1} V", m.voltage);
                    println!("Current: {:.3} A", m.current);
                    println!("Power: {:.1} W", m.power);
                    println!("Energy: {:.3} kWh", m.energy);
                    println!("Frequency: {:.1} Hz", m.frequency);
                    println!("Power factor: {:.2}", m.pf);
                    println!("Alarm: {}", if m.alarm { "on" } else { "off" });
                }
            }
        }
        ("reset", _) => {
            pzem.reset_energy(timeout)
                .unwrap_or_else(|e| die!("Could not reset the energy counter: {}", e));
        }
        ("addr", arg) => {
            if let Some(v) = arg.unwrap().value_of("new value") {
                let v = u8::from_str_radix(v, 16)
                    .unwrap_or_else(|e| die!("Cannot parse the address `{}`: {}", v, e));
                pzem.set_addr(v, timeout)
                    .unwrap_or_else(|e| die!("Could not set the address: {}", e));
            } else {
                println!(
                    "Current slave address: {:x}",
                    pzem.get_addr(timeout)
                        .unwrap_or_else(|e| die!("Could not read PZEM004T: {}", e))
                );
            }
        }
        ("threshold", arg) => {
            if let Some(v) = arg.unwrap().value_of("new value") {
                let v: u16 = v
                    .parse()
                    .unwrap_or_else(|e| die!("Cannot parse the threshold `{}`: {}", v, e));
                pzem.set_threshold(v, timeout)
                    .unwrap_or_else(|e| die!("Could not set the threshold: {}", e));
            } else {
                println!(
                    "Current alarm threshold: {}",
                    pzem.get_threshold(timeout)
                        .unwrap_or_else(|e| die!("Could not read PZEM004T: {}", e))
                );
            }
        }
        _ => unreachable!(),
    }
}
