import os
import sys
import argparse
import re

#QUESTIONS:
# - inter-workgroup vs intra-workgroup?
# - put it in the test
# - thread 00 thread 0 workgroup

#for multiple workgroups
#thread = r'THREAD(?P<tid>[0-9]+)\,(?P<wg>[0-9]+)'
td = r'THREAD(?P<tid>[0-9]+)'
a_exch_br = r'atomic_exch_branch\((?P<arg0>[0-9]), (?P<arg1>[0-9]), (?P<arg2>[0-9]), (?P<arg3>[0-9])\)'
a_chk_br = r'atomic_chk_branch\((?P<arg0>[0-9]),(?P<arg1>[0-9]),(?P<arg2>[0-9])\)'
a_st = r'atomic_store\((?P<arg0>[0-9]),(?P<arg1>[0-9])\)'

def parse_a_exch_br(file, thread, pc):
    line = file.readline()
    print(line)
    args = re.match(a_exch_br, line)
    statement = f'''\t\t\tcase {pc}u {{
                    if(atomicExchange(&mem_{args['arg0']}, {args['arg2']}) == {args['arg1']}){{
                        pc = {args['arg3']}u;
                    }}
                    else {{
                        pc = pc + 1u;
                    }}
                    break;
                }}
    '''
    return statement

def parse_a_chk_br(file, thread, pc):
    line = file.readline()
    print(line)
    args = re.match(a_chk_br, line)
    statement = f'''\t\t\tcase {pc}u {{
                    //UNCLEAR FROM TEST CASES WHETHER THIS IS INTENDED BEHAVIOR
                    if(atomicAdd(&mem_{args['arg0']}, 0) == {args['arg2']}) {{
                        pc = {args['arg1']};
                    }}
                    else {{
                        pc = pc + 1;
                    }}
                    break;
                }}
    '''
    return statement

def parse_a_st(file, thread, pc):
    line = file.readline()
    print(line)
    args = re.match(a_st, line)
    statement = f'''\t\t\tcase {pc}u {{
                    atomicExchange(&mem_{args['arg0']}, {args['arg1']});
                    pc = pc + 1;
                    break;
                }}
    '''
    return statement

def parse_thread(wgsl_kernel, thread_id, file):
    pc = 0
    thread = f'\tif(gid_x == {thread_id})' + '''{
        terminate = 0u;
        while (true) {
            if(terminate == 1) {
                break;
            }
            switch pc {
    '''
    while (True):
        #peek next line
        pos = file.tell()
        line = file.readline()
        print(pc)
        file.seek(pos)
        if(re.match(td, line)):
            #end of thread
            print("end of thread reached")
            break
        elif(line == '\n'):
            #whitespace, ignore (advance to next line)
            line = file.readline()
            continue
        elif(re.match(a_exch_br, line)):
            thread += parse_a_exch_br(file, thread, pc)
        elif(re.match(a_chk_br, line)):
            thread += parse_a_chk_br(file, thread, pc)
        elif(re.match(a_st, line)):
            thread += parse_a_st(file, thread, pc)
        elif(line == ''):
            #end of file reached
            break
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
    thread += '\t\t}\n\t\t}\n\t}\n'
    return thread

def gen_wgsl(target_file, wgsl_name='test'):
    #TODO support declaring multiple mem locations
    wgsl_kernel = """@group(0)
@binding(0)
var<storage,read_write> counter: atomic<u32>;
var<workgroup> mem_0: atomic<i32>;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var gid_x:u32 = global_id.x;
    var pc:u32 = 0;
    var terminate:u32;
"""
    #TODO think about how to do intra workgroup stuff
    num_threads = 0;
    num_workgroups = 0;
    with open(target_file, 'r') as file:
        while (True):
            line = file.readline()
            if (line == ''):
                #end of file
                break;
            elif (line == '\n'):
                #whitespace, ignore
                continue
            elif (re.match(td, line)):
                args = re.match(td, line)
                wgsl_kernel += parse_thread(wgsl_kernel, args['tid'], file)
        file.close()
    #add to the 'done' counter when program finishes
    wgsl_kernel += '''\tatomicAdd(&counter,1u);
}
'''
    #add number of threads and workgroups so the wgpu stuff knows what to do
    wgsl_kernel = f'//{num_threads},{num_workgroups}\n' + wgsl_kernel
    with open(wgsl_name + '.wgsl', 'w') as out_file:
        out_file.write(wgsl_kernel)
        out_file.close()
    

def gen_crate(build_dir):
    if(os.is_dir(build_dir)):
        os.system(f'cd {build_dir}')
    else:
        os.system(f'mkdir {build_dir}')
        os.system(f'cd {build_dir}')
    os.system(f'cargo new litmus-tests')
    #TODO finish adding stuff
    os.system(f'cargo add ')
    #TODO write stuff to main

def run_test(dir, wgsl_name, num_threads, power_mode='low_power'):
    if(os.is_dir(dir)):
        os.system(f'cd {build_dir}')
    else:
        print("specified directory does not exist!")
        exit()
    pass
    #return success or timeout (fail)
    

def main_2():
    parser = argparse.ArgumentParser()

    parser.add_argument('-b', '--build', help='build crate')
    parser.add_argument('-d', '--build_dir', help='directory to build crate (defualts to .)')
    parser.add_argument('-tf', '--test_file', help='path to test file')
    parser.add_argument('-r', '--run_test', help='run the test')
    #see if you can pass the name of the device you want
    parser.add_argument('-p', '--power_mode', help='high power or low power')
    #work group stuff (inter workgroup vs intra workgroup)
    parser.add_argument('-wg', '--workgroups', help='UNIMPLEMENTED!')
    parser.add_argument('-w', '--wasm', help='compile to wasm 32 unknown-unknown')
    args = parser.parse_args()

    if(args.build):
        if(args.build_dir):
            gen_crate(ags.build_dir)
        else:
            gen_crate(sys.pwd())

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-tf', '--test_file', help='path to test file')
    parser.add_argument('-g', '--gen_wgsl', help='generate wgsl')
    args = parser.parse_args()
    if(args.gen_wgsl):
        if(args.test_file):
            gen_wgsl(args.test_file, 'test')
        else:
            print('pls specify path to test file')
if __name__ == '__main__':
    main()