//! A Wasm module can be compiled with multiple compilers.
//!
//! This example illustrates how to use the Singlepass compiler.
//!
//! You can run the example directly by executing in Wasmer root:
//!
//! ```shell
//! cargo run --example compiler-singlepass --release --features "singlepass"
//! ```
//!
//! Ready?
//!

use std::any::Any;
use std::arch::x86_64::*;
use wasmer::{imports, wat2wasm, Instance, Module, Store, Value};
use wasmer_compiler_singlepass::Singlepass;
use wasmer_engine_universal::Universal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = wat2wasm(
        r#"(module
                (func (export "add") (param f64 f64) (result f64)
                (f64.add (local.get 0) (local.get 1)))
                (func (export "add20") (param   f64 f64 f64 f64 f64
                                                f64 f64 f64 f64 f64
                                                f64 f64 f64 f64 f64
                                                f64 f64 f64 f64 f64) (result f64)
                    (f64.add
                        (f64.add
                            (f64.add
                                (f64.add
                                    (f64.add (local.get 0)  (local.get 1))
                                    (f64.add (local.get 2)  (local.get 3)))
                                (f64.add
                                    (f64.add (local.get 4)  (local.get 5))
                                    (f64.add (local.get 6)  (local.get 7))))
                                (f64.add
                                    (f64.add
                                        (f64.add (local.get 8)  (local.get 9))
                                    (f64.add (local.get 10) (local.get 11)))
                            (f64.add
                                (f64.add (local.get 12) (local.get 13))
                                (f64.add (local.get 14) (local.get 15))
                                )
                            )
                        )
                        (f64.add
                            (f64.add (local.get 16) (local.get 17))
                            (f64.add (local.get 18) (local.get 19))))
                    )
                (func (export "add20r") (param   f64 f64 f64 f64 f64
                                                f64 f64 f64 f64 f64
                                                f64 f64 f64 f64 f64
                                                f64 f64 f64 f64 f64) (result f64)
                    (f64.add (local.get 0)
                    (f64.add (local.get 1)
                    (f64.add (local.get 2)
                    (f64.add (local.get 3)
                    (f64.add (local.get 4)
                    (f64.add (local.get 5)
                    (f64.add (local.get 6)
                    (f64.add (local.get 7)
                    (f64.add (local.get 8)
                    (f64.add (local.get 9)
                    (f64.add (local.get 10)
                    (f64.add (local.get 11)
                    (f64.add (local.get 12)
                    (f64.add (local.get 13)
                    (f64.add (local.get 14)
                    (f64.add (local.get 15)
                    (f64.add (local.get 16)
                    (f64.add (local.get 17)
                    (f64.add (local.get 18)
                             (local.get 19)
                    ))))))))))))))))))))
                (func $double_then_add (param f64 f64) (result f64)
                    (f64.add (f64.mul (local.get 0) (f64.const 2))
                             (f64.mul (local.get 1) (f64.const 2)))
                )
                (func (export "double_then_add20") (param f64 f64 f64 f64 f64
                                             f64 f64 f64 f64 f64
                                             f64 f64 f64 f64 f64
                                             f64 f64 f64 f64 f64) (result f64)
                   (f64.add
                             (f64.add
                                      (f64.add (f64.add (call $double_then_add (local.get 0)  (local.get 1))
                                                        (call $double_then_add (local.get 2)  (local.get 3)))
                                               (f64.add (call $double_then_add (local.get 4)  (local.get 5))
                                                        (call $double_then_add (local.get 6)  (local.get 7))))
                                      (f64.add
                                               (f64.add (call $double_then_add (local.get 8)  (local.get 9))
                                                        (call $double_then_add (local.get 10) (local.get 11)))
                                               (f64.add (call $double_then_add (local.get 12) (local.get 13))
                                                        (call $double_then_add (local.get 14) (local.get 15)))))

                             (f64.add (call $double_then_add (local.get 16) (local.get 17))
                                      (call $double_then_add (local.get 18) (local.get 19))))
                )
            )"#
            .as_bytes(),
    )?;

    // Use Singlepass compiler with the default settings
    let compiler = Singlepass::default();

    // Create the store
    let store = Store::new(&Universal::new(compiler).engine());

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    // Create an empty import object.
    let import_object = imports! {};

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&module, &import_object)?;

    // let sum = instance.exports.get_function("add")?;
    //
    // let results = sum.call(&[
    //     Value::F64(1.0f32),
    //     Value::F64(2.0f32)
    // ])?;
    //
    // println!("Results: {:?}", results);

    let sum = instance.exports.get_function("add20r")?;

    let results = sum.call(&[
        Value::F64(1.0f64),
        Value::F64(2.0f64),
        Value::F64(3.0f64),
        Value::F64(4.0f64),
        Value::F64(5.0f64),
        Value::F64(6.0f64),
        Value::F64(7.0f64),
        Value::F64(8.0f64),
        Value::F64(9.0f64),
        Value::F64(10.0f64),
        Value::F64(11.0f64),
        Value::F64(12.0f64),
        Value::F64(13.0f64),
        Value::F64(14.0f64),
        Value::F64(15.0f64),
        Value::F64(16.0f64),
        Value::F64(17.0f64),
        Value::F64(18.0f64),
        Value::F64(19.0f64),
        Value::F64(20.0f64),
    ])?;

    println!("Results: {:?}", results);

    let sum = instance.exports.get_function("double_then_add20")?;
    let results = sum.call(&[
        Value::F64(11.0f64),
        Value::F64(12.0f64),
        Value::F64(13.0f64),
        Value::F64(14.0f64),
        Value::F64(15.0f64),
        Value::F64(16.0f64),
        Value::F64(17.0f64),
        Value::F64(18.0f64),
        Value::F64(19.0f64),
        Value::F64(20.0f64),
        Value::F64(1.0f64),
        Value::F64(2.0f64),
        Value::F64(3.0f64),
        Value::F64(4.0f64),
        Value::F64(5.0f64),
        Value::F64(6.0f64),
        Value::F64(7.0f64),
        Value::F64(8.0f64),
        Value::F64(9.0f64),
        Value::F64(10.0f64),
    ])?;

    println!("Results: {:?}", results);

    Ok(())
}

