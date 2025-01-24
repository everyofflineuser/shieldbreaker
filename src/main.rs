mod bsod;
mod uacbypass;

fn main() {
    println!("Hello, world!");
    //bsod::start_bsod();
    uacbypass::test_uac();
}
