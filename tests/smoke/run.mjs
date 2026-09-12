import { transform } from "@swc/core";
import { resolve } from "node:path";

const output = await transform("try {} catch (error) { }", {
  jsc: {
    parser: {
      syntax: "ecmascript"
    },
    experimental: {
      plugins: [[resolve("target/wasm32-wasip1/release/swc_plugin_minify_catch_param.wasm"), {}]]
    }
  }
});

console.log(output.code);
