# OpenPRoT Stack Architecture and Integration Models

**Note**: OpenPRoT is a collection of crates. Each crate provides a focused set of APIs or functionality, and downstream projects can depend on any subset as needed.

## 1. Library-First Design

The openprot workspace is a set of libraries that:
- Exposes all core APIs, traits, and types for downstream integration
- Avoids hard dependencies on any one OS or middleware; uses traits and feature flags for flexibility
- Provides composable building blocks for Platform Root of Trust implementations

## 2. Workspace Structure

The OpenPRoT workspace is organized as a modular set of libraries and supporting infrastructure. Each directory serves a specific architectural or integration purpose:

```
/Cargo.toml                # Workspace manifest, includes all crates and manages features
/openprot/                 # Main library crate: top-level APIs, traits, and types for PRoT logic
/hal/                      # Hardware Abstraction Layer traits for device drivers
    /blocking/             # Blocking/synchronous HAL traits (e.g., for bare-metal or RTOS)
    /async/                # Async HAL traits (for async/await-based drivers)
    /nb/                   # Non-blocking HAL traits (for polling-based drivers)
/platform/                # Platform abstraction and OS portability
    /traits/               # OS abstraction traits for cross-platform compatibility
    /impls/                # Platform-specific implementations
        /linux/            # Linux platform implementation
        /tock/             # Tock OS platform implementation
        /hubris/           # Hubris OS platform implementation
/services/                 # Core services provided by OpenPRoT
    /telemetry/            # Telemetry, monitoring, and logging service
    /storage/              # Persistent storage service and abstractions

/docs/                     # Project and API documentation (mdBook, guides, architecture docs)
/xtask/                    # Custom build/dev automation tasks (Rust binary crate)
/tests/                    # Integration/system tests for the workspace
```

### Key Components

**`openprot/`**: The main entry point for downstream users. Re-exports core traits, types, and provides the high-level API surface for PRoT logic. Keeps the workspace cohesive and easy to depend on.

**`hal/`**: Defines all hardware abstraction traits, organized by execution model. This enables portability and composability for device drivers and hardware access.
- **`blocking/`**: Traits for blocking/synchronous operations (e.g., SPI, I2C, GPIO)
- **`async/`**: Traits for async/await-based drivers, compatible with modern async runtimes
- **`nb/`**: Traits for non-blocking, polling-based drivers (common in embedded Rust)

**`platform/`**: Provides OS abstraction and platform-specific implementations for cross-platform compatibility.
- **`traits/`**: Core OS abstraction traits that define the interface contract
- **`impls/`**: Platform-specific implementations of the abstraction traits
  - **`linux/`, `tock/`, `hubris/`**: Concrete implementations for each supported platform

**`services/`**: Contains core service libraries that provide reusable functionality to the rest of the stack and to downstream integrators.
- **`telemetry/`**: Implements telemetry, monitoring, and logging APIs for observability and diagnostics
- **`storage/`**: Provides persistent storage abstractions and implementations, such as key-value stores or secure storage backends



**`docs/`**: All documentation, including architecture, integration guides, and API docs. Use mdBook or similar tools for maintainability.

**`xtask/`**: Rust binary crate for custom build, test, and dev automation tasks. Keeps build logic out of main crates.

**`tests/`**: Workspace-level integration and system tests, ensuring all crates work together as intended.

## 3. Middleware as Optional/Swappable

Each protocol (MCTP, SPDM, PLDM) is managed as an external Cargo dependency, with trait-based interfaces for swappability. The main openprot crate depends on these via optional features:

```toml
[features]
mctp = ["dep:mctp-rs"]
spdm = ["dep:spdm-rs"] 
pldm = ["dep:pldm-rs"]
default = []
```

Use Rust's trait objects or generics to allow the integrator to provide their own implementation if desired.

## 4. Downstream Integration

Downstream projects (e.g. opentitan) add openprot as a dependency in their workspace. They can:
- Select which middleware to enable via feature flags
- Provide their own implementations by implementing the required traits
- Link the OS kernel, apps, and openprot together to produce the final binary

## 5. Exposing Generic APIs (Example)

```rust
pub trait MctpMiddleware { /* ... */ }
pub trait SpdmMiddleware { /* ... */ }
// ...etc

pub struct OpenProt<T: MctpMiddleware, U: SpdmMiddleware> {
    pub mctp: T,
    pub spdm: U,
    // ...
}
```

Downstream integrators can construct OpenProt with their own middleware implementations.

## 6. Providing a Reference Stack

Create a new crate (e.g. `openprot-stack` or `reference-stack`) that:
- Wires up concrete implementations of middleware, HAL, and platform abstractions
- Exposes a simple API or entrypoint for the reference stack

### Example Structure
```
/openprot-stack/
  /src/lib.rs
  /src/main.rs       # (optional, if you want a binary)
```

```rust
use openprot::{OpenProt, MctpMiddleware, SpdmMiddleware};
use mctp_rs::MctpDefault;
use spdm_rs::SpdmDefault;

pub fn make_reference_stack() -> OpenProt<MctpDefault, SpdmDefault> {
    OpenProt {
        mctp: MctpDefault::new(),
        spdm: SpdmDefault::new(),
        // ...
    }
}
```

## 7. Documentation and Examples

- Document the expected integration workflow in README.md and docs/
- Provide examples for both custom and reference stack usage
- Include platform-specific integration guides

## Summary

- **openprot** = generic, composable collection of libraries
- **Protocol dependencies** = external crates managed via Cargo (e.g., `mctp-rs`, `spdm-rs`, `pldm-rs`)
- **Reference stack** = separate crate wiring up fixed components  
- **Downstream** = links openprot, OS, apps, and selects/implements middleware
- **Structure** = modular, with clear separation of core, middleware, and platform abstraction

This structure follows Rust community conventions with clear separation of concerns, minimal nesting, and intuitive naming that reflects the architectural boundaries of the system.