macro_rules! test_binop {
    ($method: expr, $op: tt) => (
        fn _test() -> Result<(), Box<dyn std::error::Error>> {
            println!("Running {} test...", $method);
            let wat =
                r#"
                (module
                  (type $method_t (func (param f64 f64) (result f64)))
                  (func $method_f (type $method_t) (param $x f64) (param $y f64) (result f64)
                    local.get $x
                    local.get $y
                    f64.method)
                  (export "method" (func $method_f)))
                "#
                .replace("method", $method);
            // Let's declare the Wasm module with the text representation.
            let wasm_bytes = wat2wasm(wat.as_bytes(),
            )?;

            // Use Singlepass compiler with the default settings
            let compiler = Singlepass::default();

            // Create the store
            let store = Store::new(&Universal::new(compiler).engine());

            println!("Compiling module...");
            // Let's compile the Wasm module.
            let module = Module::new(&store, wasm_bytes)?;

            // Create an empty import object.
            let import_object = imports! {};

            println!("Instantiating module...");
            // Let's instantiate the Wasm module.
            let instance = Instance::new(&module, &import_object)?;

            test_float64_bin_op!(instance, $method, $op, 2.2, 2.4);
            test_float64_bin_op!(instance, $method, $op, 2.2, -2.4);
            test_float64_bin_op!(instance, $method, $op, 2.4, 2.4);
            test_float64_bin_op!(instance, $method, $op, 2.4, -2.4);
            test_float64_bin_op!(instance, $method, $op, 2.6, 2.4);
            test_float64_bin_op!(instance, $method, $op, 2.6, -2.4);
            test_float64_bin_op!(instance, $method, $op, -2.2, 2.4);
            test_float64_bin_op!(instance, $method, $op, -2.2, -2.4);
            test_float64_bin_op!(instance, $method, $op, -2.4, 2.4);
            test_float64_bin_op!(instance, $method, $op, -2.4, -2.4);
            test_float64_bin_op!(instance, $method, $op, -2.6, 2.4);
            test_float64_bin_op!(instance, $method, $op, -2.6, -2.4);

            Ok(())
        }
        _test()?
    );
    ($method: expr; $op: ident) => (
        fn _test() -> Result<(), Box<dyn std::error::Error>> {
            println!("Running {} test...", $method);
            let wat =
                r#"
                (module
                  (type $method_t (func (param f64 f64) (result f64)))
                  (func $method_f (type $method_t) (param $x f64) (param $y f64) (result f64)
                    local.get $x
                    local.get $y
                    f64.method)
                  (export "method" (func $method_f)))
                "#
                .replace("method", $method);
            // Let's declare the Wasm module with the text representation.
            let wasm_bytes = wat2wasm(wat.as_bytes(),
            )?;

            // Use Singlepass compiler with the default settings
            let compiler = Singlepass::default();

            // Create the store
            let store = Store::new(&Universal::new(compiler).engine());

            println!("Compiling module...");
            // Let's compile the Wasm module.
            let module = Module::new(&store, wasm_bytes)?;

            // Create an empty import object.
            let import_object = imports! {};

            println!("Instantiating module...");
            // Let's instantiate the Wasm module.
            let instance = Instance::new(&module, &import_object)?;

            test_float64_bin_op!(instance, $method, 2.2, 2.4; $op(2.2, 2.4));
            test_float64_bin_op!(instance, $method, 2.2, -2.4; $op(2.2, -2.4));
            test_float64_bin_op!(instance, $method, 2.4, 2.4; $op(2.4, 2.4));
            test_float64_bin_op!(instance, $method, 2.4, -2.4; $op(2.4, -2.4));
            test_float64_bin_op!(instance, $method, 2.6, 2.4; $op(2.6, 2.4));
            test_float64_bin_op!(instance, $method, 2.6, -2.4; $op(2.6, -2.4));
            test_float64_bin_op!(instance, $method, -2.2, 2.4; $op(-2.2, 2.4));
            test_float64_bin_op!(instance, $method, -2.2, -2.4; $op(-2.2, -2.4));
            test_float64_bin_op!(instance, $method, -2.4, 2.4; $op(-2.4, 2.4));
            test_float64_bin_op!(instance, $method, -2.4, -2.4; $op(-2.4, -2.4));
            test_float64_bin_op!(instance, $method, -2.6, 2.4; $op(-2.6, 2.4));
            test_float64_bin_op!(instance, $method, -2.6, -2.4; $op(-2.6, -2.4));

            Ok(())
        }
        _test()?
    );
}

