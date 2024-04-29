use std::{borrow::Cow, str::FromStr};
use wgpu::util::DeviceExt;
use wasm_bindgen::prelude::*;
use std::env;
use std::fs;


// Indicates a u32 overflow in an intermediate Collatz value
const OVERFLOW: u32 = 0xffffffff;

#[cfg_attr(test, allow(dead_code))]
async fn run() {
    //pass file name and number of threads
    //put num threads in a comment at the top of the wgsl

    let args: Vec<String> = env::args().collect();
    let kernel_file : &str;
    let num_threads : u32;

    if args.len() > 1{
        // sus
        kernel_file = &&args[1];
        print!("{}", kernel_file);
        num_threads = *(&args[2].parse::<u32>().unwrap_or(0));
        //support power mode in the future
    } else {
        kernel_file = "litmus_test.wgsl";
        num_threads = 2;
        println!("No argumets provided, defaulting to litmus_test.wgsl, 2 threads")
    }
    println!("program started");
    let steps = execute_gpu(num_threads, &kernel_file).await.unwrap();
    println!("execute_gpu finished");
    let disp_steps: Vec<String> = steps
        .iter()
        .map(|&n| match n {
            OVERFLOW => "OVERFLOW".to_string(),
            _ => n.to_string(),
        })
        .collect();

    println!("Threads finished: {}", disp_steps.join(", "));
    #[cfg(target_arch = "wasm32")]
    log::info!("Threads finished: {}", disp_steps.join(", "));
}

#[cfg_attr(test, allow(dead_code))]
async fn execute_gpu(num_threads: u32, kernel_file: &str) -> Option<Vec<u32>> {
    // Instantiates instance of WebGPU
    println!("got into exec gpu");
    let instance = wgpu::Instance::default();
    println!("got instance");
    // `request_adapter` instantiates the general connection to the GPU
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await?;
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
    println!("got device");
    execute_gpu_inner(&device, &queue, num_threads, &kernel_file).await
}

async fn execute_gpu_inner(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    num_threads: u32,
    kernel_file: &str,
) -> Option<Vec<u32>> {
    // Loads the shader from WGSL
    // rust is so silly
    let contents = match fs::read_to_string(kernel_file) {
        Ok(contents) => contents,
        Err(e) => {
            panic!("failed to read wgsl! {}", e);
        }
    };
    let cs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        //source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!(kernel_file))),
        //doesn't work because macro, use std::fs?
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(&contents)),
    });
    let dummy: i32 = 0;
    //println!("got shader");
    // Gets the size in bytes of the buffer.
    let size = std::mem::size_of_val(&dummy) as wgpu::BufferAddress;
    //println!("got buffer");
    // Instantiates buffer without data.
    // `usage` of buffer specifies how it can be used:
    //   `BufferUsages::MAP_READ` allows it to be read (outside the shader).
    //   `BufferUsages::COPY_DST` allows it to be the destination of the copy.
    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    //println!("instantiated buffer");

    // Instantiates buffer with data (`numbers`).
    // Usage allowing the buffer to be:
    //   A storage buffer (can be bound within a bind group and thus available to a shader).
    //   The destination of a copy.
    //   The source of a copy.
    let storage_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer"),
        contents: bytemuck::cast_slice(&[dummy]),
        usage: wgpu::BufferUsages::STORAGE
            | wgpu::BufferUsages::COPY_DST
            | wgpu::BufferUsages::COPY_SRC,
    });
    //println!("instantiated buffer with numbers");
    // A bind group defines how buffers are accessed by shaders.
    // It is to WebGPU what a descriptor set is to Vulkan.
    // `binding` here refers to the `binding` of a buffer in the shader (`layout(set = 0, binding = 0) buffer`).

    // A pipeline specifies the operation of a shader

    // Instantiates the pipeline.
    let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: None,
        layout: None,
        module: &cs_module,
        entry_point: "main",
        //constants: &Default::default(),
    });
    //println!("got compute pipeling");
    // Instantiates the bind group, once again specifying the binding of buffers.
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
    // Sets adds copy operation to command encoder.
    // Will copy data from storage buffer on GPU to staging buffer on CPU.
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
        Some(result)
    } else {
        panic!("failed to run compute on gpu!")
    }
    //println!("got result or failed");
}
fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
        pollster::block_on(run());
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
        wasm_bindgen_futures::spawn_local(run());
    }
}

