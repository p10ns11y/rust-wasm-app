// extern tells the rust other language can use this method
#[no_mangle]
pub extern fn add_one(x: u32) -> u32 {
    x + 1
}

// For step 2
extern {
    fn appendNumberToBody(x: u32);
    fn alert(y: u32);
}

#[no_mangle]
pub extern fn run() {
    unsafe {
        appendNumberToBody(56);
        alert(34567);
    }
}
