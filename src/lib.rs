#![allow(warnings)]
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
use std::sync::mpsc::channel;
#[macro_use]
extern crate lazy_static;


pub mod constants;
pub mod sphero;
pub mod packet;
pub mod power;
pub mod driving;
pub mod user_io;

pub use self::constants::*;
pub use self::sphero::*;
pub use self::packet::*;
pub use self::power::*;
pub use self::driving::*;
pub use self::user_io::*;





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