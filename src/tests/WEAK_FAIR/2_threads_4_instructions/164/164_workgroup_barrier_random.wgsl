struct RWBuffer {
    counter: atomic<u32>,
    MAX_THREADS: u32,
    NUM_TESTING_THREADS: u32,
    //we currently only go up to 3 threads
    rand_idx_0: u32,
    rand_idx_1: u32,
    rand_idx_2: u32,
      mem_0: atomic<i32>,


};
@group(0)
@binding(0)
var<storage,read_write> rwBuffer: RWBuffer;

@compute
@workgroup_size(256, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>,
        @builtin(local_invocation_id) local_id: vec3<u32>,
        @builtin(workgroup_id) workgroup_id: vec3<u32>) {
    var gid_x:u32 = global_id.x;
    var pc:u32 = 0u;
    var terminate:u32;
    
    var local_x:u32 = local_id.x;
    var workgroup_x:u32 = workgroup_id.x;

    if(workgroup_x == 0 && local_x == rwBuffer.rand_idx_0 % 256){
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
                        if(atomicLoad(&rwBuffer.mem_0) == 0) {
                            terminate = 1u;
                        }
                        else { 
                            pc = pc + 1u;
                        }
                        break;
                    }
			case 2u {
                        if(atomicExchange(&rwBuffer.mem_0, 1) == 1){
                            pc = 1u;
                        }
                        else {
                            pc = pc + 1u;
                        }
                        break;
                    }

                case 3u {
                    terminate = 1u;
                    break;
                }
                default {
                    //shouldn't happen 
                }
    		}
		}
	}
if(workgroup_x == 1 && local_x == rwBuffer.rand_idx_1 % 256){
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
                    terminate = 1u;
                    break;
                }
                default {
                    //shouldn't happen 
                }
    		}
		}
	}

    workgroupBarrier();
	atomicAdd(&rwBuffer.counter,1u);
}
