//3,0
@group(0)
@binding(0)
var<storage,read_write> counter: atomic<u32>;
var<workgroup> mem_0: atomic<i32>;

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
                        if(atomicExchange(&mem_0, 1) == 1){
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
                        if(atomicExchange(&mem_0, 0) == 0){
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
	if(gid_x == 2){
        terminate = 0u;
        while (true) {
            if(terminate == 1u) {
                break;
            }
            switch pc {
    			case 0u {
                        if(atomicExchange(&mem_0, 1) == 0){
                            terminate = 1u;
                        }
                        else {
                            pc = pc + 1u;
                        }
                        break;
                    }
        			case 1u {
                        if(atomicLoad(&mem_0) == 1) {
                            pc = 0u;
                        }
                        else {
                            pc = pc + 1u;
                        }
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
	atomicAdd(&counter,1u);
}
