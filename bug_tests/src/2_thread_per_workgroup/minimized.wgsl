
@group(0)
@binding(0)
var<storage,read_write> counter: atomic<u32>;
var<workgroup> mem_0: atomic<i32>;

@compute
@workgroup_size(2)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var gid_x:u32 = global_id.x;
	if(gid_x == 0){
        atomicStore(&mem_0, 1);
	}
	if(gid_x == 1){
        // can't mem_0 can't ever be 2
        while (atomicLoad(&mem_0) != 2) {

        }
	}
	atomicAdd(&counter,1u);
}