#### Nvidia Issue Tests
These tests should not terminate, but do terminate on the Nvidia 4070 with driver release 535.
They have not terminated on any other GPU so far (Intel and Apple).

#### Running in the browser
In the bug_tests/ directory start a local server (e.g. python3 -m http.server) and open it in chrome. Select one of the tests and run by clicking the 'run test' button. If the test terminates it will display the number of threads finished (2). If it hangs, it will either display 'running test...' forever or terminate with 0 threads.
To apply changes to the wgpu or wgsls, run:
`wasm-pack build --target=web`
to rebuild the wasm

#### Setting Up Chrome and the Server
1. enable wgpu: go to chrome://flags and enable ”WebGPU Developer Features”
2. 

#### Running natively
In the bug_tests/ directory use
`cargo run` 
to run the default test or
`cargo run <wgsl_file> <num_threads>
to run a specific test. All of these tests are intended to be run with num_threads=2.