#![no_std] // unlink standard library
#![no_main]

use core::panic::PanicInfo;

/*

    the bang '!' ('never' type) defines a 'diverging' function (a function that never returns)
    
    PanicInfo contains the file and line where a panic happened,
    along with an optional panic message
    
*/
#[panic_handler] // defines what function should be called on a panic
fn panic(_info: &PanicInfo) -> ! {

    // loop forever, the function never returns
    loop {}
}

// no_mangle disables name mangling
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
