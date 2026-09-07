try {
    throw new Error();
} catch  {}
try {
    throw new Error();
} catch  {
    if (globalThis) {
        const e = "error";
        console.log(e);
    }
}
try {
    throw new Error();
} catch  {
    function fn(e) {
        console.log(e);
    }
    fn("error");
}
