use embassy_time::{Duration, Timer};
use printhor_hwa_common::TrackedStaticCell;
use crate::device::AdcPinTrait;
type PinsCell<T> = std::sync::Mutex<T>;

pub type PinStateRef = &'static PinsCell<PinState>;

const NUM_PINS: usize = 100usize;
/// Pin state for state persistence, inter-conexion, automation and monitoring
/// Purpose: simulation only
pub(crate) struct PinState {
    digital: [bool; NUM_PINS],
}
impl PinState {
    const fn new() -> Self {
        Self {
            digital: [false; NUM_PINS]
        }
    }

    #[inline]
    pub(crate) fn set(&mut self, id: u8, state: bool) {
        self.digital[id as usize] = state
    }

    #[inline]
    pub(crate) fn get(&self, id: u8) -> bool {
        self.digital[id as usize]
    }
}

static GLOBAL_PIN_STATE: TrackedStaticCell<PinsCell<PinState>> = TrackedStaticCell::new();

#[inline]
pub(crate) fn init_pin_state() -> PinStateRef {
    GLOBAL_PIN_STATE.init("GlobaPinState", PinsCell::new(PinState::new()))
}

pub struct MockedIOPin {
    id: u8,
    global_state: PinStateRef,
}

#[allow(unused)]
impl MockedIOPin {
    pub(crate) const fn new(id: u8, global_state: PinStateRef) -> Self {
        Self { id, global_state }
    }

    pub fn set_high(&mut self) {
        match self.global_state.try_lock() {
            Ok(mut g) => g.set(self.id, true),
            Err(_) => {
                panic!("TODO")
            }
        }
    }

    pub fn set_low(&mut self) {
        match self.global_state.try_lock() {
            Ok(mut g) => g.set(self.id, true),
            Err(_) => {
                panic!("TODO")
            }
        }
    }

    pub fn is_set_high(&mut self) -> bool {
        self.is_high()
    }

    pub fn is_set_low(&mut self) -> bool {
        self.is_low()
    }

    #[allow(unused)]
    pub fn is_low(&self) -> bool {
        match self.global_state.try_lock() {
            Ok(g) => g.get(self.id) == false,
            Err(_) => {
                panic!("TODO")
            }
        }
    }

    #[allow(unused)]
    pub fn is_high(&self) -> bool {
        match self.global_state.try_lock() {
            Ok(g) => g.get(self.id) == true,
            Err(_) => {
                panic!("TODO")
            }
        }
    }

    #[allow(unused)]
    pub async fn wait_for_any_edge<'b>(&'b mut self) {
        //println!("wait_edge");
        Timer::after(Duration::from_secs(10)).await;
    }
}

#[cfg(any(feature = "with-hotend", feature = "with-hotbed"))]
impl<T> AdcPinTrait<crate::board::mocked_peripherals::MockedAdc<T>> for MockedIOPin {

}

#[cfg(any(feature = "with-hotend", feature = "with-hotbed"))]
impl<T> AdcPinTrait<crate::board::mocked_peripherals::MockedAdc<T>> for u8 {

}

#[cfg(any(feature = "with-hotend", feature = "with-hotbed"))]
impl AdcPinTrait<u8> for u8 {

}

#[cfg(any(feature = "with-hotend", feature = "with-hotbed"))]
impl AdcPinTrait<u8> for MockedIOPin {

}

#[cfg(feature = "with-hotbed")]
impl<'a, ADC, Word, PIN> embedded_hal::adc::OneShot<ADC, Word, PIN> for MockedIOPin
where PIN: embedded_hal::adc::Channel<ADC>
{

    type Error = u8;
    fn read(&mut self, _: &mut PIN) -> Result<Word, nb::Error<u8>> {
        todo!()
    }

}


#[cfg(feature = "with-spi")]
impl<T> embedded_hal::digital::v2::OutputPin for MockedIOPin<'_, T> {
    type Error = core::convert::Infallible;
    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}