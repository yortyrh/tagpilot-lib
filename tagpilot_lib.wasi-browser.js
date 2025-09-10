import {
  createOnMessage as __wasmCreateOnMessageForFsProxy,
  getDefaultContext as __emnapiGetDefaultContext,
  instantiateNapiModuleSync as __emnapiInstantiateNapiModuleSync,
  WASI as __WASI,
} from '@napi-rs/wasm-runtime'

const __wasi = new __WASI({
  version: 'preview1',
})

const __wasmUrl = new URL('./tagpilot_lib.wasm32-wasi.wasm', import.meta.url).href
const __emnapiContext = __emnapiGetDefaultContext()

const __sharedMemory = new WebAssembly.Memory({
  initial: 4000,
  maximum: 65536,
  shared: true,
})

const __wasmFile = await fetch(__wasmUrl).then((res) => res.arrayBuffer())

const {
  instance: __napiInstance,
  module: __wasiModule,
  napiModule: __napiModule,
} = __emnapiInstantiateNapiModuleSync(__wasmFile, {
  context: __emnapiContext,
  asyncWorkPoolSize: 4,
  wasi: __wasi,
  onCreateWorker() {
    const worker = new Worker(new URL('./wasi-worker-browser.mjs', import.meta.url), {
      type: 'module',
    })

    return worker
  },
  overwriteImports(importObject) {
    importObject.env = {
      ...importObject.env,
      ...importObject.napi,
      ...importObject.emnapi,
      memory: __sharedMemory,
    }
    return importObject
  },
  beforeInit({ instance }) {
    for (const name of Object.keys(instance.exports)) {
      if (name.startsWith('__napi_register__')) {
        instance.exports[name]()
      }
    }
  },
})
export default __napiModule.exports
export const clearTags = __napiModule.exports.clearTags
export const clearTagsToBuffer = __napiModule.exports.clearTagsToBuffer
export const readCoverImageFromBuffer = __napiModule.exports.readCoverImageFromBuffer
export const readCoverImageFromFile = __napiModule.exports.readCoverImageFromFile
export const readTags = __napiModule.exports.readTags
export const readTagsFromBuffer = __napiModule.exports.readTagsFromBuffer
export const writeCoverImageToBuffer = __napiModule.exports.writeCoverImageToBuffer
export const writeCoverImageToFile = __napiModule.exports.writeCoverImageToFile
export const writeTags = __napiModule.exports.writeTags
export const writeTagsToBuffer = __napiModule.exports.writeTagsToBuffer
