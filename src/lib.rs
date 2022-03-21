use std::thread;
use std::time::Duration;
use rand::{Rng, thread_rng};
#[cfg(target_os = "linux")]
use btleplug::bluez::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "windows")]
use btleplug::winrtble::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "macos")]
use btleplug::corebluetooth::{adapter::Adapter, manager::Manager};
use btleplug::api::{bleuuid::uuid_from_u16, Central, CentralEvent, Peripheral, WriteType};
use uuid::Uuid;
use std::io;
use std::io::prelude::*;
#[macro_use]
extern crate lazy_static;


mod constants;
mod core;
mod packet;
mod power;

pub use self::constants::*;
pub use self::core::*;
pub use self::packet::*;
pub use self::power::*;

pub fn connect() {
  let manager = Manager::new().unwrap();

  // get the first bluetooth adapter
  let adapters = manager.adapters().unwrap(); 
  let central = adapters.into_iter().nth(0).unwrap();
  //let event_receiver = central.event_receiver().unwrap();
  central.start_scan().unwrap();
  'connect: loop {
    // find the device we're interested in
    match central.peripherals().into_iter()
        .find(|p| p.properties().local_name.iter()
            .any(|name| name.contains("SB-"))) {
     Some(sphero) => {
      println!("{:?}", sphero);
      print!("Connecting to Sphero...");
      io::stdout().flush().unwrap();
      match sphero.connect() {
        Ok(()) => {
          println!("Done.");
          print!("Discovering characteristics...");
          sphero.discover_characteristics().unwrap();
          println!("Done.");
          let charstics = sphero.characteristics();
          let cmd_charstic = charstics.iter().find(|c| c.uuid == *SpheroCharacteristicApiV2).unwrap();
          let mut packet = Packet::new(DeviceId::Power, PowerCommand::Wake as u8, None);
          sphero.write(
            &cmd_charstic,
            &packet.build(),
            WriteType::WithoutResponse
          ).unwrap();
          thread::sleep(Duration::from_secs(5));
          let mut packet = Packet::new(DeviceId::Power, PowerCommand::EnterSoftSleep as u8, None);
          sphero.write(
            &cmd_charstic,
            &packet.build(),
            WriteType::WithoutResponse
          ).unwrap();
          sphero.disconnect();
          break 'connect;
        }
        Err(x) => println!("Error connecting to Sphero {:?}", x),
      }
     }
     None => (),
    }

  }
}

/*
    // find the characteristic we want
    let chars = light.characteristics();
    let cmd_char = chars.iter().find(|c| c.uuid == LIGHT_CHARACTERISTIC_UUID).unwrap();

    // dance party
    let mut rng = thread_rng();
    for _ in 0..20 {
        let color_cmd = vec![0x56, rng.gen(), rng.gen(), rng.gen(), 0x00, 0xF0, 0xAA];
        light.write(&cmd_char, &color_cmd, WriteType::WithoutResponse).unwrap();
        thread::sleep(Duration::from_millis(200));
    }*/