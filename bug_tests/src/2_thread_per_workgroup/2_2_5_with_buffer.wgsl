
struct RWBuffer {
    counter: atomic<u32>,
    buffer_dummy: i32,
};

@group(0)
@binding(0)
var<storage,read_write> rwBuffer: RWBuffer;
var<workgroup> mem_0: atomic<i32>;

@compute
@workgroup_size(2)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var gid_x:u32 = global_id.x;
    var local_dummy:i32;
	if(gid_x == 0){
        // attemp to stop the compiler from optimizing
        atomicStore(&mem_0, rwBuffer.buffer_dummy);
	}
	if(gid_x == 1){
        // mem_0 can't ever be 2, should spin forever
        while(atomicLoad(&mem_0) != 2) {
            local_dummy = local_dummy + 1;
		}
	}
	atomicAdd(&rwBuffer.counter,1u);
}