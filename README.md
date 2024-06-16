# forward_progress_litmus_tests

#### Github Pages Website
This branch is linked to the [github pages website](https://ucsc-chpl.github.io/forward_progress_litmus_tests/). Changes pushed to this branch will automatically be published to the webpage

#### Dependancies
- rust (`$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`)
- rust crates (`cargo update` in this directory)

#### Running in the Browser
Steps to run:
- Navigate to the root of this repository
- Compile using the command `wasm-pack build --target web`. This will generate the webGPU bindings and javascript in the pkg/ directory.
- Make sure webGPU is enabled in chrome (chrome://gpu, webGPU should have hardware acceleration enabled)
- Start a local server in the root of this repo with `python3 -m http.server`
  - if you see something like 'failed to initialize webGPU' it probably means chrome is blocking the gpu because this is an http server. Launch a secure server (https) instead.
- view in the browser at localhost:8000/ (use chrome!). 
- Select a test and run with the run button.
- If a test case finishes it will display the number of threads that terminated, otherwise it should display test failed after 15 seconds. It may not display this if the test case is seriously messing with chrome or the computer. 

#### Running Natively
Steps to run: 
- Navigate to the root of this repository
- to run the default test (2 threads 2 instructions test 5) run `cargo run`
- to run a specific test run `cargo run <path_to_wgsl> <num_workgroups>`
  - The wgsls are in src/tests/ and are sorted by progress model
  - Currently there is 1 thread per workgroup

#### Generating wgsls and the website
compile_wgsl.py takes a .txt file from the alloy_forward_progress directory and generates a wgsl. To use:
`python3 compile_wgsl.py -g 1 -tf /path/to/test.txt -o /path/to/outfile.wgsl`

gen_website.py contains the script for generating all of the wgsls from the alloy_forward_progress repo and generating the HTML files for the website. 

To generate everything: (wgsls, rust lib.rs, and HTML):
`python3 gen_website.py -c 1 --alloyfp_path <path_to_webtest_dir> -r 1 -o <path to src/lib.rs> -i 1`

To generate the wgsls:
`python3 gen_website.py -c 1 --alloyfp_path <path_to_webtest_dir>`

To generate the rust/wgpu code:
`python3 gen_website.py -r 1 -o <path to src/lib.rs>`

To generate the HTML files:
`python3 gen_website.py -i 1`

Note that wasm-pack should be re-run whenever changes to the wgsls or rust code are made. 



