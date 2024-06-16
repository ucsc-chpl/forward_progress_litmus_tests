## M2 Kernel panic Isolated Test Case
#### Steps to recreate
1. launch a local server (steps in readme)
2. navigate to the m2 kernel panic page
3. run test
4. scroll down then wait two minutes. Not sure why, but it seems that the kernel panic won't happen until you try to scroll down, then exactly 2 minutes later the kernel panic happens

#### Contents
This test case launches 500 runs of the strong fair 3_threads_4_instruction test case. Note that it is launched with 2 threads, instead of 3, as the html had a bug.