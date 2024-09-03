import os
import sys
import compile_wgsl_new as compile_wgsl
import shutil
import argparse
import re
import subprocess
from website_constants import Paths, HTML_all, HTML_Per_Test, HTML_All_Runner
from wgpu_constants import WGPU_Runner

# to do: deny-list certain directories (like test and m2_kernel_panic) <3
# to do: ALL RUNNER SUS!! currently just checks that the number of finished threads is non-zero. Usually this works because if they don't all finish the program hangs and the driver kills it w/o copying the buffer back over BUT different driver could do different stuff
# Potential features
# - check if files already exist before writing, add a --clobber flag to overwrite them if they do otherwise keep old files
# generates wgsls from alloy forward progress tests, sorted by progress model

# output file directory:
# hsa
# | 2_thread_2_inst
# | | 1
# | | | 1_simple.txt
# | | | 1.txt
# | | | 1.wgsl
# | | | 1.png
# | 2_thread_2_inst
# etc

# website structure
# main page
# | hsa
# | | test_all
# | | 2_thread_2_inst
# | | | 1
# | | | 2 ...etc
# | | 2_thread_3_inst ..etc
# | obe ...etc

# TODO Add all runner for HSA, OBE, HSA-OBE, LOBE (one button)
# TODO figure out how to actually kill the test in the browser
# TODO put scripts in separate javascript files instead of just in a script block in the HTML
# TODO fix number of threads called

#timeout test in website after 15 seconds (this doesn't actually kill the test)
timeout_ms = 15000

#format with model
model_index_pa = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{model} Tests</title>
</head>
<body>
    <b>{model} Tests</b>
    <ul>
"""

model_index_end = """
    </ul>
