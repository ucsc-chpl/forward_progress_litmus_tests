from enum import Enum

class WGPU_Runner(Enum):
    INCLUDES_STR = '''
use std::borrow::Cow;
use wgpu::util::DeviceExt;
use wasm_bindgen::prelude::*;
use log::{info};
'''
    RUN_FN_STR = '''
#[wasm_bindgen]
pub async fn run(num_threads: i32, kernel_file: &str, num_workgroups: u32) -> u32 {
    info!("Program started, running kernel");

    let threads_finished = execute_gpu(num_threads, kernel_file, num_workgroups).await.unwrap();

    info!("Finished execute_gpu");
    let disp_steps: String = threads_finished.to_string();

    info!("Threads finished: {}", disp_steps);
    return threads_finished;
}
'''

    EXECUTE_GPU_FN_STR = '''
#[wasm_bindgen]
pub async fn execute_gpu(num_threads: i32, kernel_file: &str, num_workgroups: u32) -> Option<u32> {
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
?test_paths? &_ => {
                    device.create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: None,
                        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_2_instructions/3/3.wgsl"))),
                    })
                }
    };
    const NUM_SCALARS: usize = 3;
    const DUMMY_BUFFER_SIZE: usize = 253;

    let MAX_THREADS: i32 = 32;
    let NUM_TESTING_THREADS: i32 = num_threads;
    let threads_finished: i32 = 0;
    // its all zeros anyway, this buffer is used for all of the memory locs
    let mem: [i32; DUMMY_BUFFER_SIZE] = [0; DUMMY_BUFFER_SIZE];
    let mut data_in: [i32; DUMMY_BUFFER_SIZE + NUM_SCALARS] = [0; DUMMY_BUFFER_SIZE + NUM_SCALARS];

    data_in[0] = threads_finished;
    data_in[1] = MAX_THREADS;
    data_in[2] = NUM_TESTING_THREADS;

    data_in[3..].copy_from_slice(&mem);

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
        cpass.dispatch_workgroups(num_workgroups as u32, 1, 1); // Number of cells to run, the (x,y,z) size of item being processed
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
'''

    ADD_TEST_STR = '''
                "?test_path?" => {
                    device.create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: None,
                        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("?test_path?"))),
                    })
                },
'''