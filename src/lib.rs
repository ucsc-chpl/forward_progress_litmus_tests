
use std::borrow::Cow;
use wgpu::util::DeviceExt;
use wasm_bindgen::prelude::*;
use log::{info};
use rand::prelude::*;

//test
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use mut_static::MutStatic;
use once_cell::unsync::OnceCell;

// current plan is just create these in init, may have to move more into here IDK

pub struct GPU_Objects {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl GPU_Objects {
    pub async fn new() -> GPU_Objects {
        let instance = wgpu::Instance::default();
        let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await
        .unwrap();
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
        Self {device, queue}
    }
}
/*
unsafe async fn global_gpu_data() -> &'static GPU_Objects {
    static INSTANCE: OnceCell<GPU_Objects> = OnceCell::new();
    INSTANCE.get_or_init(|| async {
        GPU_Objects::new().await.unwrap();
    })
}
*/

static mut GPU_OBJECTS: Option<GPU_Objects> = None;

#[wasm_bindgen]
pub async fn init_GPU_Objects() {
    unsafe {
        GPU_OBJECTS = Some(
            {
                let objects = GPU_Objects::new().await;
                objects
            }
        );
    }
}


fn get_GPU_Objects() -> &'static GPU_Objects {
    unsafe{ GPU_OBJECTS.as_ref().expect("GPU_OBJECTS not initialized") }
}

//end test
#[wasm_bindgen]
pub async fn run(num_threads: i32, kernel_file: &str, num_workgroups: u32) -> u32 {
    info!("Program started, running kernel");
    let i=0;
    let mut threads_finished = 0;
    for i in 0..10{
        threads_finished += execute_gpu(num_threads, kernel_file, num_workgroups).await.unwrap();
    }
    

    info!("Finished execute_gpu");
    let disp_steps: String = threads_finished.to_string();

    info!("Threads finished: {}", disp_steps);
    return threads_finished;
}

#[wasm_bindgen]
pub async fn execute_gpu(num_threads: i32, kernel_file: &str, num_workgroups: u32) -> Option<u32> {
    /*info!("Got into exec gpu");
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
    */
    //init_GPU_Objects().await;
    let gpu_objects = get_GPU_Objects();
    let device = &gpu_objects.device;
    let queue = &gpu_objects.queue;
    let cs_module: wgpu::ShaderModule = match kernel_file {
                "tests/HSA/2_threads_2_instructions/3/3_single.wgsl" => {
                    device.create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: None,
                        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_2_instructions/3/3_single.wgsl"))),
                    })
                },
 &_ => {
                    device.create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: None,
                        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("tests/HSA/2_threads_2_instructions/3/3.wgsl"))),
                    })
                }
    };
    const NUM_SCALARS: usize = 6;
    const DUMMY_BUFFER_SIZE: usize = 250;
    //const MAX_THREADS = 32;

    let MAX_THREADS: i32 = 32;
    let NUM_TESTING_THREADS: i32 = num_threads;
    let threads_finished: i32 = 0;
    // its all zeros anyway, this buffer is used for all of the memory locs
    let mem: [i32; DUMMY_BUFFER_SIZE] = [0; DUMMY_BUFFER_SIZE];
    let mut data_in: [i32; DUMMY_BUFFER_SIZE + NUM_SCALARS] = [0; DUMMY_BUFFER_SIZE + NUM_SCALARS];

    // the type doesn't matter as long as the bits are correct
    let rand_ind_0 = rand::random::<i32>();
    let rand_ind_1 = rand::random::<i32>();
    let rand_ind_2 = rand::random::<i32>();

    data_in[0] = threads_finished;
    data_in[1] = MAX_THREADS;
    data_in[2] = NUM_TESTING_THREADS;
    data_in[3] = rand_ind_0;
    data_in[4] = rand_ind_1;
    data_in[5] = rand_ind_2;

    data_in[6..].copy_from_slice(&mem);

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