macro_rules! test_cmpop {
    ($method: expr; $op: ident) => (
        fn _test() -> Result<(), Box<dyn std::error::Error>> {
            println!("Running {} test...", $method);
            let wat =
                r#"
                (module
                  (type $method_t (func (param f64 f64) (result i32)))
                  (func $method_f (type $method_t) (param $x f64) (param $y f64) (result i32)
                    local.get $x
                    local.get $y
                    f64.method)
                  (export "method" (func $method_f)))
                "#
                .replace("method", $method);
            // Let's declare the Wasm module with the text representation.
            let wasm_bytes = wat2wasm(wat.as_bytes(),
            )?;

            // Use Singlepass compiler with the default settings
            let compiler = Singlepass::default();

            // Create the store
            let store = Store::new(&Universal::new(compiler).engine());

            println!("Compiling module...");
            // Let's compile the Wasm module.
            let module = Module::new(&store, wasm_bytes)?;

            // Create an empty import object.
            let import_object = imports! {};

            println!("Instantiating module...");
            // Let's instantiate the Wasm module.
            let instance = Instance::new(&module, &import_object)?;

            test_float64_bin_op!(instance, $method, 2.2, 2.4; $op(2.2, 2.4));
            test_float64_bin_op!(instance, $method, 2.2, -2.4; $op(2.2, -2.4));
            test_float64_bin_op!(instance, $method, 2.4, 2.4; $op(2.4, 2.4));
            test_float64_bin_op!(instance, $method, 2.4, -2.4; $op(2.4, -2.4));
            test_float64_bin_op!(instance, $method, 2.6, 2.4; $op(2.6, 2.4));
            test_float64_bin_op!(instance, $method, 2.6, -2.4; $op(2.6, -2.4));
            test_float64_bin_op!(instance, $method, -2.2, 2.4; $op(-2.2, 2.4));
            test_float64_bin_op!(instance, $method, -2.2, -2.4; $op(-2.2, -2.4));
            test_float64_bin_op!(instance, $method, -2.4, 2.4; $op(-2.4, 2.4));
            test_float64_bin_op!(instance, $method, -2.4, -2.4; $op(-2.4, -2.4));
            test_float64_bin_op!(instance, $method, -2.6, 2.4; $op(-2.6, 2.4));
            test_float64_bin_op!(instance, $method, -2.6, -2.4; $op(-2.6, -2.4));

            Ok(())
        }
        _test()?
    );
}

