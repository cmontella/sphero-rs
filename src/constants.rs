use uuid::Uuid;

#[derive(Debug)]
pub enum GenericCharacteristic {
  DeviceName = 0x2a00,
  ClientCharacteristicConfiguration = 0x2902,
  PeripheralPreferredConnectionParameters = 0x2a04,
}

lazy_static! {
  pub static ref SpheroCharacteristicForceBand: Uuid = Uuid::parse_str("00020005-574f-4f20-5370-6865726f2121").unwrap();
  pub static ref SpheroCharacteristicApiV2: Uuid = Uuid::parse_str("00010002-574f-4f20-5370-6865726f2121").unwrap();
}

pub enum  Api2Error {
  success = 0x00,
  bad_device_id = 0x01,
  bad_command_id = 0x02,
  not_yet_implemented = 0x03,
  command_is_restricted = 0x04,
  bad_data_length = 0x05,
  command_failed = 0x06,
  bad_parameter_value = 0x07,
  busy = 0x08,
  bad_target_id = 0x09,
  target_unavailable = 0x0a,
  unknown = 0xff,
}

#[derive(Debug, Copy, Clone)]
pub enum DeviceId {
  ApiProcessor = 0x10,
  SystemInfo = 0x11,
  SystemModes = 0x12,
  Power = 0x13,
  Driving = 0x16,
  Animatronics = 0x17,
  Sensors = 0x18,
  PeerConnection = 0x19,
  UserIo = 0x1a,
  StorageCommand = 0x1b,
  SecondaryMcuFirmwareUpdateCommand = 0x1d,
  WifiCommand = 0x1e,
  FactoryTest = 0x1f,
  MacroSystem = 0x20,
  Proto = 0xfe,
}

pub enum ToyKind {
  Unknown,
  Ollie,
  R2D2,
  R2Q5,
  BB8,
  BB9E,
  LightningMcQueen,
  SpheroSPRK,
  SpheroMini,
  SpheroBolt,
}

impl ToyKind {

  pub fn prefix(&self) -> &str {
    match self {
      ToyKind::Unknown => "Noprefix",
      ToyKind::Ollie => "2B",
      ToyKind::R2D2 => "D2",
      ToyKind::R2Q5 => "Q5-",
      ToyKind::BB8 => "BB-",
      ToyKind::BB9E => "GB-",
      ToyKind::LightningMcQueen => "LM-",
      ToyKind::SpheroSPRK => "SK-",
      ToyKind::SpheroMini => "SM-",
      ToyKind::SpheroBolt => "SB-",
    }
  }

}