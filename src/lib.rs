use std::borrow::Cow;
use wgpu::util::DeviceExt;
use wasm_bindgen::prelude::*;
use log::{info};

#[wasm_bindgen]
pub async fn run(num_threads: u32, kernel_file: &str) -> u32 {
    info!("Program started, running kernel");

    let threads_finished = execute_gpu(num_threads, kernel_file).await.unwrap();

    info!("Finished execute_gpu");
    let disp_steps: String = threads_finished.to_string();

    info!("Threads finished: {}", disp_steps);
    return threads_finished;
}

#[wasm_bindgen]
pub async fn execute_gpu(num_threads: u32, kernel_file: &str) -> Option<u32> {
    info!("Got into exec gpu");
    let instance = wgpu::Instance::default();
    info!("Got instance");
    // `request_adapter` instantiates the general connection to the GPU
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await?;
    println!("got adapter");
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::downlevel_defaults(),
            },
            None,
        )
        .await
        .unwrap();
    info!("Running test on {}", adapter.get_info().name);
    let cs_module: wgpu::ShaderModule = match kernel_file {
        "tests/LOBE/3_threads_4_instructions/95/95.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/95/95.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/93/93.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/93/93.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/38/38.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/38/38.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/91/91.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/31/31.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/31/31.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/4/4.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/61/61.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/61/61.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/92/92.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/92/92.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/48/48.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/48/48.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/14/14.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/52/52.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/52/52.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/29/29.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/29/29.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/56/56.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/56/56.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/86/86.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/86/86.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/84/84.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/55/55.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/55/55.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/34/34.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/34/34.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/94/94.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/94/94.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/80/80.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/80/80.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/82/82.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/82/82.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/64/64.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/64/64.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/36/36.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/36/36.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/10/10.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/10/10.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/100/100.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/100/100.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/66/66.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/66/66.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/72/72.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/99/99.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/13/13.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/46/46.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_4_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_4_instructions/30/30.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_3_instructions/7/7.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_3_instructions/7/7.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_3_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_3_instructions/18/18.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_3_instructions/11/11.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_3_instructions/11/11.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_3_instructions/16/16.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_3_instructions/16/16.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_3_instructions/20/20.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_3_instructions/20/20.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_3_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_3_instructions/3/3.wgsl"))),
            })
        }
        "tests/LOBE/3_threads_3_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/3_threads_3_instructions/13/13.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/159/159.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/159/159.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/79/79.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/79/79.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/81/81.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/81/81.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/156/156.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/156/156.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/118/118.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/118/118.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/134/134.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/134/134.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/137/137.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/137/137.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/119/119.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/119/119.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/48/48.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/48/48.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/60/60.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/60/60.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/107/107.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/107/107.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/116/116.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/155/155.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/155/155.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/28/28.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/28/28.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/122/122.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/122/122.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/113/113.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/113/113.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/84/84.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/102/102.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/102/102.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/76/76.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/76/76.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/133/133.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/133/133.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/75/75.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/75/75.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/126/126.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/126/126.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/87/87.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/71/71.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/71/71.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/139/139.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/139/139.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/161/161.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/161/161.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/120/120.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/120/120.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/117/117.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/117/117.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/99/99.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/46/46.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/30/30.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_4_instructions/124/124.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_4_instructions/124/124.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/159/159.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/159/159.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/47/47.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/47/47.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/112/112.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/112/112.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/91/91.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/4/4.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/81/81.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/81/81.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/22/22.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/22/22.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/132/132.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/132/132.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/151/151.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/151/151.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/157/157.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/157/157.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/50/50.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/50/50.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/90/90.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/90/90.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/158/158.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/158/158.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/136/136.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/136/136.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/60/60.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/60/60.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/168/168.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/168/168.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/14/14.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/18/18.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/116/116.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/9/9.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/9/9.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/103/103.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/103/103.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/175/175.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/175/175.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/148/148.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/148/148.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/56/56.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/56/56.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/105/105.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/105/105.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/17/17.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/17/17.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/101/101.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/101/101.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/12/12.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/12/12.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/89/89.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/89/89.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/126/126.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/126/126.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/131/131.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/131/131.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/87/87.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/166/166.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/166/166.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/160/160.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/160/160.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/167/167.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/167/167.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/161/161.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/161/161.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/128/128.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/128/128.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/123/123.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/123/123.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/19/19.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/19/19.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/72/72.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/42/42.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/42/42.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/149/149.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/149/149.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/13/13.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/142/142.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/142/142.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/108/108.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/108/108.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/146/146.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/146/146.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/46/46.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/62/62.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/62/62.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/63/63.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/63/63.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_3_instructions/124/124.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_3_instructions/124/124.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_2_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_2_instructions/6/6.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_2_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_2_instructions/5/5.wgsl"))),
            })
        }
        "tests/LOBE/2_threads_2_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE/2_threads_2_instructions/3/3.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/93/93.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/93/93.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/38/38.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/38/38.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/91/91.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/4/4.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/92/92.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/92/92.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/14/14.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/29/29.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/29/29.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/86/86.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/86/86.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/84/84.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/94/94.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/94/94.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/80/80.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/80/80.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/82/82.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/82/82.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/36/36.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/36/36.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/10/10.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/10/10.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/100/100.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/100/100.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/72/72.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/99/99.wgsl"))),
            })
        }
        "tests/HSA/3_threads_4_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_4_instructions/30/30.wgsl"))),
            })
        }
        "tests/HSA/3_threads_3_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_3_instructions/18/18.wgsl"))),
            })
        }
        "tests/HSA/3_threads_3_instructions/11/11.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_3_instructions/11/11.wgsl"))),
            })
        }
        "tests/HSA/3_threads_3_instructions/20/20.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_3_instructions/20/20.wgsl"))),
            })
        }
        "tests/HSA/3_threads_3_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_3_instructions/3/3.wgsl"))),
            })
        }
        "tests/HSA/3_threads_3_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/3_threads_3_instructions/13/13.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/159/159.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/159/159.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/156/156.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/156/156.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/118/118.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/118/118.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/134/134.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/134/134.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/137/137.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/137/137.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/48/48.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/48/48.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/107/107.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/107/107.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/116/116.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/155/155.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/155/155.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/28/28.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/28/28.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/122/122.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/122/122.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/84/84.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/102/102.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/102/102.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/76/76.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/76/76.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/133/133.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/133/133.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/75/75.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/75/75.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/126/126.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/126/126.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/87/87.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/139/139.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/139/139.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/120/120.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/120/120.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/99/99.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/46/46.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/30/30.wgsl"))),
            })
        }
        "tests/HSA/2_threads_4_instructions/124/124.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_4_instructions/124/124.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/47/47.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/47/47.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/91/91.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/4/4.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/81/81.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/81/81.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/132/132.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/132/132.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/151/151.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/151/151.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/50/50.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/50/50.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/90/90.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/90/90.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/158/158.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/158/158.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/136/136.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/136/136.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/60/60.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/60/60.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/168/168.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/168/168.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/14/14.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/18/18.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/116/116.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/103/103.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/103/103.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/175/175.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/175/175.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/148/148.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/148/148.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/56/56.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/56/56.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/105/105.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/105/105.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/17/17.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/17/17.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/101/101.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/101/101.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/12/12.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/12/12.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/89/89.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/89/89.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/131/131.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/131/131.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/166/166.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/166/166.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/160/160.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/160/160.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/161/161.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/161/161.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/128/128.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/128/128.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/123/123.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/123/123.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/19/19.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/19/19.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/72/72.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/149/149.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/149/149.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/13/13.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/142/142.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/142/142.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/108/108.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/108/108.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/146/146.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/146/146.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/46/46.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/62/62.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/62/62.wgsl"))),
            })
        }
        "tests/HSA/2_threads_3_instructions/124/124.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_3_instructions/124/124.wgsl"))),
            })
        }
        "tests/HSA/2_threads_2_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_2_instructions/6/6.wgsl"))),
            })
        }
        "tests/HSA/2_threads_2_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_2_instructions/5/5.wgsl"))),
            })
        }
        "tests/HSA/2_threads_2_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_2_instructions/3/3.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/95/95.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/95/95.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/93/93.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/93/93.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/38/38.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/38/38.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/91/91.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/31/31.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/31/31.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/4/4.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/92/92.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/92/92.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/14/14.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/29/29.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/29/29.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/86/86.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/86/86.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/84/84.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/94/94.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/94/94.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/80/80.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/80/80.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/82/82.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/82/82.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/36/36.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/36/36.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/10/10.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/10/10.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/100/100.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/100/100.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/72/72.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/99/99.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/13/13.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_4_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_4_instructions/30/30.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_3_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_3_instructions/18/18.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_3_instructions/11/11.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_3_instructions/11/11.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_3_instructions/20/20.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_3_instructions/20/20.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_3_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_3_instructions/3/3.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/3_threads_3_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/3_threads_3_instructions/13/13.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/159/159.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/159/159.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/171/171.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/171/171.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/79/79.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/79/79.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/81/81.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/81/81.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/156/156.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/156/156.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/118/118.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/118/118.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/22/22.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/22/22.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/134/134.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/134/134.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/50/50.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/50/50.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/137/137.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/137/137.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/119/119.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/119/119.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/48/48.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/48/48.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/60/60.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/60/60.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/107/107.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/107/107.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/168/168.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/168/168.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/58/58.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/58/58.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/116/116.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/52/52.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/52/52.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/155/155.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/155/155.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/28/28.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/28/28.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/122/122.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/122/122.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/88/88.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/88/88.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/115/115.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/115/115.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/113/113.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/113/113.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/140/140.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/140/140.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/84/84.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/102/102.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/102/102.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/55/55.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/55/55.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/76/76.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/76/76.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/101/101.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/101/101.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/169/169.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/169/169.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/133/133.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/133/133.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/75/75.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/75/75.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/110/110.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/110/110.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/126/126.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/126/126.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/54/54.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/54/54.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/131/131.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/131/131.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/87/87.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/71/71.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/71/71.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/139/139.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/139/139.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/161/161.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/161/161.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/120/120.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/120/120.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/128/128.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/128/128.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/145/145.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/145/145.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/170/170.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/170/170.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/117/117.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/117/117.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/99/99.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/172/172.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/172/172.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/142/142.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/142/142.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/46/46.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/30/30.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/63/63.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/63/63.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/124/124.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/124/124.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_4_instructions/83/83.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_4_instructions/83/83.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/6/6.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/95/95.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/95/95.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/159/159.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/159/159.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/5/5.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/47/47.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/47/47.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/32/32.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/32/32.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/171/171.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/171/171.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/93/93.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/93/93.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/59/59.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/59/59.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/112/112.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/112/112.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/91/91.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/31/31.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/31/31.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/4/4.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/81/81.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/81/81.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/23/23.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/23/23.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/156/156.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/156/156.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/118/118.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/118/118.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/85/85.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/85/85.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/22/22.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/22/22.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/132/132.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/132/132.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/163/163.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/163/163.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/114/114.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/114/114.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/130/130.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/130/130.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/151/151.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/151/151.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/125/125.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/125/125.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/157/157.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/157/157.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/147/147.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/147/147.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/50/50.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/50/50.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/90/90.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/90/90.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/158/158.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/158/158.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/96/96.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/96/96.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/92/92.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/92/92.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/43/43.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/43/43.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/39/39.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/39/39.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/137/137.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/137/137.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/136/136.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/136/136.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/173/173.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/173/173.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/57/57.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/57/57.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/53/53.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/53/53.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/60/60.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/60/60.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/107/107.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/107/107.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/168/168.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/168/168.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/14/14.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/18/18.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/150/150.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/150/150.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/116/116.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/1/1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/1/1.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/165/165.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/165/165.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/52/52.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/52/52.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/155/155.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/155/155.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/9/9.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/9/9.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/28/28.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/28/28.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/0/0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/0/0.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/129/129.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/129/129.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/174/174.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/174/174.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/144/144.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/144/144.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/154/154.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/154/154.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/88/88.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/88/88.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/103/103.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/103/103.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/175/175.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/175/175.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/29/29.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/29/29.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/16/16.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/16/16.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/148/148.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/148/148.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/113/113.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/113/113.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/56/56.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/56/56.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/140/140.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/140/140.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/105/105.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/105/105.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/135/135.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/135/135.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/86/86.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/86/86.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/84/84.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/102/102.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/102/102.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/55/55.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/55/55.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/76/76.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/76/76.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/17/17.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/17/17.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/101/101.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/101/101.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/34/34.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/34/34.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/169/169.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/169/169.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/94/94.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/94/94.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/80/80.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/80/80.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/12/12.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/12/12.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/75/75.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/75/75.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/110/110.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/110/110.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/82/82.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/82/82.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/89/89.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/89/89.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/126/126.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/126/126.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/20/20.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/20/20.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/21/21.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/21/21.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/64/64.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/64/64.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/54/54.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/54/54.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/162/162.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/162/162.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/131/131.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/131/131.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/87/87.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/166/166.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/166/166.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/160/160.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/160/160.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/36/36.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/36/36.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/100/100.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/100/100.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/66/66.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/66/66.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/167/167.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/167/167.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/71/71.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/71/71.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/139/139.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/139/139.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/104/104.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/104/104.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/152/152.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/152/152.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/161/161.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/161/161.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/141/141.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/141/141.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/120/120.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/120/120.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/128/128.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/128/128.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/123/123.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/123/123.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/97/97.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/97/97.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/145/145.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/145/145.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/170/170.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/170/170.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/8/8.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/8/8.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/19/19.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/19/19.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/45/45.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/45/45.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/72/72.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/35/35.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/35/35.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/127/127.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/127/127.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/42/42.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/42/42.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/27/27.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/27/27.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/78/78.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/78/78.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/2/2.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/2/2.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/99/99.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/149/149.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/149/149.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/13/13.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/69/69.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/69/69.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/153/153.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/153/153.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/68/68.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/68/68.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/172/172.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/172/172.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/142/142.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/142/142.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/108/108.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/108/108.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/146/146.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/146/146.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/46/46.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/62/62.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/62/62.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/73/73.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/73/73.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/63/63.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/63/63.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/124/124.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/124/124.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_3_instructions/83/83.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_3_instructions/83/83.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_2_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_2_instructions/6/6.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_2_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_2_instructions/5/5.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_2_instructions/1/1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_2_instructions/1/1.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_2_instructions/0/0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_2_instructions/0/0.wgsl"))),
            })
        }
        "tests/HSA_OBE_STRONG/2_threads_2_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE_STRONG/2_threads_2_instructions/3/3.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_4_instructions/171/171.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_4_instructions/171/171.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_4_instructions/118/118.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_4_instructions/118/118.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_4_instructions/119/119.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_4_instructions/119/119.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_4_instructions/168/168.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_4_instructions/168/168.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_4_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_4_instructions/116/116.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_4_instructions/169/169.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_4_instructions/169/169.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_4_instructions/170/170.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_4_instructions/170/170.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_4_instructions/117/117.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_4_instructions/117/117.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_4_instructions/172/172.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_4_instructions/172/172.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/6/6.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/95/95.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/95/95.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/159/159.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/159/159.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/5/5.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/47/47.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/47/47.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/32/32.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/32/32.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/171/171.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/171/171.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/93/93.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/93/93.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/59/59.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/59/59.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/112/112.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/112/112.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/31/31.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/31/31.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/4/4.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/23/23.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/23/23.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/156/156.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/156/156.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/118/118.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/118/118.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/85/85.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/85/85.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/22/22.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/22/22.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/163/163.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/163/163.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/114/114.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/114/114.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/130/130.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/130/130.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/125/125.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/125/125.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/157/157.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/157/157.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/147/147.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/147/147.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/96/96.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/96/96.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/92/92.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/92/92.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/43/43.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/43/43.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/39/39.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/39/39.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/137/137.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/137/137.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/173/173.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/173/173.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/57/57.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/57/57.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/53/53.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/53/53.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/107/107.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/107/107.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/168/168.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/168/168.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/150/150.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/150/150.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/1/1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/1/1.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/165/165.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/165/165.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/52/52.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/52/52.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/155/155.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/155/155.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/9/9.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/9/9.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/28/28.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/28/28.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/0/0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/0/0.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/129/129.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/129/129.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/174/174.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/174/174.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/144/144.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/144/144.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/154/154.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/154/154.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/88/88.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/88/88.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/29/29.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/29/29.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/16/16.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/16/16.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/148/148.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/148/148.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/113/113.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/113/113.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/140/140.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/140/140.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/135/135.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/135/135.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/86/86.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/86/86.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/84/84.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/102/102.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/102/102.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/55/55.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/55/55.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/76/76.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/76/76.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/17/17.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/17/17.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/34/34.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/34/34.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/169/169.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/169/169.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/94/94.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/94/94.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/80/80.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/80/80.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/75/75.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/75/75.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/110/110.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/110/110.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/82/82.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/82/82.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/126/126.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/126/126.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/20/20.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/20/20.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/21/21.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/21/21.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/64/64.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/64/64.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/162/162.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/162/162.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/131/131.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/131/131.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/87/87.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/166/166.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/166/166.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/160/160.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/160/160.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/36/36.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/36/36.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/100/100.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/100/100.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/66/66.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/66/66.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/167/167.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/167/167.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/71/71.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/71/71.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/139/139.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/139/139.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/152/152.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/152/152.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/141/141.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/141/141.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/120/120.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/120/120.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/145/145.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/145/145.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/170/170.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/170/170.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/8/8.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/8/8.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/45/45.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/45/45.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/35/35.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/35/35.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/127/127.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/127/127.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/42/42.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/42/42.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/27/27.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/27/27.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/2/2.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/2/2.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/99/99.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/69/69.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/69/69.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/153/153.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/153/153.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/68/68.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/68/68.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/172/172.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/172/172.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/146/146.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/146/146.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/62/62.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/62/62.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/73/73.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/73/73.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/63/63.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/63/63.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_3_instructions/83/83.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_3_instructions/83/83.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_2_instructions/1/1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_2_instructions/1/1.wgsl"))),
            })
        }
        "tests/OBE_STRONG/2_threads_2_instructions/0/0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE_STRONG/2_threads_2_instructions/0/0.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/95/95.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/95/95.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/93/93.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/93/93.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/38/38.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/38/38.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/91/91.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/31/31.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/31/31.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/4/4.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/61/61.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/61/61.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/92/92.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/92/92.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/48/48.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/48/48.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/14/14.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/52/52.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/52/52.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/29/29.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/29/29.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/56/56.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/56/56.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/86/86.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/86/86.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/84/84.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/55/55.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/55/55.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/34/34.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/34/34.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/94/94.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/94/94.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/80/80.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/80/80.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/82/82.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/82/82.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/64/64.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/64/64.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/36/36.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/36/36.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/10/10.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/10/10.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/100/100.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/100/100.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/66/66.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/66/66.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/72/72.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/99/99.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/13/13.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/46/46.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_4_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_4_instructions/30/30.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_3_instructions/7/7.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_3_instructions/7/7.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_3_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_3_instructions/18/18.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_3_instructions/11/11.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_3_instructions/11/11.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_3_instructions/16/16.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_3_instructions/16/16.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_3_instructions/20/20.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_3_instructions/20/20.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_3_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_3_instructions/3/3.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/3_threads_3_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/3_threads_3_instructions/13/13.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/159/159.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/159/159.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/171/171.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/171/171.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/79/79.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/79/79.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/81/81.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/81/81.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/156/156.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/156/156.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/118/118.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/118/118.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/22/22.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/22/22.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/134/134.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/134/134.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/50/50.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/50/50.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/137/137.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/137/137.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/119/119.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/119/119.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/48/48.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/48/48.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/60/60.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/60/60.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/107/107.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/107/107.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/168/168.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/168/168.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/58/58.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/58/58.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/116/116.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/52/52.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/52/52.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/155/155.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/155/155.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/28/28.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/28/28.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/122/122.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/122/122.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/88/88.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/88/88.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/115/115.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/115/115.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/113/113.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/113/113.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/140/140.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/140/140.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/84/84.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/102/102.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/102/102.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/55/55.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/55/55.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/76/76.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/76/76.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/101/101.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/101/101.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/169/169.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/169/169.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/133/133.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/133/133.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/75/75.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/75/75.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/110/110.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/110/110.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/126/126.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/126/126.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/54/54.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/54/54.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/131/131.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/131/131.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/87/87.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/71/71.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/71/71.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/139/139.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/139/139.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/161/161.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/161/161.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/120/120.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/120/120.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/128/128.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/128/128.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/145/145.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/145/145.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/170/170.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/170/170.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/117/117.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/117/117.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/99/99.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/172/172.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/172/172.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/142/142.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/142/142.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/46/46.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/30/30.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/63/63.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/63/63.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/124/124.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/124/124.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_4_instructions/83/83.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_4_instructions/83/83.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/6/6.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/95/95.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/95/95.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/159/159.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/159/159.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/5/5.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/47/47.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/47/47.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/32/32.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/32/32.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/171/171.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/171/171.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/93/93.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/93/93.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/59/59.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/59/59.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/112/112.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/112/112.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/91/91.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/31/31.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/31/31.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/4/4.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/81/81.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/81/81.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/23/23.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/23/23.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/156/156.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/156/156.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/118/118.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/118/118.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/85/85.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/85/85.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/22/22.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/22/22.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/132/132.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/132/132.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/163/163.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/163/163.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/114/114.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/114/114.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/130/130.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/130/130.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/151/151.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/151/151.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/125/125.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/125/125.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/157/157.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/157/157.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/147/147.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/147/147.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/50/50.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/50/50.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/90/90.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/90/90.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/158/158.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/158/158.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/96/96.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/96/96.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/92/92.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/92/92.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/43/43.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/43/43.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/39/39.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/39/39.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/137/137.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/137/137.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/136/136.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/136/136.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/173/173.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/173/173.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/57/57.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/57/57.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/53/53.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/53/53.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/60/60.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/60/60.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/107/107.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/107/107.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/168/168.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/168/168.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/14/14.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/18/18.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/150/150.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/150/150.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/116/116.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/1/1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/1/1.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/165/165.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/165/165.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/52/52.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/52/52.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/155/155.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/155/155.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/9/9.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/9/9.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/28/28.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/28/28.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/0/0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/0/0.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/129/129.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/129/129.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/174/174.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/174/174.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/144/144.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/144/144.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/154/154.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/154/154.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/88/88.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/88/88.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/103/103.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/103/103.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/175/175.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/175/175.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/29/29.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/29/29.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/16/16.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/16/16.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/148/148.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/148/148.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/113/113.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/113/113.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/56/56.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/56/56.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/140/140.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/140/140.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/105/105.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/105/105.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/135/135.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/135/135.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/86/86.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/86/86.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/84/84.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/102/102.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/102/102.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/55/55.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/55/55.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/76/76.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/76/76.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/17/17.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/17/17.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/101/101.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/101/101.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/34/34.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/34/34.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/169/169.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/169/169.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/94/94.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/94/94.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/80/80.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/80/80.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/12/12.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/12/12.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/75/75.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/75/75.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/110/110.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/110/110.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/82/82.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/82/82.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/89/89.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/89/89.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/126/126.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/126/126.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/20/20.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/20/20.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/21/21.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/21/21.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/64/64.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/64/64.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/54/54.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/54/54.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/162/162.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/162/162.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/131/131.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/131/131.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/87/87.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/166/166.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/166/166.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/160/160.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/160/160.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/36/36.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/36/36.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/100/100.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/100/100.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/66/66.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/66/66.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/167/167.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/167/167.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/71/71.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/71/71.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/139/139.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/139/139.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/104/104.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/104/104.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/152/152.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/152/152.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/161/161.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/161/161.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/141/141.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/141/141.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/120/120.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/120/120.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/128/128.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/128/128.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/123/123.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/123/123.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/97/97.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/97/97.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/145/145.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/145/145.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/170/170.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/170/170.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/8/8.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/8/8.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/19/19.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/19/19.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/45/45.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/45/45.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/72/72.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/35/35.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/35/35.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/127/127.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/127/127.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/42/42.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/42/42.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/27/27.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/27/27.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/78/78.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/78/78.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/2/2.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/2/2.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/99/99.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/149/149.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/149/149.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/13/13.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/69/69.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/69/69.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/153/153.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/153/153.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/68/68.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/68/68.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/172/172.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/172/172.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/142/142.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/142/142.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/108/108.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/108/108.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/146/146.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/146/146.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/46/46.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/62/62.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/62/62.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/73/73.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/73/73.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/63/63.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/63/63.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/124/124.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/124/124.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_3_instructions/83/83.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_3_instructions/83/83.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_2_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_2_instructions/6/6.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_2_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_2_instructions/5/5.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_2_instructions/1/1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_2_instructions/1/1.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_2_instructions/0/0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_2_instructions/0/0.wgsl"))),
            })
        }
        "tests/LOBE_STRONG/2_threads_2_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/LOBE_STRONG/2_threads_2_instructions/3/3.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/95/95.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/95/95.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/93/93.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/93/93.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/38/38.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/38/38.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/91/91.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/31/31.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/31/31.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/4/4.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/92/92.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/92/92.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/14/14.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/29/29.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/29/29.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/86/86.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/86/86.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/84/84.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/94/94.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/94/94.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/80/80.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/80/80.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/82/82.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/82/82.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/36/36.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/36/36.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/10/10.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/10/10.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/100/100.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/100/100.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/72/72.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/99/99.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/13/13.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_4_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_4_instructions/30/30.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_3_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_3_instructions/18/18.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_3_instructions/11/11.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_3_instructions/11/11.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_3_instructions/20/20.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_3_instructions/20/20.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_3_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_3_instructions/3/3.wgsl"))),
            })
        }
        "tests/HSA_OBE/3_threads_3_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/3_threads_3_instructions/13/13.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/159/159.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/159/159.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/79/79.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/79/79.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/81/81.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/81/81.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/156/156.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/156/156.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/118/118.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/118/118.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/134/134.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/134/134.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/137/137.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/137/137.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/119/119.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/119/119.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/48/48.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/48/48.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/60/60.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/60/60.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/107/107.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/107/107.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/116/116.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/155/155.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/155/155.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/28/28.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/28/28.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/122/122.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/122/122.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/113/113.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/113/113.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/84/84.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/102/102.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/102/102.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/76/76.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/76/76.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/133/133.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/133/133.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/75/75.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/75/75.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/126/126.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/126/126.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/87/87.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/71/71.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/71/71.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/139/139.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/139/139.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/161/161.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/161/161.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/120/120.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/120/120.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/117/117.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/117/117.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/99/99.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/46/46.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/30/30.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_4_instructions/124/124.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_4_instructions/124/124.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/159/159.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/159/159.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/47/47.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/47/47.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/112/112.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/112/112.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/91/91.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/4/4.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/81/81.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/81/81.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/22/22.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/22/22.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/132/132.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/132/132.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/151/151.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/151/151.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/157/157.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/157/157.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/50/50.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/50/50.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/90/90.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/90/90.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/158/158.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/158/158.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/136/136.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/136/136.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/60/60.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/60/60.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/168/168.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/168/168.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/14/14.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/18/18.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/116/116.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/9/9.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/9/9.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/103/103.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/103/103.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/175/175.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/175/175.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/148/148.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/148/148.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/56/56.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/56/56.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/105/105.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/105/105.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/17/17.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/17/17.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/101/101.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/101/101.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/12/12.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/12/12.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/89/89.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/89/89.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/126/126.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/126/126.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/131/131.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/131/131.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/87/87.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/166/166.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/166/166.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/160/160.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/160/160.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/167/167.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/167/167.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/161/161.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/161/161.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/128/128.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/128/128.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/123/123.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/123/123.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/19/19.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/19/19.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/72/72.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/42/42.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/42/42.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/149/149.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/149/149.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/13/13.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/142/142.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/142/142.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/108/108.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/108/108.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/146/146.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/146/146.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/46/46.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/62/62.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/62/62.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/63/63.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/63/63.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_3_instructions/124/124.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_3_instructions/124/124.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_2_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_2_instructions/6/6.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_2_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_2_instructions/5/5.wgsl"))),
            })
        }
        "tests/HSA_OBE/2_threads_2_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_OBE/2_threads_2_instructions/3/3.wgsl"))),
            })
        }
        "tests/OBE/2_threads_4_instructions/118/118.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_4_instructions/118/118.wgsl"))),
            })
        }
        "tests/OBE/2_threads_4_instructions/119/119.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_4_instructions/119/119.wgsl"))),
            })
        }
        "tests/OBE/2_threads_4_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_4_instructions/116/116.wgsl"))),
            })
        }
        "tests/OBE/2_threads_4_instructions/117/117.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_4_instructions/117/117.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/159/159.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/159/159.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/47/47.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/47/47.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/112/112.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/112/112.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/4/4.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/22/22.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/22/22.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/157/157.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/157/157.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/168/168.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/168/168.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/9/9.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/9/9.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/148/148.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/148/148.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/17/17.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/17/17.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/126/126.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/126/126.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/131/131.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/131/131.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/87/87.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/166/166.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/166/166.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/160/160.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/160/160.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/167/167.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/167/167.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/42/42.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/42/42.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/146/146.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/146/146.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/62/62.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/62/62.wgsl"))),
            })
        }
        "tests/OBE/2_threads_3_instructions/63/63.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/OBE/2_threads_3_instructions/63/63.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/93/93.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/93/93.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/38/38.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/38/38.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/91/91.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/4/4.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/92/92.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/92/92.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/14/14.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/29/29.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/29/29.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/86/86.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/86/86.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/84/84.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/94/94.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/94/94.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/80/80.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/80/80.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/82/82.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/82/82.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/36/36.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/36/36.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/10/10.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/10/10.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/100/100.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/100/100.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/72/72.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/99/99.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_4_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_4_instructions/30/30.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_3_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_3_instructions/18/18.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_3_instructions/11/11.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_3_instructions/11/11.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_3_instructions/20/20.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_3_instructions/20/20.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_3_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_3_instructions/3/3.wgsl"))),
            })
        }
        "tests/HSA_STRONG/3_threads_3_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/3_threads_3_instructions/13/13.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/159/159.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/159/159.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/171/171.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/171/171.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/156/156.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/156/156.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/118/118.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/118/118.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/134/134.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/134/134.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/137/137.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/137/137.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/48/48.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/48/48.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/107/107.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/107/107.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/168/168.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/168/168.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/116/116.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/155/155.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/155/155.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/28/28.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/28/28.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/122/122.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/122/122.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/84/84.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/102/102.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/102/102.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/76/76.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/76/76.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/169/169.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/169/169.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/133/133.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/133/133.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/75/75.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/75/75.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/126/126.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/126/126.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/87/87.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/139/139.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/139/139.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/120/120.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/120/120.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/170/170.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/170/170.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/99/99.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/172/172.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/172/172.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/142/142.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/142/142.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/46/46.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/30/30.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_4_instructions/124/124.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_4_instructions/124/124.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/6/6.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/95/95.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/95/95.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/5/5.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/47/47.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/47/47.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/32/32.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/32/32.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/171/171.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/171/171.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/93/93.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/93/93.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/59/59.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/59/59.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/91/91.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/31/31.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/31/31.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/4/4.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/81/81.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/81/81.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/23/23.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/23/23.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/156/156.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/156/156.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/118/118.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/118/118.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/85/85.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/85/85.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/132/132.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/132/132.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/163/163.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/163/163.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/114/114.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/114/114.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/130/130.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/130/130.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/151/151.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/151/151.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/125/125.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/125/125.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/147/147.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/147/147.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/50/50.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/50/50.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/90/90.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/90/90.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/158/158.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/158/158.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/96/96.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/96/96.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/92/92.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/92/92.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/43/43.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/43/43.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/39/39.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/39/39.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/137/137.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/137/137.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/136/136.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/136/136.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/173/173.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/173/173.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/57/57.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/57/57.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/53/53.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/53/53.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/60/60.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/60/60.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/107/107.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/107/107.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/168/168.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/168/168.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/14/14.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/18/18.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/150/150.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/150/150.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/116/116.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/1/1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/1/1.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/165/165.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/165/165.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/52/52.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/52/52.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/155/155.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/155/155.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/28/28.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/28/28.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/0/0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/0/0.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/129/129.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/129/129.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/174/174.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/174/174.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/144/144.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/144/144.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/154/154.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/154/154.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/88/88.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/88/88.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/103/103.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/103/103.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/175/175.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/175/175.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/29/29.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/29/29.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/16/16.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/16/16.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/148/148.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/148/148.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/113/113.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/113/113.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/56/56.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/56/56.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/140/140.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/140/140.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/105/105.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/105/105.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/135/135.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/135/135.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/86/86.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/86/86.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/84/84.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/102/102.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/102/102.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/55/55.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/55/55.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/76/76.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/76/76.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/17/17.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/17/17.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/101/101.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/101/101.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/34/34.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/34/34.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/169/169.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/169/169.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/94/94.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/94/94.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/80/80.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/80/80.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/12/12.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/12/12.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/75/75.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/75/75.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/110/110.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/110/110.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/82/82.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/82/82.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/89/89.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/89/89.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/20/20.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/20/20.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/21/21.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/21/21.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/64/64.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/64/64.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/54/54.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/54/54.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/162/162.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/162/162.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/131/131.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/131/131.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/166/166.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/166/166.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/160/160.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/160/160.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/36/36.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/36/36.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/100/100.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/100/100.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/66/66.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/66/66.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/71/71.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/71/71.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/139/139.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/139/139.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/104/104.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/104/104.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/152/152.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/152/152.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/161/161.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/161/161.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/141/141.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/141/141.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/120/120.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/120/120.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/128/128.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/128/128.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/123/123.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/123/123.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/97/97.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/97/97.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/145/145.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/145/145.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/170/170.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/170/170.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/8/8.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/8/8.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/19/19.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/19/19.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/45/45.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/45/45.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/72/72.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/35/35.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/35/35.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/127/127.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/127/127.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/27/27.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/27/27.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/78/78.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/78/78.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/2/2.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/2/2.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/99/99.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/149/149.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/149/149.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/13/13.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/69/69.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/69/69.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/153/153.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/153/153.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/68/68.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/68/68.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/172/172.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/172/172.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/142/142.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/142/142.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/108/108.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/108/108.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/146/146.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/146/146.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/46/46.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/62/62.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/62/62.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/73/73.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/73/73.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/124/124.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/124/124.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_3_instructions/83/83.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_3_instructions/83/83.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_2_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_2_instructions/6/6.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_2_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_2_instructions/5/5.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_2_instructions/1/1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_2_instructions/1/1.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_2_instructions/0/0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_2_instructions/0/0.wgsl"))),
            })
        }
        "tests/HSA_STRONG/2_threads_2_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA_STRONG/2_threads_2_instructions/3/3.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/6/6.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/15/15.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/15/15.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/95/95.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/95/95.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/5/5.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/47/47.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/47/47.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/32/32.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/32/32.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/25/25.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/25/25.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/93/93.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/93/93.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/79/79.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/79/79.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/38/38.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/38/38.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/59/59.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/59/59.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/91/91.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/31/31.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/31/31.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/4/4.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/81/81.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/81/81.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/23/23.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/23/23.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/7/7.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/7/7.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/85/85.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/85/85.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/22/22.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/22/22.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/33/33.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/33/33.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/44/44.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/44/44.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/65/65.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/65/65.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/40/40.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/40/40.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/50/50.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/50/50.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/90/90.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/90/90.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/96/96.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/96/96.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/61/61.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/61/61.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/92/92.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/92/92.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/43/43.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/43/43.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/39/39.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/39/39.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/24/24.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/24/24.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/48/48.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/48/48.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/57/57.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/57/57.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/53/53.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/53/53.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/60/60.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/60/60.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/14/14.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/26/26.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/26/26.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/70/70.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/70/70.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/18/18.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/58/58.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/58/58.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/37/37.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/37/37.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/1/1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/1/1.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/52/52.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/52/52.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/9/9.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/9/9.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/28/28.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/28/28.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/0/0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/0/0.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/74/74.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/74/74.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/88/88.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/88/88.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/103/103.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/103/103.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/11/11.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/11/11.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/29/29.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/29/29.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/16/16.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/16/16.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/56/56.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/56/56.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/51/51.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/51/51.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/86/86.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/86/86.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/84/84.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/102/102.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/102/102.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/55/55.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/55/55.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/76/76.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/76/76.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/17/17.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/17/17.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/101/101.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/101/101.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/34/34.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/34/34.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/94/94.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/94/94.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/80/80.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/80/80.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/12/12.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/12/12.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/75/75.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/75/75.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/82/82.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/82/82.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/89/89.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/89/89.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/20/20.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/20/20.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/21/21.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/21/21.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/3/3.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/64/64.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/64/64.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/54/54.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/54/54.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/41/41.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/41/41.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/87/87.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/98/98.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/98/98.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/36/36.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/36/36.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/10/10.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/10/10.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/100/100.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/100/100.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/66/66.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/66/66.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/77/77.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/77/77.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/71/71.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/71/71.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/104/104.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/104/104.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/97/97.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/97/97.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/8/8.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/8/8.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/19/19.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/19/19.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/45/45.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/45/45.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/72/72.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/35/35.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/35/35.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/42/42.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/42/42.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/27/27.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/27/27.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/78/78.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/78/78.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/2/2.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/2/2.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/49/49.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/49/49.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/99/99.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/13/13.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/69/69.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/69/69.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/68/68.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/68/68.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/46/46.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/30/30.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/62/62.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/62/62.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/73/73.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/73/73.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/67/67.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/67/67.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/63/63.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/63/63.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_4_instructions/83/83.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_4_instructions/83/83.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/6/6.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/15/15.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/15/15.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/5/5.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/4/4.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/7/7.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/7/7.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/14/14.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/18/18.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/1/1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/1/1.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/9/9.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/9/9.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/0/0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/0/0.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/11/11.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/11/11.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/16/16.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/16/16.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/17/17.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/17/17.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/12/12.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/12/12.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/20/20.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/20/20.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/3/3.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/10/10.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/10/10.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/8/8.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/8/8.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/19/19.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/19/19.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/2/2.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/2/2.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/3_threads_3_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/3_threads_3_instructions/13/13.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/6/6.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/15/15.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/15/15.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/95/95.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/95/95.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/159/159.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/159/159.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/5/5.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/47/47.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/47/47.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/32/32.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/32/32.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/25/25.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/25/25.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/171/171.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/171/171.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/93/93.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/93/93.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/79/79.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/79/79.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/38/38.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/38/38.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/59/59.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/59/59.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/112/112.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/112/112.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/91/91.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/31/31.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/31/31.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/4/4.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/81/81.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/81/81.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/23/23.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/23/23.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/121/121.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/121/121.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/7/7.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/7/7.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/156/156.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/156/156.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/118/118.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/118/118.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/85/85.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/85/85.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/22/22.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/22/22.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/132/132.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/132/132.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/33/33.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/33/33.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/163/163.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/163/163.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/114/114.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/114/114.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/130/130.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/130/130.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/134/134.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/134/134.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/151/151.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/151/151.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/44/44.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/44/44.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/65/65.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/65/65.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/125/125.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/125/125.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/157/157.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/157/157.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/40/40.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/40/40.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/147/147.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/147/147.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/50/50.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/50/50.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/90/90.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/90/90.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/158/158.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/158/158.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/96/96.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/96/96.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/61/61.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/61/61.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/92/92.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/92/92.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/43/43.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/43/43.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/39/39.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/39/39.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/137/137.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/137/137.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/119/119.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/119/119.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/136/136.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/136/136.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/24/24.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/24/24.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/138/138.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/138/138.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/48/48.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/48/48.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/57/57.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/57/57.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/53/53.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/53/53.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/60/60.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/60/60.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/107/107.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/107/107.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/168/168.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/168/168.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/14/14.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/26/26.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/26/26.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/70/70.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/70/70.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/18/18.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/58/58.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/58/58.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/164/164.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/164/164.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/150/150.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/150/150.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/116/116.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/37/37.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/37/37.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/1/1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/1/1.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/165/165.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/165/165.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/52/52.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/52/52.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/155/155.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/155/155.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/9/9.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/9/9.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/28/28.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/28/28.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/0/0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/0/0.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/129/129.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/129/129.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/122/122.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/122/122.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/74/74.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/74/74.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/144/144.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/144/144.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/154/154.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/154/154.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/88/88.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/88/88.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/106/106.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/106/106.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/103/103.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/103/103.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/11/11.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/11/11.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/29/29.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/29/29.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/16/16.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/16/16.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/115/115.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/115/115.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/148/148.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/148/148.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/113/113.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/113/113.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/56/56.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/56/56.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/140/140.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/140/140.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/51/51.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/51/51.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/105/105.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/105/105.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/109/109.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/109/109.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/135/135.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/135/135.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/86/86.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/86/86.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/143/143.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/143/143.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/84/84.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/102/102.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/102/102.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/55/55.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/55/55.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/76/76.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/76/76.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/17/17.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/17/17.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/101/101.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/101/101.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/34/34.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/34/34.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/169/169.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/169/169.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/133/133.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/133/133.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/94/94.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/94/94.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/80/80.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/80/80.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/12/12.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/12/12.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/75/75.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/75/75.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/110/110.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/110/110.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/82/82.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/82/82.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/89/89.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/89/89.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/126/126.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/126/126.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/20/20.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/20/20.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/21/21.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/21/21.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/3/3.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/64/64.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/64/64.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/54/54.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/54/54.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/162/162.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/162/162.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/131/131.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/131/131.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/41/41.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/41/41.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/87/87.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/166/166.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/166/166.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/160/160.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/160/160.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/98/98.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/98/98.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/36/36.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/36/36.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/10/10.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/10/10.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/100/100.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/100/100.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/66/66.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/66/66.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/77/77.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/77/77.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/167/167.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/167/167.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/71/71.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/71/71.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/139/139.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/139/139.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/104/104.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/104/104.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/152/152.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/152/152.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/161/161.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/161/161.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/141/141.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/141/141.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/120/120.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/120/120.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/128/128.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/128/128.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/123/123.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/123/123.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/97/97.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/97/97.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/145/145.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/145/145.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/170/170.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/170/170.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/8/8.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/8/8.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/19/19.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/19/19.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/45/45.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/45/45.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/72/72.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/35/35.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/35/35.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/127/127.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/127/127.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/42/42.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/42/42.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/27/27.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/27/27.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/78/78.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/78/78.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/117/117.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/117/117.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/2/2.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/2/2.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/49/49.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/49/49.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/111/111.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/111/111.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/99/99.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/149/149.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/149/149.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/13/13.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/69/69.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/69/69.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/153/153.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/153/153.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/68/68.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/68/68.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/172/172.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/172/172.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/142/142.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/142/142.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/108/108.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/108/108.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/146/146.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/146/146.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/46/46.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/30/30.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/62/62.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/62/62.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/73/73.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/73/73.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/67/67.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/67/67.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/63/63.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/63/63.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/124/124.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/124/124.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_4_instructions/83/83.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_4_instructions/83/83.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/6/6.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/15/15.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/15/15.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/95/95.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/95/95.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/159/159.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/159/159.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/5/5.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/47/47.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/47/47.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/32/32.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/32/32.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/25/25.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/25/25.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/171/171.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/171/171.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/93/93.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/93/93.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/79/79.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/79/79.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/38/38.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/38/38.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/59/59.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/59/59.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/112/112.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/112/112.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/91/91.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/31/31.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/31/31.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/4/4.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/81/81.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/81/81.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/23/23.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/23/23.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/121/121.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/121/121.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/7/7.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/7/7.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/156/156.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/156/156.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/118/118.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/118/118.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/85/85.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/85/85.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/22/22.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/22/22.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/132/132.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/132/132.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/33/33.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/33/33.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/163/163.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/163/163.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/114/114.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/114/114.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/130/130.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/130/130.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/134/134.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/134/134.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/151/151.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/151/151.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/44/44.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/44/44.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/65/65.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/65/65.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/125/125.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/125/125.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/157/157.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/157/157.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/40/40.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/40/40.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/147/147.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/147/147.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/50/50.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/50/50.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/90/90.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/90/90.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/158/158.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/158/158.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/96/96.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/96/96.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/61/61.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/61/61.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/92/92.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/92/92.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/43/43.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/43/43.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/39/39.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/39/39.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/137/137.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/137/137.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/119/119.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/119/119.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/136/136.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/136/136.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/24/24.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/24/24.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/138/138.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/138/138.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/48/48.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/48/48.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/173/173.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/173/173.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/57/57.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/57/57.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/53/53.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/53/53.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/60/60.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/60/60.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/107/107.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/107/107.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/168/168.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/168/168.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/14/14.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/26/26.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/26/26.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/70/70.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/70/70.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/18/18.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/58/58.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/58/58.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/164/164.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/164/164.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/150/150.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/150/150.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/116/116.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/37/37.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/37/37.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/1/1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/1/1.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/165/165.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/165/165.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/52/52.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/52/52.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/155/155.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/155/155.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/9/9.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/9/9.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/28/28.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/28/28.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/0/0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/0/0.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/129/129.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/129/129.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/122/122.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/122/122.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/74/74.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/74/74.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/174/174.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/174/174.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/144/144.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/144/144.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/154/154.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/154/154.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/88/88.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/88/88.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/106/106.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/106/106.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/103/103.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/103/103.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/175/175.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/175/175.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/11/11.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/11/11.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/29/29.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/29/29.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/16/16.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/16/16.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/115/115.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/115/115.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/148/148.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/148/148.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/113/113.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/113/113.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/56/56.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/56/56.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/140/140.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/140/140.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/51/51.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/51/51.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/105/105.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/105/105.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/109/109.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/109/109.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/135/135.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/135/135.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/86/86.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/86/86.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/143/143.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/143/143.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/84/84.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/102/102.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/102/102.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/55/55.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/55/55.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/76/76.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/76/76.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/17/17.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/17/17.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/101/101.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/101/101.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/34/34.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/34/34.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/169/169.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/169/169.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/133/133.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/133/133.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/94/94.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/94/94.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/80/80.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/80/80.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/12/12.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/12/12.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/75/75.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/75/75.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/110/110.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/110/110.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/82/82.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/82/82.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/89/89.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/89/89.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/126/126.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/126/126.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/20/20.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/20/20.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/21/21.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/21/21.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/3/3.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/64/64.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/64/64.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/54/54.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/54/54.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/162/162.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/162/162.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/131/131.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/131/131.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/41/41.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/41/41.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/87/87.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/166/166.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/166/166.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/160/160.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/160/160.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/98/98.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/98/98.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/36/36.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/36/36.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/10/10.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/10/10.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/100/100.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/100/100.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/66/66.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/66/66.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/77/77.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/77/77.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/167/167.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/167/167.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/71/71.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/71/71.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/139/139.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/139/139.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/104/104.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/104/104.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/152/152.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/152/152.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/161/161.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/161/161.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/141/141.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/141/141.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/120/120.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/120/120.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/128/128.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/128/128.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/123/123.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/123/123.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/97/97.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/97/97.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/145/145.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/145/145.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/170/170.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/170/170.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/8/8.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/8/8.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/19/19.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/19/19.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/45/45.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/45/45.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/72/72.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/35/35.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/35/35.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/127/127.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/127/127.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/42/42.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/42/42.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/27/27.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/27/27.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/78/78.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/78/78.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/117/117.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/117/117.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/2/2.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/2/2.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/49/49.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/49/49.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/111/111.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/111/111.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/99/99.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/149/149.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/149/149.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/13/13.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/69/69.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/69/69.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/153/153.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/153/153.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/68/68.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/68/68.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/172/172.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/172/172.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/142/142.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/142/142.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/108/108.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/108/108.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/146/146.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/146/146.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/46/46.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/30/30.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/62/62.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/62/62.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/73/73.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/73/73.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/67/67.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/67/67.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/63/63.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/63/63.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/124/124.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/124/124.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_3_instructions/83/83.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_3_instructions/83/83.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_2_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_2_instructions/6/6.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_2_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_2_instructions/5/5.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_2_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_2_instructions/4/4.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_2_instructions/7/7.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_2_instructions/7/7.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_2_instructions/1/1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_2_instructions/1/1.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_2_instructions/0/0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_2_instructions/0/0.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_2_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_2_instructions/3/3.wgsl"))),
            })
        }
        "tests/STRONG_FAIR/2_threads_2_instructions/2/2.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/STRONG_FAIR/2_threads_2_instructions/2/2.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/6/6.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/15/15.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/15/15.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/95/95.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/95/95.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/5/5.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/47/47.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/47/47.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/32/32.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/32/32.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/25/25.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/25/25.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/93/93.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/93/93.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/79/79.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/79/79.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/38/38.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/38/38.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/59/59.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/59/59.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/91/91.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/31/31.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/31/31.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/4/4.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/81/81.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/81/81.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/23/23.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/23/23.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/7/7.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/7/7.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/85/85.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/85/85.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/22/22.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/22/22.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/33/33.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/33/33.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/44/44.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/44/44.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/65/65.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/65/65.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/40/40.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/40/40.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/50/50.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/50/50.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/90/90.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/90/90.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/96/96.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/96/96.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/61/61.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/61/61.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/92/92.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/92/92.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/43/43.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/43/43.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/39/39.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/39/39.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/24/24.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/24/24.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/48/48.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/48/48.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/57/57.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/57/57.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/53/53.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/53/53.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/60/60.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/60/60.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/14/14.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/26/26.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/26/26.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/70/70.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/70/70.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/58/58.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/58/58.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/37/37.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/37/37.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/1/1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/1/1.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/52/52.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/52/52.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/9/9.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/9/9.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/28/28.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/28/28.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/0/0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/0/0.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/74/74.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/74/74.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/88/88.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/88/88.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/103/103.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/103/103.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/11/11.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/11/11.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/29/29.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/29/29.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/16/16.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/16/16.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/56/56.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/56/56.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/51/51.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/51/51.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/86/86.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/86/86.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/84/84.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/102/102.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/102/102.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/55/55.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/55/55.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/76/76.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/76/76.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/17/17.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/17/17.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/101/101.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/101/101.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/34/34.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/34/34.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/94/94.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/94/94.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/80/80.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/80/80.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/12/12.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/12/12.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/75/75.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/75/75.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/82/82.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/82/82.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/89/89.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/89/89.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/21/21.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/21/21.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/3/3.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/64/64.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/64/64.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/54/54.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/54/54.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/41/41.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/41/41.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/87/87.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/98/98.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/98/98.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/36/36.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/36/36.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/10/10.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/10/10.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/100/100.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/100/100.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/66/66.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/66/66.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/77/77.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/77/77.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/71/71.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/71/71.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/104/104.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/104/104.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/97/97.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/97/97.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/8/8.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/8/8.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/45/45.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/45/45.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/72/72.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/35/35.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/35/35.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/42/42.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/42/42.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/27/27.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/27/27.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/78/78.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/78/78.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/2/2.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/2/2.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/49/49.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/49/49.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/99/99.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/13/13.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/69/69.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/69/69.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/68/68.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/68/68.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/46/46.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/30/30.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/62/62.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/62/62.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/73/73.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/73/73.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/67/67.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/67/67.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/63/63.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/63/63.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_4_instructions/83/83.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_4_instructions/83/83.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/6/6.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/15/15.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/15/15.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/5/5.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/4/4.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/7/7.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/7/7.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/14/14.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/18/18.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/1/1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/1/1.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/9/9.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/9/9.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/0/0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/0/0.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/11/11.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/11/11.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/16/16.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/16/16.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/17/17.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/17/17.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/12/12.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/12/12.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/20/20.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/20/20.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/3/3.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/10/10.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/10/10.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/8/8.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/8/8.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/19/19.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/19/19.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/2/2.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/2/2.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/3_threads_3_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/3_threads_3_instructions/13/13.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/95/95.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/95/95.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/159/159.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/159/159.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/47/47.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/47/47.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/25/25.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/25/25.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/93/93.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/93/93.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/79/79.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/79/79.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/38/38.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/38/38.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/112/112.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/112/112.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/31/31.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/31/31.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/81/81.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/81/81.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/121/121.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/121/121.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/156/156.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/156/156.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/118/118.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/118/118.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/85/85.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/85/85.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/132/132.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/132/132.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/134/134.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/134/134.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/44/44.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/44/44.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/65/65.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/65/65.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/125/125.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/125/125.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/157/157.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/157/157.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/40/40.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/40/40.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/147/147.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/147/147.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/158/158.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/158/158.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/96/96.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/96/96.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/61/61.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/61/61.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/92/92.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/92/92.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/43/43.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/43/43.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/39/39.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/39/39.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/137/137.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/137/137.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/119/119.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/119/119.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/136/136.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/136/136.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/24/24.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/24/24.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/138/138.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/138/138.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/48/48.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/48/48.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/60/60.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/60/60.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/107/107.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/107/107.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/26/26.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/26/26.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/70/70.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/70/70.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/164/164.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/164/164.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/116/116.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/37/37.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/37/37.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/1/1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/1/1.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/165/165.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/165/165.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/155/155.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/155/155.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/9/9.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/9/9.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/28/28.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/28/28.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/0/0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/0/0.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/122/122.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/122/122.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/74/74.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/74/74.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/154/154.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/154/154.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/106/106.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/106/106.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/103/103.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/103/103.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/11/11.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/11/11.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/29/29.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/29/29.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/148/148.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/148/148.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/113/113.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/113/113.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/105/105.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/105/105.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/109/109.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/109/109.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/135/135.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/135/135.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/86/86.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/86/86.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/84/84.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/84/84.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/102/102.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/102/102.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/76/76.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/76/76.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/34/34.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/34/34.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/133/133.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/133/133.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/94/94.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/94/94.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/80/80.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/80/80.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/75/75.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/75/75.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/126/126.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/126/126.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/3/3.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/64/64.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/64/64.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/41/41.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/41/41.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/87/87.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/166/166.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/166/166.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/160/160.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/160/160.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/98/98.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/98/98.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/36/36.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/36/36.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/10/10.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/10/10.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/66/66.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/66/66.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/77/77.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/77/77.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/167/167.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/167/167.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/71/71.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/71/71.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/139/139.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/139/139.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/104/104.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/104/104.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/152/152.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/152/152.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/161/161.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/161/161.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/120/120.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/120/120.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/123/123.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/123/123.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/97/97.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/97/97.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/8/8.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/8/8.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/45/45.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/45/45.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/35/35.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/35/35.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/127/127.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/127/127.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/42/42.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/42/42.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/27/27.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/27/27.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/78/78.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/78/78.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/117/117.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/117/117.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/2/2.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/2/2.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/49/49.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/49/49.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/99/99.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/99/99.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/149/149.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/149/149.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/69/69.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/69/69.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/153/153.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/153/153.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/68/68.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/68/68.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/108/108.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/108/108.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/146/146.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/146/146.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/46/46.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/30/30.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/67/67.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/67/67.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_4_instructions/124/124.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_4_instructions/124/124.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/159/159.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/159/159.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/47/47.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/47/47.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/25/25.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/25/25.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/79/79.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/79/79.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/38/38.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/38/38.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/112/112.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/112/112.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/91/91.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/91/91.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/4/4.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/81/81.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/81/81.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/121/121.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/121/121.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/7/7.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/7/7.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/22/22.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/22/22.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/132/132.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/132/132.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/33/33.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/33/33.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/134/134.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/134/134.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/151/151.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/151/151.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/44/44.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/44/44.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/65/65.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/65/65.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/157/157.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/157/157.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/50/50.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/50/50.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/90/90.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/90/90.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/158/158.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/158/158.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/61/61.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/61/61.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/119/119.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/119/119.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/136/136.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/136/136.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/24/24.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/24/24.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/138/138.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/138/138.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/48/48.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/48/48.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/60/60.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/60/60.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/168/168.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/168/168.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/14/14.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/14/14.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/26/26.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/26/26.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/70/70.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/70/70.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/18/18.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/18/18.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/58/58.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/58/58.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/164/164.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/164/164.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/116/116.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/116/116.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/37/37.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/37/37.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/9/9.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/9/9.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/74/74.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/74/74.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/106/106.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/106/106.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/103/103.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/103/103.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/175/175.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/175/175.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/148/148.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/148/148.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/56/56.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/56/56.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/51/51.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/51/51.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/105/105.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/105/105.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/109/109.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/109/109.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/143/143.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/143/143.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/17/17.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/17/17.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/101/101.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/101/101.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/133/133.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/133/133.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/12/12.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/12/12.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/89/89.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/89/89.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/126/126.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/126/126.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/3/3.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/131/131.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/131/131.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/41/41.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/41/41.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/87/87.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/87/87.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/166/166.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/166/166.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/160/160.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/160/160.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/98/98.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/98/98.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/10/10.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/10/10.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/77/77.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/77/77.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/167/167.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/167/167.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/161/161.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/161/161.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/128/128.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/128/128.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/123/123.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/123/123.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/19/19.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/19/19.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/72/72.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/72/72.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/42/42.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/42/42.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/117/117.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/117/117.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/49/49.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/49/49.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/149/149.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/149/149.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/13/13.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/13/13.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/142/142.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/142/142.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/108/108.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/108/108.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/146/146.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/146/146.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/46/46.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/46/46.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/30/30.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/30/30.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/62/62.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/62/62.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/67/67.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/67/67.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/63/63.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/63/63.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_3_instructions/124/124.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_3_instructions/124/124.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_2_instructions/6/6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_2_instructions/6/6.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_2_instructions/5/5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_2_instructions/5/5.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_2_instructions/4/4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_2_instructions/4/4.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_2_instructions/7/7.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_2_instructions/7/7.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_2_instructions/3/3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_2_instructions/3/3.wgsl"))),
            })
        }
        "tests/WEAK_FAIR/2_threads_2_instructions/2/2.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/WEAK_FAIR/2_threads_2_instructions/2/2.wgsl"))),
            })
        } 
        test => {
            panic!("Unknown test: {test_name}", test_name=test);
        }
    };
    
    let threads_finished: i32 = 0;
    let mem_0: i32 = 0;
    let mem_1: i32 = 0;
    let mem_2: i32 = 0;

    let data_in = [threads_finished, mem_0, mem_1, mem_2];

    let size = std::mem::size_of_val(&data_in) as wgpu::BufferAddress;

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(&data_in),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });

    // Instantiates the pipeline.
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &cs_module,
        entry_point: "main",
    });

    let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage_buffer.as_entire_binding(),
        }],
    });
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: None,
            timestamp_writes: None,
        });
        cpass.set_pipeline(&compute_pipeline);
        cpass.set_bind_group(0, &bind_group, &[]);
        cpass.insert_debug_marker("compute collatz iterations");
        cpass.dispatch_workgroups(num_threads as u32, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }

    encoder.copy_buffer_to_buffer(&storage_buffer, 0, &staging_buffer, 0, size);
    queue.submit(Some(encoder.finish()));
    let buffer_slice = staging_buffer.slice(..);
    let (sender, receiver) = flume::bounded(1);
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    device.poll(wgpu::Maintain::wait()).panic_on_timeout();
    if let Ok(Ok(())) = receiver.recv_async().await {
        // Gets contents of buffer
        let data = buffer_slice.get_mapped_range();
        let result = bytemuck::cast_slice(&data).to_vec();
        drop(data);
        staging_buffer.unmap(); // Unmaps buffer from memory
                                // If you are familiar with C++ these 2 lines can be thought of similarly to:
                                //   delete myPointer;
                                //   myPointer = NULL;
                                // It effectively frees the memory
        Some(result[0])
    } else {
        info!("GPU timeout, test failed!");
        Some(0)
    }
}

#[wasm_bindgen]
pub async fn get_gpu_name() -> Option<String> {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("No suitable GPU adapters found on the system!");
    info!("GPU adapter: {}", adapter.get_info().name);
    Some(adapter.get_info().name)
}