macro_rules! test_unop {
    ($method: expr, $op: ident, $test_neg: expr) => {
        fn _test() -> Result<(), Box<dyn std::error::Error>> {
            println!("Running {} test...", $method);
            let wat = r#"
                (module
                  (type $method_t (func (param f64) (result f64)))
                  (func $method_f (type $method_t) (param $x f64) (result f64)
                    local.get $x
                    f64.method)
                  (export "method" (func $method_f)))
                "#
            .replace("method", $method);
            // Let's declare the Wasm module with the text representation.
            let wasm_bytes = wat2wasm(wat.as_bytes())?;

            // Use Singlepass compiler with the default settings
            let compiler = Singlepass::default();

            // Create the store
            let store = Store::new(&Universal::new(compiler).engine());

            println!("Compiling module...");
            // Let's compile the Wasm module.
            let module = Module::new(&store, wasm_bytes)?;

            // Create an empty import object.
            let import_object = imports! {};

            println!("Instantiating module...");
            // Let's instantiate the Wasm module.
            let instance = Instance::new(&module, &import_object)?;

            // test even
            test_float64_un_op!(instance, $method, 2.0, $op(2.0));
            test_float64_un_op!(instance, $method, 2.1, $op(2.1));
            test_float64_un_op!(instance, $method, 2.3, $op(2.3));
            test_float64_un_op!(instance, $method, 2.5, $op(2.5));
            test_float64_un_op!(instance, $method, 2.7, $op(2.7));
            test_float64_un_op!(instance, $method, 2.9, $op(2.9));

            // test odd
            test_float64_un_op!(instance, $method, 1.0, $op(1.0));
            test_float64_un_op!(instance, $method, 1.1, $op(1.1));
            test_float64_un_op!(instance, $method, 1.3, $op(1.3));
            test_float64_un_op!(instance, $method, 1.5, $op(1.5));
            test_float64_un_op!(instance, $method, 1.7, $op(1.7));
            test_float64_un_op!(instance, $method, 1.9, $op(1.9));

            if $test_neg {
                // test even
                test_float64_un_op!(instance, $method, -2.0, $op(-2.0));
                test_float64_un_op!(instance, $method, -2.1, $op(-2.1));
                test_float64_un_op!(instance, $method, -2.3, $op(-2.3));
                test_float64_un_op!(instance, $method, -2.5, $op(-2.5));
                test_float64_un_op!(instance, $method, -2.7, $op(-2.7));
                test_float64_un_op!(instance, $method, -2.9, $op(-2.9));

                // test odd
                test_float64_un_op!(instance, $method, -1.0, $op(-1.0));
                test_float64_un_op!(instance, $method, -1.1, $op(-1.1));
                test_float64_un_op!(instance, $method, -1.3, $op(-1.3));
                test_float64_un_op!(instance, $method, -1.5, $op(-1.5));
                test_float64_un_op!(instance, $method, -1.7, $op(-1.7));
                test_float64_un_op!(instance, $method, -1.9, $op(-1.9));
            }

            Ok(())
        }
        _test()?
    };
}

