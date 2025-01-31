import init from "../pkg/litmus_test_web.js";
import * as wasm_mod from "../pkg/litmus_test_web.js";


var test_json;
fetch('../tests.json')
  .then(response => response.json())  // Parse the JSON data
  .then(json => {
    test_json = json;
  })
  .catch(error => {
    console.error('Error loading JSON:', error);
  });

let models = [
    "HSA",
    "HSA_OBE",
    "LOBE",
    "LOBE_STRONG",
    "OBE",
    "OBE_STRONG",
    "WEAK_FAIR",
    "HSA_OBE_STRONG",
    "HSA_STRONG",
    "STRONG_FAIR",
    
];

let probably_wont_crash_models = [
    "HSA",
    "HSA_OBE",
    "LOBE",
    "LOBE_STRONG",
    "OBE",
    "OBE_STRONG",
];

let heuristics = [
    "single",
    "round_robin",
    "chunked",
    "workgroup_barrier",
    "workgroup_barrier_random",
    "workgroup_round_robin",
    "workgroup_chunked",
    "workgroup_round_robin_random",
    "workgroup_chunked_random"
];

let counter = 0;

export function select_all() {
    let model;
    for (model of models) {
        document.getElementById(`${model}`).checked = true;
    }
}

export function select(models_s) {
    let model;
    for (model of models_s) {
        document.getElementById(`${model}`).checked = true;
    }
}

export function clear_selections() {
    let model;
    for (model of models) {
        document.getElementById(`${model}`).checked = false;
    }
}

export async function init_stuff() {
    select(probably_wont_crash_models);
    document.getElementById("run_button").onclick = run_all_tests;
    document.getElementById("select_all").onclick = select_all;
    document.getElementById("clear_selections").onclick = clear_selections;
    await init();
    await wasm_mod.init_gpu_objects();
}

function get_models() {
    let models_to_run = [];
    let model;
    for (model of models) {
        if(document.getElementById(model).checked) {
            models_to_run.push(model);
        }
    }
    return models_to_run;
}

function reset_status(models_to_run) {
    let model;
    for (model of models_to_run) {
        document.getElementById(`${model}_status`).hidden = false;
        document.getElementById(`${model}_status`).textContent = `${model}: test not yet run`;
    }
}

function hide_results() {
    let model;
    for (model of models) {
        document.getElementById(`${model}_status`).hidden = true;
    }
}

function parse_test_type(test_type) {
    const regex = /^(\d+)_threads_(\d+)_instructions$/;
    const match = test_type.match(regex);
    if(match) {
        const threads = parseInt(match[1], 10);
        const instructions = parseInt(match[2], 10);
        return [threads, instructions];
    }
    else {
        console.log("ERROR when parsing test type!!");
        return;
    }
}

async function run_test(model, threads, instructions, test_num, heuristic) {
    if (heuristics.slice(0, 3).includes(heuristic)) {
        return wasm_mod.run(threads, `tests/${model}/${threads}_threads_${instructions}_instructions/${test_num}/${test_num}_${heuristic}.wgsl`, threads, true);
    }
    else {
        return wasm_mod.run(threads, `tests/${model}/${threads}_threads_${instructions}_instructions/${test_num}/${test_num}_${heuristic}.wgsl`, 32, true);
    }
}

async function wait(ms) {
    await new Promise(r => setTimeout(r, ms));
}

async function run_model_tests(model, status) {
    let some_failed = false;
    let results = [];
    let test_results;
    for (let heuristic of heuristics) {
        results = [];
        status.textContent = `${model}: running ${heuristic} tests...`;
        for (let threads_instructions of Object.keys(test_json[model])){
            let thread_insts = parse_test_type(threads_instructions);
            let threads = thread_insts[0];
            let instructions = thread_insts[1];
            for (let test_num of test_json[model][threads_instructions]){
                results.push(run_test(model, threads, instructions, test_num, heuristic));
            }
        }
        //sus? 
        //sus
        test_results = results.map((test) => 
            test.then((result) => {
                counter+= 1;
                document.getElementById('counter').textContent = `Num tests finished: ${counter}`;
                return result;
            })
        );
        test_results = await Promise.all(test_results);
        for (let result of test_results) {
            if (Number.isInteger(result) && result > 0) {
                console.log(result);
                some_failed |= false;
            }
            else{
                console.log(result);
                some_failed |= true;
            }
        }
    }
    return some_failed;
}

export async function run_all_tests() {
    let models_to_run = get_models();
    hide_results();
    reset_status(models_to_run);
    counter = 0;
    if (models_to_run.length > 0) {
        document.getElementById("results").hidden = false;
    }
    else {
        // display some error message to select tests
        return;
    }
    let model;
    for (model of models_to_run) {
        const model_status = document.getElementById(`${model}_status`);
        model_status.textContent = `${model}: running tests...`;
        const some_failed = await run_model_tests(model, model_status);
        model_status.textContent = `${model}: tests finished`+ (some_failed ? ", some tests failed." : ", all tests passed.");
        await new Promise(r => setTimeout(r, 500));
    }
}

await init_stuff();