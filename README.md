# forward_progress_litmus_tests
wgpu forward progress tests. 

#### main_with_workgroup_bug branch
This branch contains all of the test cases compiled with the workgroup bug, i.e. the variables that were supposed to be global variables were worgroup variables. 

#### litmus test web (website)
This crate runs a litmus test in the browser. Steps to run:
- Compile use the command `wasm-pack build --target web`. 
- Start a local server in the litmus_test_web/ directory with `python3 -m http.server`
- view in the browser at localhost:8000/ (use chrome!). 
- Click view tests then select a test and run with the run button.
- If a test case finishes it will display the number of threads that terminated.

#### Simple Parser
the implementation of the Alloy litmus tests to wgsl is in simple_parser/run_test.py. To use:

python3 run_test.py -g 1 -tf /path/to/test.txt -r 1

this will copy the test into the litmus_test crate.

#### Sort_Tests.py
This script generates the website with tests sorted by progress model. 

#### litmus test crate
the litmus test crate runs the tests with wgpu. The default (cargo run with no arguments) runs the litmus_test.wgsl kernel with 2 threads. To run with a specific wgsl:

cargo run your_kernel.wgsl num_threads

#### issues
case statements: the wgsls use case statements to simulate gotos. Test case 2_threads_2_instructions 5 terminates with 2 threads when it is refactored to not use switch statements (2_thread_2_inst_5.wgsl in limus_test/), but with them fails. Same for replacing case statements with if statements.
