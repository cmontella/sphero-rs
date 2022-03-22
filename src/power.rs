use crate::packet::*;
use crate::*;
use crate::sphero::*;
use std::sync::mpsc::Sender;

pub enum BatteryVoltageStates {
  Ok = 0x01,
  Low = 0x02,
  Critical = 0x03,
}

pub enum BatteryLMQStates {
  Charged = 0x01,
  Charging = 0x02,
  NotCharging = 0x03,
  Ok = 0x04,
  Low = 0x05,
  Critical = 0x06,
}

pub enum ChargerStates {
  NotCharging = 0x01,
  Charging = 0x02,
  Charged = 0x03,
}

pub enum PowerCommand {
  EnterDeepSleep = 0x00,
  EnterSoftSleep = 0x01,
  GetUsbState = 0x02,
  GetBatteryVoltage = 0x03,
  GetBatteryStateLmq = 0x04,  // value from Core library Used by LMQ
  EnableBatteryStateChangeNotification = 0x05,
  BatteryStateChangeLmq = 0x06, // value from Core library used by LMQ
  Wake = 0x0d,
  GetBatteryPercentage = 0x10,
  SetPowerOptions = 0x12,
  GetPowerOptions = 0x13,
  GetBatteryState = 0x17,
  WillSleepAsync = 0x19,
  SleepAsync = 0x1a,
  BatteryStateChanged = 0x1f,
}

#[derive(Debug)]
pub struct Power {pub sender: Sender<SpheroMessage>}

impl Power {

  pub fn enter_deep_sleep(&self) {
    self.sender.send(SpheroMessage::Send((DeviceId::Power,PowerCommand::EnterDeepSleep as u8)));
  }

  pub fn enter_soft_sleep(&self) {
    self.sender.send(SpheroMessage::Send((DeviceId::Power,PowerCommand::EnterSoftSleep as u8)));
  }

    /*
    

    def get_battery_voltage(self) -> float:
        """
        Returns battery voltage. Allows to determine the level of charge
        :float return: battery voltage in volts
        """

        response = self.request(PowerCommand.get_battery_voltage)
        return int.from_bytes(response.data, "big") / 100
*/
  pub fn wake(&self) {
    self.sender.send(SpheroMessage::Send((DeviceId::Power,PowerCommand::Wake as u8)));
  }
        /*
    def get_battery_state_LMQ(self) -> BatteryLMQStates:
        """
        Get battery state without known voltage constants
        :return BatteryVoltageStates:
        """

        response = self.request(PowerCommand.get_battery_state_LMQ, timeout=100)
        return BatteryLMQStates(response.data[0])

    def get_battery_state(self) -> BatteryVoltageStates:
        """
        Get battery state without known voltage constants
        :return BatteryVoltageStates:
        """

        response = self.request(PowerCommand.get_battery_state)
        return BatteryVoltageStates(response.data[0])

    def battery_state_changed(self) -> ChargerStates:
        """
        Charging status information
        :return ChargerStates:
        """

        response = self.request(PowerCommand.battery_state_changed)
        return ChargerStates(response.data[0])

    def get_battery_percentage(self) -> int:
        """
        Returns battery percentage
        """

        response = self.request(PowerCommand.get_battery_percentage, timeout = 1000)
        return response.data[0]
        */
}

