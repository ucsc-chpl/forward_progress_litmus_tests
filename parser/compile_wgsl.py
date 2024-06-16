import os
import sys
import argparse
import re

# TODO make all variables either signed or unsigned (currently consistent but arbitrary)
# TODO make the number of global buffer variables passed based on the max mem locations in the test cases (currently hard coded and some aren't used)
# TODO think about how to do intra workgroup stuff

#for multiple threads per workgroup (not yet implemented)
#thread = r'THREAD(?P<tid>[0-9]+)\,(?P<wg>[0-9]+)'

# thread header
td = r'THREAD(?P<tid>[0-9]+)'

# atomic exchange branch instruction 
# if(atomic_exch(arg0, arg2) == arg1) goto arg3
# arg0 = memory address
# arg1 = value to compare
# arg2 = value to exchange
# arg3 = branch pc
a_exch_br = r'atomic_exch_branch\((?P<arg0>[0-9]),(?P<arg1>[0-9]),(?P<arg2>[0-9]),((?P<arg3>([0-9]|END)))\)'

# atomic check branch instruction
# if (atomicLoad(arg0) == arg1) goto arg2
# arg0 = mem location to check
# arg1 = value to check against
# arg2 = branch pc
a_chk_br = r'atomic_chk_branch\((?P<arg0>[0-9]),(?P<arg1>[0-9]),(?P<arg2>([0-9]|END))\)'

# atomic store
# you know how a store works
# arg0 = mem location to store to
# arg1 = value to store
a_st = r'atomic_store\((?P<arg0>[0-9]),(?P<arg1>[0-9])\)'

# parses atomic exch branch instructions, returns associated wgsl code
def parse_a_exch_br(file, thread, pc, mem_locs):
    line = file.readline()
    args = re.match(a_exch_br, line)
    mem_locs.add(args['arg0'])
    if(args['arg3'] == 'END'):
        statement = f'''\t\t\tcase {pc}u {{
                        if(atomicExchange(&rwBuffer.mem_{args['arg0']}, {args['arg2']}) == {args['arg1']}){{
                            terminate = 1u;
                        }}
                        else {{
                            pc = pc + 1u;
                        }}
                        break;
                    }}
        '''
    else:
        statement = f'''\t\t\tcase {pc}u {{
                        if(atomicExchange(&rwBuffer.mem_{args['arg0']}, {args['arg2']}) == {args['arg1']}){{
                            pc = {args['arg3']}u;
                        }}
                        else {{
                            pc = pc + 1u;
                        }}
                        break;
                    }}
        '''
    return statement

# parses atomic check branch instructions, returns associated wgsl code
def parse_a_chk_br(file, thread, pc, mem_locs, target_file):
    line = file.readline()
    #print(line)
    args = re.match(a_chk_br, line)
    mem_locs.add(args['arg0'])
    if (args['arg2'] == 'END'):
        statement = f'''\t\t\tcase {pc}u {{
                        if(atomicLoad(&rwBuffer.mem_{args['arg0']}) == {args['arg1']}) {{
                            terminate = 1u;
                        }}
                        else {{
                            pc = pc + 1u;
                        }}
                        break;
                    }}
        '''
    else:
        statement = f'''\t\t\tcase {pc}u {{
                        if(atomicLoad(&rwBuffer.mem_{args['arg0']}) == {args['arg1']}) {{
                            pc = {args['arg2']}u;
                        }}
                        else {{
                            pc = pc + 1u;
                        }}
                        break;
                    }}
        '''
    return statement

# parses atomic store instructions, returns associated wgsl code
def parse_a_st(file, thread, pc, mem_locs):
    line = file.readline()
    #print(line)
    args = re.match(a_st, line)
    mem_locs.add(args['arg0'])
    statement = f'''\t\t\tcase {pc}u {{
                    atomicStore(&rwBuffer.mem_{args['arg0']}, {args['arg1']});
                    pc = pc + 1u;
                    break;
                }}
    '''
    return statement

