cfg_if::cfg_if! {
    if #[cfg(feature = "with-serial-port-1")] {
        pub type UartPort1Device = crate::board::mocked_peripherals::MockedUart;
        pub type UartPort1Tx = crate::board::mocked_peripherals::MockedUartTx;
        pub type UartPort1Rx = crate::board::mocked_peripherals::MockedUartRx;
        pub type UartPort1TxControllerRef = crate::board::ControllerRef<UartPort1Tx>;
        pub type UartPort1RxInputStream = crate::board::mocked_peripherals::MockedUartRxInputStream;
    }
}
cfg_if::cfg_if! {
    if #[cfg(feature = "with-serial-port-2")] {
        pub type UartPort2Device = crate::board::mocked_peripherals::MockedUartSink;
        pub type UartPort2Tx = crate::board::mocked_peripherals::MockedUartSinkTx;
        pub type UartPort2Rx = crate::board::mocked_peripherals::MockedUartSinkRx;
        pub type UartPort2TxControllerRef = crate::board::ControllerRef<UartPort2Tx>;
        pub type UartPort2RxInputStream = crate::board::mocked_peripherals::MockedUartSinkRxInputStream;
    }
}

#[cfg(any(feature = "with-hotend", feature = "with-hotbed"))]
pub type AdcImpl<T> = crate::board::mocked_peripherals::MockedAdc<T>;

#[cfg(any(feature = "with-hotend", feature = "with-hotbed"))]
pub trait AdcTrait {}

#[cfg(any(feature = "with-hotend", feature = "with-hotbed"))]
pub trait AdcPinTrait<T> {}

#[cfg(feature = "with-trinamic")]
pub type UartTrinamic = crate::board::comm::SingleWireSoftwareUart;

#[cfg(feature = "with-trinamic")]
pub use crate::board::mocked_peripherals::{MockedTrinamicDriver, trinamic_driver_simulator};

#[cfg(feature = "with-trinamic")]
pub use crate::board::comm::AxisChannel;

#[cfg(feature = "with-trinamic")]
pub type TMCUartCh1Pin = crate::board::MockedIOPin;
#[cfg(feature = "with-trinamic")]
pub type TMCUartCh2Pin = crate::board::MockedIOPin;
#[cfg(feature = "with-trinamic")]
pub type TMCUartCh3Pin = crate::board::MockedIOPin;
#[cfg(feature = "with-trinamic")]
pub type TMCUartCh4Pin = crate::board::MockedIOPin;


#[cfg(feature = "with-spi")]
pub type Spi = crate::board::mocked_peripherals::MockedSpi;

#[cfg(feature = "with-spi")]
pub type SpiDeviceRef = crate::board::ControllerRef<Spi>;

#[cfg(feature = "with-fan-layer")]
pub type PwmFanLayer = crate::board::mocked_peripherals::MockedPwm;

#[cfg(feature = "with-fan-extra-1")]
pub type PwmFanExtra1 = crate::board::mocked_peripherals::MockedPwm;

#[cfg(feature = "with-probe")]
pub type PwmServo = crate::board::mocked_peripherals::MockedPwm;

#[cfg(feature = "with-hotend")]
pub type PwmHotend = crate::board::mocked_peripherals::MockedPwm;

#[cfg(feature = "with-hotbed")]
pub type PwmHotbed = crate::board::mocked_peripherals::MockedPwm;

#[cfg(feature = "with-laser")]
pub type PwmLaser = crate::board::mocked_peripherals::MockedPwm;

#[cfg(feature = "with-fan-layer")]
pub use crate::board::mocked_peripherals::PwmChannel;

#[cfg(feature = "with-sdcard")]
pub type SDCardBlockDevice = crate::board::mocked_peripherals::MockledSDCardBlockDevice;

#[cfg(feature = "with-sdcard")]
pub type SDCardBlockDeviceRef = crate::board::ControllerRef<SDCardBlockDevice>;

#[cfg(feature = "with-hotend")]
pub type AdcHotendPeripheral = u8;

#[cfg(any(feature = "with-hotend", feature = "with-hotbed"))]
pub type AdcHotendHotbed = AdcImpl<u8>;

#[cfg(feature = "with-hotend")]
pub type AdcHotendPin = crate::board::mocked_peripherals::MockedIOPin;

#[cfg(feature = "with-hotbed")]
pub type AdcHotbedPeripheral = u8;

#[cfg(feature = "with-hotbed")]
pub type AdcHotbedPin = crate::board::mocked_peripherals::MockedIOPin;

pub type Watchdog = crate::board::mocked_peripherals::MockedWatchdog<'static, u8>;

#[cfg(feature = "with-display")]
pub type DisplayDevice = crate::board::mocked_peripherals::SimulatorDisplayDevice;
#[cfg(feature = "with-display")]
pub type DisplayScreen<UI> = crate::board::mocked_peripherals::SimulatorDisplayScreen<UI>;

