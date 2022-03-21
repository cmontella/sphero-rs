use sphero_rs::sphero::*;
use sphero_rs::driving::*;
use std::thread;
use std::time::Duration;


pub fn main() {

  let mut sphero = Sphero::new().unwrap();

  sphero.power.wake();
  thread::sleep(Duration::from_secs(1));
  sphero.driving.drive_with_heading(0xFF,0x0000,Direction::Forward);
  thread::sleep(Duration::from_secs(4));
  sphero.power.enter_soft_sleep();
  sphero.disconnect();
  loop{}

}

// SB-C714
/*
F3:4F:CD:46:C7:14 properties: PeripheralProperties { 
    address: F3:4F:CD:46:C7:14, 
    address_type: Public, 
    local_name: Some("SB-C714"), 
    tx_power_level: Some(-71), 
    manufacturer_data: {}, 
    service_data: {}, 
    services: [], 
    discovery_count: 4, 
    has_scan_response: true 
}, 
characteristics: {}
*/