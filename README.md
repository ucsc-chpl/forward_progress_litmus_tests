# forward_progress_litmus_tests
wgpu forward progress tests. 

#### Running in the Browser
Steps to run:
- Navigate to the root directory
- Compile use the command `wasm-pack build --target web`. 
- Start a local server in the litmus_test_web/ directory with `python3 -m http.server`
- view in the browser at localhost:8000/ (use chrome!). 
- Click view tests then select a test and run with the run button.
- If a test case finishes it will display the number of threads that terminated.

#### Running Natively
Steps to run: 
- Navigate to the root directory
- to run the default test (2 threads 2 instructions test 5) run `cargo run`
- to run a specific test run `cargo run <path_to_wgsl> <num_workgroups>`
  - The wgsls are in src/tests/ and are sorted by progress model
  - Currently there is 1 thread per workgroup
#### Simple Parser
the implementation of the Alloy litmus tests to wgsl is in simple_parser/run_test.py. To use:

python3 run_test.py -g 1 -tf /path/to/test.txt -r 1

this will copy the test into the litmus_test crate.

#### Sort_Tests.py
This script generates the website with tests sorted by progress model. 

