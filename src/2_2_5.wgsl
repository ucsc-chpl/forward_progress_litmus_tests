//2,0
@group(0)
@binding(0)
var<storage,read_write> counter: atomic<u32>;
var<workgroup> mem_0: atomic<i32>;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var gid_x:u32 = global_id.x;
    var pc:u32 = 0;
    var spin:u32;
	if(gid_x == 0){
        atomicStore(&mem_0, 1);
	}
	if(gid_x == 1){
        spin = 1u;
        while (spin == 1u) {
            if(atomicLoad(&mem_0) == 0) {
                spin = 1u;
            }
            else {
                spin = 0u;
            }
		}
	}
	atomicAdd(&counter,1u);
}
