/* tslint:disable */
/* eslint-disable */
/**
* @param {number} num_threads
* @param {string} kernel_file
* @returns {Promise<number>}
*/
export function run(num_threads: number, kernel_file: string): Promise<number>;
/**
* @param {number} num_threads
* @param {string} kernel_file
* @returns {Promise<number | undefined>}
*/
export function execute_gpu(num_threads: number, kernel_file: string): Promise<number | undefined>;
/**
* @returns {Promise<string | undefined>}
*/
export function get_gpu_name(): Promise<string | undefined>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly run: (a: number, b: number, c: number) => number;
  readonly execute_gpu: (a: number, b: number, c: number) => number;
  readonly get_gpu_name: () => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly wasm_bindgen__convert__closures__invoke1_mut__h6b6d2e29e3206c02: (a: number, b: number, c: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hcda99035a652a04e: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h094c2402486d7f8f: (a: number, b: number, c: number, d: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;