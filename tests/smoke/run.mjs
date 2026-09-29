import { transform } from "@swc/core";
import { resolve } from "node:path";

const output = await transform("try {} catch (error) { }", {
  jsc: {
    parser: {
      syntax: "ecmascript"
    },
    experimental: {
      plugins: [[resolve("index.wasm"), {}]]
    }
  }
});

console.log(output.code);
