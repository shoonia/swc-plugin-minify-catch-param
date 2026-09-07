# swc-plugin-minify-catch-param

SWC Wasm plugin that removes unused simple catch clause parameters.

```js
try {
  // ...
} catch (error) {
  // ...
}
```

becomes:

```js
try {
  // ...
} catch {
  // ...
}
```

The plugin mirrors the Babel plugin behavior for simple identifier catch parameters:

- removes only unused identifier parameters
- preserves used identifier parameters
- preserves destructuring catch parameters
- preserves documented conservative cases such as same-name `var` declarations and member-property name matches
- ignores dynamic string usage such as `eval("console.log(error)")`

## Build

Install the SWC Wasm target once:

```bash
rustup target add wasm32-wasip1
```

Build the plugin:

```bash
cargo build --target wasm32-wasip1 --release
```

The generated plugin file is:

```text
target/wasm32-wasip1/release/swc_plugin_minify_catch_param.wasm
```

## Use With SWC

Install SWC in your application:

```bash
npm install --save-dev @swc/core @swc/cli
```

Add the plugin to `.swcrc`.

For a local build of this repository:

```json
{
  "jsc": {
    "parser": {
      "syntax": "typescript",
      "tsx": true
    },
    "experimental": {
      "plugins": [
        [
          "./target/wasm32-wasip1/release/swc_plugin_minify_catch_param.wasm",
          {}
        ]
      ]
    }
  }
}
```

Then run SWC:

```bash
npx swc src --out-dir dist
```

If you publish this plugin as an npm package, use the package name instead of the local Wasm path:

```json
{
  "jsc": {
    "experimental": {
      "plugins": [
        ["swc-plugin-minify-catch-param", {}]
      ]
    }
  }
}
```

## Use With Vite

Vite does not run arbitrary SWC plugins by default. For React projects, use `@vitejs/plugin-react-swc`, which exposes SWC plugin support through its `plugins` option.

Install the Vite React SWC plugin:

```bash
npm install --save-dev @vitejs/plugin-react-swc
```

Configure `vite.config.ts` with the locally built Wasm file:

```ts
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";

export default defineConfig({
  plugins: [
    react({
      plugins: [
        [
          "./target/wasm32-wasip1/release/swc_plugin_minify_catch_param.wasm",
          {},
        ],
      ],
    }),
  ],
});
```

If the plugin is published to npm, configure it by package name:

```ts
import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";

export default defineConfig({
  plugins: [
    react({
      plugins: [["swc-plugin-minify-catch-param", {}]],
    }),
  ],
});
```

For non-React Vite projects, use a Vite plugin that delegates transforms to `@swc/core` and accepts normal SWC options, then pass this plugin through `jsc.experimental.plugins`.

## Compatibility Notes

SWC Wasm plugin compatibility depends on both:

- the Rust `swc_core` version used by this crate
- the `@swc/core` version used by your app or bundler

This crate uses `.cargo/config.toml` with `swc_ast_unknown` for modern SWC Wasm plugin compatibility. Prefer `@swc/core` `1.15.0` or newer when possible.

## Development

```bash
cargo fmt --check
cargo test
cargo clippy --all-targets -- -D warnings
cargo build --target wasm32-wasip1 --release
```

The optional smoke test requires a package manager and `@swc/core`:

```bash
npm install
npm run test:smoke
```

## References

- [SWC plugin getting started](https://swc.rs/docs/plugin/ecmascript/getting-started)
- [SWC Wasm plugin compatibility](https://swc.rs/docs/plugin/ecmascript/compatibility)
- [Vite plugins: `@vitejs/plugin-react-swc`](https://vite.dev/plugins/)
