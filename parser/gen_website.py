# generates wgsls from alloy forward progress tests, sorted by progress model

# output file directory:
# hsa
# | 2_thread_2_inst
# | | 1
# | | | 1_simple.txt
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

# TODO
# X (needs testing) display 'test running' for the all runners
#   - display number of tests finished
# X add index.html in each of the x_threads_x_instructions directories
# - fix image display

import os
import sys
import compile_wgsl
import shutil
import argparse
import re
import subprocess

repo_base_dir = os.getcwd().replace('/parser', '')
wgsl_base_path="tests/"
dest_path = repo_base_dir + "/src/tests/"
test_path_default = "/home/nrehman/AlloyForwardProgress/artifact/web_test_explorer/"

timeout_ms = 15000

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

style_stuff = """<style>
    .content {{
        position: absolute;
        top: 0;
        right: 0;
    }}
  </style>
  <div>
    <p><iframe src="{text_file}" frameborder="0" height="400"
      width="95%"></iframe></p>
  </div>
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
    import init from "../../../../../pkg/litmus_test_web.js";
    import * as wasm_mod from "../../../../../pkg/litmus_test_web.js";

    init().then(() => {{
      // Event listener for test case 5
      document.getElementById('run_button').addEventListener('click', () => {{
        const outputDiv = document.getElementById('run_output');
        outputDiv.textContent = `running test...`;
        const resultPromise = wasm_mod.run(2, "{test_name}");
        if (resultPromise instanceof Promise) {{
        resultPromise.then(result => {{
            if(result == 0){{
              outputDiv.textContent = `Threads finished: ${{result}} 
 Refresh page to continue`;
            }}
            else {{
              outputDiv.textContent = `Threads finished: ${{result}}`;
            }}
          }});
        }} else {{ 
          outputDiv.textContent = `Threads finished: ${{resultPromise}}`;
        }}
      }});
    }});
  </script>
"""

#format with model
model_index_pa = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{model} Tests</title>
</head>
<body>
    <b>{model} Tests</b>
    <ul>
"""

model_index_end = """
    </ul>
