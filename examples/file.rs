extern crate e_drone;

use e_drone::{*};


fn main() {
    let mut eb: file::EncryptedBinary = file::EncryptedBinary::new();

    eb.read("fw_drone_4_drone_p6_20.8.13_20200818.eb");

    println!("{:?}", eb.header);
}
