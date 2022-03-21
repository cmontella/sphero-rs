use std::thread;
use std::time::Duration;
use rand::{Rng, thread_rng};
#[cfg(target_os = "linux")]
use btleplug::bluez::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "windows")]
use btleplug::winrtble::{adapter::Adapter, manager::Manager};
#[cfg(target_os = "macos")]
use btleplug::corebluetooth::{adapter::Adapter, manager::Manager};
use btleplug::api::{bleuuid::uuid_from_u16, Characteristic, Central, CentralEvent, Peripheral, WriteType};
use uuid::Uuid;
use std::io;
use std::io::prelude::*;
use std::sync::Mutex;
use std::sync::mpsc::{channel, Sender, Receiver};

use crate::constants::*;
use crate::*;

/*class Sphero:
    """
    High-level API for communicate with sphero toy
    """

    def __init__(
            self,
            mac_address: str,
            toy_type: Toy = Toy.unknown,
            ble_adapter_cls: ClassVar[AbstractBleAdapter] = BleAdapter
    ):
        self.mac_address = mac_address
        self.type = toy_type
        self._ble_adapter_cls = ble_adapter_cls
        self._ble_adapter = None

    @property
    def ble_adapter(self):
        if self._ble_adapter is None:
            raise PySpheroException("Use Sphero as context manager")
        return self._ble_adapter

    @ble_adapter.setter
    def ble_adapter(self, value):
        self._ble_adapter = value

    def __enter__(self):
        self._ble_adapter = self._ble_adapter_cls(self.mac_address)
        # if self.type is Toy.unknown:
        #     self.type = TOY_BY_PREFIX.get(self.name[:3], Toy.unknown)
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.ble_adapter.close()

    @cached_property
    def system_info(self) -> SystemInfo:
        return SystemInfo(ble_adapter=self.ble_adapter)

    @cached_property
    def power(self) -> Power:
        return Power(ble_adapter=self.ble_adapter)

    @cached_property
    def driving(self) -> Driving:
        return Driving(ble_adapter=self.ble_adapter)

    @cached_property
    def api_processor(self) -> ApiProcessor:
        return ApiProcessor(ble_adapter=self.ble_adapter)

    @cached_property
    def user_io(self) -> UserIO:
        return UserIO(ble_adapter=self.ble_adapter)

    @cached_property
    def sensor(self) -> Sensor:
        return Sensor(ble_adapter=self.ble_adapter)

    @cached_property
    def animatronics(self) -> Animatronics:
        return Animatronics(ble_adapter=self.ble_adapter)*/

use std::rc::Rc;
use std::cell::RefCell;

pub enum SpheroMessage {
  Send((DeviceId,u8)),
  Disconnect,
}

pub struct Sphero {
  pub mac_address: String,
  pub kind: ToyKind,
  pub sender: Sender<SpheroMessage>,
  pub characteristic: Characteristic,
  pub power: power::Power,
}

impl Sphero {

  pub fn new() -> Result<Sphero,i32> {

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
        Some(sphero_bt) => {
          println!("{:?}", sphero_bt);
          print!("Connecting to Sphero...");
          io::stdout().flush().unwrap();
          match sphero_bt.connect() {
            Ok(()) => {
              println!("Done.");
              print!("Discovering characteristics...");
              sphero_bt.discover_characteristics().unwrap();
              println!("Done.");
              let characteristics = sphero_bt.characteristics();
              let characteristic = characteristics.iter().find(|c| c.uuid == *SpheroCharacteristicApiV2).unwrap().clone();
              let c = characteristic.clone();
              let (sender, bt_receiver) = channel();
              //let (bt_sender, receiver) = channel();
              thread::spawn(move || {
                'receive: loop {
                  if let Ok(msg) = bt_receiver.recv() {
                    match msg {
                      SpheroMessage::Send((device_id,cmd)) => {
                        let mut packet = Packet::new(device_id, cmd, None);
                        println!("Sending {:?}", packet.build());
                        sphero_bt.write(
                          &characteristic,
                          &packet.build(),
                          WriteType::WithoutResponse
                        ).unwrap();
                      }
                      SpheroMessage::Disconnect => {
                        println!("Disconnect");
                        sphero_bt.disconnect();
                        break 'receive;
                      }
                    }
                  }
                }
              });
              let sphero = Sphero {
                mac_address: "".to_string(),
                kind: ToyKind::Unknown,
                characteristic: c,
                power: power::Power{sender: sender.clone()},
                sender: sender,
              };
              return Ok(sphero);
            }
            Err(x) => println!("Error connecting to Sphero {:?}", x),
          }      
        }
        None => (),
      } 
    }
  }

  pub fn disconnect(&mut self) {
    self.sender.send(SpheroMessage::Disconnect);
  }      

}