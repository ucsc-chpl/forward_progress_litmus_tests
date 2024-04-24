import os
import sys
import argparse
import re

#QUESTIONS:
# - inter-workgroup vs intra-workgroup?


def gen_wgsl(in_file, wgsl_name='test.wgsl'):
    #parse file by thread, generate the wgsl
    wgsl_kernel = ''
    num_thread = 0
    with open(in_file, 'r') as file:
        while(True):
            line = file.readline()
            #not correct
            if(line == ''):
                #add more stuff
                break
            else if (re.match('THREAD[0-9]', line)):
                num_threads += 1
                cur_pc = 0
                thread = "if (gid_x == " + line.replace("THREAD", "") + ") {"
                while(True):
                    line = file.readline()
                    if(re.match('THREAD[0-9]')):
                        #do this without a goto??
                        break
                    if (re.match('atomic_exch_branch', line)):
                        args = re.match('atomic_exch_branch\((?P<arg0>[0-9]), (?P<arg1>[0-9]), (?P<arg2>[0-9]), (?P<arg3>[0-9])\)')
                        # switch on args
                        pass
                    else if(re.match('atomic_store', line)):
                        args = re.match('atomic_store\((?P<arg0>[0-9]), (?P<arg1>[0-9])\)')
                        # switch on args
                        pass
                    cur_pc += 1
    return wgsl_name, num_threads

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
    

def main():
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
        else
            gen_crate(sys.pwd())
if __name__ == '__main__':