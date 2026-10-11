# Flutter Crash Guide (2026)

**Latest stable:** Flutter 3.47 (Aug 2026) · Dart 3.13
**Next:** 3.50 (Nov 2026). Quarterly releases.

## 2026 highlights
- **Impeller** is now the default renderer on macOS, Windows, Linux (already default on iOS/Android).
- **Material & Cupertino** split into standalone packages (updates ship without waiting for SDK releases).
- **Widget Previews** tool is stable (3.47): preview widgets without running the full app.
- Per-platform assets in `pubspec.yaml` (3.41).
- iOS: UIScene lifecycle supported (3.38).
- Android: migrate to built-in Kotlin Gradle plugin.
- WebAssembly (WasmGC) supported for web.

> Verify details in the official release notes: https://docs.flutter.dev/release/release-notes

---

## 1. Setup

```bash
flutter doctor
flutter create my_app
cd my_app
flutter run
```

Structure: `lib/main.dart` (entry), `pubspec.yaml` (deps/assets), `android/ ios/ web/ windows/ macos/ linux/`.

Minimal app:

```dart
import 'package:flutter/material.dart';

void main() => runApp(const MyApp());

class MyApp extends StatelessWidget {
  const MyApp({super.key});
  @override
  Widget build(BuildContext context) => MaterialApp(
        theme: ThemeData(colorSchemeSeed: Colors.indigo, useMaterial3: true),
        home: const Scaffold(body: Center(child: Text('Hello'))),
      );
}
```

---

## 2. Core Tokens (Building Blocks)

### 2.1 Widget types

| Token | Difference | Shines for |
|---|---|---|
| `StatelessWidget` | Immutable, no internal state | Static UI, labels, layouts, icons |
| `StatefulWidget` | Has `State` object, `setState()` | Local UI state: toggles, forms, animations |
| `InheritedWidget` | Passes data down the tree | Theme, locale, DI (base of Provider/Riverpod) |
| `HookWidget` (flutter_hooks) | Hooks inside stateless-style widget | Less boilerplate for controllers/animations |

### 2.2 Layout

| Token | Difference | Shines for |
|---|---|---|
| `Row` | Horizontal children | Toolbars, icon + text |
| `Column` | Vertical children | Forms, stacked content |
| `Stack` | Overlap children | Badges, image + overlay text |
| `Expanded` / `Flexible` | Expanded fills space; Flexible may be smaller | Splitting space in Row/Column |
| `Container` | Padding, margin, decoration, size in one | Quick styling (prefer specific widgets for perf) |
| `SizedBox` | Fixed size / spacer | Gaps, constrained size (cheaper than Container) |
| `Padding` / `Align` / `Center` | Single-purpose layout | Clean, lightweight positioning |
| `Wrap` | Flows to next line | Chips, tags |
| `LayoutBuilder` | Builds from parent constraints | Responsive layouts |
| `MediaQuery` | Screen size, padding, text scale | Device-level adaptation |
| `Spacer` | Flexible empty space | Pushing items apart in Row/Column |

### 2.3 Scrolling / lists

| Token | Difference | Shines for |
|---|---|---|
| `ListView` | Simple scrolling list | Short lists |
| `ListView.builder` | Lazy, builds visible items only | Long/infinite lists |
| `GridView.builder` | Lazy grid | Galleries, product grids |
| `SingleChildScrollView` | Scrolls one child fully built | Small content that overflows |
| `CustomScrollView` + Slivers | Composable scroll effects | Collapsing app bars, mixed lists/grids |
| `PageView` | Swipe pages | Onboarding, carousels |

### 2.4 Navigation

| Token | Difference | Shines for |
|---|---|---|
| `Navigator` (push/pop) | Imperative stack | Simple flows, dialogs |
| `go_router` | Declarative, URL-based, deep links | Most production apps, web |
| `auto_route` | Code-generated typed routes | Large apps wanting type safety |
| `BottomNavigationBar` / `NavigationBar` (M3) | Bottom tabs | Mobile top-level destinations |
| `NavigationRail` / `Drawer` | Side nav | Tablet/desktop |

### 2.5 Inputs & forms

| Token | Difference | Shines for |
|---|---|---|
| `TextField` | Standalone input | Search, single field |
| `TextFormField` + `Form` | Validation + grouping | Login, signup, multi-field forms |
| `ElevatedButton` / `FilledButton` | Primary actions | Main CTA |
| `OutlinedButton` / `TextButton` | Secondary / low emphasis | Cancel, links |
| `IconButton` | Icon only | Toolbars |
| `Checkbox` / `Switch` / `Radio` | Boolean / single choice | Settings |
| `GestureDetector` | Raw gestures, no visual feedback | Custom taps/drags |
| `InkWell` | Tap with ripple | Tappable cards/tiles |