</body>
</html>
"""

def bn(path):
    return os.path.basename(path)

def copy_test(dest_path, test_dir, test, cur_test_path, model, png, text, s_text):
    os.makedirs(dest_path + '/' + model + '/' + bn(test_dir) + '/' + bn(test) + '/', exist_ok=True)
    shutil.copyfile(png, dest_path + '/' + model + '/' + bn(test_dir) + '/' + bn(test) + '/' + bn(test) + '.png')
    shutil.copyfile(text, dest_path + '/' + model + '/' + bn(test_dir) + '/' + bn(test) + '/' + bn(test) + '.txt')
    shutil.copyfile(s_text, dest_path + '/' + model + '/' + bn(test_dir) + '/' + bn(test) + '/' + bn(test) + '_simple.txt')
    run_test.gen_wgsl(cur_test_path + '/' + bn(test) + '.txt', dest_path + '/' + model + '/' + bn(test_dir) + '/' + bn(test) + '/' + bn(test) +'.wgsl')

def gen_wgsls_by_model(dest_path, test_path):
    for test_dir in [d for d in os.listdir(test_path) if os.path.isdir(os.path.join(test_path, d))]:
        for test in [d for d in os.listdir(os.path.join(test_path, test_dir)) if d not in ['csv', 'distinguishing', 'testExplorer.html', 'timestamps.txt']]:
            with open(os.path.join(test_path,test_dir,test) + '/' + 'label_' + os.path.basename(test) + '.txt', 'r') as test_file:
                for line_no, line in enumerate(test_file, start=1):
                    cur_test_path = test_path + test_dir + '/' + os.path.basename(test) + '/'
                    png = cur_test_path + os.path.basename(test) + '.png'
                    text = cur_test_path + os.path.basename(test) + '.txt'
                    s_test = cur_test_path + os.path.basename(test) + '_simple.txt'
                    #WEAK VARIANTS
                    if(line_no == 7):
                        if(line.strip().replace('OBE - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'OBE', png, text, s_test)
                   
                    if(line_no == 8):
                        if(line.strip().replace('HSA - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'HSA', png, text, s_test)

                    if(line_no == 9):
                        if(line.strip().replace('HSA_OBE - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'HSA_OBE', png, text, s_test)

                    if(line_no == 10):
                        if(line.strip().replace('LOBE - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'LOBE', png, text, s_test)

                    if(line_no == 11):
                        if(line.strip().replace('WEAK_FAIR - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'WEAK_FAIR', png, text, s_test)

                    #strong variants:
                    if(line_no == 14):
                        if(line.strip().replace('OBE_STRONG - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'OBE_STRONG', png, text, s_test)

                    if(line_no == 15):
                        if(line.strip().replace('HSA_STRONG - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'HSA_STRONG', png, text, s_test)
                    
                    if(line_no == 16):
                        if(line.strip().replace('HSA_OBE_STRONG - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'HSA_OBE_STRONG', png, text, s_test)
                    
                    if(line_no == 17):
                        if(line.strip().replace('LOBE_STRONG - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'LOBE_STRONG', png, text, s_test)

                    if(line_no == 18):
                        if(line.strip().replace('STRONG_FAIR - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'STRONG_FAIR', png, text, s_test)


        """
        for model in os.listdir(test_path + test_dir + '/distinguishing/'):
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
"""
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
            if os.path.isdir(os.path.join(dest_path, model, thread_inst)):
                for test in os.listdir(dest_path + '/' + os.path.basename(model) + '/' + os.path.basename(thread_inst)):
                    if(os.path.isdir(os.path.join(dest_path, model, thread_inst, test))):
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

def gen_index_html_all_runner(dest_path, wgsl_base_path, model):
    promise_all = "     Promise.all([\n"
    out_divs = ""
    tests = ""
    num_tests = 0
    for thread_inst in os.listdir(dest_path + '/' + model):
        if(os.path.isdir(os.path.join(dest_path, model, thread_inst))):
            for test in os.listdir(dest_path + '/' + os.path.basename(model) + '/' + os.path.basename(thread_inst)):
                if(os.path.isdir(os.path.join(dest_path, model, thread_inst, test))):
                    # test stuff
                    test_in = wgsl_base_path + os.path.basename(model) + '/' + os.path.basename(thread_inst) + '/' + os.path.basename(test) + '/' + os.path.basename(test) + '.wgsl'
                    test_desc = re.match("(?P<num_threads>[0-9])_threads_(?P<num_inst>[0-9])_instructions", thread_inst)
                    # add to this promise to the promise all statement
                    promise_all += f"       resultPromise{num_tests},\n"
                    # add this test to the test outputs
                    out_divs += f"""<div id="output{num_tests}">{test_desc['num_threads']} instructions, {test_desc['num_inst']}, test {test}</div>\n"""
                    tests += f"""
        let resultPromise{num_tests} = Promise.race([wasm_mod.run(2, "{test_in}"), new Promise((resolve, _) => {{
                setTimeout(() => {{
                resolve(0);
                }}, {timeout_ms});
            }})]);
        resultPromise{num_tests}.then((result) => {{
            if(result == 0) {{
                document.getElementById("output{num_tests}").textContent = "{test_desc['num_threads']} instructions, {test_desc['num_inst']}, test {test} : failed";
            }} else {{
                document.getElementById("output{num_tests}").textContent = "{test_desc['num_threads']} instructions, {test_desc['num_inst']}, test {test} : passed";
            }}
        }});

"""
                    num_tests += 1
    promise_all += """]).then((results) => {
          for(let th_fin of results) {
            if(th_fin == 0) {
              tests_failed = tests_failed + 1;
            }
            else {
              tests_passed = tests_passed + 1;
            }
          }
          outputDiv.textContent = `All Tests Finished. Tests Passed: ${tests_passed} Tests Failed: ${tests_failed}`;
        }
      );
"""
    index = preamble
    index += """<button id="run_button">run all tests</button>\n"""
    index += """<div id="run_output"></div>\n\n"""
    index += out_divs 
    index += initwebgpu
    index += """
  <script type="module">
    import init from "../../../../pkg/litmus_test_web.js";
    import * as wasm_mod from "../../../../pkg/litmus_test_web.js";

    init().then(() => {
      // Event listener for test case 5
      document.getElementById('run_button').addEventListener('click', () => {
        let resultPromise;
        const outputDiv = document.getElementById('run_output');
        var tests_passed = 0;
        var tests_failed = 0;
        var tests_completed = 0;
        outputDiv.textContent = "running tests...";
"""
    index += tests
    index += promise_all
    index += f"""

      }});
    }});
  </script>
"""
    #FIX THIS!
    index += """</body>
