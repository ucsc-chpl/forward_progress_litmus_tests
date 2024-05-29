use std::borrow::Cow;
use wgpu::util::DeviceExt;
use wasm_bindgen::prelude::*;
use log::{info};

// this lad will be called from the javascript
//refactor later
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
        "minimized.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("minimized.wgsl"))),
            })
        }
        "minimized_with_dummy.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("minimized_with_dummy.wgsl"))),
            })
        }
        "2_2_5.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_5.wgsl"))),
            })
        }
        "2_2_5_with_buffer.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_5_with_buffer.wgsl"))),
            })
        }
        "2_2_5_with_pc.wgsl" => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_5_with_pc.wgsl"))),
            })
        }
        _ => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_5.wgsl"))),
            })
        }
    };
    
    // counter in wgsl
    let threads_finished: i32 = 0;
    // for buffer test, buffer_dummy in wgsl
    let dummy: i32 = 1;
    let data_in = [threads_finished, dummy];

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
        //dispatch threads
        cpass.dispatch_workgroups(num_threads as u32, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
    }

    encoder.copy_buffer_to_buffer(&storage_buffer, 0, &staging_buffer, 0, size);

    // Submits command encoder for processing
    queue.submit(Some(encoder.finish()));

    let buffer_slice = staging_buffer.slice(..);
  
    // Sets the buffer up for mapping, sending over the result of the mapping back to us when it is finished.
    let (sender, receiver) = flume::bounded(1);
    buffer_slice.map_async(wgpu::MapMode::Read, move |v| sender.send(v).unwrap());

    // Poll the device in a blocking manner so that our future resolves.
    // In an actual application, `device.poll(...)` should
    // be called in an event loop or on another thread.
    device.poll(wgpu::Maintain::wait()).panic_on_timeout();

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
        //panic!("failed to run on gpu!");
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
    info!("GPU adapter: {}", adapter.get_info().name);
    Some(adapter.get_info().name)
}




