use sphero_rs::sphero::*;
use sphero_rs::driving::*;
//use sphero_rs::packet::*;
//use sphero_rs::constants::*;
use std::thread;
use std::time::Duration;


// SB-C714
// F3:4F:CD:46:C7:14
pub fn main() {

  let mut sphero = Sphero::new().unwrap();

  sphero.power.wake();
  let speed: u8 = 50;
  thread::sleep(Duration::from_millis(1000));
  sphero.driving.drive_with_heading(speed,0x0000,Direction::Forward);
  thread::sleep(Duration::from_millis(1000));
  sphero.driving.drive_with_heading(speed,0x0000,Direction::Reverse);
  thread::sleep(Duration::from_millis(1000));
  sphero.driving.drive_with_heading(0x00,0x0000,Direction::Forward);
  sphero.power.enter_soft_sleep();
  sphero.disconnect();
  loop{}

}