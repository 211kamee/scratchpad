# "Classes" & Access Modifiers in C

> **Key idea:** C has **no classes**, **no methods**, **no inheritance** and **no access modifier keywords** (`public`/`private`/`protected`).
> You build class-like behavior by convention:
> **`struct` (data) + functions that take a pointer to it (methods) + header/source split (visibility).**

---

## 1. Class vs C — Mapping

| OOP Concept (Java/C++) | C Equivalent |
|---|---|
| `class` fields | `struct` members |
| Methods | Functions taking `Type *self` as first argument |
| `this` | Explicit pointer parameter (e.g. `self`) |
| Constructor | `Type *type_new(...)` or `void type_init(Type *)` |
| Destructor | `void type_free(Type *)` — **must be called manually** |
| Static method | Plain function |
| Namespace | Name prefix (`account_deposit`) |
| Interface / virtual methods | Struct of **function pointers** (vtable) |
| Inheritance | Struct embedding (base struct as **first member**) |
| `public` | Declared in the **`.h` header** |
| `private` | **Opaque struct** + `static` functions in the **`.c` file** |
| `protected` | ❌ No equivalent (only by convention, e.g. a "private" header) |

---

## 2. Basic "Class": struct + functions

```c
#include <stdio.h>

typedef struct {
    char owner[32];
    double balance;
} BankAccount;

/* "Constructor" */
void account_init(BankAccount *self, const char *owner) {
    snprintf(self->owner, sizeof self->owner, "%s", owner);
    self->balance = 0.0;
}

/* "Methods" — the object pointer is passed explicitly */
void account_deposit(BankAccount *self, double amount) {
    self->balance += amount;
}

double account_balance(const BankAccount *self) {   /* const = read-only, like &self */
    return self->balance;
}

int main(void) {
    BankAccount acc;
    account_init(&acc, "Arpit");
    account_deposit(&acc, 500.0);
    printf("%.2f\n", account_balance(&acc));
    acc.balance = 1e9;   /* ⚠️ nothing stops this — all fields are public */
    return 0;
}
```

- `self` is just a convention; C has no `this`.
- `const BankAccount *` = read-only receiver; `BankAccount *` = mutating receiver.
- Struct members are **always accessible** wherever the struct definition is visible.

---

## 3. Access Modifiers in C (Visibility)

C controls visibility through **translation units** (`.c` files) and **headers** (`.h`), not keywords on members.

| Mechanism | Effect | OOP Analogy |
|---|---|---|
| Declared in `.h` | Callable from any file that `#include`s it | `public` |
| `static` function / global in `.c` | Visible **only in that `.c` file** (internal linkage) | `private` method / static field |
| **Opaque struct** (`typedef struct X X;` in `.h`, full definition in `.c`) | Callers can't see or touch members | `private` fields |
| `extern` declaration | Shares a global across files | `public static` field |
| `const` pointer parameter | Function promises not to modify | read-only access |
| Not in the `.h` (but non-`static`) | Still linkable if someone declares it ⚠️ | "package-private" by convention only |

### 3.1 `static` → file-private

```c
/* account.c */
static int validate(double amount) {   /* private helper: invisible to other .c files */
    return amount > 0;
}

static int account_count = 0;           /* private "class-level" variable */
```

> ⚠️ `static` has **two meanings**:
> - At **file scope** → internal linkage (private to the file).
> - **Inside a function** → variable keeps its value between calls.

---

## 4. Encapsulation: the Opaque Pointer Pattern

This is the standard way to get **truly private fields** in C.

### `account.h` — public interface
```c
#ifndef ACCOUNT_H
#define ACCOUNT_H

typedef struct BankAccount BankAccount;   /* incomplete type: fields hidden */

BankAccount *account_new(const char *owner);   /* constructor */
void         account_free(BankAccount *self);  /* destructor */

int    account_deposit(BankAccount *self, double amount);
double account_balance(const BankAccount *self);   /* getter */

#endif
```

### `account.c` — private implementation
```c
#include <stdlib.h>
#include <string.h>
#include "account.h"

struct BankAccount {          /* full definition only visible here */
    char   owner[32];
    double balance;
};

static int validate(double amount) { return amount > 0; }   /* private */

BankAccount *account_new(const char *owner) {
    BankAccount *self = malloc(sizeof *self);
    if (!self) return NULL;
    strncpy(self->owner, owner, sizeof self->owner - 1);
    self->owner[sizeof self->owner - 1] = '\0';
    self->balance = 0.0;
    return self;
}

void account_free(BankAccount *self) { free(self); }

int account_deposit(BankAccount *self, double amount) {
    if (!validate(amount)) return -1;   /* invariants enforced here */
    self->balance += amount;
    return 0;
}

double account_balance(const BankAccount *self) { return self->balance; }
```

