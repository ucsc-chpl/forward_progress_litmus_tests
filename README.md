# forward_progress_litmus_tests
wgpu forward progress tests. 

#### Simple Parser
the implementation of the Alloy litmus tests to wgsl is in simple_parser/run_test.py. To use:

python3 run_test.py -g 1 -tf /path/to/test.txt -r 1

this will copy the test into the litmus_test crate.

#### litmus test crate
the litmus test crate runs the tests with wgpu. The default (cargo run with no arguments) runs the litmus_test.wgsl kernel with 2 threads. To run with a specific wgsl:

cargo run your_kernel.wgsl num_threads