macro_rules! test_float64_bin_op {
    ($instance: ident, $method: expr, $op: tt, $_lhs: expr, $_rhs: expr) => (
        let lhs_f: f64 = ($_lhs);
        let rhs_f: f64 = ($_rhs);
        let out_f: f64 = lhs_f $op rhs_f;
        let lhs = Value::F64(lhs_f);
        let rhs = Value::F64(rhs_f);

        let foo = $instance.exports.get_function($method)?;
        println!("Calling {} {:?} + {:?} function...", $method, lhs, rhs);
        let results = foo.call(&[lhs, rhs])?;

        match results.to_vec()[0] {
            Value::F64(f) => {
                println!("{} == {}", f, out_f);
                assert_eq!(f, out_f);
            }
            _ => println!("wrong!"),
        }
        assert_eq!(results.to_vec(), vec![Value::F64(out_f)]);
    );
    ($instance: ident, $method: expr, $_lhs: expr, $_rhs: expr; $_out: expr) => (
        let lhs_f: f64 = ($_lhs);
        let rhs_f: f64 = ($_rhs);
        let out_f = ($_out);
        let out_f = &out_f as &dyn Any;
        let lhs = Value::F64(lhs_f);
        let rhs = Value::F64(rhs_f);

        let foo = $instance.exports.get_function($method)?;
        println!("Calling {} {:?} + {:?} function...", $method, lhs, rhs);
        let results = foo.call(&[lhs, rhs])?;

        if let Some(out_f) = out_f.downcast_ref::<f64>() {
            let out_f = *out_f;
            match results.to_vec()[0] {
                Value::F64(f) => {
                    println!("float {} == {}", f, out_f);
                    assert_eq!(f, out_f);
                }
                _ => println!("wrong!"),
            }
            assert_eq!(results.to_vec(), vec![Value::F64(out_f)]);
        } else if let Some(out_b) = out_f.downcast_ref::<i32>() {
            let out_b = *out_b;
            match results.to_vec()[0] {
                Value::I32(b) => {
                    println!("bool {} == {}", b, out_b);
                    assert_eq!(b, out_b);
                }
                _ => println!("wrong!"),
            }
            assert_eq!(results.to_vec(), vec![Value::I32(out_b)]);
        } else {
            println!("wrong out type!");
            panic!("wrong out type!");
        }
    )
}

macro_rules! test_float64_un_op {
    ($instance: ident, $method: expr, $_src: expr, $_out: expr) => {
        let src_f: f64 = ($_src);
        let out_f = ($_out);
        let out_f = &out_f as &dyn Any;
        let src = Value::F64(src_f);

        let foo = $instance.exports.get_function($method)?;
        println!("Calling {} {:?} function...", $method, src);
        let results = foo.call(&[src])?;

        if let Some(out_f) = out_f.downcast_ref::<f64>() {
            let out_f = *out_f;
            match results.to_vec()[0] {
                Value::F64(f) => {
                    println!("float {} == {}", f, out_f);
                    assert_eq!(f, out_f);
                }
                _ => println!("wrong!"),
            }
            assert_eq!(results.to_vec(), vec![Value::F64(out_f)]);
        } else if let Some(out_b) = out_f.downcast_ref::<i32>() {
            let out_b = *out_b;
            match results.to_vec()[0] {
                Value::I32(b) => {
                    println!("bool {} == {}", b, out_b);
                    assert_eq!(b, out_b);
                }
                _ => println!("wrong!"),
            }
            assert_eq!(results.to_vec(), vec![Value::I32(out_b)]);
        } else {
            println!("wrong out type!");
            panic!("wrong out type!");
        }
    };
}

macro_rules! test_float64_demote_op {
    ($instance: ident, $method: expr, $_src: expr, $_out: expr) => {
        let src_f: f64 = ($_src);
        let out_f: f32 = ($_out);
        let out_f = &out_f as &dyn Any;
        let src = Value::F64(src_f);

        let foo = $instance.exports.get_function($method)?;
        println!("Calling {} {:?} function...", $method, src);
        let results = foo.call(&[src])?;

        if let Some(out_f) = out_f.downcast_ref::<f32>() {
            let out_f = *out_f;
            match results.to_vec()[0] {
                Value::F32(f) => {
                    println!("float {} == {}", f, out_f);
                    assert_eq!(f, out_f);
                }
                _ => println!("wrong!"),
            }
            assert_eq!(results.to_vec(), vec![Value::F32(out_f)]);
        } else {
            println!("wrong out type!");
            panic!("wrong out type!");
        }
    };
}

