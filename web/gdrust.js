// thin wrapper over the wasm exports, no bindgen needed
export async function load(url) {
  const { instance } = await WebAssembly.instantiateStreaming(fetch(url), {});
  return wrap(instance);
}

export function wrap(instance) {
  const x = instance.exports;
  const enc = new TextEncoder();
  const dec = new TextDecoder();
  // memory.buffer changes when wasm grows, so always re-read it
  const read = (ptr, len) => dec.decode(new Uint8Array(x.memory.buffer, ptr, len));

  return {
    compile(src) {
      const bytes = enc.encode(src);
      const ptr = x.gd_alloc(bytes.length);
      new Uint8Array(x.memory.buffer, ptr, bytes.length).set(bytes);
      const n = x.gd_compile(ptr, bytes.length);
      x.gd_free(ptr, bytes.length);
      const ll = x.gd_log_len();
      return {
        ok: x.gd_out_ok() === 1,
        text: read(x.gd_out_ptr(), n),
        objects: x.gd_out_objects(),
        log: ll ? read(x.gd_log_ptr(), ll) : "",
      };
    },
  };
}
