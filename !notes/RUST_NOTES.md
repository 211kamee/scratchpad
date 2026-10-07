# 🦀 Comprehensive Rust — Study Notes
> Source: *Comprehensive Rust* (Google Android team, Martin Geisler) — PDF.
> Structure: **Rust Fundamentals (Days 1–4)** + Deep Dives (Android, Chromium, Bare-Metal, Concurrency, Idiomatic Rust, Unsafe).

---

## 📑 Contents
0. [Setup & Cargo](#0-setup--cargo)
1. [Day 1 AM — Basics, Types, Control Flow](#day-1-am)
2. [Day 1 PM — Tuples, Arrays, References, User Types](#day-1-pm)
3. [Day 2 AM — Pattern Matching, Methods/Traits, Generics](#day-2-am)
4. [Day 2 PM — Closures, Std Types, Std Traits](#day-2-pm)
5. [Day 3 AM — Memory Management, Smart Pointers](#day-3-am)
6. [Day 3 PM — Borrowing, Lifetimes](#day-3-pm)
7. [Day 4 AM — Iterators, Modules, Testing](#day-4-am)
8. [Day 4 PM — Error Handling, Unsafe Rust](#day-4-pm)
9. [Deep Dive: Concurrency](#deep-dive-concurrency)
10. [Deep Dive: Bare-Metal](#deep-dive-bare-metal)
11. [Deep Dive: Android & Chromium](#deep-dive-android--chromium)
12. [Deep Dive: Idiomatic Rust](#deep-dive-idiomatic-rust)
13. [Deep Dive: Unsafe](#deep-dive-unsafe)
14. [Cheat Sheet](#cheat-sheet)

---

## 0. Setup & Cargo
- Install via **rustup** → gives `rustc` (compiler), `cargo` (build + deps + test runner), `rustup` (toolchain manager).
- Editor support: **rust-analyzer** (VS Code, Vim, Emacs) or RustRover.
- Release every **6 weeks**; channels: `stable`, `beta`, `nightly`.
- **Editions**: 2015, 2018, 2021, **2024 (current)** — opt-in per crate in `Cargo.toml`; crates of different editions interoperate.

```bash
cargo new exercise     # new binary project
cargo run              # build + run (debug)
cargo check            # fast type check
cargo build --release  # optimized → target/release/
cargo test / clippy / fmt
```

---

<a id="day-1-am"></a>
## 1. Day 1 AM — Basics, Types, Control Flow

### What is Rust?
- Statically compiled (LLVM backend), 1.0 in 2015. Runs on x86, ARM, WASM; firmware → servers.
- **No runtime, no GC**, C++-level control.

### Benefits
| Category | Details |
|---|---|
| Compile-time memory safety | No uninit vars, double-free, use-after-free, NULL, forgotten locked mutexes, data races, iterator invalidation |
| No undefined runtime behavior | Bounds-checked arrays; integer overflow defined (panic/wrap) |
| Modern features | Enums + pattern matching, generics, zero-overhead FFI, great errors, built-in deps & testing |

### Hello World
```rust
fn main() {
    println!("Hello 🌍!"); // `!` = macro (variable args; no overloading in Rust)
}
```

### Variables & Values
```rust
let x: i32 = 10;   // immutable by default
let mut y = 5;     // mutable
```
| Type | Examples |
|---|---|
| Signed `i8..i128, isize` | `-10`, `1_000`, `123_i64` |
| Unsigned `u8..u128, usize` | `0`, `10_u16` |
| Float `f32, f64` | `3.14`, `2_f32` |
| `char` (32-bit Unicode) | `'a'`, `'∞'` |
| `bool` (8-bit) | `true` |
- `isize/usize` = pointer width.
- **Overflow**: panics in debug, wraps in release. Explicit: `wrapping_*`, `saturating_*`, `checked_*`.
- **Type inference**: unconstrained int → `i32`, float → `f64`. Not "any type" — same machine code.

### Control Flow
- **Blocks are expressions**; last expr (no `;`) = value. With `;` → `()`.
```rust
let size = if x < 20 { "small" } else { "large" };

match val { 1 => .., 10 => .., _ => .. }  // exhaustive, no fall-through

while x >= 10 { x /= 2; }
for x in 1..5 {}      // 1..=5 inclusive
let r = loop { break 42; }; // only `loop` returns non-trivial value

'outer: for i in 0..3 { for j in 0..3 { break 'outer; } }
```

### Functions & Macros
```rust
fn gcd(a: u32, b: u32) -> u32 { if b > 0 { gcd(b, a % b) } else { a } }
```
- No overloading, no default args. Unit return `()` inferred.
- Macros: `println!`, `format!`, `dbg!` (prints & returns), `todo!`, `assert!/assert_eq!/assert_ne!`, `unreachable!`, `eprintln!`.

**Exercises:** Fibonacci (u32 overflows ~n=48 → panic in debug), Collatz length.

---

<a id="day-1-pm"></a>
## 2. Day 1 PM — Tuples, Arrays, References, User Types

### Arrays & Tuples
```rust
let mut a: [i8; 5] = [5, 4, 3, 2, 1]; // length is part of type; on stack
let z = [0; 1024];
let t: (i8, bool) = (7, true); t.0;  // () = unit type
let (l, m, r) = (1, 5, 3);           // irrefutable destructuring
```
- `{:?}` debug print, `{:#?}` pretty print.
- `for` works on arrays (via `IntoIterator`), not tuples.

### References
```rust
let r: &char = &a;          // shared (read-only), never null
let x = &mut point.0;       // exclusive (read-write)
*x = 20;
```
- Auto-deref on method calls (no `->`).
- `let mut r: &T` (rebindable) ≠ `let r: &mut T` (mutate target).

### Slices & Strings
```rust
let s: &[i32] = &a[2..4];   // also &a[..], &a[2..]
let s1: &str = "World";      // borrowed UTF-8 slice (like string_view)
let mut s2 = String::from("Hello "); // owned buffer (like Vec<u8>)
s2.push_str(s1);
```
- Byte strings `b"abc"`, raw strings `r#"..."#`.
- Slicing on non-char boundary → panic.
- **References can't outlive data** (dangling refs impossible).

### User-Defined Types
```rust
struct Person { name: String, age: u8 }   // named struct
let p = Person { name, age };             // shorthand
let j = Person { name: "J".into(), ..p }; // struct update

struct Point(i32, i32);                   // tuple struct
struct Newtons(f64);                      // newtype (units/validation)

enum PlayerMove { Pass, Run(Direction), Teleport { x: u32, y: u32 } }

type Item = CarryableConcreteItem;        // alias (prefer newtype)
const SIZE: usize = 3;                    // inlined; const fn usable
static BANNER: &str = "Hi";               // fixed address, must be Sync
```
- Niche optimization: `Option<&T>` same size as `&T`.
- `#[repr(u32)]` to control discriminant.

---

<a id="day-2-am"></a>
## 3. Day 2 AM — Pattern Matching, Methods, Traits, Generics

### Patterns
```rust
let (_, b, c) = tuple;  let (.., last) = tuple;  let [first, .., last] = arr;

match input {
    'q' => ..,
    'a' | 's' => ..,
    '0'..='9' => ..,
    key if key.is_lowercase() => .., // match guard
    _ => ..,
}
match opt { outer @ Some(inner) => .., None => .. }
match m { Move { delta: (x, 0), repeat } => .., _ => .. }
```
- A bare variable name in a pattern **binds** (doesn't compare) — use a `const`.

### Let control flow
```rust
if let Ok(d) = result { .. }
while let Some(c) = name.pop() { .. }
let Some(s) = maybe else { return Err(..); }; // else must diverge
```

### Methods
```rust
impl CarRace {
    fn new(name: &str) -> Self { .. }     // associated fn (no receiver)
    fn add_lap(&mut self, lap: i32) {}    // exclusive borrow
    fn print_laps(&self) {}               // shared borrow
    fn finish(self) {}                    // takes ownership
}
```

### Traits
```rust
trait Pet {
    fn talk(&self) -> String;
    fn greet(&self) { println!("{}", self.talk()); } // default
}
impl Pet for Dog { fn talk(&self) -> String { .. } }
trait Pet2: Animal {}                 // supertrait
trait Multiply { type Output; ... }   // associated type (chosen by implementer)
#[derive(Debug, Clone, Default)]      // derive macros
```
- Unlike Go: matching methods isn't enough; needs explicit `impl`.

### Generics
```rust
fn pick<T>(c: bool, l: T, r: T) -> T { if c { l } else { r } }
fn dup<T: Clone>(a: T) -> (T, T) { (a.clone(), a.clone()) }
fn dup2<T>(a: T) -> (T, T) where T: Clone { .. }
struct VerbosityFilter<L> { inner: L }
impl<L: Logger> Logger for VerbosityFilter<L> { .. }
impl From<u32> for Foo { .. }         // generic trait
fn add(x: impl Into<i32>) -> i32 { .. }
fn pair() -> impl Debug { .. }
fn dynamic(pet: &dyn Pet) { .. }      // vtable, fat pointer
```
- **Monomorphization** = zero-cost; generics checked once for all types (unlike C++ templates).
- No specialization (yet). `dyn Trait` needs indirection (`&`, `Box`).

---

<a id="day-2-pm"></a>
## 4. Day 2 PM — Closures, Std Types, Std Traits

### Closures
```rust
let double = |n| n * 2;
let add = |x: f32| -> f32 { x + 1.0 };
let f = move |v| ..;  // capture by value
```
- Capture by least-demanding: `&` → `&mut` → move.
| Trait | Meaning |
|---|---|
| `Fn` | Shared ref; callable repeatedly & concurrently |
| `FnMut` | May mutate; repeatedly, not concurrently |
| `FnOnce` | Consumes; callable once |
- Accept `FnOnce` if possible (most flexible for caller). Store closures in struct via generic `P: Fn(..)`; call `(self.p)(..)`.

### Standard Library
- Layers: `core` (no OS/alloc) ⊂ `alloc` (Vec, Box, Arc) ⊂ `std`.
- Docs: `rustup doc --std`, https://std.rs, docs.rs. `///` doc comments (Markdown), `//!` inner docs.

| Type | Notes |
|---|---|
| `Option<T>` | `unwrap`, `expect`; niche-optimized |
| `Result<T,E>` | Forced error handling |
| `String` | `len()` = bytes; `chars()`; `Deref<Target=str>` |
| `Vec<T>` | `vec![]`, `push`, `get`→Option, `retain`, `dedup`; `Deref<Target=[T]>` |
| `HashMap<K,V>` | Not in prelude; `entry().or_insert()`, `HashMap::from([..])` |

### Std Traits
| Trait | Use |
|---|---|
| `PartialEq`/`Eq` | `==`; Eq = reflexive |
| `PartialOrd`/`Ord` | `<`; Ord total (`cmp`) |
| `std::ops::Add`… | Operator overloading (`type Output`) |
| `From`/`Into` | Infallible conversion; impl `From`, bound on `Into` |
| `as` | Explicit casts, always defined (truncation) |
| `TryFrom/TryInto` | Fallible conversions |
| `Read`/`BufRead`/`Write` | Abstract over byte sources/sinks |
| `Default` | `..Default::default()` struct update |

---

<a id="day-3-am"></a>
## 5. Day 3 AM — Memory Management, Smart Pointers

### Memory
- **Stack**: fixed size, fast. **Heap**: dynamic size. `String` = (ptr, len, cap) on stack + bytes on heap.
- C: manual; Java/Go/Python: GC; **Rust: ownership checked at compile time**.

### Ownership & Moves
```rust
let s1 = String::from("Hi");
let s2 = s1;          // move — s1 unusable
let s3 = s2.clone();  // explicit deep copy
```
- One owner; dropped at scope end. Passing to fn moves.
- Opposite of C++ default (copy).
- **Copy types** (ints etc.) are bitwise-copied; `#[derive(Copy, Clone)]`. Copy can't be on types with `Drop`.
- Shared refs are Copy; `&mut` is not.

### Drop
```rust
impl Drop for Droppable { fn drop(&mut self) { .. } }
drop(a); // std::mem::drop — take ownership, drop early
```

**Exercise:** Builder pattern (`fn version(mut self, ..) -> Self`).

### Smart Pointers
| Type | Use |
|---|---|
| `Box<T>` | Heap, single owner, never null; recursive types, large data |
| `Rc<T>` | Shared ownership (single thread); `Rc::clone` is cheap; `strong_count`, `make_mut` |
| `Weak<T>` | Break cycles (`Rc::downgrade`) |
| `Box<dyn Trait>` | Owned trait object (fat pointer: data + vtable) |

```rust
enum List<T> { Element(T, Box<List<T>>), Nil }
let pets: Vec<Box<dyn Pet>> = vec![Box::new(Cat{..}), Box::new(Dog{..})];
```
**Exercise:** Binary tree with `Subtree<T>(Option<Box<Node<T>>>)`.

---

<a id="day-3-pm"></a>
## 6. Day 3 PM — Borrowing, Lifetimes

### Borrow rules
1. References can't **outlive** the value.
2. **Aliasing rule**: many `&T` **XOR** one `&mut T`.
- **Non-lexical lifetimes**: borrow ends at last use.
- Prevents: mutating vec while referencing element, iterator invalidation, data races.

### Interior Mutability
```rust
let cell = Cell::new(5); cell.set(123);           // no refs to inner, no runtime check
let rc = RefCell::new(5); *rc.borrow_mut() = 1;   // runtime check, panics on conflict
```
- Also `OnceCell` / `OnceLock` (init on first use).

### Lifetimes
```rust
fn pick<'a>(c: bool, a: &'a i32, b: &'a i32) -> &'a i32 { .. }       // borrows both
fn find_nearest<'a>(points: &'a [Point], q: &Point) -> &'a Point {..} // borrows one
struct Highlight<'doc> { slice: &'doc str }
```
- Compiler checks **signatures only**, not bodies.
- **Elision rules**: (1) each arg gets a lifetime; (2) one input → all outputs; (3) `&self` lifetime → outputs.
- `'b: 'a` = 'b outlives 'a. `'static` = whole program.
- Prefer owned data in structs.

**Exercise:** Protobuf parsing with `&'a [u8]` slices (zero-copy).

---

<a id="day-4-am"></a>
## 7. Day 4 AM — Iterators, Modules, Testing

### Iterators
```rust
impl Iterator for SliceIter<'s> {
    type Item = &'s i32;
    fn next(&mut self) -> Option<Self::Item> { .. }
}
let r: i32 = (1..=10).filter(|x| x % 2 == 0).map(|x| x * x).sum();
for (i, c) in arr.iter().enumerate() {}
let v: Vec<_> = it.collect();  // or .collect::<Vec<_>>() turbofish
```
- **Lazy**; adapters chain; consumers (`sum`, `count`, `collect`).
- `FromIterator` powers `collect` (even `Result<Vec<_>,E>`).
- `IntoIterator` powers `for`: `[T;N]`→`T`, `&[T;N]`→`&T`, `&mut`→`&mut T`.

### Modules
```rust
mod foo { pub fn f() {} }   // private by default
mod garden;                 // garden.rs or garden/mod.rs
pub use storage::DiskStorage; // re-export
use crate::x; use super::y; use self::z;
```
- Crate root: `src/lib.rs` / `src/main.rs`.
- Visibility: `pub`, `pub(crate)`, `pub(super)`, `pub(in path)`.
- **Struct fields private by default** → encapsulation is module-based. Enum variants always public.
- Traits must be in scope to call their methods.

### Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn t() { assert_eq!(first_word("Hi there"), "Hi"); }
}
```
- Integration tests in `tests/` (public API only).
- **Doc tests**: code in `///` runs on `cargo test` (`#` hides lines).
- Lints: compiler + **Clippy**; `cargo fix`.

**Exercise:** Luhn algorithm (reject non-digits, require ≥2 digits).

---

<a id="day-4-pm"></a>
## 8. Day 4 PM — Error Handling, Unsafe Rust

### Panics
- For bugs/unrecoverable errors; unwinds stack. `panic::catch_unwind` (rare, e.g. servers). Use `Vec::get` to avoid panics.

### Result & `?`
```rust
fn read_username(path: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    fs::File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}
```
- `expr?` ≡ `match expr { Ok(v) => v, Err(e) => return Err(From::from(e)) }`.
- `main` may return `Result<(), E: Debug>`.
- `Option ↔ Result`: `ok_or`, `.ok()`; `map_err` for one-off conversions.

### Custom errors
- Convention: derive `Debug`, impl `Display` + `std::error::Error`, impl `From<Inner>`.
```rust
#[derive(Debug, thiserror::Error)]
enum ReadUsernameError {
    #[error("I/O error: {0}")] IoError(#[from] io::Error),
    #[error("Found no username in {0}")] EmptyUsername(String),
}
```
- `Box<dyn Error>` — any error (apps, not library APIs).
- **anyhow**: `anyhow::Result`, `.context()`, `.with_context()`, `bail!`, `downcast`.

### Unsafe Rust — 5 superpowers
1. Dereference raw pointers (`&raw mut x`, `*const/*mut T`)
2. Access/modify `static mut`
3. Access `union` fields
4. Call `unsafe` fns (incl. `unsafe extern "C"`; can mark `safe fn`)
5. Implement `unsafe` traits (`Send`, `Sync`, zerocopy `IntoBytes`)

- Keep unsafe small; wrap in safe abstraction; every block gets `// SAFETY:` comment; unsafe fns get `# Safety` doc.
- 2024 edition: unsafe ops inside `unsafe fn` still need `unsafe {}` block.
- `slice::from_raw_parts` length = **elements**, not bytes.

**Exercise:** Safe FFI wrapper for `opendir/readdir/closedir` (CString → `*const c_char` → CStr → `&[u8]` → OsStr → OsString; `Drop` calls `closedir`).

---

## 9. Deep Dive: Concurrency

### Threads
```rust
let h = thread::spawn(move || 42);  let v = h.join().unwrap();
thread::scope(|s| { s.spawn(|| dbg!(s_ref.len())); }); // can borrow
```
- Main exiting ends program; panics are per-thread (payload via `Any::downcast_ref`).

### Channels (`std::sync::mpsc`)
- `channel()` unbounded (send never blocks); `sync_channel(n)` bounded (0 = rendezvous).
- `Sender` is `Clone`, `Receiver` not. `Err` = other side dropped.

### Send & Sync
- **Send**: safe to move `T` to another thread. **Sync**: `&T` is Send.
| | Examples |
|---|---|
| Send + Sync | primitives, `String`, `Vec`, `Arc`, `Mutex`, atomics |
| Send + !Sync | `Cell`, `RefCell`, `mpsc::Receiver` |
| !Send + Sync | `MutexGuard` |
| !Send + !Sync | `Rc`, raw pointers |

### Shared State
```rust
let v = Arc::new(Mutex::new(vec![10]));
let v2 = Arc::clone(&v);
thread::spawn(move || v2.lock().unwrap().push(1));
```
- Poisoned mutex after panic → `PoisonError::into_inner`. `RwLock` alternative.

**Exercises:** Dining philosophers (swap last chopsticks to avoid deadlock), multi-threaded link checker.

### Async
```rust
async fn count_to(n: i32) { .. }
#[tokio::main] async fn main() { count_to(10).await; tokio::spawn(task); }
```
- `async fn` → `impl Future`; **lazy** until polled. `Future::poll(Pin<&mut Self>, &mut Context) -> Poll<T>`.
- Compiled to a **state machine**; recursive async needs `Box::pin`.
- Runtimes: **Tokio**, smol. Tasks = lightweight threads.
- Async channels (`tokio::sync::mpsc`), `join_all` / `join!` (wait all), `select!` (first ready).

### Async Pitfalls
- **Blocking the executor**: use `tokio::time::sleep`, `spawn_blocking`; don't hold sync mutex across `.await`.
- **Pin**: self-referential futures can't move; `Box::pin`, `pin!`.
- **Async traits**: native since 1.75 but no `dyn`; `async_trait` crate (heap alloc).
- **Cancellation**: dropping a future at `.await` can lose state — keep buffers in struct (cancel-safe).

**Exercises:** Async dining philosophers, broadcast chat (tokio-websockets + `broadcast` channel).

---

## 10. Deep Dive: Bare-Metal
- `#![no_std]` + `#![no_main]` + `#[panic_handler]`. Use `core`; `alloc` needs a `#[global_allocator]` (e.g., `buddy_system_allocator`).
- **Microcontrollers** (micro:bit v2, nRF52833) abstraction ladder:
  Raw MMIO (`write_volatile`) → **PAC** (svd2rust) → **HAL** (`embedded-hal` traits) → **Board support crate**.
- **Type-state pattern**: pin config encoded in types; illegal transitions = compile errors.
- Tools: **probe-rs**, `cargo embed`, GDB, RTT. Frameworks: RTIC, Embassy, TockOS, Hubris.
- **Application processors** (QEMU aarch64 `virt`): entry asm (MMU, BSS, stack, vectors), inline `asm!` (PSCI via HVC), `read_volatile/write_volatile`, never hold refs to MMIO; `&raw` for field pointers.
- PL011 UART driver → `bitflags`, `#[repr(C)]` register struct → **safe-mmio** (`UniqueMmioPointer`, `field!`) → no `unsafe` in driver.
- `log` crate logger, exception vector, **aarch64-rt** (`entry!`, `initial_pagetable!`, `exception_handlers!`).
- Useful crates: `zerocopy`, `aarch64-paging`, `buddy_system_allocator`, `tinyvec`, `spin`.
- Android: `vmbase` for crosvm VMs. **Exercise:** PL031 RTC driver + GIC interrupt.

---

## 11. Deep Dive: Android & Chromium

### Android (Soong `Android.bp`)
- Rules: `rust_binary`, `rust_library`, `rust_ffi`, `rust_test`, `rust_bindgen`, `rust_protobuf`, `rust_fuzz`…
- **AIDL/Binder**: generated trait → impl service → `BnXxx::new_binder` → `add_service` → `join_thread_pool`; client via `get_interface`. Methods take `&self` (thread pool → use Mutex for state).
- AIDL types: `in` arrays → `&[T]`, out/inout → `&mut Vec<T>`, returns → `Vec<T>`; parcelables, `ParcelFileDescriptor`.
- Testing: `atest`, **GoogleTest** matchers, **Mockall** (`#[automock]`). Logging: `log` crate → logcat.
- Interop: C via `bindgen`; export with `#[unsafe(no_mangle)] pub extern "C"`; C++ via **CXX** (`#[cxx::bridge]`); Java via `jni` crate.

### Chromium
- Build with `gn`/`ninja`: `rust_static_library` (`allow_unsafe = true` if needed).
- Tests: `rust_gtest_interop` (`#[gtest]`, `expect_eq!`), `chromium::import!`.
- C++ interop via **CXX**; no exceptions → return bool/out-params; Rust as "leaf nodes".
- Third-party crates: `Cargo.toml` + `gnrt_config.toml` → `run_gnrt.py vendor/gen`; audit (cargo vet/audit, Rule of Two).

---

## 12. Deep Dive: Idiomatic Rust

### API Design
- Prioritize clarity at **call site**. Doc comments: summary → details → `# Examples / # Panics / # Errors / # Safety`. Document *what/why*, not *how*; avoid redundancy.
- **Naming conventions**:
| Pattern | Meaning |
|---|---|
| `new`, `with_capacity` | Constructors |
| `is_*` | bool check |
| `*_mut` | mutable access variant |
| `try_*` | fallible → Result |
| `from_*` / `into_*` / `to_*` | convert (construct / consume / borrow→owned) |
| `as_*`, `*_ref` | cheap reference view |
| `*_by`, `*_by_key`, `*_with` | custom comparator / projection / closure |
- Implement common traits: `Debug` (hide secrets), `Display`, `PartialEq/Eq`, `PartialOrd/Ord`, `Hash`, `Clone`, `Copy` (derive with Clone), `From`, `TryFrom`, serde.

### Leveraging the Type System
- **Newtype**: distinct types, enforce invariants at construction (beware `DerefMut` leaks).
- **RAII / Drop**: guards, `MutexGuard`, **drop bombs** (panic if not committed), `mem::forget`, scope guards (`scopeguard`), `Option`/`ManuallyDrop` to move out in drop. Drop may be skipped (`exit`, `forget`, abort).
- **Extension traits** (`StrExt`, blanket `impl<T: Display>`), method resolution conflicts.
- **Typestate**: consume `self` → return new state type; generics `Serializer<S>` for nested states.
- **Borrow checker for invariants**: single-use values (nonce), aliasing XOR mutability (transactions), `PhantomData` tags & lifetimes (`OwnedFd`/`BorrowedFd`).
- **Token types**: private constructor = proof (permission tokens, MutexGuard); **branding** with invariant lifetimes (`for<'a>` closures, `PhantomData<*mut &'id ()>`).

### Polymorphism
- Traits = static duck typing; blanket & conditional impls; **orphan rule**; `Sized`/`?Sized`; monomorphization cost.
- **No inheritance** → composition + supertraits. `dyn Trait` opt-in (dyn-compatible: no generics, assoc consts, `Self` returns). `Any` for downcasting.
- Prefer generics/enums first; **sealed traits** (private supertrait) or enums to limit extension.

---

## 13. Deep Dive: Unsafe
- Unsafe Rust = superset of Safe Rust; `unsafe` **shifts responsibility** compiler → programmer.
- Two roles: **creating** (`unsafe fn`, `unsafe trait`) vs **using** (`unsafe {}`, `unsafe impl`).
- **Safety preconditions**: validity, alignment, aliasing, initialization, provenance, lifetimes, bounds (even creating out-of-bounds ptr is UB).
- **Soundness**: sound fn can't trigger UB when preconditions are met. 3 sound shapes:
  1. pure safe fn, 2. safe fn fully encapsulating unsafe, 3. `unsafe fn` with documented preconditions.
- Avoid "crying wolf" (marking safe code unsafe).
- **MaybeUninit**: create → write → `assume_init`; arrays of `MaybeUninit<u8>`; doesn't drop inner `T`.
- **Pinning**: moves are memcpy; `Pin<Ptr>` prevents moves of `!Unpin` types (`PhantomPinned`); careful `Drop` (no `ptr::read`, use `ManuallyDrop`).
- **FFI**: C as lowest common denominator; Rust↔C↔C++ differences (errors, strings, nullability, ownership, panics, exceptions, relocatability); `safe fn` in `unsafe extern`; opaque types `struct X { _private: [u8; 0] }`.

---

## 14. Cheat Sheet
| Concept | Rule |
|---|---|
| Mutability | `let` immutable; `mut` opt-in |
| Ownership | One owner; assignment moves |
| Borrowing | many `&T` XOR one `&mut T` |
| Null | `Option<T>` |
| Errors | `Result<T,E>` + `?`; thiserror (lib) / anyhow (app) |
| Polymorphism | Generics (static) / `dyn Trait` (dynamic) |
| Heap | `Box`, `Vec`, `String` |
| Sharing | `Rc` (1 thread) / `Arc` (threads) |
| Interior mutability | `Cell`, `RefCell`, `Mutex`, `RwLock` |
| Cleanup | `Drop` (RAII) |
| Thread safety | `Send`, `Sync` |
| Async | `async/.await` + Tokio |
| Unsafe | Small, encapsulated, `// SAFETY:` comments |

---
*Further reading (from the course): The Rust Book, Rust by Example, Rustonomicon, Async Book, Embedded Book, Rust API Guidelines.*
