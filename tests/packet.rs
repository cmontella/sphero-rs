extern crate sphero_rs;
use sphero_rs::packet::*;
use sphero_rs::constants::*;

#[test]
pub fn test_packet_init() {
  let packet = Packet::new(DeviceId::Test, 0x42, None);
  assert_eq!(packet.device_id,DeviceId::Test);
  assert_eq!(packet.command_id,0x42);
  assert_eq!(packet.flags,0x0a);
  assert_eq!(packet.target_id,None);
  assert_eq!(packet.source_id,None);
  assert!(packet.data.is_empty());
}

#[test]
pub fn test_packet_source_id() {
  let mut packet = Packet::new(DeviceId::Test, 0x42, None);
  packet.set_source_id(0x01);
  assert_eq!(packet.device_id,DeviceId::Test);
  assert_eq!(packet.command_id,0x42);
  assert_eq!(packet.flags,0x2a);
  assert_eq!(packet.target_id,None);
  assert_eq!(packet.source_id,Some(0x01));
  assert!(packet.data.is_empty());
}

#[test]
pub fn test_packet_target_id() {
  let mut packet = Packet::new(DeviceId::Test, 0x42, None);
  packet.set_target_id(0x01);
  assert_eq!(packet.device_id,DeviceId::Test);
  assert_eq!(packet.command_id,0x42);
  assert_eq!(packet.flags,0x1a);
  assert_eq!(packet.target_id,Some(0x01));
  assert_eq!(packet.source_id,None);
  assert!(packet.data.is_empty());
}

#[test]
pub fn test_packet_flags() {
  let mut packet = Packet::new(DeviceId::Test, 0x42, Some(0x01));
  assert_eq!(packet.device_id,DeviceId::Test);
  assert_eq!(packet.command_id,0x42);
  assert_eq!(packet.flags,0x01);
  assert_eq!(packet.target_id,None);
  assert_eq!(packet.source_id,None);
  assert!(packet.data.is_empty());
}

#[test]
pub fn test_packet_flags_with_source_id() {
  let mut packet = Packet::new(DeviceId::Test, 0x42, Some(0x01));
  packet.set_source_id(0x02);
  assert_eq!(packet.device_id,DeviceId::Test);
  assert_eq!(packet.command_id,0x42);
  assert_eq!(packet.flags,0x01);
  assert_eq!(packet.target_id,None);
  assert_eq!(packet.source_id,Some(0x02));
  assert!(packet.data.is_empty());
}

#[test]
pub fn test_packet_flags_with_target_id() {
  let mut packet = Packet::new(DeviceId::Test, 0x42, Some(0x01));
  packet.set_target_id(0x02);
  assert_eq!(packet.device_id,DeviceId::Test);
  assert_eq!(packet.command_id,0x42);
  assert_eq!(packet.flags,0x01);
  assert_eq!(packet.source_id,None);
  assert_eq!(packet.target_id,Some(0x02));
  assert!(packet.data.is_empty());
}

#[test]
pub fn test_packet_id() {
  let packet = Packet::new(DeviceId::Test, 0x42, None);
  assert_eq!(packet.id,(DeviceId::Test, 0x42));
}

#[test]
pub fn test_packet_api_error() {
  let mut packet = Packet::new(DeviceId::Test, 0x42, Some(0x01));
  packet.data = vec![0x00,0x16];
  assert_eq!(packet.api_error(),Api2Error::Success);
  assert_eq!(packet.data.len(),1);
  assert_eq!(packet.api_error(),Api2Error::Success);
  assert_eq!(packet.data.len(),1);
}

#[test]
pub fn test_packet_api_error_without_flags() {
  let mut packet = Packet::new(DeviceId::Test, 0x42 ,None);
  packet.data = vec![0x00,0x16];
  assert_eq!(packet.api_error(),Api2Error::Success);
  assert_eq!(packet.data.len(),2);
}

#[test]
pub fn test_packet_api_error_unknown() {
  let mut packet = Packet::new(DeviceId::Test, 0x42 ,Some(0x01));
  packet.data = vec![0x15,0x16];
  assert_eq!(packet.api_error(),Api2Error::Unknown);
  assert_eq!(packet.data.len(),1);
}

#[test]
pub fn test_packet_payload() {
  let mut packet = Packet::new(DeviceId::Test, 0x42, None);
  packet.data = vec![0x15,0x16];
  packet.sequence = 1;
  let expected = vec![0x0a, 0x23, 0x42, 0x01, 0x15, 0x16];
  let mut payload = packet.packet_payload();
  assert_eq!(payload,expected);
}

#[test]
pub fn test_packet_payload_with_source_id() {
  let mut packet = Packet::new(DeviceId::Test, 0x42, None);
  packet.set_source_id(0x02);
  packet.data = vec![0x15,0x16];
  packet.sequence = 1;

  let expected = vec![0x2a, 0x02, 0x23, 0x42, 0x01, 0x15, 0x16];
  let mut payload = packet.packet_payload();
  assert_eq!(payload,expected);
}

