use bug_tests::run;
use wasm_bindgen::prelude::*;
#[cfg(not(target_arch = "wasm32"))]
use std::env;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::builder().filter_level(log::LevelFilter::Info).init();
        let args: Vec<String> = env::args().collect();
        if(args.len() != 3) {
            println!("No arguments or incorrect number of arguments passed.");
            println!("Defaulting to 2_2_5.wgsl with 2 threads");
            pollster::block_on(run(2, "2_2_5.wgsl"));
        } else {
            let wgsl_file = &args[1];
            // All of these tests are indended to be run with 2 threads.
            let num_threads: u32 = (&args[2]).parse().expect("Failed to convert string to u32. Usage: cargo run <wgsl_file> <num_threads>");
            pollster::block_on(run(num_threads, wgsl_file));
        }
        
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
    }
}