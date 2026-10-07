# 🦀 "Classes" & Access Modifiers in Rust

> **Key idea:** Rust has **no `class` keyword** and **no inheritance**.
> A "class" is built from three separate pieces:
> **`struct`/`enum` (data) + `impl` (behavior) + `trait` (shared interface).**

---

## 1. Class vs Rust — Mapping

| OOP Concept (Java/C++/C#) | Rust Equivalent |
|---|---|
| `class` fields | `struct` fields |
| Methods | `impl` block |
| Constructor | Associated fn, by convention `new()` |
| `this` | `self` / `&self` / `&mut self` |
| Static method | Associated fn (no `self`) |
| Destructor | `Drop` trait |
| Interface / abstract class | `trait` |
| Inheritance | ❌ → **Composition** + **supertraits** |
| Virtual / polymorphism | Generics (static) or `dyn Trait` (dynamic) |
| `public` / `private` | `pub` / default private (module-based) |

---

## 2. Data — `struct`

```rust
pub struct BankAccount {
    pub owner: String,   // public field
    balance: f64,        // private field (default)
}
```

- Fields are **private by default**.
- No default field values → use `Default` trait or a constructor.

---

## 3. Behavior — `impl` Block

```rust
impl BankAccount {
    // Constructor (associated function — no `self`)
    pub fn new(owner: &str) -> Self {
        Self { owner: owner.to_string(), balance: 0.0 }
    }

    // Read-only method (shared borrow)
    pub fn balance(&self) -> f64 {
        self.balance
    }

    // Mutating method (exclusive borrow)
    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    // Consuming method (takes ownership)
    pub fn close(self) -> f64 {
        self.balance        // `self` is dropped after this
    }

    // Private helper
    fn validate(&self, amount: f64) -> bool {
        amount > 0.0
    }
}

fn main() {
    let mut acc = BankAccount::new("Arpit"); // static call with ::
    acc.deposit(500.0);                      // method call with .
    println!("{}", acc.balance());
    let final_amount = acc.close();          // acc can't be used anymore
}
```

### Receiver types (`self`)
| Receiver | Meaning | Caller can reuse object? |
|---|---|---|
| `&self` | Read-only borrow | ✅ |
| `&mut self` | Mutable borrow | ✅ |
| `self` | Takes ownership | ❌ (moved) |
| `mut self` | Takes ownership, can mutate | ❌ |
| *(none)* | Associated / static fn | — |

- `Self` = alias for the type the `impl` is for.
- Multiple `impl` blocks per type are allowed.

---

## 4. Interfaces — `trait`

```rust
pub trait Account {
    fn balance(&self) -> f64;            // required
    fn describe(&self) -> String {       // default method
        format!("Balance: {}", self.balance())
    }
}

impl Account for BankAccount {
    fn balance(&self) -> f64 { self.balance }
}
```

- Must be implemented explicitly (`impl Trait for Type`) — not duck-typed like Go.
- **Supertrait** ("trait inheritance" — only adds a requirement):
  ```rust
  trait Savings: Account { fn interest(&self) -> f64; }
  ```

---

## 5. No Inheritance → Composition

```rust
pub struct Address { city: String }

pub struct User {
    id: u32,
    address: Address,   // "has-a", not "is-a"
}
```
Why: avoids hidden overrides, implicit dynamic dispatch, and unclear data ownership.

---

## 6. Polymorphism

```rust
// Static dispatch (monomorphized, zero-cost)
fn print_it(a: &impl Account) { println!("{}", a.describe()); }

// Dynamic dispatch (vtable), allows mixed types
let accounts: Vec<Box<dyn Account>> = vec![Box::new(BankAccount::new("A"))];
```

---

## 7. Destructor — `Drop`

```rust
impl Drop for BankAccount {
    fn drop(&mut self) { println!("Account closed"); }
}
```
Runs automatically when the value goes out of scope (RAII).

---

## 8. Access Modifiers (Visibility)

> In Rust, **privacy is per module, not per type.**
> Everything is **private by default** — visible only in the current module and its child modules.

### The modifiers
| Modifier | Visible to |
|---|---|
| *(none)* | Current module + its descendants |
| `pub` | Everyone (any crate that can reach it) |
| `pub(crate)` | Whole current crate only |
| `pub(super)` | Parent module |
| `pub(self)` | Current module (same as private) |
| `pub(in path::to::mod)` | A specific ancestor module |

### Comparison with other languages
| Java / C++ | Rust (approx.) |
|---|---|
| `public` | `pub` |
| `private` | *(default)* — but scoped to **module**, not class |
| `protected` | ❌ none (no inheritance) — closest: `pub(super)` / `pub(crate)` |
| package-private / `internal` | `pub(crate)` |

### Example
```rust
mod bank {
    pub struct Account {
        pub owner: String,     // readable/writable outside
        balance: f64,          // hidden outside `bank`
    }

    impl Account {
        pub fn new(owner: &str) -> Self {
            Self { owner: owner.into(), balance: 0.0 }
        }
        pub fn balance(&self) -> f64 { self.balance }
        pub(crate) fn audit(&self) {}   // anywhere in this crate
        fn secret(&self) {}             // only inside `bank`
    }

    pub mod reports {
        use super::Account;
        pub fn show(a: &Account) {
            println!("{}", a.balance);  // ✅ child module sees parent's private items
        }
    }
}

fn main() {
    let a = bank::Account::new("Arpit");
    println!("{}", a.owner);        // ✅ pub field
    // println!("{}", a.balance);   // ❌ private field
    // let b = bank::Account { owner: "x".into(), balance: 9.0 }; // ❌ can't build: private field
    // a.secret();                  // ❌ private method
    a.audit();                      // ✅ pub(crate)
}
```

### Rules to remember
1. **Private by default** — items, fields, methods.
2. **Parent and sibling items are always visible** to child modules.
3. A `pub struct` can still have **private fields** → outsiders **cannot construct it** directly; they must use `new()`. This is how invariants are enforced (encapsulation).
4. **Enum variants are always public** if the enum is `pub`.
5. `pub` on a method inside a private type is limited by the type's visibility.
6. Traits must be **in scope** (`use`) to call their methods.
7. Re-export with `pub use` to give a shorter public path.

---

## 9. Encapsulation Pattern (Getter + Validation)

```rust
mod user {
    pub struct Username(String);           // private inner field

    impl Username {
        pub fn new(s: String) -> Result<Self, String> {
            if s.is_empty() { return Err("empty".into()); }
            Ok(Self(s))
        }
        pub fn as_str(&self) -> &str { &self.0 } // read-only access
    }
}
```
Every `Username` is guaranteed valid because the only way to create one is `new()`.

---

## 10. Summary

- **Class = `struct` + `impl` + `trait`.**
- **Constructor = `new()`**, destructor = `Drop`.
- **No inheritance** — use composition and traits.
- **Visibility is module-based**, private by default; use `pub`, `pub(crate)`, `pub(super)`, `pub(in path)` to open it up.
