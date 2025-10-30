#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let peripherals = unsafe { ra4m1::Peripherals::steal() };

    // 14 bit analog digital converter =============================================================
    let module_stop = peripherals.MSTP;
    // activate adc
    module_stop.mstpcrd.write( |w| {w.mstpd16().bit(false)} );

    let adc = peripherals.ADC140;
    // set reference voltage
    adc.adhvrefcnt.write( |w| {w.hvsel()._00()
                                        .lvsel()._1()} );
    // port 0 pin 14 is AN009 and A0
    // activate AD conversion for AN009
    adc.adansa0.write( |w| {w.ansa09().bit(true)} );

    // read sensor data
    loop {
        // open adc control register and start conversion
        adc.adcsr.write( |w| {w.adst().set_bit()} );
        while adc.adcsr.read().adst().bit_is_set() {}

        let value = adc.addr[9].read().bits();
        if value < 1100 {
            smile_down(&peripherals.PORT0, &peripherals.PORT2)
        } else {
            smile_up(&peripherals.PORT0, &peripherals.PORT2)
        }
    }
}

fn smile_up(port0: &ra4m1::PORT0, port2: &ra4m1::PORT2) {
    // configure led-matrix pins as output:
    // P003 0
    // P004 1
    // P011 2
    // P012 3
    // P013 4
    // P015 5
    // P204 6
    // P205 7
    // P206 8
    // P212 9
    // P213 10
    // clean output registers
    port0.podr().write( |w| unsafe { w.bits(0)} );
    port2.podr().write( |w| unsafe { w.bits(0)} );

    for _ in 1..1000 {
        // led 15, 16
        port0.podr().write( |w| unsafe { w.bits(1<<12) });
        port0.pdr().write( |w| unsafe { w.bits((1<<12)|(1<<3)) });
        port0.podr().write( |w| unsafe { w.bits(1<<3) });
        port0.pdr().write( |w| unsafe { w.bits(0) });
        // led 17, 28
        port2.podr().write( |w| unsafe { w.bits(1<<4) });
        port2.pdr().write( |w| unsafe { w.bits((1<<4)|(1<<6)) });
        port2.podr().write( |w| unsafe { w.bits(1<<6) });
        port2.pdr().write( |w| unsafe { w.bits(0) });
        // 21, 22
        port2.podr().write( |w| unsafe { w.bits(1<<4) });
        port2.pdr().write( |w| unsafe { w.bits((1<<4)|(1<<5)) });
        port2.podr().write( |w| unsafe { w.bits(1<<5) });
        port2.pdr().write( |w| unsafe { w.bits(0) });
        // 33, 34
        port0.podr().write( |w| unsafe { w.bits(1<<12) });
        port0.pdr().write( |w| unsafe { w.bits((1<<12)|(1<<15)) });
        port0.podr().write( |w| unsafe { w.bits(1<<15) });
        port0.pdr().write( |w| unsafe { w.bits(0) });
        // 63
        port2.podr().write( |w| unsafe { w.bits(1<<6) });
        port0.podr().write( |w| unsafe { w.bits(0) });
        port2.pdr().write( |w| unsafe { w.bits(1<<6) });
        port0.pdr().write( |w| unsafe { w.bits(1<<11) });
        port2.pdr().write( |w| unsafe { w.bits(0) });
        port0.pdr().write( |w| unsafe { w.bits(0) });
        // 76
        port2.podr().write( |w| unsafe { w.bits(1<<13) });
        port0.podr().write( |w| unsafe { w.bits(0) });
        port2.pdr().write( |w| unsafe { w.bits(1<<13) });
        port0.pdr().write( |w| unsafe { w.bits(1<<12) });
        port2.pdr().write( |w| unsafe { w.bits(0) });
        port0.pdr().write( |w| unsafe { w.bits(0) });
        // 77, 78
        port2.podr().write( |w| unsafe { w.bits(1<<13) });
        port0.podr().write( |w| unsafe { w.bits(0) });
        port2.pdr().write( |w| unsafe { w.bits(1<<13) });
        port0.pdr().write( |w| unsafe { w.bits(1<<13) });
        port2.podr().write( |w| unsafe { w.bits(0) });
        port0.podr().write( |w| unsafe { w.bits(1<<13) });
        port2.pdr().write( |w| unsafe { w.bits(0) });
        port0.pdr().write( |w| unsafe { w.bits(0) });
        // 79, 80
        port2.podr().write( |w| unsafe { w.bits(1<<6) });
        port2.pdr().write( |w| unsafe { w.bits((1<<6)|(1<<13)) });
        port2.podr().write( |w| unsafe { w.bits(1<<13) });
        port2.pdr().write( |w| unsafe { w.bits(0) });
        // 81
        port0.podr().write( |w| unsafe { w.bits(1<<3) });
        port2.podr().write( |w| unsafe { w.bits(0) });
        port2.pdr().write( |w| unsafe { w.bits(1<<13) });
        port0.pdr().write( |w| unsafe { w.bits(1<<3) });
        port2.pdr().write( |w| unsafe { w.bits(0) });
        port0.pdr().write( |w| unsafe { w.bits(0) });
        // 70
        port0.podr().write( |w| unsafe { w.bits(1<<11) });
        port0.pdr().write( |w| unsafe { w.bits((1<<11)|(1<<15)) });
        port0.pdr().write( |w| unsafe { w.bits(0) });
    }
}

