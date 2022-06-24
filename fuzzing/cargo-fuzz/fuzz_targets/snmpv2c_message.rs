#![no_main]
use libfuzzer_sys::fuzz_target;
mod roundtrip;

fuzz_target!(|data: &[u8]| {
    roundtrip!(data, rasn_snmp::v2c::Message::<bool>);
});
