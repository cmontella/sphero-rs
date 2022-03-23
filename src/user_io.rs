use crate::*;
use std::sync::mpsc::Sender;


pub struct Color{
  pub red: u8,
  pub green: u8,
  pub blue: u8,
}

impl Color {
  pub fn to_vec(&self) -> Vec<u8> {
    vec![self.red & 0xFF, self.green & 0xFF, self.blue & 0xFF]
  }
}


pub struct Pixel {
  pub x: u8,
  pub y: u8,
}

impl Pixel {
  pub fn to_vec(&self) -> Vec<u8> {
    vec![self.x & 0x07, self.y & 0x07]
  }
}

pub enum Led {
  FrontRed = 0x01,
  FrontGreen = 0x02,
  FrontBlue = 0x04,
  BlackRed = 0x08,
  BackGreen = 0x10,
  BackBlue = 0x20,
  All = 0x3F,
}

pub enum FrameRotation {
  Normal = 0x00,
  Degrees90 = 0x01,
  Degrees180 = 0x02,
  Degress270 = 0x03,
}

pub enum CapacitiveTouchLocation {
  Hood = 0x0,
  LeftDoor = 0x1,
  RightDoor = 0x2,
  Roof = 0x3,
  Trunk = 0x4,
}

pub enum UserIOCommand {
  EnableGestureEventNotification = 0x00,
  GestureEvent = 0x01,
  EnableButtonEventNotification = 0x02,
  ButtonEvent = 0x03,
  SetLed = 0x04,
  ReleaseLedRequest = 0x05,
  PlayHapticPattern = 0x06,
  PlayAudioFile = 0x07,
  SetAudioVolume = 0x08,
  GetAudioVolume = 0x09,
  StopAllAudio = 0x0a,
  CapTouchEnable = 0x0b,
  AmbientLightSensorEnable = 0x0c,
  EnableIr = 0x0d,
  SetAllLeds = 0x0e,
  SetBacklightIntensity = 0x0f,
  CapTouchIndication = 0x10,
  EnableDebugData = 0x11,
  AssertLcdResetPin = 0x12,
  SetHeadlights = 0x13,
  SetTaillights = 0x14,
  PlayTestTone = 0x18,
  StartIdleLed = 0x19,
  ToyCommands = 0x20,
  ToyEvents = 0x21,
  SetUserProfile = 0x22,
  GetUserProfile = 0x23,
  SetAllLeds32BitMask = 0x1a,
  SetAllLeds64BitMask = 0x1b,
  SetAllLeds8BitMask = 0x1c,
  SetLedMatrixPixel = 0x2d,
  SetLedMatrixOneColor = 0x2f,
  SetLedMatrixFrameRotation = 0x3a,
  SetLedMatrixTextScrolling = 0x3b,
  SetLedMatrixTextScrollingNotify = 0x3c,
  SetLedMatrixSingleCharacter = 0x42,
}


#[derive(Debug)]
pub struct UserIo {pub sender: Sender<SpheroMessage>}

impl UserIo {

  /*
  Set leds colors
  Sphero Bolt only.
  For other toys use set_all_leds
  */
  pub fn set_all_leds_8_bit_mask(&self, front_color: Color, back_color: Color) {
    let leds = Led::All;
    let mut data= vec![leds as u8];
    data.append(&mut front_color.to_vec());
    data.append(&mut back_color.to_vec());
    self.sender.send(SpheroMessage::SendFull((
      DeviceId::UserIo,
      UserIOCommand::SetAllLeds8BitMask as u8,
      0x11,
      data,
      0x00,
    )));
  }

  pub fn set_led_matrix_one_color(&self, color: Color) {
    self.sender.send(SpheroMessage::SendFull((
      DeviceId::UserIo,
      UserIOCommand::SetLedMatrixOneColor as u8,
      0x12,
      color.to_vec(),
      0x00,
    )));
  }

  pub fn set_led_matrix_pixel(&self, pixel: Pixel, color: Color) {
    let mut data = pixel.to_vec();
    data.append(&mut color.to_vec());
    self.sender.send(SpheroMessage::SendFull((
      DeviceId::UserIo,
      UserIOCommand::SetLedMatrixPixel as u8,
      0x12,
      data,
      0x00,
    )));
  }

