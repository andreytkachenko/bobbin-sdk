#![no_std]
#![no_main]

#[macro_use]
extern crate discovery_stm32f429i as board;

use board::prelude::*;
use board::bobbin_sys::board::*;

static mut DATA: [u8; 1024] = [0u8; 1024];

#[no_mangle]
pub extern "C" fn main() -> ! {
    let brd = board::init();
    let mut mcu = brd.mcu();
    match run(&mut mcu) {
        Ok(_) => println!("Ok"),
        Err(_) => println!("Error"),
    }
    loop {}
}

pub fn run(mcu: &mut GetHeap) -> Result<(), Error> {
    for i in 0..1024 {
        unsafe { DATA[i] = i as u8; }
    }

    // println!("Memory Test");
    // println!("{:?}", sys.memory());

    let heap = mcu.heap();
    // println!("Initial Heap: {:?}", heap);

    unsafe { heap.extend(4096) }

    // println!("{:?}", heap);

    #[derive(Debug)]
    pub struct Abc { 
        a: u32,
        b: u32,
        c: u32,
    };

    let _v = heap.try_new(Abc { a: 10, b: 20, c: 30 })?;
    // println!("v @ {:p}: {:?}", v, v);
    // println!("{:?}", heap);

    let _data = heap.try_slice(0u16, 1024)?;
    // println!("data @ {:p}", data);
    // println!("{:?}", heap);

    heap.try_align(512)?;
    // println!("{:?}", heap);    
    heap.freeze();
    // println!("{:?}", heap);    
    Ok(())
}

