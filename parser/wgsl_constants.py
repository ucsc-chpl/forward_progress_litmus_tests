from enum import Enum
# should I also make a heursitic enum??
class Heuristics(Enum):
    SINGLE = 'SINGLE'
    ROUND_ROBIN = 'ROUND_ROBIN'
    CHUNKED = 'CHUNKED'

class Inst_REs(Enum):
    # TD_ID_RE   = r'THREAD(?P<tid>[0-9]+)\,(?P<wg>[0-9]+)'
    # thread header
    TD_RE     = r'THREAD(?P<tid>[0-9]+)'
    # atomic exchange branch instruction 
    # if(atomic_exch(arg0, arg2) == arg1) goto arg3
    # arg0 = memory address
    # arg1 = value to compare
    # arg2 = value to exchange
    # arg3 = branch pc
    A_EXCH_BR_RE = r'atomic_exch_branch\((?P<arg_0>[0-9]),(?P<arg_1>[0-9]),(?P<arg_2>[0-9]),((?P<arg_3>([0-9]|END)))\)'
    # atomic check branch instruction
    # if (atomicLoad(arg0) == arg1) goto arg2
    # arg0 = mem location to check
    # arg1 = value to check against
    # arg2 = branch pc
    A_CHK_BR_RE  = r'atomic_chk_branch\((?P<arg_0>[0-9]),(?P<arg_1>[0-9]),(?P<arg_2>([0-9]|END))\)'
    # atomic check branch instruction
    # if (atomicLoad(arg0) == arg1) goto arg2
    # arg0 = mem location to check
    # arg1 = value to check against
    # arg2 = branch pc
    A_ST_RE      = r'atomic_store\((?P<arg_0>[0-9]),(?P<arg_1>[0-9])\)'

# strings that are the same across wgsls
class BP_Strings(Enum):
    STRUCT_STR = '''struct RWBuffer {{
    counter: atomic<u32>,
    MAX_THREADS: i32,
    NUM_TESTING_THREADS: u32,
    {mem_locs}
}};'''

    # ERM
    SINGLE_THREAD_STR = '''if(gid_x == ?thread_id?){
        terminate = 0u;
        while (true) {
            if(terminate == 1u) {
                break;
            }
            switch pc {
?pc_insts?
                case ?last_pc?u {
                    terminate = 1u;
                    break;
                }
                default {
                    //shouldn't happen 
                }
    		}
		}
	}
'''

    ROUND_ROBIN_THREAD_STR = '''if(gid_x % num_testing_threads == ?thread_id?){
        terminate = 0u;
        while (true) {
            if(terminate == 1u) {
                break;
            }
            switch pc {
?pc_insts?
                case ?last_pc?u {
                    terminate = 1u;
                    break;
                }
                default {
                    //shouldn't happen 
                }
    		}
		}
	}
'''
    CHUNKED_THREAD_STR = '''if(gid_x / index == ?thread_id?){
        terminate = 0u;
        while (true) {
            if(terminate == 1u) {
                break;
            }
            switch pc {
?pc_insts?
                case ?last_pc?u {
                    terminate = 1u;
                    break;
                }
                default {
                    //shouldn't happen 
                }
    		}
		}
	}
'''
    BOILER_PLATE_SINGLE_STR = '''
@group(0)
@binding(0)
var<storage,read_write> rwBuffer: RWBuffer;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var gid_x:u32 = global_id.x;
    var pc:u32 = 0u;
    var terminate:u32;
'''
    BOILER_PLATE_ROUND_ROBIN_STR = '''
    var total_num_threads:i32 = rwBuffer.MAX_THREADS;
    var num_testing_threads:u32 = ?num_testing_threads?u;
    var index = gid_x / num_testing_threads;
'''
# percent symbol sus!
    BOILER_PLATE_CHUNKED_STR = '''
    var total_num_threads:u32 = rwBuffer.MAX_THREADS;
    var num_testing_threads:u32 = ?num_testing_threads?u;
    var chunk_size:u32 = total_num_threads / num_testing_threads;
    var index = gid_x % chunk_size;
'''

class Inst_Strings_Single(Enum):
    A_EXCH_BR_STR = '''\t\t\tcase ?pc?u {
                        if(atomicExchange(&rwBuffer.mem_?arg_0?, ?arg_2?) == ?arg_1?){
                            ?goto?
                        }
                        else {
                            pc = pc + 1u;
                        }
                        break;
                    }
'''

    A_CHK_BR_STR = '''\t\t\tcase ?pc?u {
                        if(atomicLoad(&rwBuffer.mem_?arg_0?) == ?arg_1?) {
                            ?goto?
                        }
                        else { 
                            pc = pc + 1u;
                        }
                        break;
                    }
'''
    A_ST_STR = '''\t\t\tcase ?pc?u {
                    atomicStore(&rwBuffer.mem_?arg_0?, ?arg_1?);
                    pc = pc + 1u;
                    break;
                }
    '''

# erm should probably do this with inheritance
class Inst_Strings_Round_Robin(Enum):
    A_EXCH_BR_STR = '''\t\t\tcase ?pc?u {
                        if(atomicExchange(&rwBuffer.mem_?arg_0?[index], ?arg_2?) == ?arg_1?){
                            ?goto?
                        }
                        else {
                            pc = pc + 1u;
                        }
                        break;
                    }
'''

    A_CHK_BR_STR = '''\t\t\tcase ?pc?u {
                        if(atomicLoad(&rwBuffer.mem_?arg_0?[index]) == ?arg_1?) {
                            ?goto?
                        }
                        else { 
                            pc = pc + 1u;
                        }
                        break;
                    }
'''
    A_ST_STR = '''\t\t\tcase ?pc?u {
                    atomicStore(&rwBuffer.mem_?arg_0?[index], ?arg_1?);
                    pc = pc + 1u;
                    break;
                }
    '''
# erm should probably do this with inheritance
class Inst_Strings_Chunked(Enum):
    A_EXCH_BR_STR = '''\t\t\tcase ?pc?u {
                        if(atomicExchange(&rwBuffer.mem_?arg_0?[index], ?arg_2?) == ?arg_1?){
                            ?goto?
                        }
                        else {
                            pc = pc + 1u;
                        }
                        break;
                    }
'''

    A_CHK_BR_STR = '''\t\t\tcase ?pc?u {
                        if(atomicLoad(&rwBuffer.mem_?arg_0?[index]) == ?arg_1?) {
                            ?goto?
                        }
                        else { 
                            pc = pc + 1u;
                        }
                        break;
                    }
'''
    A_ST_STR = '''\t\t\tcase ?pc?u {
                    atomicStore(&rwBuffer.mem_?arg_0?[index], ?arg_1?);
                    pc = pc + 1u;
                    break;
                }
    '''