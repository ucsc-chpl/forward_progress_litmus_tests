use litmus_test_web::run;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen(start)]
pub fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::builder().filter_level(log::LevelFilter::Info).init();
        pollster::block_on(run(2, "round_robin.wgsl"));
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        info!("logger initialized");
        //bro what this mean
        //wasm_bindgen_futures::spawn_local(run());
    }
}