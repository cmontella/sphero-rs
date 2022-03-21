use std::sync::Mutex;
use crate::*;

lazy_static! {
  static ref SEQUENCE: Mutex<u8> = Mutex::new(0);
}


static ssequence: u8 = 0x00;
static start: u8 = 0x8d;
static end: u8 = 0xd8;
static escape: u8 = 0xab;
static escape_mask: u8 = 0x88;
static escaped_bytes: (u8,u8,u8) = (start & !escape_mask, end & !escape_mask, escape & !escape_mask);
static bad_bytes: (u8,u8,u8) = (start, end, escape);

pub enum Flag {
  RequestsResponse = 0x2,
  RequestsOnlyErrorResponse = 0x4,
  ResetsInactivityTimeout = 0x8,
  CommandHasTargetId = 0x10,
  CommandHasSourceId = 0x20,
  Response = 0x1,
}

/*"""
Packet structure:
---------------------------------
- start      [1 byte]
- flags      [1 byte]
- source_id  [1 byte] (optional)
- target_id  [1 byte] (optional)
- device_id  [1 byte]
- command_id [1 byte]
- data       [n byte]
- checksum   [1 byte]
- end        [1 byte]
---------------------------------
Usually the first data byte is the api_v2 response code
"""*/

#[derive(Debug)   ]
pub struct Packet {
  pub device_id: DeviceId,
  pub command_id: u8,
  pub flags: u8,
  pub target_id: Option<u8>,
  pub source_id: Option<u8>,
  pub sequence: u8,
  pub data: Vec<u8>,
}

impl Packet {

  pub fn new(device_id: DeviceId, command_id: u8, flags: Option<u8>) -> Packet {
    let mut pflags: u8 = 0;
    if flags == None {
      pflags = Flag::RequestsResponse as u8 | Flag::ResetsInactivityTimeout as u8;
    }

    let psequence = Packet::generate_sequence();

    Packet {
      device_id,
      command_id,
      flags: pflags,
      target_id: None,
      source_id: None,
      sequence: psequence,
      data: Vec::new(),
    }
  }

  pub fn set_source_id(&mut self, source_id: u8) {
    self.flags = self.flags | Flag::CommandHasSourceId as u8;
    self.source_id = Some(source_id);
  }

  pub fn set_target_id(&mut self, target_id: u8) {
    self.flags = self.flags | Flag::CommandHasTargetId as u8;
    self.target_id = Some(target_id);
  }

  fn generate_sequence() -> u8 {
    let mut seq = SEQUENCE.lock().unwrap();
    *seq = *seq + 1;
    *seq
  }

  /*
      @cached_property
    def api_error(self) -> Api2Error:
        if self.flags & Flag.response.value and len(self.data) > 0:
            return Api2Error(self.data.pop(0))  # delete first byte from data
        return Api2Error.success
        */

/*
    @staticmethod
    def _unescape_response_data(response_data) -> List[int]:
        raw_data = []

        iter_response_data = iter(response_data)
        for b in iter_response_data:
            # escaping byte allowing escaping start/end/escaping bytes
            if b == Packet.escape:
                # next byte is escaping
                b = next(iter_response_data, None)
                if b not in Packet.escaped_bytes:
                    raise PySpheroRuntimeError(f"Bad escaping byte {b:#04x}")

                b |= Packet.escape_mask

            raw_data.append(b)

        return raw_data
  */

  /*
      @classmethod
    def from_response(cls, response_data: List[int]) -> "Packet":
        """
        Create packet from raw data
        :param response_data: raw data from peripheral
        :return Packet: response packet
        """
        response_data = Packet._unescape_response_data(response_data)
        start, flags, *data, checksum, end = response_data
        if start != cls.start or end != cls.end:
            raise PySpheroRuntimeError(
                f"Bad response packet: wrong start or end byte (start: {start:#04x}, end: {end:#04x})"
            )

        target_id = None
        if flags & Flag.command_has_target_id.value:
            target_id = data.pop(0)

        source_id = None
        if flags & Flag.command_has_source_id.value:
            source_id = data.pop(0)

        device_id, command_id, sequence, *data = data

        packet = cls(
            flags=flags,
            target_id=target_id,
            source_id=source_id,
            device_id=device_id,
            command_id=command_id,
            sequence=sequence,
            data=data,
        )

        calc_checksum = packet.checksum
        if calc_checksum != checksum:
            raise PySpheroRuntimeError(
                f"Bad response checksum. (Expected: {checksum:#04x}, obtained: {calc_checksum:#04x})"
            )

        return packet*/

  pub fn packet_payload(&mut self) -> Vec<u8> { 
    let mut head = vec![self.flags];

    if self.target_id != None {
      head.push(self.target_id.unwrap());
    }

    if self.source_id != None {
      head.push(self.source_id.unwrap());
    }

    head.push(self.device_id as u8);
    head.push(self.command_id);
    head.push(self.sequence);
    head.append(&mut self.data);
    head
  }

  pub fn checksum(&mut self) -> u8 {
    0xFF - (self.packet_payload().iter().sum::<u8>() & 0xFF)
  }

  pub fn build(&mut self) -> Vec<u8> {
    let mut full_packet = self.packet_payload();
    full_packet.push(self.checksum());

    let mut escaped_full_packet = vec![];
    for i in full_packet {
      if i == start || i == end || i == escape {
        let mut escaped = vec![escape, i & !escape_mask];
        escaped_full_packet.append(&mut escaped);
      } else {
        escaped_full_packet.push(i);
      }
    }
    let mut packet = vec![start];
    packet.append(&mut escaped_full_packet);
    packet.push(end);
    packet
  }

}