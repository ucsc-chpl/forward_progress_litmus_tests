@group(0)
@binding(0)
var<storage, read_write> counter: atomic<u32>; // this is used as both input and output for convenience
var<workgroup> test: atomic<i32>;

fn add_one(n_base: u32, global_id: u32) -> u32{
    var n: u32 = n_base;
    n = n + global_id;
    return n;
}

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
                    if(atomicExchange(&test, 1) == 0) {
                        pc = 0u;
                    }
                    else {
                        pc = pc + 1u;
                    }
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
                    if(atomicExchange(&test, 0) == 0) {
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