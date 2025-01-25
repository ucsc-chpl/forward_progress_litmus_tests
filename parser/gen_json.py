import os
import json

test_json = {
    "HSA" : {
        "2_threads_2_instructions" : [],
        "2_threads_3_instructions" : [],
        "2_threads_4_instructions" : [],
        "3_threads_3_instructions" : [],
        "3_threads_4_instructions" : [],
    },
    "HSA_OBE" : {
        "2_threads_2_instructions" : [],
        "2_threads_3_instructions" : [],
        "2_threads_4_instructions" : [],
        "3_threads_3_instructions" : [],
        "3_threads_4_instructions" : [],
    },
    "HSA_OBE_STRONG" : {
        "2_threads_2_instructions" : [],
        "2_threads_3_instructions" : [],
        "2_threads_4_instructions" : [],
        "3_threads_3_instructions" : [],
        "3_threads_4_instructions" : [],
    },
    "HSA_STRONG" : {
        "2_threads_2_instructions" : [],
        "2_threads_3_instructions" : [],
        "2_threads_4_instructions" : [],
        "3_threads_3_instructions" : [],
        "3_threads_4_instructions" : [],
    },
    "LOBE" : {
        "2_threads_2_instructions" : [],
        "2_threads_3_instructions" : [],
        "2_threads_4_instructions" : [],
        "3_threads_3_instructions" : [],
        "3_threads_4_instructions" : [],
    },
    "LOBE_STRONG" : {
        "2_threads_2_instructions" : [],
        "2_threads_3_instructions" : [],
        "2_threads_4_instructions" : [],
        "3_threads_3_instructions" : [],
        "3_threads_4_instructions" : [],
    },
    "OBE" : {
        "2_threads_2_instructions" : [],
        "2_threads_3_instructions" : [],
        "2_threads_4_instructions" : [],
        "3_threads_3_instructions" : [],
        "3_threads_4_instructions" : [],
    },
    "OBE_STRONG" : {
        "2_threads_2_instructions" : [],
        "2_threads_3_instructions" : [],
        "2_threads_4_instructions" : [],
        "3_threads_3_instructions" : [],
        "3_threads_4_instructions" : [],
    },
    "STRONG_FAIR" : {
        "2_threads_2_instructions" : [],
        "2_threads_3_instructions" : [],
        "2_threads_4_instructions" : [],
        "3_threads_3_instructions" : [],
        "3_threads_4_instructions" : [],
    },
    "WEAK_FAIR" : {
        "2_threads_2_instructions" : [],
        "2_threads_3_instructions" : [],
        "2_threads_4_instructions" : [],
        "3_threads_3_instructions" : [],
        "3_threads_4_instructions" : [],
    },

}
# generates all of the wgsls and sorts them by progress model
def gen_wgsls_by_model(dest_path, test_path):
    for test_dir in [d for d in os.listdir(test_path) if os.path.isdir(os.path.join(test_path, d))]:
        for test in [d for d in os.listdir(os.path.join(test_path, test_dir)) if d not in ['csv', 'distinguishing', 'testExplorer.html', 'timestamps.txt']]:
            with open(os.path.join(test_path,test_dir,test) + '/' + 'label_' + os.path.basename(test) + '.txt', 'r') as test_file:
                for line_no, line in enumerate(test_file, start=1):
                    #WEAK VARIANTS
                    if(line_no == 7):
                        if(line.strip().replace('OBE - Termination: ', '') == 'PASS'):
                            test_json['OBE'][test_dir].append(test)
                   
                    if(line_no == 8):
                        if(line.strip().replace('HSA - Termination: ', '') == 'PASS'):
                            test_json['HSA'][test_dir].append(test)
                    if(line_no == 9):
                        if(line.strip().replace('HSA_OBE - Termination: ', '') == 'PASS'):
                            test_json['HSA_OBE'][test_dir].append(test)
                    if(line_no == 10):
                        if(line.strip().replace('LOBE - Termination: ', '') == 'PASS'):
                            test_json['LOBE'][test_dir].append(test)
                    if(line_no == 11):
                        if(line.strip().replace('WEAK_FAIR - Termination: ', '') == 'PASS'):
                            test_json['WEAK_FAIR'][test_dir].append(test)
                    #strong variants:
                    if(line_no == 14):
                        if(line.strip().replace('OBE_STRONG - Termination: ', '') == 'PASS'):
                            test_json['OBE_STRONG'][test_dir].append(test)
                    if(line_no == 15):
                        if(line.strip().replace('HSA_STRONG - Termination: ', '') == 'PASS'):
                            test_json['HSA_STRONG'][test_dir].append(test)
                    if(line_no == 16):
                        if(line.strip().replace('HSA_OBE_STRONG - Termination: ', '') == 'PASS'):
                            test_json['HSA_OBE_STRONG'][test_dir].append(test)
                    if(line_no == 17):
                        if(line.strip().replace('LOBE_STRONG - Termination: ', '') == 'PASS'):
                            test_json['LOBE_STRONG'][test_dir].append(test)
                    if(line_no == 18):
                        if(line.strip().replace('STRONG_FAIR - Termination: ', '') == 'PASS'):  
                            test_json['STRONG_FAIR'][test_dir].append(test)
    f = open('../tests.json', 'w')
    json.dump(test_json, f, indent=4)

if __name__ == '__main__':
    gen_wgsls_by_model(None, '/Users/Tanya/tyler_research/AlloyForwardProgress/artifact/web_test_explorer')