use rsps::ps::macos::AArch64;
use rsps::ps::Ps;
use std::env::consts;

fn main() {
    println!("{}, {}, {}", consts::ARCH, consts::OS, consts::FAMILY);
    AArch64::display(AArch64::exec().unwrap());
}