fn _test_demote() -> Result<(), Box<dyn std::error::Error>> {
    println!("Running {} test...", "promote");
    let wat = r#"
        (module
          (type $demote_t (func (param f64) (result f32)))
          (func $demote_f (type $demote_t) (param $x f64) (result f32)
            local.get $x
            f32.demote_f64)
          (export "de64" (func $demote_f)))
        "#;
    // Let's declare the Wasm module with the text representation.
    let wasm_bytes = wat2wasm(wat.as_bytes())?;

    // Use Singlepass compiler with the default settings
    let compiler = Singlepass::default();

    // Create the store
    let store = Store::new(&Universal::new(compiler).engine());

    println!("Compiling module...");
    // Let's compile the Wasm module.
    let module = Module::new(&store, wasm_bytes)?;

    // Create an empty import object.
    let import_object = imports! {};

    println!("Instantiating module...");
    // Let's instantiate the Wasm module.
    let instance = Instance::new(&module, &import_object)?;

    // test even
    test_float64_demote_op!(instance, "de64", 2.0, _demote_f64(2.0));
    test_float64_demote_op!(instance, "de64", 2.1, _demote_f64(2.1));
    test_float64_demote_op!(instance, "de64", 2.3, _demote_f64(2.3));
    test_float64_demote_op!(instance, "de64", 2.5, _demote_f64(2.5));
    test_float64_demote_op!(instance, "de64", 2.7, _demote_f64(2.7));
    test_float64_demote_op!(instance, "de64", 2.9, _demote_f64(2.9));

    // test odd
    test_float64_demote_op!(instance, "de64", 1.0, _demote_f64(1.0));
    test_float64_demote_op!(instance, "de64", 1.1, _demote_f64(1.1));
    test_float64_demote_op!(instance, "de64", 1.3, _demote_f64(1.3));
    test_float64_demote_op!(instance, "de64", 1.5, _demote_f64(1.5));
    test_float64_demote_op!(instance, "de64", 1.7, _demote_f64(1.7));
    test_float64_demote_op!(instance, "de64", 1.9, _demote_f64(1.9));

    // test even
    test_float64_demote_op!(instance, "de64", -2.0, _demote_f64(-2.0));
    test_float64_demote_op!(instance, "de64", -2.1, _demote_f64(-2.1));
    test_float64_demote_op!(instance, "de64", -2.3, _demote_f64(-2.3));
    test_float64_demote_op!(instance, "de64", -2.5, _demote_f64(-2.5));
    test_float64_demote_op!(instance, "de64", -2.7, _demote_f64(-2.7));
    test_float64_demote_op!(instance, "de64", -2.9, _demote_f64(-2.9));

    // test odd
    test_float64_demote_op!(instance, "de64", -1.0, _demote_f64(-1.0));
    test_float64_demote_op!(instance, "de64", -1.1, _demote_f64(-1.1));
    test_float64_demote_op!(instance, "de64", -1.3, _demote_f64(-1.3));
    test_float64_demote_op!(instance, "de64", -1.5, _demote_f64(-1.5));
    test_float64_demote_op!(instance, "de64", -1.7, _demote_f64(-1.7));
    test_float64_demote_op!(instance, "de64", -1.9, _demote_f64(-1.9));

    Ok(())
}

fn _max_f64(lhs: f64, rhs: f64) -> f64 {
    unsafe {
        let lhs = _mm_set1_pd(lhs);
        let rhs = _mm_set1_pd(rhs);
        let ret = _mm_max_pd(lhs, rhs);
        _mm_cvtsd_f64(ret)
    }
}

fn _min_f64(lhs: f64, rhs: f64) -> f64 {
    unsafe {
        let lhs = _mm_set1_pd(lhs);
        let rhs = _mm_set1_pd(rhs);
        let ret = _mm_min_pd(lhs, rhs);
        _mm_cvtsd_f64(ret)
    }
}