</body>
</html>
"""

# python format is a nightmare with all of these curly braces
def cust_format(string: str, replacements: dict):
    for key, value in replacements.items():
        string = string.replace('?' + key + '?', str(value))
    return string

# alias for basename
def bn(path):
    return os.path.basename(path)

# copies the relevant files from the alloy_forward_progress repo and generates the wgsl
def copy_test(dest_path, test_dir, test, cur_test_path, model, png, text, s_text):
    os.makedirs(dest_path + '/' + model + '/' + bn(test_dir) + '/' + bn(test) + '/', exist_ok=True)
    shutil.copyfile(png, dest_path + '/' + model + '/' + bn(test_dir) + '/' + bn(test) + '/' + bn(test) + '.png')
    shutil.copyfile(text, dest_path + '/' + model + '/' + bn(test_dir) + '/' + bn(test) + '/' + bn(test) + '.txt')
    shutil.copyfile(s_text, dest_path + '/' + model + '/' + bn(test_dir) + '/' + bn(test) + '/' + bn(test) + '_simple.txt')
    compile_wgsl.gen_wgsl(cur_test_path + '/' + bn(test) + '.txt', dest_path + '/' + model + '/' + bn(test_dir) + '/' + bn(test) + '/' + bn(test) +'_single.wgsl', heuristic='single')
    compile_wgsl.gen_wgsl(cur_test_path + '/' + bn(test) + '.txt', dest_path + '/' + model + '/' + bn(test_dir) + '/' + bn(test) + '/' + bn(test) +'_round_robin.wgsl', heuristic='round_robin')
    compile_wgsl.gen_wgsl(cur_test_path + '/' + bn(test) + '.txt', dest_path + '/' + model + '/' + bn(test_dir) + '/' + bn(test) + '/' + bn(test) +'_chunked.wgsl', heuristic='chunked')
    compile_wgsl.gen_wgsl(cur_test_path + '/' + bn(test) + '.txt', dest_path + '/' + model + '/' + bn(test_dir) + '/' + bn(test) + '/' + bn(test) +'_workgroup_barrier.wgsl', heuristic='workgroup_barrier')
    compile_wgsl.gen_wgsl(cur_test_path + '/' + bn(test) + '.txt', dest_path + '/' + model + '/' + bn(test_dir) + '/' + bn(test) + '/' + bn(test) +'_workgroup_barrier_random.wgsl', heuristic='workgroup_barrier_random')

# generates all of the wgsls and sorts them by progress model
def gen_wgsls_by_model(dest_path, test_path):
    for test_dir in [d for d in os.listdir(test_path) if os.path.isdir(os.path.join(test_path, d))]:
        for test in [d for d in os.listdir(os.path.join(test_path, test_dir)) if d not in ['csv', 'distinguishing', 'testExplorer.html', 'timestamps.txt']]:
            with open(os.path.join(test_path,test_dir,test) + '/' + 'label_' + os.path.basename(test) + '.txt', 'r') as test_file:
                for line_no, line in enumerate(test_file, start=1):
                    cur_test_path = test_path + test_dir + '/' + os.path.basename(test) + '/'
                    png = cur_test_path + os.path.basename(test) + '.png'
                    text = cur_test_path + os.path.basename(test) + '.txt'
                    s_test = cur_test_path + os.path.basename(test) + '_simple.txt'
                    #WEAK VARIANTS
                    if(line_no == 7):
                        if(line.strip().replace('OBE - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'OBE', png, text, s_test)
                   
                    if(line_no == 8):
                        if(line.strip().replace('HSA - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'HSA', png, text, s_test)

                    if(line_no == 9):
                        if(line.strip().replace('HSA_OBE - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'HSA_OBE', png, text, s_test)

                    if(line_no == 10):
                        if(line.strip().replace('LOBE - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'LOBE', png, text, s_test)

                    if(line_no == 11):
                        if(line.strip().replace('WEAK_FAIR - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'WEAK_FAIR', png, text, s_test)

                    #strong variants:
                    if(line_no == 14):
                        if(line.strip().replace('OBE_STRONG - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'OBE_STRONG', png, text, s_test)

                    if(line_no == 15):
                        if(line.strip().replace('HSA_STRONG - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'HSA_STRONG', png, text, s_test)
                    
                    if(line_no == 16):
                        if(line.strip().replace('HSA_OBE_STRONG - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'HSA_OBE_STRONG', png, text, s_test)
                    
                    if(line_no == 17):
                        if(line.strip().replace('LOBE_STRONG - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'LOBE_STRONG', png, text, s_test)

                    if(line_no == 18):
                        if(line.strip().replace('STRONG_FAIR - Termination: ', '') == 'PASS'):
                            copy_test(dest_path, test_dir, test, cur_test_path, 'STRONG_FAIR', png, text, s_test)

# TODO change num_threads to num_workgroups
# generates lib.rs for test website
def gen_runner_web(dest_path, wgsl_base_path, outfile="/home/nrehman/forward_progress_litmus_tests/src/lib.rs"):
    runner_s = WGPU_Runner.INCLUDES_STR.value
    runner_s += WGPU_Runner.RUN_FN_STR.value

    # get all of the include strs
    tests = ''
    for model in os.listdir(dest_path):
        print(model)
        for thread_inst in os.listdir(dest_path + os.path.basename(model)):
            if os.path.isdir(os.path.join(dest_path, model, thread_inst)):
                for test in os.listdir(dest_path + '/' + os.path.basename(model) + '/' + os.path.basename(thread_inst)):
                    if(os.path.isdir(os.path.join(dest_path, model, thread_inst, test))):
                        # single
                        test_in = wgsl_base_path + os.path.basename(model) + '/' + os.path.basename(thread_inst) + '/' + os.path.basename(test) + '/' + os.path.basename(test) + '_single.wgsl'
                        tests += cust_format(WGPU_Runner.ADD_TEST_STR.value, {'test_path': test_in})
                        # round robin
                        test_in = wgsl_base_path + os.path.basename(model) + '/' + os.path.basename(thread_inst) + '/' + os.path.basename(test) + '/' + os.path.basename(test) + '_round_robin.wgsl'
                        tests += cust_format(WGPU_Runner.ADD_TEST_STR.value, {'test_path': test_in})
                        # chunked
                        test_in = wgsl_base_path + os.path.basename(model) + '/' + os.path.basename(thread_inst) + '/' + os.path.basename(test) + '/' + os.path.basename(test) + '_chunked.wgsl'
                        tests += cust_format(WGPU_Runner.ADD_TEST_STR.value, {'test_path': test_in})
                        # workgroup_barrier
                        test_in = wgsl_base_path + os.path.basename(model) + '/' + os.path.basename(thread_inst) + '/' + os.path.basename(test) + '/' + os.path.basename(test) + '_workgroup_barrier.wgsl'
                        tests += cust_format(WGPU_Runner.ADD_TEST_STR.value, {'test_path': test_in})
                        # workgroup_barrier random
                        test_in = wgsl_base_path + os.path.basename(model) + '/' + os.path.basename(thread_inst) + '/' + os.path.basename(test) + '/' + os.path.basename(test) + '_workgroup_barrier_random.wgsl'
                        tests += cust_format(WGPU_Runner.ADD_TEST_STR.value, {'test_path': test_in})
                        
    runner_s += cust_format(WGPU_Runner.EXECUTE_GPU_FN_STR.value, {'test_paths' : tests})
    with open(outfile, 'w') as file:
        file.write(runner_s)
        file.close()
        #print(runner_s)

# generates the all_runner html for a given model
def gen_index_html_all_runner(dest_path, wgsl_base_path, model): 
    promise_all = HTML_All_Runner.PROMISE_START_STR.value
    out_divs = ""
    single_tests = ""
    round_robin_tests = ""
    chunked_tests = ""
    workgroup_barrier_tests = ""
    workgroup_barrier_random_tests = ""
    num_tests = 0
    for thread_inst in os.listdir(dest_path + '/' + model):
        if(os.path.isdir(os.path.join(dest_path, model, thread_inst))):
            for test in os.listdir(dest_path + '/' + os.path.basename(model) + '/' + os.path.basename(thread_inst)):
                if(os.path.isdir(os.path.join(dest_path, model, thread_inst, test))):
                    # test stuff
                    test_in = wgsl_base_path + os.path.basename(model) + '/' + os.path.basename(thread_inst) + '/' + os.path.basename(test) + '/' + os.path.basename(test)
                    test_desc = re.match("(?P<num_threads>[0-9])_threads_(?P<num_inst>[0-9])_instructions", thread_inst)
                    # add to this promise to the promise all statement
                    promise_all += HTML_All_Runner.PROMISE_STR.value.format(num_tests=num_tests)
                    # add this test to the test outputs
                    out_divs += HTML_All_Runner.TEST_DIV_STR.value.format(num_tests=num_tests, instructions=test_desc['num_inst'], num_threads=test_desc['num_threads'], test=test)

                    single_tests += HTML_All_Runner.RUN_TEST_STR.value.format(num_tests=num_tests, 
                                                                       num_threads=test_desc['num_threads'], 
                                                                       test_in=test_in + '_single.wgsl', 
                                                                       timeout_ms=timeout_ms,
                                                                       num_inst=test_desc['num_inst'],
                                                                       test=test,
                                                                       num_workgroups=test_desc['num_threads'])
                    
                    round_robin_tests += HTML_All_Runner.RUN_TEST_STR.value.format(num_tests=num_tests, 
                                                                       num_threads=test_desc['num_threads'], 
                                                                       test_in=test_in + '_round_robin.wgsl', 
                                                                       timeout_ms=timeout_ms,
                                                                       num_inst=test_desc['num_inst'],
                                                                       test=test,
                                                                       num_workgroups=32)
                    chunked_tests += HTML_All_Runner.RUN_TEST_STR.value.format(num_tests=num_tests, 
                                                                       num_threads=test_desc['num_threads'], 
                                                                       test_in=test_in + '_chunked.wgsl', 
                                                                       timeout_ms=timeout_ms,
                                                                       num_inst=test_desc['num_inst'],
                                                                       test=test,
                                                                       num_workgroups=32)
                    workgroup_barrier_tests += HTML_All_Runner.RUN_TEST_STR.value.format(num_tests=num_tests, 
                                                                       num_threads=test_desc['num_threads'], 
                                                                       test_in=test_in + '_workgroup_barrier.wgsl', 
                                                                       timeout_ms=timeout_ms,
                                                                       num_inst=test_desc['num_inst'],
                                                                       test=test,
                                                                       num_workgroups=test_desc['num_threads'])
                    workgroup_barrier_random_tests += HTML_All_Runner.RUN_TEST_STR.value.format(num_tests=num_tests, 
                                                                       num_threads=test_desc['num_threads'], 
                                                                       test_in=test_in + '_workgroup_barrier_random.wgsl', 
                                                                       timeout_ms=timeout_ms,
                                                                       num_inst=test_desc['num_inst'],
                                                                       test=test,
                                                                       num_workgroups=test_desc['num_threads'])
                    num_tests += 1
    promise_all += HTML_All_Runner.PROMISE_END_STR.value
    index = HTML_all.PREAMBLE_STR.value
    index += HTML_All_Runner.RUN_BUTTON_STR.value
    index += HTML_All_Runner.RUN_OUTPUT_STR.value
    index += out_divs 
    index += HTML_all.INIT_WEBGPU_STR.value
    index += HTML_All_Runner.SCRIPT_INIT_STR.value
    # single tests runner
    index += HTML_All_Runner.BUTTON_CLICK_START_STR.value.format(heuristic='single')
    index += single_tests
    index += promise_all
    index += HTML_All_Runner.BUTTON_CLICK_END_STR.value
    # round robin tests runner
    index += HTML_All_Runner.BUTTON_CLICK_START_STR.value.format(heuristic='round_robin')
    index += round_robin_tests
    index += promise_all
    index += HTML_All_Runner.BUTTON_CLICK_END_STR.value
    # chunked tsts runner
    index += HTML_All_Runner.BUTTON_CLICK_START_STR.value.format(heuristic='chunked')
    index += chunked_tests
    index += promise_all
    index += HTML_All_Runner.BUTTON_CLICK_END_STR.value
    # workgroup_barrier
    index += HTML_All_Runner.BUTTON_CLICK_START_STR.value.format(heuristic='workgroup_barrier')
    index += workgroup_barrier_tests
    index += promise_all
    index += HTML_All_Runner.BUTTON_CLICK_END_STR.value
    # workgroup_barrier random
    index += HTML_All_Runner.BUTTON_CLICK_START_STR.value.format(heuristic='workgroup_barrier_random')
    index += workgroup_barrier_random_tests
    index += promise_all
    index += HTML_All_Runner.BUTTON_CLICK_END_STR.value
    index += HTML_All_Runner.SCRIPT_END_STR.value
    # TODO: create a model index file with all of the const strings labeled in comments
    # what if instead of building litmus tests based off of progress models we built progress models based on litmus tests??
    index += HTML_All_Runner.POSTSCRIPT_STR.value
    out_path = dest_path + '/' + model + '/' + 'all_runner' + '/'
    os.makedirs(out_path, exist_ok=True)
    with open(out_path + 'index.html', 'w') as file:
        file.write(index)
        file.close()

# generates HTML for single test
def gen_index_html_per_test_runner(test_name, target_dir, img, text_file, dest_path, num_threads):
    if os.path.isdir(os.path.join(dest_path.replace('/tests', '') + target_dir)):
        index = HTML_all.PREAMBLE_STR.value
        index += HTML_Per_Test.RUN_BUTTON_STR.value 
        index += HTML_all.INIT_WEBGPU_STR.value
        # FIXME: get correct value for num_threads
        index += HTML_Per_Test.RUN_STR.value.format(test_name=test_name, num_threads=num_threads) 
        index += HTML_Per_Test.STYLE_STR.value.format(img_name=img, text_file=text_file)
        with open(os.path.join(dest_path.replace('/tests', ''), target_dir) + 'index.html', 'w') as file:
            file.write(index)
            file.close()
    else:
        print(f"gen_index_html_per_test_runner() recieved non dir target dir: {target_dir} skipping")

# generates all of the HTML files
def gen_index_html(dest_path, wgsl_base_path):
    # premble
    # button defs
    # initwebgpu script
    # run stuff
    # style stuff

    # I actually have no idea whats going on here 0.o
    for model in os.listdir(dest_path):
        print(f"generating all runner index.html for {model}")
        gen_index_html_all_runner(dest_path, wgsl_base_path, model)
        m_index = model_index_pa.format(model=model)
        m_index += f"""    <li><a href="./all_runner/">Run All Tests</a></li>\n"""
        for thread_inst in os.listdir(dest_path + '/' + model):
            if os.path.isdir(os.path.join(dest_path, model, thread_inst)):
                test_desc = re.match("(?P<num_threads>[0-9])_threads_(?P<num_inst>[0-9])_instructions", thread_inst)
                if(thread_inst != 'all_runner' and thread_inst != 'index.html'):
                    t_index = model_index_pa.format(model=f"{test_desc['num_threads']} threads, {test_desc['num_inst']} instructions")
                    m_index += f"""    <li><a href="./{thread_inst}/">{test_desc['num_threads']} threads, {test_desc['num_inst']} instructions</a></li>\n"""
                    if(os.path.isdir(os.path.join(dest_path, model, thread_inst))):
                        for test in os.listdir(dest_path + '/' + os.path.basename(model) + '/' + os.path.basename(thread_inst)):
                            if(os.path.isdir(os.path.join(dest_path, model, thread_inst, test))):
                                t_index += f"""    <li><a href="./{test}/">Test {test}</a></li>\n"""
                                test_target_dir = wgsl_base_path + os.path.basename(model) + '/' + os.path.basename(thread_inst) + '/' + os.path.basename(test) + '/'
                                test_in = test_target_dir + os.path.basename(test)
                                test_img = os.path.basename(test) + '.png'
                                text_file = os.path.basename(test) + '_simple.txt'
                                gen_index_html_per_test_runner(test_in, test_target_dir, test_img, text_file, dest_path, test_desc['num_threads'])
                        t_index += model_index_end
                        with open(os.path.join(dest_path, model, thread_inst) + '/index.html', 'w') as t_outfile:
                            t_outfile.write(t_index)
                            t_outfile.close()
        m_index += model_index_end
        with open(dest_path + '/' + model + '/' + 'index.html', 'w') as file:
            file.write(m_index)
            file.close()

# for testing, ignore
def test():
    validate_wgsls(dest_path)

# this is an absolute mess
if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('-c', '--compile', help='compile wgsls', default=False)
    parser.add_argument('--alloyfp_path', help='path to alloy forward progress directory', default='../../AlloyForwardProgress/artifact/web_test_explorer/')
    parser.add_argument('-r', '--make_runner', help='makes the rust stuff', default=True)
    parser.add_argument('-o', '--outfile', help='outfile for lib.rs, default is src/lib.rs', default='../src/lib.rs')
    parser.add_argument('-i', '--make_index', help='makes index.htmls', default=False)
    parser.add_argument('-t', '--test', help='runs the test function. for debugging, ignore.', default=False)
    args = parser.parse_args()
    if(args.test):
        test()
    if(args.compile):
        if(args.alloyfp_path):
            if(os.path.isdir(args.alloyfp_path)):
                gen_wgsls_by_model(Paths.DEST_PATH.value, args.alloyfp_path + '/')
            else:
                print("invalid path to AlloyForwardProgress repo! Exiting!")
                exit()
        else:
            print("no path to AlloyForwardProgress repo was given, defaulting to naomi's path <3")
            gen_wgsls_by_model(Paths.DEST_PATH.value, Paths.DEFAULT_TEST_PATH.value)

    if(args.make_runner):
        if(args.outfile):
            gen_runner_web(Paths.DEST_PATH.value, Paths.WGSL_BASE_PATH.value, outfile=args.outfile)
    if(args.make_index):
        gen_index_html(Paths.DEST_PATH.value, Paths.WGSL_BASE_PATH.value)

    # command to do everything (generate wgsls and htmls):
    # python3 gen_website.py -c 1 --alloyfp_path <path_to_webtest_dir> -r 1 -o <path to src/lib.rs> -i 1
    # Example (for naomi): python3 gen_website.py -c 1 --alloyfp_path /home/nrehman/AlloyForwardProgress/artifact/web_test_explorer/ -r 1 -o /home/nrehman/forward_progress_litmus_tests/src/lib.rs -i 1

    # command to just generate wgsls:
    # python3 gen_website.py -c 1 --alloyfp_path <path_to_webtest_dir>

    # command to generate the rust code:
    # python3 gen_website.py -r 1 -o <path to src/lib.rs>

    # command to generate the index.htmls
    # python3 gen_website.py -i 1

    # to validate all of the wgsls run ./val_wgsls.sh

