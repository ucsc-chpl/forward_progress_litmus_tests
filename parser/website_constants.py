from enum import Enum
import os

class Paths(Enum):
    REPO_BASE_DIR       = os.getcwd().replace('/parser', '')
    WGSL_BASE_PATH      = 'tests/'
    DEST_PATH           = REPO_BASE_DIR + '/src/tests/'
    DEFAULT_TEST_PATH   = '/home/nrehman/AlloyForwardProgress/artifact/web_test_explorer/'

# strings shared between all HTML files
class HTML_all(Enum):
    PREAMBLE_STR = '''
<!doctype html>
<html lang="en-US">
<head>
  <meta charset="utf-8" />
  <title>litmus test</title>
</head>
<body>
  <p id="arch"></p>
  <p id="vendor"></p>
  <p id="desc"></p>
  <p id="device"></p>
'''
    INIT_WEBGPU_STR = '''  <script>
    async function initWebGPU() {
        try {
            const adapter = await navigator.gpu.requestAdapter();
            const adapterInfo = await adapter.requestAdapterInfo();
            const arch = document.getElementById('arch');
            arch.textContent = `Architecture: ${adapterInfo.architecture}`;
            const vendor = document.getElementById('vendor');
            vendor.textContent = `Vendor: ${adapterInfo.vendor}`;
            const device = document.getElementById('device');
            device.textContent = `Device: ${adapterInfo.device}`;
            const desc = document.getElementById('desc');
            desc.textContent = `Description: ${adapterInfo.description}`;

        } catch (error) {
            console.error('Error initializing WebGPU:', error);
            document.write('<p>Error initializing WebGPU: ' + error + '</p>');
        }
    }
    window.onload = initWebGPU;
  </script>
  '''

class HTML_Per_Test(Enum):
    RUN_BUTTON_STR = '''
<button id="single_run_button">run single test</button>
<div id="single_run_output"></div>
<button id="round_robin_run_button">run round robin test</button>
<div id="round_robin_run_output"></div>
<button id="chunked_run_button">run chunked test</button>
<div id="chunked_run_output"></div>
'''
    STYLE_STR = '''<style>
    .content {{
        position: absolute;
        top: 0;
        right: 0;
    }}
  </style>
  <div>
    <p><iframe src="{text_file}" frameborder="0" height="400"
      width="95%"></iframe></p>
  </div>
  <div class="container">
    <div class="content">
      <img src="{img_name}" alt="state transition diagram" width="500">
    </div>
  </div>
</body>
</html>
'''
    RUN_STR = '''
  <script type="module">
    import init from "../../../../../pkg/litmus_test_web.js";
    import * as wasm_mod from "../../../../../pkg/litmus_test_web.js";

    init().then(() => {{
      // Event listener for test case 5
      document.getElementById('single_run_button').addEventListener('click', () => {{
        const outputDiv = document.getElementById('single_run_output');
        outputDiv.textContent = `running test...`;
        const resultPromise = wasm_mod.run({num_threads}, "{test_name}_single.wgsl", {num_threads});
        resultPromise.then(result => {{
            if(result == 0){{
              outputDiv.textContent = `Threads finished: ${{result}} 
 Refresh page to continue`;
            }}
            else {{
              outputDiv.textContent = `Threads finished: ${{result}}`;
            }}
        }});
      }});
      // Event listener for test case 5
      document.getElementById('round_robin_run_button').addEventListener('click', () => {{
        const outputDiv = document.getElementById('round_robin_run_output');
        outputDiv.textContent = `running test...`;
        const resultPromise = wasm_mod.run({num_threads}, "{test_name}_round_robin.wgsl", 32);
        resultPromise.then(result => {{
            if(result == 0){{
              outputDiv.textContent = `Threads finished: ${{result}} 
 Refresh page to continue`;
            }}
            else {{
              outputDiv.textContent = `Threads finished: ${{result}}`;
            }}
        }});
      }});
      // Event listener for test case 5
      document.getElementById('chunked_run_button').addEventListener('click', () => {{
        const outputDiv = document.getElementById('chunked_run_output');
        outputDiv.textContent = `running test...`;
        const resultPromise = wasm_mod.run({num_threads}, "{test_name}_chunked.wgsl", 32);
        resultPromise.then(result => {{
            if(result == 0){{
              outputDiv.textContent = `Threads finished: ${{result}} 
 Refresh page to continue`;
            }}
            else {{
              outputDiv.textContent = `Threads finished: ${{result}}`;
            }}
        }});
      }});
    }});
  </script>
'''

class HTML_All_Runner(Enum):
    PROMISE_START_STR   = '     Promise.all([\n'
    PROMISE_STR         = '       resultPromise{num_tests},\n'
    PROMISE_END_STR     = ''']).then((results) => {
          for(let th_fin of results) {
            if(th_fin == 0) {
              tests_failed = tests_failed + 1;
            }
            else {
              tests_passed = tests_passed + 1;
            }
          }
          outputDiv.textContent = `All Tests Finished. Tests Passed: ${tests_passed} Tests Failed: ${tests_failed}`;
        }
      );
'''
    TEST_DIV_STR        = '<div id="output{num_tests}">{instructions} instructions, {num_threads} threads, test {test}</div>\n'
    RUN_TEST_STR        = '''
        let resultPromise{num_tests} = Promise.race([wasm_mod.run({num_threads}, "{test_in}", {num_workgroups}), new Promise((resolve, _) => {{
                setTimeout(() => {{
                resolve(0);
                }}, {timeout_ms});
            }})]);
        resultPromise{num_tests}.then((result) => {{
            if(result == 0) {{
                document.getElementById("output{num_tests}").textContent = "{num_threads} instructions, {num_inst}, test {test} : failed";
            }} else {{
                document.getElementById("output{num_tests}").textContent = "{num_threads} instructions, {num_inst}, test {test} : passed";
            }}
        }});
'''
    RUN_BUTTON_STR = '''<button id="single_run_button">run all single tests</button>
<button id="round_robin_run_button">run all round robin tests</button>
<button id="chunked_run_button">run all chunked tests</button>
'''
    RUN_OUTPUT_STR = '''<div id="single_run_output"></div>
<div id="round_robin_run_output"></div>
<div id="chunked_run_output"></div>
\n'''

    SCRIPT_INIT_STR = '''
  <script type="module">
    import init from "../../../../pkg/litmus_test_web.js";
    import * as wasm_mod from "../../../../pkg/litmus_test_web.js";

    init().then(() => {'''

    BUTTON_CLICK_START_STR = '''
      // Event listener for test case 5
      document.getElementById('{heuristic}_run_button').addEventListener('click', () => {{
        let resultPromise;
        const outputDiv = document.getElementById('{heuristic}_run_output');
        var tests_passed = 0;
        var tests_failed = 0;
        var tests_completed = 0;
        outputDiv.textContent = "running tests...";
    '''
    BUTTON_CLICK_END_STR = '''
    });
'''

    SCRIPT_END_STR = '''
    });
  </script>
'''
    POSTSCRIPT_STR = '''</body>
</html>'''
