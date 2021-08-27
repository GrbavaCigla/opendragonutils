use core::list_redragon_devices;
use structopt::StructOpt;
use clap::arg_enum;

arg_enum! {
    #[derive(Debug, PartialEq)]
    pub enum LightMode {
        Breathing,
        Rainbow,
        FullLighted,
        Wave,
        GoWithoutTrace,
        Reactive,
        Flash,
        Off,
    }
}

#[derive(StructOpt, Debug)]
#[structopt(
    name = "opendracli",
    about = "CLI tool for redragon reverse-engineered driver."
)]
enum Opendracli {
    #[structopt(name = "list", about = "List all redragon devices")]
    List {},
    #[structopt(name = "set", about = "Change setting")]
    Set {
        #[structopt(long, short, help = "Order number of device from list command")]
        device: usize,

        #[structopt(long, short, help = "Light mode setting")]
        light_mode: Option<LightMode>,

        #[structopt(long, short, help = "Brightness setting")]
        brightness: Option<u8>,

        #[structopt(long, short, help = "Speed setting")]
        speed: Option<u8>,
    },
}

fn list() {
    println!("Listing all redragon devices:");
    for (i, device) in list_redragon_devices().unwrap().iter().enumerate() {
        println!("    {}. {}", i, device.product_string);
    }
}

fn set(device: usize, light_mode: Option<LightMode>, brightness: Option<u8>, speed: Option<u8>) {
    let devices = list_redragon_devices().unwrap();

    let usbdev = devices
        .iter()
        .nth(device)
        .expect(&format!("No device number {}", device));

    if let Some(lm) = light_mode {
    }

    println!("{:?}", usbdev.systen_path);
}

fn main() {
    let opt = Opendracli::from_args();

    match opt {
        Opendracli::List {} => list(),
        Opendracli::Set {
            device,
            light_mode,
            brightness,
            speed,
        } => set(device, light_mode, brightness, speed),
    };
}
