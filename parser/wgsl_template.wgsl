struct RWBuffer {
    counter: atomic<u32>,
    MAX_THREADS: u32,
    NUM_TESTING_THREADS: u32,
    //we currently only go up to 3 threads
    rand_idx_0: u32,
    rand_idx_1: u32,
    rand_idx_2: u32,
    ?mem_locs?

};
@group(0)
@binding(0)
var<storage,read_write> rwBuffer: RWBuffer;

@compute
?workgroup_size?
fn main(@builtin(global_invocation_id) global_id: vec3<u32>,
        @builtin(local_invocation_id) local_id: vec3<u32>,
        @builtin(workgroup_id) workgroup_id: vec3<u32>) {
    var gid_x:u32 = global_id.x;
    var pc:u32 = 0u;
    var terminate:u32;
    ?setup?
    ?thread_code?
    ?syncronize?
	atomicAdd(&rwBuffer.counter,1u);
}