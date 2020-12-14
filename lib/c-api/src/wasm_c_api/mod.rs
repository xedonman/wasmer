//! Implementation of the [official WebAssembly C
//! API](https://github.com/WebAssembly/wasm-c-api) for Wasmer.
//!
//! We would like to remind the reader that this official standard can
//! be characterized as a _living standard_. As such, the API is not
//! yet stable, even though it shows maturity over time. The API is
//! described by the `wasm.h` C header, which is included by
//! `wasmer_wasm.h` C header file (which contains extension of the
//! standard API, for example to provide WASI or vendor-specific
//! features).
//!
//! # Quick Guide
//!
//! Usually, the user first needs to create an [`engine`] and a
//! [`store`]. Once it's done, the user needs to create a [`module`]
//! and then [instantiate][instance] it. When instantiating the
//! module, the user is able to pass a set of
//! [imports][externals]. With an instance, the user is able to call
//! the [exports][instance::wasm_instance_exports].
//!
//! Every module comes with examples and entry points to guide the
//! discovery of this API.

#[macro_use]
pub mod macros;

/// The engine drives the compilation and the runtime.
///
/// Entry points: A default engine is created with
/// [`wasm_engine_new`][engine::wasm_engine_new] and freed with
/// [`wasm_engine_delete`][engine::wasm_engine_delete].
///
/// # Example
///
/// The simplest way to get a default engine is the following:
///
/// ```rust
/// # use inline_c::assert_c;
/// # fn main() {
/// #    (assert_c! {
/// # #include "tests/wasmer_wasm.h"
/// #
/// int main() {
///     // Create the engine.
///     wasm_engine_t* engine = wasm_engine_new();
///
///     // Check we have a valid engine!
///     assert(engine);
///
///     // Free everything.
///     wasm_engine_delete(engine);
///
///     return 0;
/// }
/// #    })
/// #    .success();
/// # }
/// ```
///
/// To configure the engine, see the [`wasm_config_new`][engine::wasm_config_new].
pub mod engine;

/// cbindgen:ignore
pub mod externals;

/// A WebAssembly instance is a stateful, executable instance of a
/// WebAssembly module.
///
/// Instance objects contain all the exported WebAssembly functions,
/// memories, tables and globals that allow interacting with
/// WebAssembly.
///
/// Entry points: A WebAssembly instance is created with
/// [`wasm_instance_new`][instance::wasm_instance_new] and freed with
/// [`wasm_instance_delete`][instance::wasm_instance_delete].
///
/// # Example
///
/// The simplest way to instantiate a Wasm module is the following:
///
/// ```rust
/// # use inline_c::assert_c;
/// # fn main() {
/// #    (assert_c! {
/// # #include "tests/wasmer_wasm.h"
/// #
/// int main() {
///     // Create the engine and the store.
///     wasm_engine_t* engine = wasm_engine_new();
///     wasm_store_t* store = wasm_store_new(engine);
///
///     // Create a WebAssembly module from a WAT definition.
///     wasm_byte_vec_t wat;
///     wasmer_byte_vec_new_from_string(&wat, "(module)");
///     wasm_byte_vec_t wasm;
///     wat2wasm(&wat, &wasm);
///
///     // Create the module.
///     wasm_module_t* module = wasm_module_new(store, &wasm);
///     assert(module);
///
///     // Instantiate the module.
///     wasm_extern_vec_t imports = WASM_EMPTY_VEC;
///     wasm_trap_t* traps = NULL;
///
///     wasm_instance_t* instance = wasm_instance_new(store, module, &imports, &traps);
///     assert(instance);
///
///     // Now do something with the instance, like calling the
///     // exports with `wasm_instance_exports`.
///
///     // Free everything.
///     wasm_instance_delete(instance);
///     wasm_module_delete(module);
///     wasm_byte_vec_delete(&wasm);
///     wasm_byte_vec_delete(&wat);
///     wasm_store_delete(store);
///     wasm_engine_delete(engine);
///
///     return 0;
/// }
/// #    })
/// #    .success();
/// # }
/// ```
///
/// cbindgen:ignore
pub mod instance;

/// A WebAssembly module contains stateless WebAssembly code that has
/// already been compiled and can be instantiated multiple times.
///
/// Entry points: A WebAssembly module is created with
/// `wasm_module_new` and freed with `wasm_module_delete`.
///
/// cbindgen:ignore
pub mod module;

/// cbindgen:ignore
pub mod store;

/// cbindgen:ignore
pub mod trap;

/// cbindgen:ignore
pub mod types;

/// cbindgen:ignore
pub mod value;

pub mod version;

#[cfg(feature = "wasi")]
pub mod wasi;

pub mod wasmer;

#[cfg(feature = "wat")]
pub mod wat;
