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

## Use With SWC

Install SWC in your application:

```bash
npm install --save-dev @swc/core swc-plugin-minify-catch-param
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
        ["swc-plugin-minify-catch-param", {}]
      ]
    }
  }
}
```

## License

[MIT](./LICENSE)
