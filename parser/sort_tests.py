# generates wgsls from alloy forward progress tests, sorted by progress model

# output file directory:
# hsa
# | 2_thread_2_inst
# | | 1
# | | | 1.txt
# | | | 1.wgsl
# | | | 1.png
# | 2_thread_2_inst
# etc

# website structure
# main page
# | hsa
# | | test_all
# | | 2_thread_2_inst
# | | | 1
# | | | 2 ...etc
# | | 2_thread_3_inst ..etc
# | obe ...etc

import os
import sys
import run_test
import shutil
import argparse

wgsl_base_path="tests/"
dest_path = "/home/nrehman/forward_progress_litmus_tests/src/tests/"
test_path = "/home/nrehman/AlloyForwardProgress/artifact/web_test_explorer/"

preamble = """
<!doctype html>
<html lang="en-US">
<head>
  <meta charset="utf-8" />
  <title>litmus test</title>
</head>
<body>
  <p id="arch"></p>
  <p id="vendor"></p>
  <p id="desc"></p>
  <p id="device"></p>
"""
run_button = """
<button id="run_button">run test</button>
<div id="run_output"></div>
"""
initwebgpu = """  <script>
    async function initWebGPU() {
        try {
            const adapter = await navigator.gpu.requestAdapter();
            const adapterInfo = await adapter.requestAdapterInfo();
            const arch = document.getElementById('arch');
            arch.textContent = `Architecture: ${adapterInfo.architecture}`;
            const vendor = document.getElementById('vendor');
            vendor.textContent = `Vendor: ${adapterInfo.vendor}`;
            const device = document.getElementById('device');
            device.textContent = `Device: ${adapterInfo.device}`;
            const desc = document.getElementById('desc');
            desc.textContent = `Description: ${adapterInfo.description}`;

        } catch (error) {
            console.error('Error initializing WebGPU:', error);
            document.write('<p>Error initializing WebGPU: ' + error + '</p>');
        }
    }
    window.onload = initWebGPU;
  </script>
  """

style_stuff = """
<style>
    .content {{
      float: right;
    }}
  </style>
  <div class="container">
    <div class="content">
      <img src="{img_name}" alt="state transition diagram" width="500">
    </div>
  </div>
</body>
</html>
"""
run_stuff = """
  <script type="module">
    import init from "../../pkg/litmus_test_web.js";
    import * as wasm_mod from "../../pkg/litmus_test_web.js";

    init().then(() => {{
      // Event listener for test case 5
      document.getElementById('run_button').addEventListener('click', () => {{
        const resultPromise = wasm_mod.run(2, "{test_name}");
        if (resultPromise instanceof Promise) {{
        resultPromise.then(result => {{
            const outputDiv = document.getElementById('run_output');
            if(result == 0){{
              outputDiv.textContent = `Threads finished: ${{result}} 
 Refresh page to continue`;
            }}
            else {{
              outputDiv.textContent = `Threads finished: ${{result}}`;
            }}
          }});
        }} else {{ 
          const outputDiv = document.getElementById('run_output');
          outputDiv.textContent = `Threads finished: ${{resultPromise}}`;
        }}
      }});
    }});
  </script>
"""

def gen_wgsls_by_model(dest_path, test_path):
    for test_dir in [d for d in os.listdir(test_path) if os.path.isdir(os.path.join(test_path, d))]:
        dest_subdir = dest_path + os.path.basename(test_dir)
        for model in os.listdir(test_path + test_dir + '/distinguishing/'):
            dest_subdir = dest_path + model.replace('.txt', '/')
            with open(test_path + test_dir + '/distinguishing/' + model, 'r') as model_file:
                for line in model_file:
                    #line.strip()
                    # copy png pic, the txt file, and the simple.txt
                    cur_test_path = test_path + test_dir + '/' + line.strip()
                    cur_dest_path = dest_path + model.replace('.txt', '/') + test_dir + '/' + line.strip()
                    if not os.path.exists(cur_dest_path):
                        os.makedirs(cur_dest_path)
                        print(f"made dir {cur_dest_path}")
                    #else:
                        #print(f"path already exists: {cur_dest_path}")
                    shutil.copyfile(cur_test_path + '/' + line.strip() + '.png', cur_dest_path + '/' + line.strip() + '.png')
                    shutil.copyfile(cur_test_path + '/' + line.strip() + '.txt', cur_dest_path + '/' + line.strip() + '.txt')
                    shutil.copyfile(cur_test_path + '/' + line.strip() + '_simple.txt', cur_dest_path + '/' + line.strip() + '_simple.txt')
                    run_test.gen_wgsl(cur_test_path + '/' + line.strip() + '.txt', cur_dest_path + '/' + line.strip() + '.wgsl')

