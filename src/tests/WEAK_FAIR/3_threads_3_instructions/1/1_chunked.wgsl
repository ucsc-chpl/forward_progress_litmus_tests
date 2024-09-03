struct RWBuffer {
    counter: atomic<u32>,
    MAX_THREADS: u32,
    NUM_TESTING_THREADS: u32,
    //we currently only go up to 3 threads
    rand_idx_0: u32,
    rand_idx_1: u32,
    rand_idx_2: u32,
      mem_0: array<atomic<i32>,16>,


};
@group(0)
@binding(0)
var<storage,read_write> rwBuffer: RWBuffer;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>,
        @builtin(local_invocation_id) local_id: vec3<u32>,
        @builtin(workgroup_id) workgroup_id: vec3<u32>) {
    var gid_x:u32 = global_id.x;
    var pc:u32 = 0u;
    var terminate:u32;
    
    var total_num_threads:u32 = rwBuffer.MAX_THREADS;
    var num_testing_threads:u32 = ?num_testing_threads?u;
    var chunk_size:u32 = total_num_threads / num_testing_threads;
    var index = gid_x % chunk_size;

    if(gid_x / chunk_size == 0){
        terminate = 0u;
        while (true) {
            if(terminate == 1u) {
                break;
            }
            switch pc {
			case 0u {
                    atomicStore(&rwBuffer.mem_0[index], 1);
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
if(gid_x / chunk_size == 1){
        terminate = 0u;
        while (true) {
            if(terminate == 1u) {
                break;
            }
            switch pc {
			case 0u {
                        if(atomicLoad(&rwBuffer.mem_0[index]) == 1) {
                            pc = 1u;
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
if(gid_x / chunk_size == 2){
        terminate = 0u;
        while (true) {
            if(terminate == 1u) {
                break;
            }
            switch pc {
			case 0u {
                        if(atomicExchange(&rwBuffer.mem_0[index], 0) == 0){
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

    
	atomicAdd(&rwBuffer.counter,1u);
}
