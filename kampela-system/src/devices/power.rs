//! Power measurement unit

use alloc::collections::vec_deque::VecDeque;
use efm32pg23_fix::SYST;
use crate::{if_in_free, parallel::{AsyncOperation, Threads}, peripherals::adc};


const VALUE_LEN: usize = 2000;

pub struct VoltageValues(VecDeque<i32>);

impl core::ops::Deref for VoltageValues {
    type Target = VecDeque<i32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for VoltageValues {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct ADC {
    threads: Threads<ADCState, 1>,
    values: VoltageValues,
    last_value: i32,
    last_clock_value: u32,
    accumulative: i32,
    period: u32,
    count: usize,
}

pub enum ADCState {
    Ready,
    Request,
}

impl Default for ADCState {
    fn default() -> Self {
        ADCState::Request
    }
}

impl VoltageValues {
    fn new(capacity: usize) -> Self {
        Self(VecDeque::with_capacity(capacity))
    }
}

impl ADC {
    pub fn get<'a>(&'a self) -> &'a VoltageValues {
        &self.values
    }
    pub fn read(&self) -> i32 {
        self.last_value * 211 / 10000
    }
}

impl AsyncOperation for ADC {
    type Init = ();
    type Input<'a> = ();
    type Output = ();

    fn new(_: ()) -> Self {
        Self{
            threads: Threads::new(ADCState::Ready),
            values: VoltageValues::new(VALUE_LEN),
            last_value: 0,
            last_clock_value: SYST::get_current(),
            accumulative: 0,
            period: 0,
            count: 0,
        }
    }

    fn advance(&mut self, _: Self::Input<'_>) {
        match self.threads.advance_state() {
            ADCState::Ready => {
                adc::reset_int_flags();
                adc::request_adc_measure();
                self.threads.change(ADCState::Request);
            },
            ADCState::Request => {
                if if_in_free(|peripherals| adc::read_int_flag(peripherals)) == Ok(true) {
                    let current = SYST::get_current();
                    let period = if current <= self.last_clock_value {
                        self.last_clock_value - current
                    } else {
                        SYST::get_ticks_per_10ms() + self.last_clock_value - current //manage reload, should work if tick < 10ms
                    };
                    self.last_clock_value = current;

                    let value = adc::read_adc();
                    self.accumulative += value * 211 / 10000 / 2;  // prevents overflow
                    self.count += 1;
                    self.last_value = value;

                    self.period += period;

                    if self.period >= SYST::get_ticks_per_10ms() * 100 {
                        if self.values.len() >= self.values.capacity() {
                            self.values.pop_front();
                        }
                        self.values.push_back(self.accumulative * 100 / self.period as i32 * 2 * SYST::get_ticks_per_10ms() as i32 / self.count as i32);

                        self.accumulative = 0;
                        self.period = 0;
                        self.count = 0;
                    }

                    adc::reset_int_flags();
                    adc::request_adc_measure();
                }
            },
        }
    }
}

