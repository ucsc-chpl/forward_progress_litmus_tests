import os
import sys
import argparse
import re

#QUESTIONS:
# how do I set up web server on shrike I can access from my laptop?
#  / how what tool is best / etc


# INITIALIZE mem in wgls
#for multiple workgroups
#thread = r'THREAD(?P<tid>[0-9]+)\,(?P<wg>[0-9]+)'
td = r'THREAD(?P<tid>[0-9]+)'
a_exch_br = r'atomic_exch_branch\((?P<arg0>[0-9]), (?P<arg1>[0-9]), (?P<arg2>[0-9]), (?P<arg3>[0-9])\)'
a_chk_br = r'atomic_chk_branch\((?P<arg0>[0-9]),(?P<arg1>[0-9]),(?P<arg2>[0-9])\)'
a_st = r'atomic_store\((?P<arg0>[0-9]),(?P<arg1>[0-9])\)'

def parse_a_exch_br(file, thread, pc, mem_locs):
    line = file.readline()
    print(line)
    args = re.match(a_exch_br, line)
    mem_locs.add(args['arg0'])
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

def parse_a_chk_br(file, thread, pc, mem_locs):
    line = file.readline()
    print(line)
    args = re.match(a_chk_br, line)
    mem_locs.add(args['arg0'])
    statement = f'''\t\t\tcase {pc}u {{
                    //UNCLEAR FROM TEST CASES WHETHER THIS IS INTENDED BEHAVIOR
                    if(atomicLoad(&mem_{args['arg0']}) == {args['arg2']}) {{
                        pc = {args['arg1']}u;
                    }}
                    else {{
                        pc = pc + 1u;
                    }}
                    break;
                }}
    '''
    return statement

def parse_a_st(file, thread, pc, mem_locs):
    line = file.readline()
    print(line)
    args = re.match(a_st, line)
    mem_locs.add(args['arg0'])
    statement = f'''\t\t\tcase {pc}u {{
                    atomicStore(&mem_{args['arg0']}, {args['arg1']});
                    pc = pc + 1u;
                    break;
                }}
    '''
    return statement

def parse_thread(wgsl_kernel, thread_id, file, mem_locs):
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
            thread += parse_a_exch_br(file, thread, pc, mem_locs)
        elif(re.match(a_chk_br, line)):
            thread += parse_a_chk_br(file, thread, pc, mem_locs)
        elif(re.match(a_st, line)):
            thread += parse_a_st(file, thread, pc, mem_locs)
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
    wgsl_kernel = ""
    #TODO think about how to do intra workgroup stuff
    mem_locs = set()
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
                wgsl_kernel += parse_thread(wgsl_kernel, args['tid'], file, mem_locs)
                num_threads += 1
        file.close()
    #add to the 'done' counter when program finishes
    wgsl_kernel += '''\tatomicAdd(&counter,1u);
}
'''
    #declare all the vars and stuff
    preamble = f'//{num_threads},{num_workgroups}\n' + """@group(0)
@binding(0)
var<storage,read_write> counter: atomic<u32>;
"""
    for loc in mem_locs:
        preamble += f"var<workgroup> mem_{loc}: atomic<i32>;\n"
    preamble += """
@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    var gid_x:u32 = global_id.x;
    var pc:u32 = 0;
    var terminate:u32;
""" 
    wgsl_kernel = preamble + wgsl_kernel
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

def run_test(wgsl_name, 
            num_threads, 
            num_workgroups=1, 
            power_mode='low_power', 
            dir='/home/nrehman/forward_progress_litmus_tests/litmus_test/'):
    if(os.path.isdir(dir)):
        os.system(f'cp {wgsl_name} {dir}')
        #os.system(f'cd {dir}')
        print(f'wgsl kernel copied into {dir}\nkernel name: {wgsl_name}\nnum_threads: {num_threads}\ncommand: cd {dir} && cargo run {wgsl_name} {num_threads}')
    else:
        print("specified directory does not exist!")
        exit()
    pass
    #cmd = f'cargo run {wgsl_name} {num_threads}'
    #os.system(cmd)
    #return success or timeout (fail)

# for testing, ignore
def test():
    pass

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
    parser.add_argument('-r', '--run', help='run application')
    parser.add_argument('-t', '--test', help='for testing, ignore')
    args = parser.parse_args()

    if(args.test):
        test()
    else:
        if(args.run):
            if(args.gen_wgsl):
                if(args.test_file):
                    gen_wgsl(args.test_file, args.test_file.replace('.txt',''))
                    with open(args.test_file.replace('.txt', '.wgsl')) as file:
                        top = re.match(r'\/\/(?P<num_threads>[0-9]+)\,(?P<num_workgroups>[0-9]+)', file.readline())
                        print(top)
                        num_threads = top['num_threads']
                        num_workgroups = top['num_workgroups']
                        file.close()
                    
                    run_test(args.test_file.replace('.txt', '.wgsl'), num_threads)
                else:
                    print('pls specify path to test file')
            else:
                if(args.test_file):
                    gen_wgsl(args.test_file, 'test')
                else:
                    print('pls specify path to test file')

if __name__ == '__main__':
    main()