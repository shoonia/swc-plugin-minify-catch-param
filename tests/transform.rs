mod setup;

use crate::setup::{syntax, visitor};
use swc_core::ecma::transforms::testing::test_transform;

pub fn run_test(input: &str, expected: &str) {
    test_transform(syntax(), Some(true), |_| visitor(), input, expected);
}

#[test]
fn the_tests() {
    let cases = [(
        "try {} catch (error) {}",
        "try {} catch  {}"
    ), (
        "try {} catch (e) {}",
        "try {} catch  {}"
    ), (
        "try {} catch (error) { const helper = () => true; helper(); }",
        "try {} catch { const helper = () => true; helper(); }"
    ), (
        "try {} catch (error) { const err = { error: 1 }; }",
        "try {} catch { const err = { error: 1 }; }"
    ), (
        "try {} catch (error) { const err = { error: error }; }",
        "try {} catch (error) { const err = { error: error }; }",
    ), (
        "try {} catch {}", 
        "try {} catch  {}"
    ), (
        "try {} catch ({message}) { console.log(message); }",
        "try {} catch ({ message }) { console.log(message);}",
    ), (
        "try {} catch ({message}) { console.log('error occurred'); }",
        "try {} catch ({ message }) { console.log('error occurred');}",
    ), (
        "try {} catch ([first]) { console.log(first); }",
        "try {} catch ([first]) { console.log(first);}",
    ), (
        "try {} catch (error) { console.log(error) }",
        "try {} catch (error) { console.log(error);}"
    ), (
        "function test() { try {} catch (error) { return error; } }",
        "function test() { try {} catch (error) { return error;  }}"
    ), (
        "function test() { try {} catch (error) { throw error; } }",
        "function test() { try {} catch (error) { throw error;  }}"
    ), (
        "try {} catch (error) { error = 'new error'; console.log(error); }",
        "try {} catch (error) { error = 'new error';    console.log(error);}",
    ), (
        "try {} catch (error) { console.log(error?.message?.toLowerCase()); }",
        "try {} catch (error) { console.log(error?.message?.toLowerCase()); }",
    ), (
        "try {} catch (error) { const message = error?.message ?? 'Unknown error'; console.log(message); }",
        "try {} catch (error) { const message = error?.message ?? 'Unknown error'; console.log(message); }",
    ), (
        "try {} catch (error) { const message = `Error: ${error}`; console.log(message); }",
        "try {} catch (error) { const message = `Error: ${error}`; console.log(message); }",
    ), (
        "try {} catch (error) { const errors = [...someArray, error]; console.log(errors); }",
        "try {} catch (error) { const errors = [...someArray, error]; console.log(errors);}",
    ), (
        "try {} catch (error) { for (const item of [error]) { console.log(item); } }",
        "try {} catch (error) { for (const item of [error]) { console.log(item); } }",
    ), (
        "try {} catch (error) { import('./logger.js').then(logger => { logger.error(error); }); }",
        "try {} catch (error) { import('./logger.js').then((logger) => { logger.error(error); }); }",
    ), (
        "try {} catch (error) { let message; console.log(message = error?.message); }",
        "try {} catch (error) { let message; console.log(message = error?.message); }",
    ), (
        "try {} catch (error) { let count = 0; count += error.count || 1; console.log(count); }",
        "try {} catch (error) { let count = 0; count += error.count || 1; console.log(count); }",
    ), (
        "try {} catch (error) { error.count++; console.log(error.count); }",
        "try {} catch (error) { error.count++; console.log(error.count); }",
    ), (
        "try {} catch (error) { delete error.stack; console.log(error); }",
        "try {} catch (error) { delete error.stack; console.log(error); }",
    ), (
        "try {} catch (error) { const customError = new Error(error.message); console.log(customError); }",
        "try {} catch (error) { const customError = new Error(error.message); console.log(customError); }",
    ), (
        "try {} catch (error) { if (error instanceof TypeError) { console.log('Type error:', error); } }",
        "try {} catch (error) { if (error instanceof TypeError) { console.log('Type error:', error); } }",
    ), (
        "try {} catch (error) { if ('message' in error) { console.log(error.message); } }",
        "try {} catch (error) { if ('message' in error) { console.log(error.message); } }",
    ), (
        "function test() { try {} catch (error) { return { error }; }}",
        "function test() { try {} catch (error) { return { error }; }}",
    ), (
        "try {} catch (error) { const { message } = error; console.log(message); }",
        "try {} catch (error) { const { message } = error; console.log(message); }",
    ), (
        "try {} catch (error) { const obj = { [error.type]: error.message }; console.log(obj); }",
        "try {} catch (error) { const obj = { [error.type]: error.message  }; console.log(obj); }",
    ), (
        "try {} catch (error) { function tag(strings, ...values) { return strings[0] + values[0]; } const message = tag`Error: ${error}`; console.log(message); }",
        "try {} catch (error) { function tag(strings, ...values) { return strings[0] + values[0];  } const message = tag`Error: ${error}`; console.log(message); }",
    ), (
        "try {} catch (error) { console.log({ error }); }",
        "try {} catch (error) { console.log({ error }); }",
    ), (
        "async function test() { try { await fetch('/api'); } catch (error) { console.log('Request failed'); } }",
        "async function test() { try { await fetch('/api'); } catch { console.log('Request failed'); } }",
    ), (
        "async function test() { try { await fetch('/api'); } catch (error) { console.log(error); } }",
        "async function test() { try { await fetch('/api');  } catch (error) { console.log(error); } }",
    ), (
        "try {} catch (error) { for (const key in error) { console.log(key, error[key]); } }",
        "try {} catch (error) { for (const key in error) { console.log(key, error[key]); } }",
    ), (
        "try {} catch (error) { const errorMap = new Map([['error', error]]); const errorSet = new Set([error]); console.log(errorMap, errorSet); }",
        "try {} catch (error) { const errorMap = new Map([['error', error]]); const errorSet = new Set([error]); console.log(errorMap, errorSet); }",
    ), (
        "try {} catch (error) { const timestamp = BigInt(Date.now()); error.timestamp = timestamp; console.log(error); }",
        "try {} catch (error) { const timestamp = BigInt(Date.now()); error.timestamp = timestamp; console.log(error); }",
    ), (
        "class ErrorLogger { #errors = []; log() { try {} catch (error) { this.#errors.push(error); console.log(this.#errors); } } }",
        "class ErrorLogger { #errors = []; log() { try {} catch (error) { this.#errors.push(error); console.log(this.#errors); } } }",
    ), (
        "class ErrorLogger { static errors = []; static log() { try {} catch (error) { this.errors.push(error); console.log(this.errors); } } }",
        "class ErrorLogger { static errors = []; static log() { try {} catch (error) { this.errors.push(error); console.log(this.errors); } } }",
    ), (
        "try {} catch (error) { // This error is not used /* Multi-line comment */ }",
        "try {} catch  {}",
    ), (
        "try { try {} catch (error) { console.log(error); }} catch (error) {}",
        "try { try {} catch (error) { console.log(error); }} catch {}",
    ), (
        "try {} catch (error) { function fn(error) { console.log(error); } fn('test');}",
        "try {} catch { function fn(error) { console.log(error); } fn('test');}",
    ), (
        "try {} catch (error) { const fn = (error) => { console.log(error); }; fn('test'); }",
        "try {} catch { const fn = (error) => { console.log(error); }; fn('test'); }",
    ), (
        "try {} catch (error) { const handler = () => { setTimeout(() => { console.log(error); }, 100); }; handler(); }",
        "try {} catch (error) { const handler = () => { setTimeout(() => { console.log(error); }, 100); }; handler(); }",
    ),
    // ( // TODO: fix var declarations
    //     "try {} catch (error) { if (true) { var error = anError(); console.log(error); } }",
    //     "try {} catch { if (true) { var error = anError(); console.log(error); } }",
    // );
    (
        "try {} catch (error) { if (true) { let error = anError(); console.log(error); } }",
        "try {} catch { if (true) { let error = anError(); console.log(error); } }",
    ), (
        "try {} catch (error) { if (true) { const error = anError(); console.log(error); } }",
        "try {} catch { if (true) { const error = anError(); console.log(error); } }",
    ), (
        "try {} catch (error) { if (true) { using error = anError(); console.log(error); } }",
        "try {} catch { if (true) { using error = anError(); console.log(error); } }",
    ), (
        "try {} catch (console) { window.console.log('this uses global console'); }",
        "try {} catch { window.console.log('this uses global console');}",
    ), (
        "try {} catch (error) { eval('console.log(error)'); }",
        "try {} catch { eval('console.log(error)'); }",
    ), (
        "function test() { try {} catch (e1) { console.log(e1); } try {} catch (e2) { } try {} catch (e3) { console.log(e3); } }",
        "function test() { try {} catch (e1) { console.log(e1); } try {} catch {} try {} catch (e3) { console.log(e3); } }",
    ), (
        "try {} catch (помилка) { console.log(помилка); }",
        "try {} catch (помилка) { console.log(помилка); }",
    ), (
        "try {} catch (veryLongErrorParameterNameThatSomeoneDecidedToUse) { console.log(veryLongErrorParameterNameThatSomeoneDecidedToUse); }",
        "try {} catch (veryLongErrorParameterNameThatSomeoneDecidedToUse) { console.log(veryLongErrorParameterNameThatSomeoneDecidedToUse); }",
    ), (
        "try {} catch (error1) { console.log(error1); }",
        "try {} catch (error1) { console.log(error1); }",
    )];

    for (input, expected) in cases {
        run_test(input, expected);
    }
}
