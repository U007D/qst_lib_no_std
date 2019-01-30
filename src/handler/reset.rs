#[no_mangle]
pub unsafe extern "C" fn Reset() -> ! {
    let _x = 42;

    loop {}
}

// The reset vector, a pointer to the reset handler
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;
