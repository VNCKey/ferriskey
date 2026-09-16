# 📖 Teoría: 01 - Rust Foundations

## 1. El Toolchain y Cargo
`rustc` compila código Rust a binario nativo. `Cargo` gestiona dependencias, la configuración en `Cargo.toml` y el árbol de dependencias en `Cargo.lock`.

## 2. Perfiles de Compilación
- **`dev`** (`cargo build`): Rápido de compilar, incluye símbolos DWARF para depuración.
- **`release`** (`cargo build --release`): Compilación optimizada O3 para producción.
