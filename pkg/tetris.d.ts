/* tslint:disable */
/* eslint-disable */
/**
* @param {any} props
* @returns {any}
*/
export function App(props: any): any;
/**
* Contains all functions exported to JS by `wasm-react`. These functions should
* be called from JS only.
*/
export class WasmReact {
  free(): void;
/**
* Set the React runtime that `wasm-react` should use.
*
* Calling this function multiple times will result in no-ops.
*
* # Example
*
* ```js
* import React from "react";
* import init, { WasmReact } from "./path/to/pkg/project.js";
*
* async function main() {
*   await init();
*   WasmReact.useReact(React);
* }
*
* main();
* ```
* @param {any} value
*/
  static useReact(value: any): void;
}
/**
*/
export class __WasmReact_ComponentWrapper {
  free(): void;
/**
* @returns {any}
*/
  render(): any;
}
/**
*/
export class __WasmReact_MemoComponentWrapper {
  free(): void;
/**
* @returns {any}
*/
  render(): any;
/**
* @param {__WasmReact_MemoComponentWrapper} other
* @returns {boolean}
*/
  eq(other: __WasmReact_MemoComponentWrapper): boolean;
}
/**
*/
export class __WasmReact_RefContainerValue {
  free(): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly App: (a: number, b: number) => void;
  readonly __wbg___wasmreact_refcontainervalue_free: (a: number) => void;
  readonly wasmreact_useReact: (a: number) => void;
  readonly __wbg_wasmreact_free: (a: number) => void;
  readonly __wbg___wasmreact_componentwrapper_free: (a: number) => void;
  readonly __wasmreact_componentwrapper_render: (a: number) => number;
  readonly __wasmreact_memocomponentwrapper_eq: (a: number, b: number) => number;
  readonly __wbg___wasmreact_memocomponentwrapper_free: (a: number) => void;
  readonly __wasmreact_memocomponentwrapper_render: (a: number) => number;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hbaf39527fd2bf25f: (a: number, b: number, c: number) => number;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hbb280000cea2c374: (a: number, b: number) => number;
  readonly _dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h781f7992d9b2b035: (a: number, b: number) => void;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0f56633ace5182b1: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke1_mut_ref__h21fba54283c8a729: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

/**
* Synchronously compiles the given `bytes` and instantiates the WebAssembly module.
*
* @param {BufferSource} bytes
*
* @returns {InitOutput}
*/
export function initSync(bytes: BufferSource): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