</html>"""
    out_path = dest_path + '/' + model + '/' + 'all_runner' + '/'
    os.makedirs(out_path, exist_ok=True)
    with open(out_path + 'index.html', 'w') as file:
        file.write(index)
        file.close()

def gen_index_html_per_test_runner(test_name, target_dir, img, text_file):
    if os.path.isdir(os.path.join(dest_path.replace('/tests', '') + target_dir)):
        index = preamble 
        index += run_button 
        index += initwebgpu 
        index += run_stuff.format(test_name=test_name) 
        index += style_stuff.format(img_name=img, text_file=text_file)
        with open(os.path.join(dest_path.replace('/tests', ''), target_dir) + 'index.html', 'w') as file:
            file.write(index)
            file.close()
    else:
        print(f"gen_index_html_per_test_runner() recieved non dir target dir: {target_dir} skipping")


# generates individual and all runner for all models
def gen_index_html(dest_path, wgsl_base_path):
    # premble
    # button defs
    # initwebgpu script
    # run stuff
    # style stuff
    for model in os.listdir(dest_path):
        print(f"generating all runner index.html for {model}")
        gen_index_html_all_runner(dest_path, wgsl_base_path, model)
        m_index = model_index_pa.format(model=model)
        m_index += f"""    <li><a href="./all_runner/">Run All Tests</a></li>\n"""
        for thread_inst in os.listdir(dest_path + '/' + model):
            if os.path.isdir(os.path.join(dest_path, model, thread_inst)):
                test_desc = re.match("(?P<num_threads>[0-9])_threads_(?P<num_inst>[0-9])_instructions", thread_inst)
                if(thread_inst != 'all_runner' and thread_inst != 'index.html'):
                    t_index = model_index_pa.format(model=f"{test_desc['num_threads']} threads, {test_desc['num_inst']} instructions")
                    m_index += f"""    <li><a href="./{thread_inst}/">{test_desc['num_threads']} threads, {test_desc['num_inst']} instructions</a></li>\n"""
                    if(os.path.isdir(os.path.join(dest_path, model, thread_inst))):
                        for test in os.listdir(dest_path + '/' + os.path.basename(model) + '/' + os.path.basename(thread_inst)):
                            if(os.path.isdir(os.path.join(dest_path, model, thread_inst, test))):
                                t_index += f"""    <li><a href="./{test}/">Test {test}</a></li>\n"""
                                test_target_dir = wgsl_base_path + os.path.basename(model) + '/' + os.path.basename(thread_inst) + '/' + os.path.basename(test) + '/'
                                test_in = test_target_dir + os.path.basename(test) + '.wgsl'
                                test_img = os.path.basename(test) + '.png'
                                text_file = os.path.basename(test) + '_simple.txt'
                                gen_index_html_per_test_runner(test_in, test_target_dir, test_img, text_file)
                        t_index += model_index_end
                        with open(os.path.join(dest_path, model, thread_inst) + '/index.html', 'w') as t_outfile:
                            t_outfile.write(t_index)
                            t_outfile.close()
        m_index += model_index_end
        with open(dest_path + '/' + model + '/' + 'index.html', 'w') as file:
            file.write(m_index)
            file.close()




def test():
    validate_wgsls(dest_path)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('-c', '--compile', help='compile wgsls')
    parser.add_argument('--alloyfp_path', help='compile wgsls')
    parser.add_argument('-r', '--make_runner', help='makes the rust stuff')
    parser.add_argument('-o', '--outfile', help='outfile for lib.rs, default is actual /src/lib.rs')
    parser.add_argument('-i', '--make_index', help='makes index.htmls')
    parser.add_argument('-t', '--test', help='runs the test function. for debugging, ignore.')
    parser.add_argument('-v', '--validate', help='validate all wgsls')
    args = parser.parse_args()
    if(args.test):
        test()
    if(args.compile):
        if(args.alloyfp_path):
            if(os.path.isdir(args.alloyfp_path)):
                gen_wgsls_by_model(dest_path, args.alloyfp_path + '/')
            else:
                print("invalid path to AlloyForwardProgress repo! Exiting!")
                exit()
            
        else:
            print("no path to AlloyForwardProgress repo was given, defaulting to naomi's path")
            gen_wgsls_by_model(dest_path, test_path_default)

    if(args.make_runner):
        if(args.outfile):
            gen_runner_web(dest_path, wgsl_base_path, outfile=args.outfile)
    if(args.make_index):
        gen_index_html(dest_path, wgsl_base_path)

