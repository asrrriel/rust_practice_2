#![no_main]
use rust_practice_2::util::serializer::*;
use libfuzzer_sys::fuzz_target;


fuzz_target!(|data: &[u8]| {
    let mut mdata = data.to_owned();
    match deserialize_database(&mut mdata) {
        Ok(v) => println!("{:?}",v),
        Err(e) => println!("{}",e)
    }
});