#[cfg(feature = "with-motion")]
pub struct MotionPins {
    pub x_enable_pin: crate::board::mocked_peripherals::MockedIOPin,
    pub y_enable_pin: crate::board::mocked_peripherals::MockedIOPin,
    pub z_enable_pin: crate::board::mocked_peripherals::MockedIOPin,
    pub e_enable_pin: crate::board::mocked_peripherals::MockedIOPin,

    pub x_endstop_pin: crate::board::mocked_peripherals::MockedIOPin,
    pub y_endstop_pin: crate::board::mocked_peripherals::MockedIOPin,
    pub z_endstop_pin: crate::board::mocked_peripherals::MockedIOPin,
    pub e_endstop_pin: crate::board::mocked_peripherals::MockedIOPin,

    pub x_step_pin: crate::board::mocked_peripherals::MockedIOPin,
    pub y_step_pin: crate::board::mocked_peripherals::MockedIOPin,
    pub z_step_pin: crate::board::mocked_peripherals::MockedIOPin,
    pub e_step_pin: crate::board::mocked_peripherals::MockedIOPin,

    pub x_dir_pin: crate::board::mocked_peripherals::MockedIOPin,
    pub y_dir_pin: crate::board::mocked_peripherals::MockedIOPin,
    pub z_dir_pin: crate::board::mocked_peripherals::MockedIOPin,
    pub e_dir_pin: crate::board::mocked_peripherals::MockedIOPin,
}

#[cfg(feature = "with-motion")]
impl MotionPins {
    #[inline]
    pub fn enable_x_stepper(&mut self) {
        self.x_enable_pin.set_low();
    }
    #[inline]
    pub fn enable_y_stepper(&mut self) {
        self.y_enable_pin.set_low();
    }
    #[inline]
    pub fn enable_z_stepper(&mut self) {
        self.z_enable_pin.set_low();
    }
    #[inline]
    pub fn enable_e_stepper(&mut self) { self.e_enable_pin.set_low(); }
    #[inline]
    pub fn disable_x_stepper(&mut self) {
        self.x_enable_pin.set_high();
    }
    #[inline]
    pub fn disable_y_stepper(&mut self) {
        self.y_enable_pin.set_high();
    }
    #[inline]
    pub fn disable_z_stepper(&mut self) {
        self.z_enable_pin.set_high();
    }
    #[inline]
    pub fn disable_e_stepper(&mut self) { self.e_enable_pin.set_high(); }
    #[inline]
    pub fn disable_all_steppers(&mut self) {
        self.x_enable_pin.set_high();
        self.y_enable_pin.set_high();
        self.z_enable_pin.set_high();
        self.e_enable_pin.set_high();
    }
}


#[cfg(feature = "with-motion")]
pub struct MotionDevice {

    #[cfg(feature = "with-trinamic")]
    pub trinamic_uart: UartTrinamic,

    pub motion_pins: MotionPins,
}


#[cfg(feature = "with-sdcard")]
pub struct CardDevice {

    pub card_spi: SDCardBlockDevice,
}

#[cfg(feature = "with-probe")]
pub struct ProbePeripherals {
    pub power_pwm: printhor_hwa_common::ControllerRef<PwmServo>,
    pub power_channel: PwmChannel,
}

#[cfg(feature = "with-hotend")]
pub struct HotendPeripherals {
    pub power_pwm: printhor_hwa_common::ControllerRef<PwmHotend>,
    pub power_channel: PwmChannel,
    pub temp_adc: printhor_hwa_common::ControllerRef<AdcHotendHotbed>,
    pub temp_pin: crate::board::mocked_peripherals::MockedIOPin,
}

#[cfg(feature = "with-hotbed")]
pub struct HotbedPeripherals {
    pub power_pwm: printhor_hwa_common::ControllerRef<PwmHotbed>,
    pub power_channel: PwmChannel,
    pub temp_adc: printhor_hwa_common::ControllerRef<AdcHotendHotbed>,
    pub temp_pin: crate::board::mocked_peripherals::MockedIOPin,
}

#[cfg(feature = "with-fan-layer")]
pub struct FanLayerPeripherals {
    pub power_pwm: printhor_hwa_common::ControllerRef<PwmFanLayer>,
    pub power_channel: PwmChannel,
}

#[cfg(feature = "with-fan-extra-1")]
pub struct FanExtra1Peripherals {
    pub power_pwm: printhor_hwa_common::ControllerRef<PwmFanExtra1>,
    pub power_channel: PwmChannel,
}

#[cfg(feature = "with-laser")]
pub struct LaserPeripherals {
    pub power_pwm: printhor_hwa_common::ControllerRef<PwmLaser>,
    pub power_channel: PwmChannel,
}