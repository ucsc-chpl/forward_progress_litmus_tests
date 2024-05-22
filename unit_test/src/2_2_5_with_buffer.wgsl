//2,0
//@group(0)
//@binding(0)
//var<storage,read_write> counter: atomic<u32>;

struct RWBuffer {
    counter: atomic<u32>,
    buffer_dummy: i32,
};

@group(0)
@binding(0)
var<storage,read_write> rwBuffer: RWBuffer;
var<workgroup> mem_0: atomic<i32>;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var gid_x:u32 = global_id.x;
    var local_dummy:i32;
	if(gid_x == 0){
        atomicStore(&mem_0, rwBuffer.buffer_dummy);
	}
	if(gid_x == 1){
        while(atomicLoad(&mem_0) != 0) {
            local_dummy = local_dummy + 1;
		}
	}
	atomicAdd(&rwBuffer.counter,1u);
}
