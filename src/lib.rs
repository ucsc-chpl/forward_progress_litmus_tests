use std::borrow::Cow;
use wgpu::util::DeviceExt;
use wasm_bindgen::prelude::*;
use log::{info};
use include_dir::include_dir;
use std::panic;
use web_sys::console;
extern crate console_error_panic_hook;

#[wasm_bindgen]
pub async fn run(num_threads: u32, kernel_file: &str) -> u32 {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console::log_1(&"Program started, running kernel".into());

    let threads_finished = execute_gpu(num_threads, kernel_file).await.unwrap();

    console::log_1(&"Finished execute_gpu".into());
    let disp_steps: String = threads_finished.to_string();

    console::log_1(&format!("Threads finished: {}", disp_steps).into());
    return threads_finished;
}

#[wasm_bindgen]
pub async fn execute_gpu(num_threads: u32, kernel_path: &str) -> Option<u32> {
    let instance = wgpu::Instance::default();
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await?;
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
    static TEST_DIR : include_dir::Dir<'_> = include_dir!("src/tests");
    let kernel_path_stripped = kernel_path.strip_prefix("tests/").unwrap();
    let kernel_file_opt = TEST_DIR.get_file(kernel_path_stripped);
    assert!(kernel_file_opt.is_some(), "Can't find shader file '{}'", kernel_path_stripped);
    let kernel_file = kernel_file_opt.unwrap();
    
    let cs_module: wgpu::ShaderModule = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(kernel_path),
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(kernel_file.contents_utf8().expect("Failed to read shader file contents!")))
    });
    
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
    console::log_1(&format!("GPU adapter: {}", adapter.get_info().name).into());
    Some(adapter.get_info().name)
}
