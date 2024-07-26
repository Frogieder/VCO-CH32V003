#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use ch32_hal::{self as hal};
// use ch32_hal::println;
use hal::adc;
use hal::delay::Delay;
// core::f64::consts::E
use hal::time::Hertz;
use hal::timer::low_level::{OutputCompareMode, Timer, CountingMode};
use hal::timer::simple_pwm::PwmPin;
use ch32_hal::timer::complementary_pwm::ComplementaryPwmPin;

mod util;

#[qingke_rt::entry]
fn main() -> ! {
    // hal::debug::SDIPrint::enable();
    let mut config = hal::Config::default();
    config.rcc = hal::rcc::Config::SYSCLK_FREQ_48MHZ_HSI;
    let mut p = hal::init(config);

    let mut analog_input = adc::Adc::new(p.ADC1, adc::Config::default());

    let _pin = PwmPin::new_ch2::<0>(p.PA1);
    let _pinn = ComplementaryPwmPin::new_ch2::<0>(p.PA2);
    let ch = hal::timer::Channel::Ch2;
    let timer = Timer::new(p.TIM1);
    let timer_f = Hertz::mhz(48).0;

    timer.set_counting_mode(CountingMode::EdgeAlignedUp);
    timer.set_frequency(Hertz::hz(400));
    timer.enable_outputs();
    timer.start();
    timer.set_output_compare_mode(ch, OutputCompareMode::PwmMode1);
    timer.set_output_compare_preload(ch, true);
    let _max_dutyy = timer.get_max_compare_value() + 1;
    timer.set_compare_value(ch, _max_dutyy / 2);
    timer.enable_channel(ch, true);
    timer.enable_complementary_channel(ch, true);

    const BUFFER_SIZE: usize = 40;
    #[allow(dead_code)]
    const MIN_DIFF: i32 = 16 * BUFFER_SIZE as i32;

    let mut queue: util::Queue<u16, BUFFER_SIZE> = util::Queue::new();
    while let Ok(_) = queue.push(0){};
    let mut sum: u16 = 0;
    #[allow(unused_assignments)]
    let mut deployed_sum: u16 = 0;

    let mut state = true;
    loop {
        sum -= queue.pop().expect("Oops~");
        let sample = analog_input.convert(&mut p.PC4, hal::adc::SampleTime::CYCLES73);
        sum += sample;
        let _ = queue.push(sample);
        // println!("{}", sum);

        deployed_sum = sum as u16;

        let average: f64 = deployed_sum as f64/(BUFFER_SIZE as f64);

        // println!("{}", average as u32);
        if average <= 6. && state {
            timer.enable_channel(ch, false);
            timer.enable_complementary_channel(ch, false);
            state = false;
        }
        if state {
            timer.enable_channel(ch, true);
            timer.enable_complementary_channel(ch, true);

            // Attempt 1 - exp with midi in mind
            // let n: u32 = (deployed_sum/(13*BUFFER_SIZE as u16) + 40).into();
            // let mut f: u32 = (440.0*util::exp(((n as f64)-69.0)*0.693147181/13.0)) as u32;
            
            // Attempt 2 - linear
            // let mut f = (deployed_sum as u32 *7 / BUFFER_SIZE as u32);

            // Attempt 3 - exp
            // let mut f = util::exp(0.009*(average - 6.)) as u32;

            // Attempt 4 - exp + sqrt
            let mut f = (util::exp(0.0084*(average - 6.)) + 0.3*util::sqrt(average-3.)) as u32;


            if f == 0 {
                f = 1;
            }

            // calculate new values
            let pclk_ticks_per_timer_period = timer_f / f;
            let psc: u16 = (((pclk_ticks_per_timer_period - 1) / (1 << 16)).try_into()).unwrap();
            let divide_by = pclk_ticks_per_timer_period / (u32::from(psc) + 1);
            let arr = (u16::try_from(divide_by - 1)).unwrap();

            // write new values to TIM1 registers
            let regs = timer.regs_basic();
            regs.psc().write_value(psc);
            regs.atrlr().write_value(arr);

            // set max duty cycle
            let max_duty = timer.get_max_compare_value() + 1;
            timer.set_compare_value(ch, max_duty / 2);
        }
        else if average > 25. {
            state = true;
        }
        
        Delay.delay_ms(1);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // let _ = hal::println!("\n\n\n{}", _info);
    loop {}
}
