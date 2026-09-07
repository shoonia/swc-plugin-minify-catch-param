use swc_core::{
    common::Mark,
    ecma::{
        parser::{EsSyntax, Syntax},
        transforms::{base::resolver, testing::test_transform},
        visit::visit_mut_pass,
    },
};
use swc_plugin_minify_catch_param::CatchParamMinifier;

fn syntax() -> Syntax {
    Syntax::Es(EsSyntax {
        jsx: false,
        ..Default::default()
    })
}

pub fn run_test(a: &str, b: &str) {
    test_transform(
        syntax(),
        Some(true),
        |_| {
            (
                resolver(Mark::new(), Mark::new(), false),
                visit_mut_pass(CatchParamMinifier),
            )
        },
        a,
        b,
    );
}

#[test]
fn removes_unused_simple_catch_params() {
    run_test("try {} catch (error) {}", "try {} catch  {}");
    run_test("try {} catch (e) {}", "try {} catch  {}");
    run_test(
        "try {} catch (error) { const helper = () => true; helper(); }",
        "try {} catch { const helper = () => true; helper(); }",
    );
}

#[test]
fn leaves_empty_and_destructuring_catches_alone() {
    run_test("try {} catch {}", "try {} catch  {}");
    run_test(
        "try {} catch ({message}) { console.log(message); }",
        "try {} catch ({ message }) { console.log(message);}",
    );
    run_test(
        "try {} catch ({message}) { console.log('error occurred'); }",
        "try {} catch ({ message }) { console.log('error occurred');}",
    );
    run_test(
        "try {} catch ([first]) { console.log(first); }",
        "try {} catch ([first]) { console.log(first);}",
    );
}

#[test]
fn keeps_used_params() {
    run_test(
        "try {} catch (error) { console.log(error) }",
        "try {} catch (error) { console.log(error);}",
    );
    run_test(
        "function test() { try {} catch (error) { return error; } }",
        "function test() { try {} catch (error) { return error;  }}",
    );
    run_test(
        "function test() { try {} catch (error) { throw error; } }",
        "function test() { try {} catch (error) { throw error;  }}",
    );
    run_test(
        "try {} catch (error) { error = 'new error'; console.log(error); }",
        "try {} catch (error) { error = 'new error';    console.log(error);}",
    );
}

#[test]
fn handles_modern_usage_sites() {
    let cases = [(
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
    )
    ];

    for (input, expected) in cases {
        run_test(input, expected);
    }
}

#[test]
fn keeps_params_used_by_operators_and_expressions() {
    let cases = [(
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
    )];

    for (input, expected) in cases {
        run_test(input, expected);
    }
}

#[test]
fn keeps_params_used_by_object_array_and_template_forms() {
    let cases = [(
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
    )];

    for (input, expected) in cases {
        run_test(input, expected);
    }
}

#[test]
fn handles_async_loops_and_collection_usage() {
    let cases = [(
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
    )];

    for (input, expected) in cases {
        run_test(input, expected);
    }
}

#[test]
fn handles_classes_and_empty_comment_catches() {
    run_test(
        "class ErrorLogger { #errors = []; log() { try {} catch (error) { this.#errors.push(error); console.log(this.#errors); } } }",
        "class ErrorLogger { #errors = []; log() { try {} catch (error) { this.#errors.push(error); console.log(this.#errors); } } }",
    );
    run_test(
        "class ErrorLogger { static errors = []; static log() { try {} catch (error) { this.errors.push(error); console.log(this.errors); } } }",
        "class ErrorLogger { static errors = []; static log() { try {} catch (error) { this.errors.push(error); console.log(this.errors); } } }",
    );
    run_test(
        "try {} catch (error) { // This error is not used /* Multi-line comment */ }",
        "try {} catch  {}",
    );
}

#[test]
fn respects_nested_and_shadowed_scopes() {
    run_test(
        "try { try {} catch (error) { console.log(error); }} catch (error) {}",
        "try { try {} catch (error) { console.log(error); }} catch {}",
    );
    run_test(
        "try {} catch (error) { function fn(error) { console.log(error); } fn('test');}",
        "try {} catch { function fn(error) { console.log(error); } fn('test');}",
    );
    run_test(
        "try {} catch (error) { const fn = (error) => { console.log(error); }; fn('test'); }",
        "try {} catch { const fn = (error) => { console.log(error); }; fn('test'); }",
    );
    run_test(
        "try {} catch (error) { const handler = () => { setTimeout(() => { console.log(error); }, 100); }; handler(); }",
        "try {} catch (error) { const handler = () => { setTimeout(() => { console.log(error); }, 100); }; handler(); }",
    );
}

#[test]
fn preserves_babel_documented_limitations() {
    // run_test( // TODO: fix var declarations
    //     "try {} catch (error) { if (true) { var error = anError(); console.log(error); } }",
    //     "try {} catch { if (true) { var error = anError(); console.log(error); } }",
    // );
    run_test(
        "try {} catch (error) { if (true) { let error = anError(); console.log(error); } }",
        "try {} catch { if (true) { let error = anError(); console.log(error); } }",
    );
    run_test(
        "try {} catch (error) { if (true) { const error = anError(); console.log(error); } }",
        "try {} catch { if (true) { const error = anError(); console.log(error); } }",
    );
    run_test(
        "try {} catch (error) { if (true) { using error = anError(); console.log(error); } }",
        "try {} catch { if (true) { using error = anError(); console.log(error); } }",
    );
    run_test(
        "try {} catch (console) { window.console.log('this uses global console'); }",
        "try {} catch { window.console.log('this uses global console');}",
    );
    run_test(
        "try {} catch (error) { eval('console.log(error)'); }",
        "try {} catch { eval('console.log(error)'); }",
    );
}

#[test]
fn handles_multiple_catches_independently() {
    run_test(
        "function test() { try {} catch (e1) { console.log(e1); } try {} catch (e2) { } try {} catch (e3) { console.log(e3); } }",
        "function test() { try {} catch (e1) { console.log(e1); } try {} catch {} try {} catch (e3) { console.log(e3); } }",
    );
}

#[test]
fn handles_unicode_and_name_shapes() {
    run_test(
        "try {} catch (помилка) { console.log(помилка); }",
        "try {} catch (помилка) { console.log(помилка); }",
    );
    run_test(
        "try {} catch (veryLongErrorParameterNameThatSomeoneDecidedToUse) { console.log(veryLongErrorParameterNameThatSomeoneDecidedToUse); }",
        "try {} catch (veryLongErrorParameterNameThatSomeoneDecidedToUse) { console.log(veryLongErrorParameterNameThatSomeoneDecidedToUse); }",
    );
    run_test(
        "try {} catch (error1) { console.log(error1); }",
        "try {} catch (error1) { console.log(error1);}",
    );
}
