/*class Sphero:
    """
    High-level API for communicate with sphero toy
    """

    def __init__(
            self,
            mac_address: str,
            toy_type: Toy = Toy.unknown,
            ble_adapter_cls: ClassVar[AbstractBleAdapter] = BleAdapter
    ):
        self.mac_address = mac_address
        self.type = toy_type
        self._ble_adapter_cls = ble_adapter_cls
        self._ble_adapter = None

    @property
    def ble_adapter(self):
        if self._ble_adapter is None:
            raise PySpheroException("Use Sphero as context manager")
        return self._ble_adapter

    @ble_adapter.setter
    def ble_adapter(self, value):
        self._ble_adapter = value

    def __enter__(self):
        self._ble_adapter = self._ble_adapter_cls(self.mac_address)
        # if self.type is Toy.unknown:
        #     self.type = TOY_BY_PREFIX.get(self.name[:3], Toy.unknown)
        return self

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.ble_adapter.close()

    @cached_property
    def system_info(self) -> SystemInfo:
        return SystemInfo(ble_adapter=self.ble_adapter)

    @cached_property
    def power(self) -> Power:
        return Power(ble_adapter=self.ble_adapter)

    @cached_property
    def driving(self) -> Driving:
        return Driving(ble_adapter=self.ble_adapter)

    @cached_property
    def api_processor(self) -> ApiProcessor:
        return ApiProcessor(ble_adapter=self.ble_adapter)

    @cached_property
    def user_io(self) -> UserIO:
        return UserIO(ble_adapter=self.ble_adapter)

    @cached_property
    def sensor(self) -> Sensor:
        return Sensor(ble_adapter=self.ble_adapter)

    @cached_property
    def animatronics(self) -> Animatronics:
        return Animatronics(ble_adapter=self.ble_adapter)*/

use crate::constants::*;
#[cfg(target_os = "windows")]
use btleplug::winrtble::{adapter::Adapter, manager::Manager};

struct Sphero {
  mac_address: String,
  kind: ToyKind,
  adapter: Option<Adapter>,
}

impl Sphero {

  pub fn new(mac_address: String) -> Sphero {
    Sphero {
      mac_address: mac_address,
      kind: ToyKind::Unknown,
      adapter: None,
    }
  }

  pub fn wake(&self) {
    //let packet = Packet::new(device_id,command_id);
    //self.request(PowerCommand.wake)
  }
        

}

/*
def request(self, command_id: Enum, timeout: float = 10, raise_api_error: bool = True, **kwargs) -> Packet:
return self.ble_adapter.write(
    self.packet(command_id=command_id.value, **kwargs),
    raise_api_error=raise_api_error,
    timeout=timeout,
)*/



/*
import random
from time import sleep

from pysphero.core import Sphero
from pysphero.driving import Direction


def main():
    mac_address = "aa:bb:cc:dd:ee:ff"
    with Sphero(mac_address=mac_address) as sphero:
        sphero.power.wake()

        for _ in range(5):
            sleep(2)
            speed = random.randint(50, 100)
            heading = random.randint(0, 360)
            print(f"Send drive with speed {speed} and heading {heading}")

            sphero.driving.drive_with_heading(speed, heading, Direction.forward)

        sphero.power.enter_soft_sleep()


if __name__ == "__main__":
    main()*/
