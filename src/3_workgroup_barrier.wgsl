struct RWBuffer {
    counter: atomic<u32>,
    MAX_THREADS: u32,
    NUM_TESTING_THREADS: u32,
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
    var local_x:u32 = local_id.x;
    var workgroup_x:u32 = workgroup_id.x;

    var pc:u32 = 0u;
    var terminate:u32;
if(workgroup_x == 0 && local_x == 0){
        terminate = 0u;
        while (true) {
            if(terminate == 1u) {
                break;
            }
            switch pc {
			case 0u {
                        if(atomicExchange(&rwBuffer.mem_0, 1) == 0){
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
if(workgroup_x == 1 && local_x == 0){
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
    // make sure check is for 256*num_workgroups
	atomicAdd(&rwBuffer.counter,1u);
}