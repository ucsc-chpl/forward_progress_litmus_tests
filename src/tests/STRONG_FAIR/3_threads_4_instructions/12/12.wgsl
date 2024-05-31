//3,0

struct RWBuffer {
    counter: atomic<u32>,
  mem_0: atomic<i32>,
};
@group(0)
@binding(0)
var<storage,read_write> rwBuffer: RWBuffer;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var gid_x:u32 = global_id.x;
    var pc:u32 = 0u;
    var terminate:u32;
	if(gid_x == 0){
        terminate = 0u;
        while (true) {
            if(terminate == 1u) {
                break;
            }
            switch pc {
    			case 0u {
                        if(atomicLoad(&rwBuffer.mem_0) == 0) {
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
                    //shouldn't happen 
                }
    		}
		}
	}
	if(gid_x == 1){
        terminate = 0u;
        while (true) {
            if(terminate == 1u) {
                break;
            }
            switch pc {
    			case 0u {
                    atomicStore(&rwBuffer.mem_0, 1);
                    pc = pc + 1u;
                    break;
                }
    			case 1u {
                    terminate = 1u;
                    break;
                }
                default {
                    //shouldn't happen 
                }
    		}
		}
	}
	if(gid_x == 2){
        terminate = 0u;
        while (true) {
            if(terminate == 1u) {
                break;
            }
            switch pc {
    			case 0u {
                        if(atomicExchange(&rwBuffer.mem_0, 0) == 0){
                            pc = 0u;
                        }
                        else {
                            pc = pc + 1u;
                        }
                        break;
                    }
        			case 1u {
                    atomicStore(&rwBuffer.mem_0, 1);
                    pc = pc + 1u;
                    break;
                }
    			case 2u {
                    terminate = 1u;
                    break;
                }
                default {
                    //shouldn't happen 
                }
    		}
		}
	}
	atomicAdd(&rwBuffer.counter,1u);
}