def gen_runner_native(dest_path):
    pass
# I should change this so its relative to the directory and not my user 0.o
def gen_runner_web(dest_path, wgsl_base_path, outfile="/home/nrehman/forward_progress_litmus_tests/src/lib.rs"):
    runner_s = """use std::borrow::Cow;
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
"""
    for model in os.listdir(dest_path):
        for thread_inst in os.listdir(dest_path + os.path.basename(model)):
            for test in os.listdir(dest_path + '/' + os.path.basename(model) + '/' + os.path.basename(thread_inst)):
                test_in = wgsl_base_path + os.path.basename(model) + '/' + os.path.basename(thread_inst) + '/' + os.path.basename(test) + '/' + os.path.basename(test) + '.wgsl'
                runner_s += f"""
        "{test_in}" => {{
            device.create_shader_module(wgpu::ShaderModuleDescriptor {{
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("{test_in}"))),
            }})
        }}"""
    runner_s += """_ => {
            device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("2_2_5.wgsl"))),
            })
        }
    };
    
    let dummy: i32 = 0;
    let size = std::mem::size_of_val(&dummy) as wgpu::BufferAddress;

    let staging_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
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
"""
    with open(outfile, 'w') as file:
        file.write(runner_s)
        file.close()
    #print(runner_s)

# website structure
# main page
# | hsa
# | | test_all
# | | 2_thread_2_inst
# | | | 1
# | | | 2 ...etc
# | | 2_thread_3_inst ..etc
# | obe ...etc

def gen_index_html_all_runner(dest_path, target_dir, wgsl_base_path, model):
    index = preamble + run_button + initwebgpu
    index += """
    init().then(() => {
      // Event listener for test case 5
      document.getElementById('run_button').addEventListener('click', () => {
"""

    for thread_inst in os.listdir(dest_path + '/' + model):
        for test in os.listdir(dest_path + '/' + os.path.basename(model) + '/' + os.path.basename(thread_inst)):
            test_in = wgsl_base_path + os.path.basename(model) + '/' + os.path.basename(thread_inst) + '/' + os.path.basename(test) + '/' + os.path.basename(test) + '.wgsl'
            index += f"""
        const resultPromise = wasm_mod.run(2, "{test_in}");
        if (resultPromise instanceof Promise) {{
        resultPromise.then(result => {{
            const outputDiv = document.getElementById('run_output');
            if(result == 0){{
              outputDiv.textContent = `Threads finished: ${{result}} 
 Refresh page to continue`;
            }}
            else {{
              outputDiv.textContent = `Threads finished: ${{result}}`;
            }}
          }});
        }} else {{ 
          const outputDiv = document.getElementById('run_output');
          outputDiv.textContent = `Threads finished: ${{resultPromise}}`;
        }}
"""
    index += """
      });
    });
  </script>
"""
    index += style_stuff
    with open(target_dir + 'index.html', 'w') as file:
        file.write(index)
        file.close()

def gen_index_html_per_test_runner(test_name, target_dir, img):
    index = preamble + run_button + initwebgpu + run_stuff.format(test_name=test_name) + style_stuff.format(img_name=img)
    with open(target_dir + 'index.html', 'w') as file:
        file.write(index)
        file.close()


def gen_index_html(dest_path, wgsl_base_path):
    # premble
    # button defs
    # initwebgpu script
    # run stuff
    # style stuff
    pass

def test():
    gen_index_html_per_test_runner('2_thread_2_inst.wgsl', '', '1.png')


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('-c', '--compile', help='compile wgsls')
    parser.add_argument('-r', '--make_runner', help='makes the rust stuff')
    parser.add_argument('-o', '--outfile', help='outfile for lib.rs, default is actual /src/lib.rs')
    parser.add_argument('-i', '--make_index', help='makes index.htmls')
    parser.add_argument('-t', '--test', help='runs the test function. for debugging.')
    args = parser.parse_args()
    if(args.test):
        test()
    if(args.compile):
        gen_wgsls_by_model(dest_path, test_path)
    if(args.make_runner):
        if(args.outfile):
            gen_runner_web(dest_path, wgsl_base_path, outfile=args.outfile)