fn _eq_f64(lhs: f64, rhs: f64) -> i32 {
    unsafe {
        let lhs = _mm_set1_pd(lhs);
        let rhs = _mm_set1_pd(rhs);
        let ret = _mm_cmp_pd(lhs, rhs, _CMP_EQ_UQ);
        let ret = _mm_castpd_si128(ret);
        let ret = _mm_extract_epi64(ret, 0);
        ret as i32 & 1
    }
}

fn _ne_f64(lhs: f64, rhs: f64) -> i32 {
    unsafe {
        let lhs = _mm_set1_pd(lhs);
        let rhs = _mm_set1_pd(rhs);
        let ret = _mm_cmp_pd(lhs, rhs, _CMP_NEQ_UQ);
        let ret = _mm_castpd_si128(ret);
        let ret = _mm_extract_epi64(ret, 0);
        ret as i32 & 1
    }
}

fn _lt_f64(lhs: f64, rhs: f64) -> i32 {
    unsafe {
        let lhs = _mm_set1_pd(lhs);
        let rhs = _mm_set1_pd(rhs);
        let ret = _mm_cmp_pd(lhs, rhs, _CMP_LT_OQ);
        let ret = _mm_castpd_si128(ret);
        let ret = _mm_extract_epi64(ret, 0);
        ret as i32 & 1
    }
}

fn _le_f64(lhs: f64, rhs: f64) -> i32 {
    unsafe {
        let lhs = _mm_set1_pd(lhs);
        let rhs = _mm_set1_pd(rhs);
        let ret = _mm_cmp_pd(lhs, rhs, _CMP_LE_OQ);
        let ret = _mm_castpd_si128(ret);
        let ret = _mm_extract_epi64(ret, 0);
        ret as i32 & 1
    }
}

fn _gt_f64(lhs: f64, rhs: f64) -> i32 {
    unsafe {
        let lhs = _mm_set1_pd(lhs);
        let rhs = _mm_set1_pd(rhs);
        let ret = _mm_cmp_pd(lhs, rhs, _CMP_GT_OQ);
        let ret = _mm_castpd_si128(ret);
        let ret = _mm_extract_epi64(ret, 0);
        ret as i32 & 1
    }
}

fn _ge_f64(lhs: f64, rhs: f64) -> i32 {
    unsafe {
        let lhs = _mm_set1_pd(lhs);
        let rhs = _mm_set1_pd(rhs);
        let ret = _mm_cmp_pd(lhs, rhs, _CMP_GE_OQ);
        let ret = _mm_castpd_si128(ret);
        let ret = _mm_extract_epi64(ret, 0);
        ret as i32 & 1
    }
}

fn _nearest_f64(src: f64) -> f64 {
    unsafe {
        let input = _mm_set1_pd(src);
        let ret = _mm_round_pd(input, _MM_FROUND_TO_NEAREST_INT);
        _mm_cvtsd_f64(ret)
    }
}
fn _floor_f64(src: f64) -> f64 {
    unsafe {
        let input = _mm_set1_pd(src);
        let ret = _mm_round_pd(input, _MM_FROUND_TO_NEG_INF);
        _mm_cvtsd_f64(ret)
    }
}
fn _ceil_f64(src: f64) -> f64 {
    unsafe {
        let input = _mm_set1_pd(src);
        let ret = _mm_round_pd(input, _MM_FROUND_TO_POS_INF);
        _mm_cvtsd_f64(ret)
    }
}
fn _trunc_f64(src: f64) -> f64 {
    unsafe {
        let input = _mm_set1_pd(src);
        let ret = _mm_round_pd(input, _MM_FROUND_TO_ZERO);
        _mm_cvtsd_f64(ret)
    }
}
fn _sqrt_f64(src: f64) -> f64 {
    unsafe {
        let input = _mm_set1_pd(src);
        let ret = _mm_sqrt_pd(input);
        _mm_cvtsd_f64(ret)
    }
}

fn _demote_f64(src: f64) -> f32 {
    unsafe {
        let input = _mm_set1_pd(src);
        let ret = _mm_cvtpd_ps(input);
        let ret = f32::from_bits(_mm_extract_ps(ret, 0) as u32);
        ret
    }
}

