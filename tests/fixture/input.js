try {
  throw new Error();
} catch (e) {
}

try {
  throw new Error();
} catch (e) {
  if (globalThis) {
    const e = "error";
    console.log(e);
  }
}

try {
  throw new Error();
} catch (e) {
  function fn(e) {
    console.log(e);
  }
  fn("error")
}
