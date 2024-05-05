# forward_progress_litmus_tests
wgpu forward progress tests. 

#### litmus test web (website)
this crate runs a litmus test in the browser. To compile use the command `wasm-pack build --target web`. Start a local server in the litmus_test_web/ directory and view in the browser (use chrome!). Click the run test button to run the test. 

#### Simple Parser
the implementation of the Alloy litmus tests to wgsl is in simple_parser/run_test.py. To use:

python3 run_test.py -g 1 -tf /path/to/test.txt -r 1

this will copy the test into the litmus_test crate.

#### litmus test crate
the litmus test crate runs the tests with wgpu. The default (cargo run with no arguments) runs the litmus_test.wgsl kernel with 2 threads. To run with a specific wgsl:

cargo run your_kernel.wgsl num_threads

#### issues
case statements: the wgsls use case statements to simulate gotos. Test case 2_threads_2_instructions 5 terminates with 2 threads when it is refactored to not use switch statements (2_thread_2_inst_5.wgsl in limus_test/), but with them fails. Same for replacing case statements with if statements.