### 2.6 Design systems

| Token | Difference | Shines for |
|---|---|---|
| Material (`MaterialApp`) | Android-style, Material 3 default | Cross-platform consistent brand UI |
| Cupertino (`CupertinoApp`) | iOS-style | iOS-native feel |
| `ThemeData` / `ColorScheme` | Central design tokens (color, type, shape) | App-wide theming, dark mode |
| `TextTheme` | Typography scale | Consistent text |
| `ThemeExtension` | Custom theme tokens | Brand-specific tokens |

### 2.7 Async & data

| Token | Difference | Shines for |
|---|---|---|
| `Future` | One async result | API call, file read |
| `Stream` | Many async results over time | Realtime data, sockets |
| `FutureBuilder` | Builds UI from a Future | One-off loading UI |
| `StreamBuilder` | Builds UI from a Stream | Live updates |
| `async` / `await` | Sequential async syntax | Clean async code |
| `Isolate` / `compute()` | Separate thread | Heavy parsing, image processing |

### 2.8 Dart language tokens

| Token | Meaning / Use |
|---|---|
| `final` | Set once at runtime |
| `const` | Compile-time constant; use `const` widgets to skip rebuilds |
| `late` | Initialized later, non-null |
| `?` / `!` / `??` / `?.` | Null safety operators |
| Records `(int, String)` | Lightweight multiple return values |
| Patterns / `switch` expressions | Destructuring, exhaustive matching |
| Sealed classes | Exhaustive state modeling |
| Extension methods | Add methods to existing types |
| Generics `List<T>` | Type-safe collections |
| Mixins (`with`) | Reusable behavior without inheritance |

---

## 3. State Management Comparison

| Option | Complexity | Difference | Best for |
|---|---|---|---|
| `setState` | Very low | Local, built-in | Single-widget state |
| `ValueNotifier` + `ValueListenableBuilder` | Low | Built-in reactive value | Small shared values |
| `InheritedWidget` / `InheritedNotifier` | Low-Med | Built-in, verbose | Learning, tiny apps |
| **Provider** | Low | Simple wrapper over InheritedWidget | Small-medium apps |
| **Riverpod** | Medium | Compile-safe, no BuildContext needed, testable | Most new apps; scalable |
| **Bloc / Cubit** | Medium-High | Events → states, strict structure | Large teams, enterprise |
| **GetX** | Low | All-in-one, magic-heavy | Quick prototypes (avoid for big codebases) |
| **MobX / Signals** | Medium | Observable/reactive | Reactive-programming fans |

**Rule of thumb:** local UI → `setState`; app-wide → Riverpod; strict architecture → Bloc.

---

## 4. Common Packages

| Need | Package |
|---|---|
| HTTP | `dio`, `http` |
| JSON models | `json_serializable`, `freezed` |
| Local storage (simple) | `shared_preferences` |
| Local DB | `drift`, `isar`/`sqflite`, `hive` |
| Firebase | `firebase_core`, `cloud_firestore`, `firebase_auth` |
| Images | `cached_network_image` |
| DI | `get_it`, `injectable` |
| Routing | `go_router` |

---

## 5. Rendering & Performance Tips
- Use `const` constructors wherever possible.
- Prefer `ListView.builder` for long lists.
- Keep `build()` cheap; split big widgets into smaller ones.
- Don't call `setState` high in the tree.
- Use `compute()`/Isolates for heavy work.
- Profile with **Flutter DevTools** (Performance, Widget Inspector).
- Use `RepaintBoundary` for expensive, isolated repaints.
- Impeller removes shader-compilation jank (now default on all major native platforms).

---

## 6. Quick Cheat Sheet

| I want to... | Use |
|---|---|
| Show static content | `StatelessWidget` |
| Handle a toggle/form field | `StatefulWidget` |
| Long list | `ListView.builder` |
| Responsive layout | `LayoutBuilder` / `MediaQuery` |
| Overlap widgets | `Stack` |
| Deep links / web URLs | `go_router` |
| Global state | Riverpod / Bloc |
| Load API data once | `FutureBuilder` or Riverpod `FutureProvider` |
| Live data | `StreamBuilder` |
| Heavy computation | `compute()` |
| Dark mode | `ThemeData` + `ColorScheme` |

---

## 7. Learning Path
1. Dart basics → null safety, async.
2. Widgets, layouts, lists.
3. Navigation (`go_router`).
4. State management (Riverpod or Bloc).
5. Networking + local storage.
6. Testing (unit, widget, integration).
7. Build & release (Android AAB, iOS IPA, web/desktop).

Official docs: https://docs.flutter.dev
