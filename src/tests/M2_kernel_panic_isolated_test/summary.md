## M2 Kernel panic Isolated Test Case
#### Steps to recreate
1. launch a local server (steps in readme)
2. navigate to the m2 kernel panic page
3. run test
4. scroll down then wait two minutes. 

#### Test Details
This test case launches 500 runs of the strong fair 3_threads_4_instruction 0 test case. Note that it is launched with 2 threads, instead of 3, as the html had a bug.

#### Notes
- Not sure why, but it seems that the kernel panic won't happen until you try to scroll down, then approximately 2 minutes later the kernel panic happens.
- I tested with 500 shaders spawned, it probably takes less than that. 
- Its very possible there are more test cases that will cause the kernel panic with a bunch launched, this is the only one I've tested. To test others, you can change the path passed to the run_lots_of_tests() function in the index.html