  pub fn set_led_matrix_picture(&self, picture: String) {
      let mut x = 0;
      let mut y = 0;
      for c in picture.chars() {
        let pixel = Pixel{x, y};
        match c {
          '🔴' => {self.set_led_matrix_pixel(pixel,Color{red: 0xFF, green: 0x00, blue: 0x00})},
          '🟠' => {self.set_led_matrix_pixel(pixel,Color{red: 0xFF, green: 0x7E, blue: 0x00})},
          '🟡' => {self.set_led_matrix_pixel(pixel,Color{red: 0xFF, green: 0xC5, blue: 0x00})},
          '🟢' => {self.set_led_matrix_pixel(pixel,Color{red: 0x00, green: 0xFF, blue: 0x00})},
          '🔵' => {self.set_led_matrix_pixel(pixel,Color{red: 0x00, green: 0x00, blue: 0xFF})},
          '🟣' => {self.set_led_matrix_pixel(pixel,Color{red: 0x8D, green: 0x21, blue: 0x96})},
          '🟤' => {self.set_led_matrix_pixel(pixel,Color{red: 0x77, green: 0x43, blue: 0x42})},
          '⚪' => {self.set_led_matrix_pixel(pixel,Color{red: 0xFF, green: 0xFF, blue: 0xFF})},
          '⚫' => {},
          _ => continue,
        }
        if x == 7 {
          x = 0;
          y += 1;
        } else {
          x += 1;
        }
      }
  }




/*


    def set_led_matrix_single_character(self, symbol: str, color: Color):
        """
        Print symbol on matrix
        :param String symbol:
        :param Color color:
        :return:
        """
        self.request(
            command_id=UserIOCommand.set_led_matrix_single_character,
            data=[*color.to_list(), ord(symbol)],
            target_id=0x12,
        )

    def set_led_matrix_text_scrolling(self, string: str, color: Color, speed: int = 0x10, repeat: bool = True):
        """
        Print text on matrix
        :param str string: max length 6 symbols
        :param Color color:
        :param int speed: max value is 0x1e (30)
        :param bool repeat:
        :return:
        """
        self.request(
            command_id=UserIOCommand.set_led_matrix_text_scrolling,
            data=[
                *color.to_list(),
                speed % 0x1e,
                int(repeat),
                *[ord(c) for c in string[:7]],
                0x00,  # end line
            ],
            target_id=0x12,
        )

    def set_led_matrix_text_scrolling_notify(self):
        self.request(
            command_id=UserIOCommand.set_led_matrix_text_scrolling_notify,
            data=[0x00],  # todo: unknown
            target_id=0x11,
        )

    def set_led_matrix_frame_rotation(self, rotation: FrameRotation = FrameRotation.normal):
        self.request(
            command_id=UserIOCommand.set_led_matrix_frame_rotation,
            data=[rotation.value],
            target_id=0x11,
        )
        
    def enable_cap_touch(self, state: bool, callback: Callable = None):
        """
        notification will be received designeting which area is touched
        """
        
        def callback_wrapper(response: Packet):
            touch_location = CapacitiveTouchLocation(response.data[0])
            return callback(touch_location)
        
        if state == True:
            self.notify(UserIOCommand.cap_touch_indication, callback_wrapper)
        else:
            self.cancel_notify()

        self.request(
            command_id=UserIOCommand.cap_touch_enable,
            data=[int(state)]
        )

    def set_headlights(self, value: int):
            self.request(
            command_id=UserIOCommand.set_headlights,
            data=[int(value)]
        )

    def set_taillights(self, value: int):
            self.request(
            command_id=UserIOCommand.set_taillights,
            data=[int(value)]
        )

    def get_audio_volume(self) -> int:
        response = self.request(
            command_id=UserIOCommand.get_audio_volume
        )
        return response.data[0]

    def set_audio_volume(self, value: int):
            self.request(
            command_id=UserIOCommand.set_audio_volume,
            data=[int(value)]
        )
        */

}