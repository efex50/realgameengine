/* tslint:disable */
/* eslint-disable */

export function __engine_start(): void;

export function __init_thread_pool(a: number): void;

export function main(): void;

export function send_message(message: any): void;

export function set_wasmjs_path(path: string): void;

export function worker_loop(ptr: number): void;

export function worker_loop_once(ptr: number): void;

export function zort(): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly __engine_start: () => void;
  readonly main: () => void;
  readonly set_wasmjs_path: (a: number, b: number) => void;
  readonly worker_loop: (a: number) => void;
  readonly worker_loop_once: (a: number) => void;
  readonly __init_thread_pool: (a: number) => void;
  readonly send_message: (a: any) => [number, number];
  readonly zort: () => any;
  readonly wasm_bindgen_7eb2272477f76a2d___convert__closures_____invoke___alloc_caa1bee655c9ad29___string__String_____: (a: number, b: number, c: number, d: number) => void;
  readonly wasm_bindgen_7eb2272477f76a2d___closure__destroy___dyn_core_6a44765f88230d85___ops__function__FnMut__alloc_caa1bee655c9ad29___string__String____Output_______: (a: number, b: number) => void;
  readonly wasm_bindgen_7eb2272477f76a2d___convert__closures_____invoke___wasm_bindgen_7eb2272477f76a2d___JsValue_____: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen_7eb2272477f76a2d___closure__destroy___dyn_core_6a44765f88230d85___ops__function__FnMut__wasm_bindgen_7eb2272477f76a2d___JsValue____Output_______: (a: number, b: number) => void;
  readonly wasm_bindgen_7eb2272477f76a2d___convert__closures_____invoke______: (a: number, b: number) => void;
  readonly wasm_bindgen_7eb2272477f76a2d___convert__closures_____invoke___wasm_bindgen_7eb2272477f76a2d___JsValue__wasm_bindgen_7eb2272477f76a2d___JsValue_____: (a: number, b: number, c: any, d: any) => void;
  readonly memory: WebAssembly.Memory;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_thread_destroy: (a?: number, b?: number, c?: number) => void;
  readonly __wbindgen_start: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput, memory?: WebAssembly.Memory, thread_stack_size?: number }} module - Passing `SyncInitInput` directly is deprecated.
* @param {WebAssembly.Memory} memory - Deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput, memory?: WebAssembly.Memory, thread_stack_size?: number } | SyncInitInput, memory?: WebAssembly.Memory): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput>, memory?: WebAssembly.Memory, thread_stack_size?: number }} module_or_path - Passing `InitInput` directly is deprecated.
* @param {WebAssembly.Memory} memory - Deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput>, memory?: WebAssembly.Memory, thread_stack_size?: number } | InitInput | Promise<InitInput>, memory?: WebAssembly.Memory): Promise<InitOutput>;
