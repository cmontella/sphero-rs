use sphero_rs::*;

// adapter retreival works differently depending on your platform right now.
// API needs to be aligned.

pub fn main() {

    connect();




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