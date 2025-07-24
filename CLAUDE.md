# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust workspace containing trait-based abstractions for peripheral devices and cryptographic algorithms. The project is designed for embedded systems and provides zero-cost abstractions for hardware peripherals and crypto operations.

## Workspace Structure

- `peripheral_traits/` - Peripheral device abstractions (I2C, I3C, OTP, block devices, system control)
- `crypto_traits/` - Cryptographic algorithm abstractions (digest, ECDSA, MAC, RSA, symmetric ciphers)
- `messaging_traits/` - Inter-service messaging abstractions (client/service patterns)
- `proposed-traits/` - Compatibility layer that re-exports all traits for backward compatibility
- `docs/` - Design documents and architecture documentation

## Common Development Commands

**Build the workspace:**
```bash
cargo build
```

**Run tests:**
```bash
cargo test
```

**Check code:**
```bash
cargo check
```

**Build examples:**
```bash
cargo build --examples
```

**Run a specific example:**
```bash
cargo run --example otp_aspeed
cargo run --example spdm_hash_negotiation
cargo run --example software_ecdsa_impl
```

**Run examples:**
```bash
cargo run --example otp_aspeed --package proposed-traits
```

**Format code:**
```bash
cargo fmt
```

**Lint code:**
```bash
cargo clippy
```

## Core Architecture

### Design Principles
- **Zero-cost abstractions** - All traits compile to optimal code without runtime overhead
- **Type safety** - Extensive use of Rust's type system to prevent misuse
- **No-std compatible** - Designed for embedded environments
- **No unsafe code** - Entire codebase uses `#![deny(unsafe_code)]`
- **Composable traits** - Modular design allowing incremental capability implementation

### Key Trait Patterns

**Error Handling Pattern:**
All modules follow a consistent error handling pattern with `ErrorKind` enums, `Error` traits, and `ErrorType` association traits.

**Algorithm/Hardware Abstraction:**
Cryptographic traits separate algorithm specification (zero-sized types) from implementation details.

**Capability-Based Composition:**
Complex devices are modeled as compositions of capability traits, allowing incremental implementation of features.

### Major Trait Categories

**Cryptographic Abstractions:**
- `digest` - Hash functions (SHA-2, SHA-3, BLAKE, etc.)
- `ecdsa` - Elliptic Curve Digital Signatures  
- `mac` - Message Authentication Codes
- `rsa` - RSA operations
- `symm_cipher` - Symmetric encryption

**Peripheral Abstractions:**
- `block_device` - Block storage devices (Flash, EEPROM, etc.)
- `i2c_target` - I2C target/slave device behavior
- `i3c_master`/`i3c_target` - I3C controller and target interfaces
- `otp` - Comprehensive One-Time Programmable memory interface
- `system_control` - Clock and reset control

**Communication:**
- `client` - Inter-service communication
- `service` - Service provider abstraction

### OTP Memory System

The OTP (One-Time Programmable) trait system is particularly sophisticated, supporting:
- Basic read/write operations (`OtpMemory<T>`)
- Multi-region devices (`OtpRegions<T>`)
- Session management (`OtpSession`)
- Write tracking (`OtpWriteTracking<T>`)
- Protection mechanisms (`OtpProtection`)
- Soak programming for difficult bits (`OtpSoakProgramming<T>`)

## Crate Selection

**For new projects, use the specific crates directly:**
- `peripheral_traits` - For embedded peripheral device abstractions
- `crypto_traits` - For cryptographic algorithm implementations  
- `messaging_traits` - For inter-service messaging patterns

**For existing projects:**
- `proposed-traits` provides a compatibility layer re-exporting all traits

## Development Guidelines

### Code Style
- Follow Rust standard conventions
- Use `#![no_std]` for embedded compatibility
- Avoid unsafe code entirely (`#![deny(unsafe_code)]`)
- Prefer associated types and const generics for zero-cost abstractions
- Use comprehensive error handling with the established patterns

### Crate Dependencies
- Peripheral traits have minimal embedded-focused dependencies
- Crypto traits can optionally include reference implementations
- Messaging traits have minimal serialization dependencies
- Each crate can be used independently

### Trait Implementation
- Implement only the capabilities your device actually supports
- Use the composable trait design to build up functionality incrementally
- Follow the established error handling patterns
- Provide meaningful `ErrorKind` mappings for implementation-specific errors

## Key Design Documents

See `docs/peripheral_traits_design_document.md` for comprehensive architectural documentation including detailed trait descriptions, design patterns, and implementation examples.