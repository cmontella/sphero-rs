use sphero_rs::sphero::*;
use sphero_rs::driving::*;
use sphero_rs::user_io::*;
//use sphero_rs::packet::*;
//use sphero_rs::constants::*;
use std::thread;
use std::time::Duration;


// SB-C714
// F3:4F:CD:46:C7:14
pub fn main() {

  let mut sphero = Sphero::new("SB-C714").unwrap();

  sphero.power.wake();
  
  thread::sleep(Duration::from_millis(1000));
  sphero.user_io.set_led_matrix_picture("
🟡🟡🟡🟡🟡🟡🟡🟡
🟡🟡🟡🟡🟡🟡🟡🟡
🟡🟡⚫🟡🟡⚫⚫⚫
🟡🟡⚫🟡🟡⚫🟡🟡
🟡🟡⚫🟡🟡⚫🟡🟡
🟡🟡⚫🟡🟡⚫🟡🟡
🟡🟡⚫🟡🟡⚫🟡🟡
🟡🟡⚫⚫⚫⚫🟡🟡".to_string());

  thread::sleep(Duration::from_millis(10000));
  sphero.power.enter_soft_sleep();
  sphero.disconnect();
  loop{}

}


/*
"
🟡🟡🟡🟡🟡🟡🟡⚫
🟡🟡🟡🟡🟡🟡🟡⚫
⚫⚫⚫⚫⚫⚫⚫⚫
🟡🟡🟡🟡🟡🟡🟡⚫
🟡🟡🟡🟡🟡🟡🟡⚫
🟡🟡⚫⚫⚫⚫⚫⚫
🟡🟡🟡🟡🟡🟡🟡⚫
🟡🟡🟡🟡🟡🟡🟡⚫"
"
🟡🟡⚫🟡🟡🟡🟡⚫
🟡🟡⚫🟡🟡🟡🟡⚫
🟡🟡⚫⚫⚫🟡🟡⚫
🟡🟡⚫⚫⚫🟡🟡⚫
🟡🟡⚫⚫⚫⚫⚫⚫
🟡🟡⚫⚫⚫🟡🟡⚫
🟡🟡🟡🟡🟡🟡🟡⚫
🟡🟡🟡🟡🟡🟡🟡⚫"
"
🟡🟡⚫⚫⚫🟡🟡⚫
🟡🟡⚫⚫⚫🟡🟡⚫
🟡🟡⚫⚫⚫🟡🟡⚫
🟡🟡⚫🟡🟡🟡🟡⚫
🟡🟡⚫🟡🟡🟡🟡⚫
🟡🟡⚫⚫⚫🟡🟡⚫
🟡🟡⚫⚫⚫🟡🟡⚫
🟡🟡⚫⚫⚫🟡🟡⚫"

*/