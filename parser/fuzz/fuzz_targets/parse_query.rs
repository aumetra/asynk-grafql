#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let _ = asynk_grafql_parser::parse_query(data);
});
