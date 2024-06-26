@group(0)
@binding(0)
var<storage, read_write> counter: atomic<u32>; // this is used as both input and output for convenience
var<workgroup> mem_0: atomic<i32>;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var gid_x:u32 = global_id.x;
    var pc:u32 = 0;
    var terminate:u32;
    if(gid_x == 0) {
        terminate = 0u;

        while (true) {
            if(terminate == 1) {
                break;
            }
            switch pc {
                case 0u {
                    if(atomicExchange(&mem_0, 1) == 0) {
                        pc = 0u;
                    }
                    else {
                        pc = pc + 1u;
                    }
                    break;
                }
                case 1u {
                    terminate = 1u;
                    break;
                }
                default {

                }
            }
        }
    }
    if (gid_x == 1) {
        while(true) {
            if(terminate == 1){
                break;
            }
            switch pc {
                case 0u {
                    if(atomicExchange(&mem_0, 0) == 0) {
                        pc = 0u;
                    }
                    else {
                        pc = pc + 1u;
                    }
                    break;
                }
                case 1u {
                    terminate = 1u;
                    break;
                }
                default {

                }
            }
        }
    }
    atomicAdd(&counter, 1u);
    //sus??
    //atomicAdd(&counter, 1u);
}