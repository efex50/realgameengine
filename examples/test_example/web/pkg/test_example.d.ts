/* tslint:disable */
/* eslint-disable */

export function main(): void;

export function send_message(message: any): void;

/**
 * Entry point for web workers
 */
export function wasm_thread_entry_point(ptr: number): void;

export function zort(): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly main: () => void;
  readonly wasm_thread_entry_point: (a: number) => void;
  readonly send_message: (a: any) => [number, number];
  readonly zort: () => any;
  readonly wasm_bindgen_1f692f92156636f___convert__closures________invoke___web_sys_a64f4ad86b41b567___features__gen_MessageEvent__MessageEvent_____: (a: number, b: number, c: any) => void;
  readonly wasm_bindgen_1f692f92156636f___closure__destroy___dyn_for__a__core_ab827750d4a5d7c7___ops__function__FnMut____a_web_sys_a64f4ad86b41b567___features__gen_MessageEvent__MessageEvent____Output_______: (a: number, b: number) => void;
  readonly wasm_bindgen_1f692f92156636f___convert__closures_____invoke___alloc_278741e700e20762___string__String_____: (a: number, b: number, c: number, d: number) => void;
  readonly wasm_bindgen_1f692f92156636f___closure__destroy___dyn_core_ab827750d4a5d7c7___ops__function__FnMut__alloc_278741e700e20762___string__String____Output_______: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
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
