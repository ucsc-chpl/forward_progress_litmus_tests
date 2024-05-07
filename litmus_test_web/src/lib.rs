use std::{borrow::Cow, str::FromStr};
use wgpu::util::DeviceExt;

use wasm_bindgen::prelude::*;
use std::env;
use std::fs;


// this lad will be called from the javascript
//refactor later
#[wasm_bindgen]
pub async fn run(num_threads: u32, kernel_file: &str) -> u32 {
    
    println!("program started");
    #[cfg(target_arch = "wasm32")]
    log::info!("program started, running kernel");

    let threads_finished = execute_gpu(num_threads, kernel_file).await.unwrap();

    println!("execute_gpu finished");
    let disp_steps: String = threads_finished.to_string();

    println!("Threads finished: {}", disp_steps);
    #[cfg(target_arch = "wasm32")]
    log::info!("Threads finished: {}", disp_steps);
    return threads_finished;
}

#[wasm_bindgen]
pub async fn execute_gpu(num_threads: u32, kernel_file: &str) -> Option<u32> {
    log::info!("got into exec gpu");
    println!("got into exec gpu");
    let instance = wgpu::Instance::default();
    log::info!("got instance");
    println!("got instance");
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
    log::info!("running test on {}", adapter.get_info().name);
    println!("got adapter");
    //execute_gpu_inner(&device, &queue).await
    let cs_module: wgpu::ShaderModule = match kernel_file {
        "2_2_0.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_0.wgsl"))),
            })
        }
        "2_2_1.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_1.wgsl"))),
            })
        }
        "2_2_2.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_2.wgsl"))),
            })
        }
        "2_2_3.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_3.wgsl"))),
            })
        }
        "2_2_4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_4.wgsl"))),
            })
        }
        "2_2_5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_5.wgsl"))),
            })
        }
        "2_2_6.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_6.wgsl"))),
            })
        }
        "2_2_7.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_7.wgsl"))),
            })
        }
        _ => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_5.wgsl"))),
            })
        }
    };
    
    let dummy: i32 = 0;
    //println!("got shader");
    // Gets the size in bytes of the buffer.
    let size = std::mem::size_of_val(&dummy) as wgpu::BufferAddress;

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    //println!("instantiated buffer");
    let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(&[dummy]),
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
        //constants: &Default::default(),
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
    //println!("bind group created");
    // A command encoder executes one or many pipelines.
    // It is to WebGPU what a command buffer is to Vulkan.
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
        //dispatch 2 threads
        cpass.dispatch_workgroups(num_threads as u32, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }
    //println!("encoder?");

    encoder.copy_buffer_to_buffer(&storage_buffer, 0, &staging_buffer, 0, size);
    //println!("copied to gpu");
    // Submits command encoder for processing
    queue.submit(Some(encoder.finish()));
    //println!("submitted to queue");
    // Note that we're not calling `.await` here.
    let buffer_slice = staging_buffer.slice(..);
    //println!("buffer slice?");
    // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
    let (sender, receiver) = flume::bounded(1);
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());
    //println!("buffer slice do something");
    // Poll the device in a blocking manner so that our future resolves.
    // In an actual application, `device.poll(...)` should
    // be called in an event loop or on another thread.
    device.poll(wgpu::Maintain::wait()).panic_on_timeout();
    //println!("poll device");
    // Awaits until `buffer_future` can be read from
    if let Ok(Ok(())) = receiver.recv_async().await {
        // Gets contents of buffer
        let data = buffer_slice.get_mapped_range();
        // Since contents are got in bytes, this converts these bytes back to u32
        let result = bytemuck::cast_slice(&data).to_vec();

        // With the current interface, we have to make sure all mapped views are
        // dropped before we unmap the buffer.
        drop(data);
        staging_buffer.unmap(); // Unmaps buffer from memory
                                // If you are familiar with C++ these 2 lines can be thought of similarly to:
                                //   delete myPointer;
                                //   myPointer = NULL;
                                // It effectively frees the memory

        // Returns data from buffer
        // Assuming `result` is of type `Result<&[_], Error>`
        Some(result[0])
    } else {
        //panic!("failed to run compute on gpu!")
        log::info!("gpu timeout, test failed!");
        Some(0)
    }
}

//change this so I can choose adapter
#[wasm_bindgen]
pub async fn get_gpu_name() -> Option<String> {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .expect("No suitable GPU adapters found on the system!");
    
    println!("got adapter");
    // `request_device` instantiates the feature specific connection to the GPU, defining some parameters,
    //  `features` being the available features.
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
    log::info!("gpu adapter: {}", adapter.get_info().name);
    Some(adapter.get_info().name)
}


/* 
#[wasm_bindgen]
pub fn load_wgsl(kernel_file: &str) -> wgpu::ShaderModule {
    let cs_module = match kernel_file {
        "2_2_5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_5.wgsl"))),
            });
        }
        "2_2_4.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_4.wgsl"))),
            });
        }
        _ => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_5.wgsl"))),
            });
        }
    };
    cs_module
}*/

#[wasm_bindgen(start)]
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

