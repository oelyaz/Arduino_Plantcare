#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;

enum SmileyMoods {
    Happy,
    Sad
}

#[entry]
fn main() -> ! {
    let peripherals = unsafe { ra4m1::Peripherals::steal() };

    initialize(&peripherals);

    let rtc_sec = peripherals.RTC.rseccnt();
    let adc = peripherals.ADC140;

    let mut smiley_mood: SmileyMoods = SmileyMoods::Sad;
    let mut sec_since_read: u8 = rtc_sec.read().bits();
    loop {
        let current_sec = rtc_sec.read().bits();
        // wrap handler
        let passed_time = if current_sec >= sec_since_read {
            current_sec - sec_since_read
        } else {
            (60 + current_sec) - sec_since_read
        };

        if passed_time > 30 {
            // open adc control register and start conversion
            adc.adcsr.write(|w| { w.adst().set_bit() });
            while adc.adcsr.read().adst().bit_is_set() {}

            let value: u16 = adc.addr[9].read().bits();

            // send data to esp32
            let datapoint:u8 = ((value.saturating_sub(800))/10) as u8;
            // check if TDR is empty
            while peripherals.SCI9.ssr().read().tdre().is_0() {}
            peripherals.SCI9.tdr.write(|w| unsafe {w.bits(datapoint)});

            // set smiley
            if value < 1100 {
                smiley_mood = SmileyMoods::Sad;
            } else {
                smiley_mood = SmileyMoods::Happy;
            }
            sec_since_read = rtc_sec.read().bits();
        }
        match smiley_mood {
            SmileyMoods::Happy => smile_up(&peripherals.PORT0, &peripherals.PORT2),
            SmileyMoods::Sad => smile_down(&peripherals.PORT0, &peripherals.PORT2)
        }
    }
}

fn initialize(periph: &ra4m1::Peripherals) {
    // 14 bit analog digital converter =============================================================
    let module_stop = &periph.MSTP;
    // activate adc
    module_stop.mstpcrd.write( |w| {w.mstpd16().bit(false)} );

    let adc = &periph.ADC140;
    // set reference voltage
    adc.adhvrefcnt.write( |w| {w
        .hvsel()._00()
        .lvsel()._1()
    });
    // port 0 pin 14 is AN009 and A0
    // activate AD conversion for AN009
    adc.adansa0.write( |w| {w.ansa09().bit(true)} );

    // real time clock =============================================================================
    // disable RTC write protection
    periph.SYSTEM.prcr.write( |w| unsafe {w
        .prkey().bits(0xA5)
        .prc1().bit(true)
    });

    // start sub-clock oscillator
    periph.SYSTEM.sosccr.write( |w| {w.sostp()._0()} );

    // clock selector RTC RCR4 (sub-clock oscillator)
    periph.RTC.rcr4.write( |w| {w.rcksel()._0()} );

    // enable RTC write protection
    periph.SYSTEM.prcr.write( |w| unsafe {w
        .prkey().bits(0xA5)
        .prc1().bit(false)
    });

    // SCI9 UART ===================================================================================
    // disable PFS write protection in PWPR
    periph.PMISC.pwpr.write( |w| {w
        .b0wi().bit(false)
        .pfswe().bit(true)
    });

    // set pin function in PFS to SCI TX, output and peripheral
    periph.PFS.p109pfs().write( |w| unsafe {w
        .psel().bits(0b00101)
        .pdr().bit(true)
        .pmr().bit(true)
    });

    // enable PFS write protection in PWPR
    periph.PMISC.pwpr.write( |w| {w
        .b0wi().bit(false)
        .pfswe().bit(false)
    });

    // disable MSTP write protection in PRCR
    periph.SYSTEM.prcr.write( |w| unsafe {w
        .prkey().bits(0xA5)
        .prc0().bit(true)
    });

    // activate SCI9 module in MSTP
    module_stop.mstpcrb.write( |w| { w.mstpb29().bit(false)} );

    // SCI9 config
    // disable RE/TE for setup
    periph.SCI9.scr().write(|w| {w
        .te().bit(false)
        .re().bit(false)
    });

    // configure uart in SMR
    periph.SCI9.smr().write( |w| {w
        .cks()._00()
        .stop()._0()
        .pe()._0()
        .cm()._0()
    });

    // set bitrate in BRR N=129, n=0 40mhz 9600bps
    periph.SCI9.brr.write( |w| unsafe {w.bits(129)} );

    // SCR
    periph.SCI9.scr().write( |w| {w
        .cke()._00()
        .te().bit(true)
        .tie().bit(false)
    });

    // enable MSTP write protection in PRCR
    periph.SYSTEM.prcr.write( |w| unsafe {w
        .prkey().bits(0xA5)
        .prc0().bit(false)
    });
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