#[test]
pub fn test_packet_payload_with_target_id() {
  let mut packet = Packet::new(DeviceId::Test, 0x42, None);
  packet.set_target_id(0x03);
  packet.data = vec![0x15,0x16];
  packet.sequence = 1;

  let expected = vec![0x1a, 0x03, 0x23, 0x42, 0x01, 0x15, 0x16];
  let mut payload = packet.packet_payload();
  assert_eq!(payload,expected);
}


#[test]
pub fn test_packet_checksum() {
  let mut packet = Packet::new(DeviceId::Test, 0x42, None);
  packet.set_target_id(0x03);
  packet.data = vec![0x15,0x16];
  packet.sequence = 1;
  assert_eq!(packet.checksum(),0x51);
}

#[test]
pub fn test_packet_build() {
  let mut packet = Packet::new(DeviceId::Test, 0x42, None);
  packet.data = vec![0x15,0x16];
  packet.sequence = 1;
  let raw_packet = vec![0x8d, 0x0a, 0x23, 0x42, 0x01, 0x15, 0x16, 0x64, 0xd8];
  assert_eq!(packet.build(),raw_packet);
}

/*
#[test]
pub fn test_packet_build_with_escape() {
  let mut packet = Packet::new(DeviceId::Test, 0x42, None);
  packet.data = vec![0xab, 0x8d];
  packet.sequence = 1;
  let raw_packet = vec![0x8d, 0x0a, 0x23, 0x42, 0x01, 0xab, 0x23, 0xab, 0x05, 0x57, 0xd8];
  assert_eq!(packet.build(),raw_packet);
}*/

/*
def test_packet_from_response():
    raw_packet = [0x8d, 0x0a, 0x23, 0x42, 0x01, 0x15, 0x16, 0x64, 0xd8]
    packet = Packet.from_response(raw_packet)
    assert packet.flags == 0x0a
    assert packet.source_id is None
    assert packet.target_id is None
    assert packet.device_id == 0x23
    assert packet.command_id == 0x42
    assert packet.sequence == 0x01
    assert packet.data == [0x15, 0x16]
    assert packet.checksum == 0x64


def test_packet_from_response_with_escape_symbols():
    raw_packet = [0x8d, 0x0a, 0x23, 0x42, 0x01, 0xab, 0x23, 0xab, 0x05, 0x57, 0xd8]
    packet = Packet.from_response(raw_packet)
    assert packet.flags == 0x0a
    assert packet.source_id is None
    assert packet.target_id is None
    assert packet.device_id == 0x23
    assert packet.command_id == 0x42
    assert packet.sequence == 0x01
    assert packet.data == [0xab, 0x8d]
    assert packet.checksum == 0x57


def test_packet_from_response_with_target_id():
    raw_packet = [0x8d, 0x1a, 0xfe, 0x23, 0x42, 0x01, 0xab, 0x23, 0xab, 0x05, 0x49, 0xd8]
    packet = Packet.from_response(raw_packet)
    assert packet.flags == 0x1a
    assert packet.source_id is None
    assert packet.target_id == 0xfe
    assert packet.device_id == 0x23
    assert packet.command_id == 0x42
    assert packet.sequence == 0x01
    assert packet.data == [0xab, 0x8d]
    assert packet.checksum == 0x49


def test_packet_from_response_with_source_id():
    raw_packet = [0x8d, 0x2a, 0xfe, 0x23, 0x42, 0x01, 0xab, 0x23, 0xab, 0x05, 0x39, 0xd8]
    packet = Packet.from_response(raw_packet)
    assert packet.flags == 0x2a
    assert packet.source_id == 0xfe
    assert packet.target_id is None
    assert packet.device_id == 0x23
    assert packet.command_id == 0x42
    assert packet.sequence == 0x01
    assert packet.data == [0xab, 0x8d]
    assert packet.checksum == 0x39


def test_packet_from_response_bad_data():
    # bad start
    raw_packet = [0xad, 0x0a, 0x23, 0x42, 0x01, 0x15, 0x16, 0x64, 0xd8]
    with pytest.raises(PySpheroRuntimeError):
        Packet.from_response(raw_packet)

    # bad end
    raw_packet = [0x8d, 0x0a, 0x23, 0x42, 0x01, 0x15, 0x16, 0x64, 0xdd]
    with pytest.raises(PySpheroRuntimeError):
        Packet.from_response(raw_packet)

    # bad checksum
    raw_packet = [0x8d, 0x0a, 0x23, 0x42, 0x01, 0x15, 0x16, 0xff, 0xd8]
    with pytest.raises(PySpheroRuntimeError):
        Packet.from_response(raw_packet)
        */