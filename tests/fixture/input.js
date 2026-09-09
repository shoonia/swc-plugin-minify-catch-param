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
  if (globalThis) {
    let e = "error";
    console.log(e);
  }
}

try {
  throw new Error();
} catch (e) {
  if (globalThis) {
    var e = "Don't work with var";
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

