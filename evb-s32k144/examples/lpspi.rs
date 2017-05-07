#![no_std]
#![no_main]

#[macro_use]
extern crate evb_s32k144 as board;

use board::hal::lpspi;
use board::uja1169::Mode;
#[no_mangle]
pub extern "C" fn main() -> ! {
    board::init();
    println!("LPSPI Test");

    let l1 = board::spi::lpspi1();
    l1.set_enabled(false);    
    
    l1.configure(lpspi::Config::default()
        .master(true)
        .sckpcs(4)
        .pcssck(9)
        .dbt(8)
        .sckdiv(8)
        .txwater(3)        
    );
    l1.set_enabled(true);
    let t = l1.target()
        .cpha(true)
        .prescale(2)
        .pcs(3)
        .framesz(15);
    // unsafe {
    //     let s = l1.lpspi;
    //     println!("CR:     {:?}", s.cr());
    //     println!("SR:     {:?}", s.sr());
    //     println!("CFGR0:  {:?}", s.cfgr0());
    //     println!("CFGR1:  {:?}", s.cfgr1());
    //     println!("CCR:    {:?}", s.ccr());
    //     println!("FCR:    {:?}", s.fcr());
    //     println!("FSR:    {:?}", s.fsr());
    //     println!("TCR:    {:?}", s.tcr());
    //     println!("RSR:    {:?}", s.rsr());
    // }    

    let u = board::uja1169::device(t);
    let r = u.reg();
    println!("ids:    {:?}", r.ids());
    println!("mc:     {:?}", r.mc());
    println!("ms:     {:?}", r.ms());
    println!("wds:    {:?}", r.wds());
    println!("sc:     {:?}", r.sc());
    println!("sbccc:  {:?}", r.sbccc());
    println!("mptnvs: {:?}", r.mtpnvs());
    println!("forced_normal: {}", u.is_forced_normal_mode());
    println!("software_development: {}", u.is_software_development_mode());
    println!("mode: {:?}", u.mode());
    println!("");
    println!("Changing to Normal mode");
    u.set_mode(Mode::Normal);
    println!("mode: {:?}", u.mode());

    println!("CAN STATUS");
    println!("0x20 CANC:   {:?}", r.canc());
    println!("0x22 TS:     {:?}", r.ts());
    println!("0x23 TEE:    {:?}", r.tee());
    println!("0x26 DR:     {:?}", r.dr());
    for i in 0..4 {
        println!("0x{:02x} ID{}     {:?}", 0x27+i, i, r.id(i));
    }
    for i in 0..4 {
        println!("0x{:02x} M{}      {:?}", 0x2b+i, i, r.m(i));
    }
    println!("0x2F FC:     {:?}", r.fc());
    for i in 0..8 {
        println!("0x{:02x} DM{}     {:?}", 0x68+i, i, r.dm(i));
    }
    
    
    loop {}
}