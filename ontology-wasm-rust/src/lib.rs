#![no_std]
use ontio_std::runtime;

#[no_mangle]
fn invoke() {
	runtime::ret(b"hello");
}