### `main.c` — user code
```c
#include <stdio.h>
#include "account.h"

int main(void) {
    BankAccount *acc = account_new("Arpit");
    account_deposit(acc, 500.0);
    printf("%.2f\n", account_balance(acc));

    /* acc->balance = 1e9;          ❌ compile error: incomplete type */
    /* BankAccount local;            ❌ compile error: size unknown   */
    /* validate(10);                 ❌ link error: static in account.c */

    account_free(acc);               /* must free manually — no RAII */
    return 0;
}
```

**Trade-offs of opaque pointers**
- ✅ Real encapsulation; ABI stays stable when private fields change.
- ❌ Requires heap allocation (callers can't know `sizeof`).
- ❌ Manual `free` — forgetting it leaks, calling twice is undefined behavior.

---

## 5. Polymorphism: Function Pointers (manual vtable)

```c
#include <stdio.h>

typedef struct Shape Shape;

typedef struct {
    double (*area)(const Shape *self);
    void   (*describe)(const Shape *self);
} ShapeVTable;

struct Shape {                 /* "base class" */
    const ShapeVTable *vtable;
};

/* ---- "Derived class" Circle ---- */
typedef struct {
    Shape  base;               /* MUST be first member */
    double radius;
} Circle;

static double circle_area(const Shape *s) {
    const Circle *c = (const Circle *)s;   /* safe: base is first member */
    return 3.14159 * c->radius * c->radius;
}
static void circle_describe(const Shape *s) {
    printf("Circle, area %.2f\n", circle_area(s));
}
static const ShapeVTable CIRCLE_VTABLE = { circle_area, circle_describe };

void circle_init(Circle *c, double r) {
    c->base.vtable = &CIRCLE_VTABLE;
    c->radius = r;
}

/* ---- "Virtual call" ---- */
void shape_describe(const Shape *s) { s->vtable->describe(s); }

int main(void) {
    Circle c;
    circle_init(&c, 2.0);
    shape_describe((Shape *)&c);   /* dispatches through vtable */
    return 0;
}
```

This is exactly what C++ compilers generate for virtual functions — C just makes you write it by hand.

---

## 6. "Inheritance": Struct Embedding

```c
typedef struct { int id; } Entity;

typedef struct {
    Entity base;      /* first member → (Entity *)&player is valid */
    char   name[16];
} Player;

void entity_print(const Entity *e) { printf("id=%d\n", e->id); }

Player p = { .base = { .id = 7 }, .name = "Arpit" };
entity_print(&p.base);          /* or (Entity *)&p */
```

- Casting works only because the C standard guarantees a pointer to a struct equals a pointer to its **first member**.
- No automatic method inheritance, no `super`, no overriding — all manual.

---

## 7. Destructors & Resource Cleanup

C has **no destructors**. Common patterns:

```c
/* Pair every _new with _free */
BankAccount *a = account_new("A");
/* ... */
account_free(a);
a = NULL;            /* avoid dangling pointer / double free */
```

```c
/* goto-cleanup pattern for multiple resources */
int process(void) {
    int rc = -1;
    FILE *f = fopen("data.txt", "r");
    if (!f) goto out;
    char *buf = malloc(1024);
    if (!buf) goto close_file;
    /* ... work ... */
    rc = 0;
    free(buf);
close_file:
    fclose(f);
out:
    return rc;
}
```

---

## 8. Naming Conventions (Namespacing)

C has a single global namespace, so prefix everything:

| Element | Convention |
|---|---|
| Type | `BankAccount` / `bank_account_t` |
| Constructor | `account_new`, `account_create`, `account_init` |
| Destructor | `account_free`, `account_destroy` |
| Method | `account_deposit(BankAccount *self, ...)` |
| Private helper | `static` + optional `_` prefix |
| Constants | `ACCOUNT_MAX_OWNER` |

---

## 9. Comparison: C vs C++ vs Rust

| Feature | C | C++ | Rust |
|---|---|---|---|
| Class keyword | ❌ | `class` / `struct` | ❌ (`struct` + `impl`) |
| Methods | Functions + `self` ptr | Member functions | `impl` block |
| Constructor | `x_new()` by convention | Language feature | `new()` by convention |
| Destructor | Manual `x_free()` | Automatic (RAII) | Automatic (`Drop`) |
| `public` | In header | `public:` | `pub` |
| `private` | Opaque struct / `static` | `private:` | Default (module-scoped) |
| `protected` | ❌ | `protected:` | ❌ |
| Inheritance | Struct embedding (manual) | ✅ | ❌ (traits + composition) |
| Polymorphism | Function-pointer vtables | `virtual` | Generics / `dyn Trait` |
| Enforced by | Compiler (incomplete types) + linker (`static`) | Compiler | Compiler (module privacy) |

---

## 10. Summary

- **"Class" in C = `struct` + functions that take a pointer to it.**
- **No access keywords** — visibility comes from:
  - **Header (`.h`)** → public API.
  - **`static`** in `.c` → private to that file.
  - **Opaque struct** → private fields.
- **Polymorphism** = function-pointer tables; **inheritance** = embed base struct as first member.
- **Lifetime is manual**: every `_new` needs a matching `_free`.
