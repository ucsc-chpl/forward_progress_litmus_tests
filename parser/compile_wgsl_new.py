import os
import sys
import argparse
import re
from wgsl_constants import Inst_REs, BP_Strings, Inst_Strings_Single, Inst_Strings_Round_Robin, Inst_Strings_Chunked, Inst_Strings_Workgroup_Barrier

# TODO make all svariables either signed or unsigned (currently consistent but arbitrary)
# TODO make the number of global buffer variables passed based on the max mem locations in the test cases (currently hard coded and some aren't used)
# TODO think about how to do intra workgroup stuff

# TODO add more constants for things like max num threads etc.




# python format is a nightmare with all of these curly braces
def cust_format(string: str, replacements: dict):
    for key, value in replacements.items():
        string = string.replace('?' + key + '?', str(value))
    return string

# parses atomic exch branch instructions, returns associated wgsl code
def parse_a_exch_br(file, pc, mem_locs, heuristic='single'):
    line = file.readline()
    args = re.match(Inst_REs.A_EXCH_BR_RE.value, line)
    mem_locs.add(args['arg_0'])
    statement = ''
    if(args['arg_3'] == 'END'):
        if heuristic == 'single':
            statement = cust_format(Inst_Strings_Single.A_EXCH_BR_STR.value, {'pc': pc,'arg_0': args['arg_0'], 'arg_1': args['arg_1'], 'arg_2' : args['arg_2'], 'goto' : 'terminate = 1u;'})
        elif heuristic == 'round_robin':
            statement = cust_format(Inst_Strings_Round_Robin.A_EXCH_BR_STR.value, {'pc': pc,'arg_0': args['arg_0'], 'arg_1': args['arg_1'], 'arg_2' : args['arg_2'], 'goto' : 'terminate = 1u;'})
        elif heuristic == 'chunked':
            statement = cust_format(Inst_Strings_Chunked.A_EXCH_BR_STR.value, {'pc': pc,'arg_0': args['arg_0'], 'arg_1': args['arg_1'], 'arg_2' : args['arg_2'], 'goto' : 'terminate = 1u;'})
        elif heuristic == 'workgroup_barrier'or 'workgroup_barrier_random':
            statement = cust_format(Inst_Strings_Workgroup_Barrier.A_EXCH_BR_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1'], 'arg_2' : args['arg_2'], 'goto' : 'terminate = 1u;'})
        else:
            print(f"invalid heuristic! {heuristic}")
    else:
        if heuristic == 'single':
            statement = cust_format(Inst_Strings_Single.A_EXCH_BR_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1'], 'arg_2' : args['arg_2'], 'goto' : f'pc = {(args["arg_3"])}u;'})
        elif heuristic == 'round_robin':
            statement = cust_format(Inst_Strings_Round_Robin.A_EXCH_BR_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1'], 'arg_2' : args['arg_2'], 'goto' : f'pc = {(args["arg_3"])}u;'})
        elif heuristic == 'chunked':
            statement = cust_format(Inst_Strings_Chunked.A_EXCH_BR_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1'], 'arg_2' : args['arg_2'], 'goto' : f'pc = {(args["arg_3"])}u;'})
        elif heuristic == 'workgroup_barrier'or 'workgroup_barrier_random':
            statement = cust_format(Inst_Strings_Workgroup_Barrier.A_EXCH_BR_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1'], 'arg_2' : args['arg_2'], 'goto' : f'pc = {(args["arg_3"])}u;'})
        else:
            print(f"invalid heuristic! {heuristic}")
    return statement

# parses atomic check branch instructions, returns associated wgsl code
def parse_a_chk_br(file, pc, mem_locs, heuristic='single'):
    line = file.readline()
    #print(line)
    args = re.match(Inst_REs.A_CHK_BR_RE.value, line)
    mem_locs.add(args['arg_0'])
    statement = ''
    if (args['arg_2'] == 'END'):
        if heuristic == 'single':
            statement = cust_format(Inst_Strings_Single.A_CHK_BR_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1'], 'goto' : 'terminate = 1u;'})
        elif heuristic == 'round_robin':
            statement = cust_format(Inst_Strings_Round_Robin.A_CHK_BR_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1'], 'goto' : 'terminate = 1u;'})
        elif heuristic == 'chunked':
            statement = cust_format(Inst_Strings_Chunked.A_CHK_BR_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1'], 'goto' : 'terminate = 1u;'})
        elif heuristic == 'workgroup_barrier'or 'workgroup_barrier_random':
            statement = cust_format(Inst_Strings_Workgroup_Barrier.A_CHK_BR_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1'], 'goto' : 'terminate = 1u;'})
        else:
            print(f"invalid heuristic! {heuristic}")
    else:
        if heuristic == 'single':
            statement = cust_format(Inst_Strings_Single.A_CHK_BR_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1'], 'goto' : f'pc = {(args["arg_1"])}u;'})
        elif heuristic == 'round_robin':
            statement = cust_format(Inst_Strings_Round_Robin.A_CHK_BR_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1'], 'goto' : f'pc = {(args["arg_1"])}u;'})
        elif heuristic == 'chunked':
            statement = cust_format(Inst_Strings_Chunked.A_CHK_BR_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1'], 'goto' : f'pc = {(args["arg_1"])}u;'})
        elif heuristic == 'workgroup_barrier'or 'workgroup_barrier_random':
            statement = cust_format(Inst_Strings_Workgroup_Barrier.A_CHK_BR_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1'], 'goto' : f'pc = {(args["arg_1"])}u;'})
        else:
            print(f"invalid heuristic! {heuristic}")
    return statement

# parses atomic store instructions, returns associated wgsl code
def parse_a_st(file, pc, mem_locs, heuristic='single'):
    line = file.readline()
    #print(line)
    args = re.match(Inst_REs.A_ST_RE.value, line)
    mem_locs.add(args['arg_0'])
    statement = ''
    if heuristic == 'single':
        statement =  cust_format(Inst_Strings_Single.A_ST_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1']})
    elif heuristic == 'round_robin':
        statement =  cust_format(Inst_Strings_Round_Robin.A_ST_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1']})
    elif heuristic == 'chunked':
        statement =  cust_format(Inst_Strings_Chunked.A_ST_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1']})
    elif heuristic == 'workgroup_barrier' or 'workgroup_barrier_random':
        statement = cust_format(Inst_Strings_Workgroup_Barrier.A_ST_STR.value, {'pc': pc, 'arg_0': args['arg_0'], 'arg_1': args['arg_1']})
    else:
        pass
    return statement

# generates the wgsl code for a single thread
def parse_thread(wgsl_kernel, thread_id, file, mem_locs, target_file, heuristic):
    pc = 0
    pc_insts = ''
    while (True):
        #peek next line
        pos = file.tell()
        line = file.readline()
        file.seek(pos)
        # wouldn't a switch statement be lovely right here

        #end of thread
        if(re.match(Inst_REs.TD_RE.value, line)):
            break
        #whitespace, ignore (advance to next line)
        elif(line == '\n'):
            line = file.readline()
            continue
        # parse atomic exch branch
        elif(re.match(Inst_REs.A_EXCH_BR_RE.value, line)):
            pc_insts += parse_a_exch_br(file, pc, mem_locs, heuristic=heuristic)
        # parse atomic check branch
        elif(re.match(Inst_REs.A_CHK_BR_RE.value, line)):
            pc_insts += parse_a_chk_br(file, pc, mem_locs, heuristic=heuristic)
        # parse atomic store
        elif(re.match(Inst_REs.A_ST_RE.value, line)):
            pc_insts += parse_a_st(file, pc, mem_locs, heuristic=heuristic)
        # end of file reached
        elif(line == ''):
            break
        # parsing error
        else:
            print(f'Parser error! offending line: {line}')
            exit()
        pc += 1
    if heuristic == 'single':
        thread = cust_format(BP_Strings.SINGLE_THREAD_STR.value, {'thread_id':thread_id, 'pc_insts':pc_insts, 'last_pc':pc})
    elif heuristic == 'round_robin':
        thread = cust_format(BP_Strings.ROUND_ROBIN_THREAD_STR.value, {'thread_id':thread_id, 'pc_insts':pc_insts, 'last_pc':pc})
    elif heuristic == 'chunked':
        thread = cust_format(BP_Strings.CHUNKED_THREAD_STR.value, {'thread_id':thread_id, 'pc_insts':pc_insts, 'last_pc':pc})
    elif heuristic == 'workgroup_barrier':
        thread = cust_format(BP_Strings.WORKGROUP_BARRIER_THREAD_STR.value, {'thread_id':thread_id, 'pc_insts':pc_insts, 'last_pc':pc})
    elif heuristic == 'workgroup_barrier_random':
        thread = cust_format(BP_Strings.WORKGROUP_BARRIER_RANDOM_THREAD_STR.value, {'thread_id':thread_id, 'pc_insts':pc_insts, 'last_pc':pc})

    return thread

# all the file i/o will tank perf on this, eventually optimize to keep stuff in mem
def gen_wgsl_single(target_file, wgsl_name):
    with open('wgsl_template.wgsl', 'r') as wgsl_template_file:
        wgsl_template_str = wgsl_template_file.read()
        print(f"parsing target file : {target_file}")
        mem_locs = set()
        num_threads = 0
        num_workgroups = 0
        thread_code_str = ''
        # generate thread code
        with open(target_file, 'r') as file:
            while (True):
                line = file.readline()
                if (line == ''):
                    #end of file
                    break
                elif (line == '\n'):
                    #whitespace, ignore
                    continue
                elif (re.match(Inst_REs.TD_RE.value, line)):
                    args = re.match(Inst_REs.TD_RE.value, line)
                    thread_code_str += parse_thread(thread_code_str, args['tid'], file, mem_locs, target_file, 'single')
                    num_threads += 1
            file.close()
        #add to the 'done' counter when program finishes
        mem_locs_str = ''
        for loc in mem_locs:
            mem_locs_str += f'  mem_{loc}: atomic<i32>,\n'
        # stitch all of the generated code together
        replacements = {'mem_locs' : mem_locs_str,
                        'workgroup_size' : r'@workgroup_size(1)',
                        'setup' : '',
                        'thread_code' : thread_code_str,
                        'syncronize' : ''}
        wgsl =cust_format(wgsl_template_str, replacements)
        print(f"num mem locs: {len(mem_locs)}")
        with open(wgsl_name, 'w') as out_file:
            out_file.write(wgsl)
            out_file.close()

def gen_wgsl_round_robin(target_file, wgsl_name):
    with open('wgsl_template.wgsl', 'r') as wgsl_template_file:
        wgsl_template_str = wgsl_template_file.read()
        print(f"parsing target file : {target_file}")
        mem_locs = set()
        num_threads = 0
        num_workgroups = 0
        thread_code_str = ''
        with open(target_file, 'r') as file:
            while (True):
                line = file.readline()
                if (line == ''):
                    #end of file
                    break
                elif (line == '\n'):
                    #whitespace, ignore
                    continue
                elif (re.match(Inst_REs.TD_RE.value, line)):
                    args = re.match(Inst_REs.TD_RE.value, line)
                    thread_code_str += parse_thread(thread_code_str, args['tid'], file, mem_locs, target_file, 'round_robin')
                    num_threads += 1
            file.close()
        mem_locs_str = ''
        for loc in mem_locs:
            mem_locs_str += f'  mem_{loc}: array<atomic<i32>,16>,\n'
        print(f"num mem locs: {len(mem_locs)}")
        # stitch all of the generated code together
        replacements = {'mem_locs' : mem_locs_str,
                        'workgroup_size' : r'@workgroup_size(1)',
                        'setup' : cust_format(BP_Strings.BOILER_PLATE_ROUND_ROBIN_STR.value,{'num_testing_threads': num_threads}),
                        'thread_code' : thread_code_str,
                        'syncronize' : ''}
        
        wgsl =cust_format(wgsl_template_str, replacements)
        print(f"num mem locs: {len(mem_locs)}")
        with open(wgsl_name, 'w') as out_file:
            out_file.write(wgsl)
            out_file.close()

def gen_wgsl_chunked(target_file, wgsl_name):
    with open('wgsl_template.wgsl', 'r') as wgsl_template_file:
        wgsl_template_str = wgsl_template_file.read()
        print(f"parsing target file : {target_file}")
        mem_locs = set()
        num_threads = 0
        num_workgroups = 0
        thread_code_str = ''
        with open(target_file, 'r') as file:
            while (True):
                line = file.readline()
                if (line == ''):
                    #end of file
                    break
                elif (line == '\n'):
                    #whitespace, ignore
                    continue
                elif (re.match(Inst_REs.TD_RE.value, line)):
                    args = re.match(Inst_REs.TD_RE.value, line)
                    thread_code_str += parse_thread(thread_code_str, args['tid'], file, mem_locs, target_file, 'chunked')
                    num_threads += 1
            file.close()
        mem_locs_str = ''
        for loc in mem_locs:
            mem_locs_str += f'  mem_{loc}: array<atomic<i32>,16>,\n'
        print(f"num mem locs: {len(mem_locs)}")
        # stitch all of the generated code together
        replacements = {'mem_locs' : mem_locs_str,
                        'workgroup_size' : r'@workgroup_size(1)',
                        'setup' : cust_format(BP_Strings.BOILER_PLATE_CHUNKED_STR.value,{'num_testing_threads': num_threads}),
                        'thread_code' : thread_code_str,
                        'syncronize' : ''}
        
        wgsl =cust_format(wgsl_template_str, replacements)
        print(f"num mem locs: {len(mem_locs)}")
        with open(wgsl_name, 'w') as out_file:
            out_file.write(wgsl)
            out_file.close()

def gen_wgsl_workgroup_barrier(target_file, wgsl_name):
    with open('wgsl_template.wgsl', 'r') as wgsl_template_file:
        wgsl_template_str = wgsl_template_file.read()
        print(f"parsing target file : {target_file}")
        mem_locs = set()
        num_threads = 0
        num_workgroups = 0
        thread_code_str = ''
        # generate thread code
        with open(target_file, 'r') as file:
            while (True):
                line = file.readline()
                if (line == ''):
                    #end of file
                    break
                elif (line == '\n'):
                    #whitespace, ignore
                    continue
                elif (re.match(Inst_REs.TD_RE.value, line)):
                    args = re.match(Inst_REs.TD_RE.value, line)
                    thread_code_str += parse_thread(thread_code_str, args['tid'], file, mem_locs, target_file, 'workgroup_barrier')
                    num_threads += 1
            file.close()
        #add to the 'done' counter when program finishes
        mem_locs_str = ''
        for loc in mem_locs:
            mem_locs_str += f'  mem_{loc}: atomic<i32>,\n'
        # stitch all of the generated code together
        replacements = {'mem_locs' : mem_locs_str,
                        'workgroup_size' : r'@workgroup_size(256, 1, 1)',
                        'setup' : BP_Strings.BOILER_PLATE_WORKGROUP_BARRIER_STR.value,
                        'thread_code' : thread_code_str,
                        'syncronize' : 'workgroupBarrier();'}
        wgsl =cust_format(wgsl_template_str, replacements)
        print(f"num mem locs: {len(mem_locs)}")
        with open(wgsl_name, 'w') as out_file:
            out_file.write(wgsl)
            out_file.close()

def gen_wgsl_workgroup_barrier_random(target_file, wgsl_name):
    with open('wgsl_template.wgsl', 'r') as wgsl_template_file:
        wgsl_template_str = wgsl_template_file.read()
        print(f"parsing target file : {target_file}")
        mem_locs = set()
        num_threads = 0
        num_workgroups = 0
        thread_code_str = ''
        # generate thread code
        with open(target_file, 'r') as file:
            while (True):
                line = file.readline()
                if (line == ''):
                    #end of file
                    break
                elif (line == '\n'):
                    #whitespace, ignore
                    continue
                elif (re.match(Inst_REs.TD_RE.value, line)):
                    args = re.match(Inst_REs.TD_RE.value, line)
                    thread_code_str += parse_thread(thread_code_str, args['tid'], file, mem_locs, target_file, 'workgroup_barrier_random')
                    num_threads += 1
            file.close()
        #add to the 'done' counter when program finishes
        mem_locs_str = ''
        for loc in mem_locs:
            mem_locs_str += f'  mem_{loc}: atomic<i32>,\n'
        # stitch all of the generated code together
        replacements = {'mem_locs' : mem_locs_str,
                        'workgroup_size' : r'@workgroup_size(256, 1, 1)',
                        'setup' : BP_Strings.BOILER_PLATE_WORKGROUP_BARRIER_STR.value,
                        'thread_code' : thread_code_str,
                        'syncronize' : 'workgroupBarrier();'}
        wgsl =cust_format(wgsl_template_str, replacements)
        print(f"num mem locs: {len(mem_locs)}")
        with open(wgsl_name, 'w') as out_file:
            out_file.write(wgsl)
            out_file.close()

def gen_wgsl(target_file, wgsl_name='test', heuristic='single'):
    if heuristic == 'single':
        gen_wgsl_single(target_file, wgsl_name)
    elif heuristic == 'round_robin':
        gen_wgsl_round_robin(target_file, wgsl_name)
    elif heuristic == 'chunked':
        gen_wgsl_chunked(target_file, wgsl_name)
    elif heuristic == 'workgroup_barrier':
        gen_wgsl_workgroup_barrier(target_file, wgsl_name)
    elif heuristic == 'workgroup_barrier_random':
        gen_wgsl_workgroup_barrier(target_file, wgsl_name)
    else:
        print("ERROR: invalid heuristic")
        exit()


# for testing, ignore
def test():
    gen_wgsl_workgroup_barrier_random('test.txt', 'test.wgsl')

# run the program with command line arguments
def main():
    # command line arguments
    parser = argparse.ArgumentParser()
    # path to test txt file (in alloy_forward_progress)
    parser.add_argument('-tf', '--test_file', help='path to test file', default='test.txt')
    # generate wgsl
    parser.add_argument('-g', '--gen_wgsl', help='generate wgsl', default=1)
    parser.add_argument('-o', '--out_file', help='path to output wgsl', default='test.wgsl')
    parser.add_argument('-t', '--test', help='for testing, ignore')

    parser.add_argument('-hu', '--heuristic', help='single, round_robin, chunked, or workgroup_barrier', default='single')
    args = parser.parse_args()

    if(args.test):
        test()
    else:
        if(args.gen_wgsl):
            if(args.test_file):
                if(args.out_file):
                    gen_wgsl(args.test_file, args.out_file, args.heuristic)
                    # i have no idea whats going on here
                    '''
                    with open(args.out_file) as file:
                        top = re.match(r'\/\/(?P<num_threads>[0-9]+)\,(?P<num_workgroups>[0-9]+)', file.readline())
                        print(top)
                        num_threads = top['num_threads']
                        num_workgroups = top['num_workgroups']
                        file.close()
                
                    gen_wgsl(args.test_file.replace('.txt', '.wgsl'), num_threads)
                    '''
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