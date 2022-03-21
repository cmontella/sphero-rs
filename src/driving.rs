use crate::packet::*;
use crate::*;
use crate::sphero::*;
use std::sync::mpsc::Sender;


pub enum Direction {
  Forward = 0x00,
  Reverse = 0x01,
}

pub enum TankDriveDirection {
  Forward = 0x00,
  InPlace = 0x08,
  Reverse = 0x18,
}

pub enum DirectionRawMotor {
  Disable = 0x00,
  Forward = 0x01,
  Reverse = 0x02,
}

pub enum StabilizationIndex {
  no_control_system = 0x00,
  full_control_system = 0x01,
  pitch_control_system = 0x02,
  roll_control_system = 0x03,
  yaw_control_system = 0x04,
  speed_and_yaw_control_system = 0x05,
}

pub enum DrivingCommand {
  RawMotor = 0x01,
  SetAckermannSteeringParameters = 0x02,
  Drift = 0x03,
  AbsoluteYawSteering = 0x04,
  EnableFlipDrive = 0x05,
  ResetYaw = 0x06,
  DriveWithHeading = 0x07,
  TankDrive = 0x08,
  RcCarDrive = 0x09,
  DriveToPosition = 0x0a,
  SetStabilization = 0x0c,
}

pub struct Driving{pub sender: Sender<SpheroMessage>}

impl Driving {
 
  /*
  speed: speed from 0 to 255
  heading: heading from 0 to 360
  Direction direction: motor rotation direction
  */
  pub fn drive_with_heading(&self, speed: u8, heading: u16, direction: Direction) {
    let speed = speed & 0xFF;
    let direction = direction as u8;
    let mut data = vec![speed];
    let heading_bytes = heading.to_be_bytes();
    data.append(&mut heading_bytes.to_vec());
    data.push(direction);
    let flags = Flag::RequestsResponse as u8 | Flag::CommandHasTargetId as u8 | Flag::ResetsInactivityTimeout as u8;
    self.sender.send(SpheroMessage::SendFull((
      DeviceId::Driving,
      DrivingCommand::DriveWithHeading as u8,
      0x12,
      data,
      flags,
    )));
  }


/*
    def set_stabilization(self, stabilization_index: StabilizationIndex):
        """
        ???

        :param StabilizationIndex stabilization_index:
        :return:
        """
        self.request(
            DrivingCommand.set_stabilization,
            target_id=0x12,
            data=[stabilization_index.value],
        )

    def raw_motor(
            self,
            left_speed: int = 0x00,
            left_direction: DirectionRawMotor = DirectionRawMotor.forward,
            right_speed=0x00,
            right_direction: DirectionRawMotor = DirectionRawMotor.forward,
    ):
        """
        Control of each motor separately

        note: it works strange :) after wake up need sleeps

        :param int left_speed: speed of left motor from 0 to 255
        :param DirectionRawMotor left_direction:
        :param int right_speed: speed of right motor from 0 to 255
        :param DirectionRawMotor right_direction:
        :return:

        """
        left_direction = left_direction.value
        right_direction = right_direction.value

        self.request(
            DrivingCommand.raw_motor,
            target_id=0x12,
            data=[
                left_direction, left_speed & 0xff,
                right_direction, right_speed & 0xff,
            ],
        )

    def reset_yaw(self):
        """
        Reset of toys yaw

        :return:
        """
        self.request(
            DrivingCommand.reset_yaw,
            target_id=0x12,
        )

    def tank_drive(
            self,
            left_speed: int = 0x00,
            right_speed=0x00,
            direction: TankDriveDirection = TankDriveDirection.forward,
    ):
        """
        ??? todo: this method not exist in android app
        :param left_speed:
        :param right_speed:
        :param direction:
        :return:
        """
        self.request(
            DrivingCommand.tank_drive,
            data=[left_speed, right_speed, direction.value],
            target_id=0x12,
        )

    def ackermann_drive(
            self,
            steering: int = 0x00,
            direction: int = 0x00
    ):
        self.request(
            DrivingCommand.set_ackermann_steering_parameters,
            data=[*steering.to_bytes(4, "big"), *direction.to_bytes(4, "big")],
            flags=Flag.resets_inactivity_timeout.value
        )

    def ackermann_reset(
            self,
            steering: int = 0x00,
            direction: int = 0x00
    ):
        self.request(
            DrivingCommand.absolute_yaw_steering,
            data=[*steering.to_bytes(4, "big"), *direction.to_bytes(4, "big")],
            flags=Flag.resets_inactivity_timeout.value
        )
        */
}