# generates the wgsl code for a single thread
def parse_thread(wgsl_kernel, thread_id, file, mem_locs, target_file):
    pc = 0
    thread = f'\tif(gid_x == {thread_id})' + '''{
        terminate = 0u;
        while (true) {
            if(terminate == 1u) {
                break;
            }
            switch pc {
    '''
    while (True):
        #peek next line
        pos = file.tell()
        line = file.readline()
        file.seek(pos)
        # wouldn't a switch statement be lovely right here

        #end of thread
        if(re.match(td, line)):
            break
        #whitespace, ignore (advance to next line)
        elif(line == '\n'):
            line = file.readline()
            continue
        # parse atomic exch branch
        elif(re.match(a_exch_br, line)):
            thread += parse_a_exch_br(file, thread, pc, mem_locs)
        # parse atomic check branch
        elif(re.match(a_chk_br, line)):
            thread += parse_a_chk_br(file, thread, pc, mem_locs, target_file)
        # parse atomic store
        elif(re.match(a_st, line)):
            thread += parse_a_st(file, thread, pc, mem_locs)
        # end of file reached
        elif(line == ''):
            break
        # parsing error
        else:
            print(f'Parser error! offending line: {line}')
            exit()
        pc += 1
    thread += f'''\t\t\tcase {pc}u {{
                    terminate = 1u;
                    break;
                }}
                default {{
                    //shouldn't happen 
                }}
    '''
    # add the last curly braces
    thread += '\t\t}\n\t\t}\n\t}\n'
    return thread

# generates the entire wgsl
def gen_wgsl(target_file, wgsl_name='test'):
    print(f"parsing target file : {target_file}")
    wgsl_kernel = ""
    mem_locs = set()
    num_threads = 0
    num_workgroups = 0
    with open(target_file, 'r') as file:
        while (True):
            line = file.readline()
            if (line == ''):
                #end of file
                break
            elif (line == '\n'):
                #whitespace, ignore
                continue
            elif (re.match(td, line)):
                args = re.match(td, line)
                wgsl_kernel += parse_thread(wgsl_kernel, args['tid'], file, mem_locs, target_file)
                num_threads += 1
        file.close()
    #add to the 'done' counter when program finishes
    wgsl_kernel += '''\tatomicAdd(&rwBuffer.counter,1u);
}
'''
    # stitch all of the generated code together
    preamble = f'//{num_threads},{num_workgroups}\n' + """
struct RWBuffer {
    counter: atomic<u32>,
"""
    for loc in mem_locs:
        preamble += f"  mem_{loc}: atomic<i32>,\n"
    print(f"num mem locs: {len(mem_locs)}")
    preamble += """};
@group(0)
@binding(0)
var<storage,read_write> rwBuffer: RWBuffer;
"""
    preamble += """
@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var gid_x:u32 = global_id.x;
    var pc:u32 = 0u;
    var terminate:u32;
""" 
    wgsl_kernel = preamble + wgsl_kernel
    with open(wgsl_name, 'w') as out_file:
        out_file.write(wgsl_kernel)
        out_file.close()

# for testing, ignore
def test():
    pass

# run the program with command line arguments
def main():
    # command line arguments
    parser = argparse.ArgumentParser()
    # path to test txt file (in alloy_forward_progress)
    parser.add_argument('-tf', '--test_file', help='path to test file')
    # generate wgsl
    parser.add_argument('-g', '--gen_wgsl', help='generate wgsl')
    parser.add_argument('-o', '--out_file', help='path to output wgsl')
    parser.add_argument('-t', '--test', help='for testing, ignore')
    args = parser.parse_args()

    if(args.test):
        test()
    else:
        if(args.gen_wgsl):
            if(args.test_file):
                if(args.out_file):
                    gen_wgsl(args.test_file, args.out_file)
                    with open(args.out_file) as file:
                        top = re.match(r'\/\/(?P<num_threads>[0-9]+)\,(?P<num_workgroups>[0-9]+)', file.readline())
                        print(top)
                        num_threads = top['num_threads']
                        num_workgroups = top['num_workgroups']
                        file.close()
                
                    run_test(args.test_file.replace('.txt', '.wgsl'), num_threads)
                else:
                    print("please specify outfile")
            else:
                print('pls specify path to test file')
        else:
            if(args.test_file):
                gen_wgsl(args.test_file, 'test')
            else:
                print('pls specify path to test file')

if __name__ == '__main__':
    main()