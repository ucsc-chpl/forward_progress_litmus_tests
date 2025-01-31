/* tslint:disable */
/* eslint-disable */
export function init_gpu_objects(): Promise<void>;
export function run(num_threads: number, kernel_file: string, num_workgroups: number, use_persistent_adapter: boolean): Promise<number>;
export function execute_gpu(num_threads: number, kernel_file: string, num_workgroups: number, use_persistent_adapter: boolean): Promise<number | undefined>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init_gpu_objects: () => any;
  readonly run: (a: number, b: number, c: number, d: number, e: number) => any;
  readonly execute_gpu: (a: number, b: number, c: number, d: number, e: number) => any;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_5: WebAssembly.Table;
  readonly closure81_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure85_externref_shim: (a: number, b: number, c: any) => void;
  readonly closure107_externref_shim: (a: number, b: number, c: any, d: any) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
