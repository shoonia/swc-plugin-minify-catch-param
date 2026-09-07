import { readFile } from "node:fs/promises";
import { transform } from "@swc/core";

const wasmPath = new URL(
  "../../target/wasm32-wasip1/release/swc_plugin_minify_catch_param.wasm",
  import.meta.url
);

await readFile(wasmPath);

const output = await transform("try {} catch (error) { console.log('failed'); }", {
  jsc: {
    parser: {
      syntax: "typescript"
    },
    experimental: {
      plugins: [[wasmPath.pathname, {}]]
    }
  }
});

if (!/catch\s*\{/.test(output.code) || /catch\s*\(error\)/.test(output.code)) {
  throw new Error(`Unexpected smoke output:\n${output.code}`);
}

console.log(output.code.trim());
