use litmus_test_web::run;
pub fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run(2, "2_2_5.wgsl"));
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        //bro what this mean
        //wasm_bindgen_futures::spawn_local(run());
    }
}