#[cfg(test)]
#[cfg(all(feature = "singlepass", feature = "softfloat"))]
mod softfloat_binop_tests {
    use super::*;
    #[test]
    fn test_compiler_singlepass_softfloat_add() -> Result<(), Box<dyn std::error::Error>> {
        test_binop!("add", +);
        Ok(())
    }

    #[test]
    fn test_compiler_singlepass_softfloat_sub() -> Result<(), Box<dyn std::error::Error>> {
        test_binop!("sub", -);
        Ok(())
    }

    #[test]
    fn test_compiler_singlepass_softfloat_mul() -> Result<(), Box<dyn std::error::Error>> {
        test_binop!("mul", *);
        Ok(())
    }

    #[test]
    fn test_compiler_singlepass_softfloat_div() -> Result<(), Box<dyn std::error::Error>> {
        test_binop!("div", /);
        Ok(())
    }

    #[test]
    fn test_compiler_singlepass_softfloat_max() -> Result<(), Box<dyn std::error::Error>> {
        test_binop!("max"; _max_f64);
        Ok(())
    }

    #[test]
    fn test_compiler_singlepass_softfloat_min() -> Result<(), Box<dyn std::error::Error>> {
        test_binop!("min"; _min_f64);
        Ok(())
    }
}

#[cfg(test)]
#[cfg(all(feature = "singlepass", feature = "softfloat"))]
mod softfloat_cmpop_tests {
    use super::*;

    #[test]
    fn test_compiler_singlepass_softfloat_eq() -> Result<(), Box<dyn std::error::Error>> {
        test_cmpop!("eq"; _eq_f64);
        Ok(())
    }
    #[test]
    fn test_compiler_singlepass_softfloat_ne() -> Result<(), Box<dyn std::error::Error>> {
        test_cmpop!("ne"; _ne_f64);
        Ok(())
    }
    #[test]
    fn test_compiler_singlepass_softfloat_lt() -> Result<(), Box<dyn std::error::Error>> {
        test_cmpop!("lt"; _lt_f64);
        Ok(())
    }
    #[test]
    fn test_compiler_singlepass_softfloat_le() -> Result<(), Box<dyn std::error::Error>> {
        test_cmpop!("le"; _le_f64);
        Ok(())
    }
    #[test]
    fn test_compiler_singlepass_softfloat_gt() -> Result<(), Box<dyn std::error::Error>> {
        test_cmpop!("gt"; _gt_f64);
        Ok(())
    }
    #[test]
    fn test_compiler_singlepass_softfloat_ge() -> Result<(), Box<dyn std::error::Error>> {
        test_cmpop!("ge"; _ge_f64);
        Ok(())
    }
}

#[cfg(test)]
#[cfg(all(feature = "singlepass", feature = "softfloat"))]
mod softfloat_unop_tests {
    use super::*;
    #[test]
    fn test_compiler_singlepass_softfloat_nearest() -> Result<(), Box<dyn std::error::Error>> {
        test_unop!("nearest", _nearest_f64, true);
        Ok(())
    }
    #[test]
    fn test_compiler_singlepass_softfloat_floor() -> Result<(), Box<dyn std::error::Error>> {
        test_unop!("floor", _floor_f64, true);
        Ok(())
    }
    #[test]
    fn test_compiler_singlepass_softfloat_ceil() -> Result<(), Box<dyn std::error::Error>> {
        test_unop!("ceil", _ceil_f64, true);
        Ok(())
    }
    #[test]
    fn test_compiler_singlepass_softfloat_trunc() -> Result<(), Box<dyn std::error::Error>> {
        test_unop!("trunc", _trunc_f64, true);
        Ok(())
    }
    #[test]
    fn test_compiler_singlepass_softfloat_sqrt() -> Result<(), Box<dyn std::error::Error>> {
        test_unop!("sqrt", _sqrt_f64, false);
        Ok(())
    }
    #[test]
    fn test_compiler_singlepass_softfloat_demote() -> Result<(), Box<dyn std::error::Error>> {
        _test_demote()
    }
}