fn smile_down(port0: &ra4m1::PORT0, port2: &ra4m1::PORT2) {
    // configure led-matrix pins as output:
    // P003 0
    // P004 1
    // P011 2
    // P012 3
    // P013 4
    // P015 5
    // P204 6
    // P205 7
    // P206 8
    // P212 9
    // P213 10
    // clean output registers
    port0.podr().write( |w| unsafe { w.bits(0)} );
    port2.podr().write( |w| unsafe { w.bits(0)} );

    for _ in 1..1000 {
        // led 15, 16
        port0.podr().write( |w| unsafe { w.bits(1<<12) });
        port0.pdr().write( |w| unsafe { w.bits((1<<12)|(1<<3)) });
        port0.podr().write( |w| unsafe { w.bits(1<<3) });
        port0.pdr().write( |w| unsafe { w.bits(0) });
        // led 17, 28
        port2.podr().write( |w| unsafe { w.bits(1<<4) });
        port2.pdr().write( |w| unsafe { w.bits((1<<4)|(1<<6)) });
        port2.podr().write( |w| unsafe { w.bits(1<<6) });
        port2.pdr().write( |w| unsafe { w.bits(0) });
        // 21, 22
        port2.podr().write( |w| unsafe { w.bits(1<<4) });
        port2.pdr().write( |w| unsafe { w.bits((1<<4)|(1<<5)) });
        port2.podr().write( |w| unsafe { w.bits(1<<5) });
        port2.pdr().write( |w| unsafe { w.bits(0) });
        // 33, 34
        port0.podr().write( |w| unsafe { w.bits(1<<12) });
        port0.pdr().write( |w| unsafe { w.bits((1<<12)|(1<<15)) });
        port0.podr().write( |w| unsafe { w.bits(1<<15) });
        port0.pdr().write( |w| unsafe { w.bits(0) });
        // 87
        port0.podr().write( |w| unsafe { w.bits(1<<4) });
        port2.podr().write( |w| unsafe { w.bits(0) });
        port0.pdr().write( |w| unsafe { w.bits(1<<4) });
        port2.pdr().write( |w| unsafe { w.bits(1<<13) });
        port2.pdr().write( |w| unsafe { w.bits(0) });
        port0.pdr().write( |w| unsafe { w.bits(0) });
        // 76
        port2.podr().write( |w| unsafe { w.bits(1<<13) });
        port0.podr().write( |w| unsafe { w.bits(0) });
        port2.pdr().write( |w| unsafe { w.bits(1<<13) });
        port0.pdr().write( |w| unsafe { w.bits(1<<12) });
        port2.pdr().write( |w| unsafe { w.bits(0) });
        port0.pdr().write( |w| unsafe { w.bits(0) });
        // 77, 78
        port2.podr().write( |w| unsafe { w.bits(1<<13) });
        port0.podr().write( |w| unsafe { w.bits(0) });
        port2.pdr().write( |w| unsafe { w.bits(1<<13) });
        port0.pdr().write( |w| unsafe { w.bits(1<<13) });
        port2.podr().write( |w| unsafe { w.bits(0) });
        port0.podr().write( |w| unsafe { w.bits(1<<13) });
        port2.pdr().write( |w| unsafe { w.bits(0) });
        port0.pdr().write( |w| unsafe { w.bits(0) });
        // 79, 80
        port2.podr().write( |w| unsafe { w.bits(1<<6) });
        port2.pdr().write( |w| unsafe { w.bits((1<<6)|(1<<13)) });
        port2.podr().write( |w| unsafe { w.bits(1<<13) });
        port2.pdr().write( |w| unsafe { w.bits(0) });
        // 81
        port0.podr().write( |w| unsafe { w.bits(1<<3) });
        port2.podr().write( |w| unsafe { w.bits(0) });
        port2.pdr().write( |w| unsafe { w.bits(1<<13) });
        port0.pdr().write( |w| unsafe { w.bits(1<<3) });
        port2.pdr().write( |w| unsafe { w.bits(0) });
        port0.pdr().write( |w| unsafe { w.bits(0) });
        // 94
        port0.podr().write( |w| unsafe { w.bits(0) });
        port2.podr().write( |w| unsafe { w.bits(1<<12) });
        port0.pdr().write( |w| unsafe { w.bits(1<<12) });
        port2.pdr().write( |w| unsafe { w.bits(1<<12) });
        port2.pdr().write( |w| unsafe { w.bits(0) });
        port0.pdr().write( |w| unsafe { w.bits(0) });
    }
}