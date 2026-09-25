# 📖 Teoría: Sesión 12 - Traits y Polimorfismo

---

## 1. ¿Qué es un Trait?

Un **Trait** define un conjunto de métodos abstractos que describen un comportamiento compartido. Es el equivalente en Rust a las *interfaces* de otros lenguajes:

```rust
trait Describible {
    fn describir(&self) -> String;

    // Método con implementación por defecto opcional
    fn es_valido(&self) -> bool {
        true
    }
}
```

Implementación para un tipo:
```rust
struct Articulo {
    titulo: String,
}

impl Describible for Articulo {
    fn describir(&self) -> String {
        format!("Artículo: {}", self.titulo)
    }
}
```

---

## 2. Traits Estándar Esenciales

1. **`std::fmt::Display`**: Permite formatear la estructura con `{}` para usuarios finales.
2. **`std::fmt::Debug`**: Permite formatear con `{:?}` para depuración (se puede derivar con `#[derive(Debug)]`).
3. **`Clone` y `Copy`**: `Clone` realiza una copia explícita profunda; `Copy` marca que el tipo puede duplicarse bit a bit en el Stack.
4. **`From` e `Into`**: Conversiones idiomáticas. Implementar `From<A> for B` implementa automáticamente `Into<B> for A`.
5. **`Default`**: Provee un constructor sin argumentos con valores iniciales estándar.

---

## 3. Despacho Estático vs. Despacho Dinámico

Rust permite elegir conscientemente entre dos modelos de polimorfismo:

### A. Despacho Estático (Static Dispatch)
Utiliza genéricos y restricciones de traits:
```rust
fn procesar<T: Describible>(item: T) {
    println!("{}", item.describir());
}
```
- **Compilador**: Monomorfiza el código generando una función especializada por cada tipo.
- **Rendimiento**: Máxima velocidad. No hay costo de indirección ni punteros; permite inlining completo por parte de LLVM.
- **Limitación**: Todos los elementos de una colección deben ser del mismo tipo concreto `T`.

### B. Despacho Dinámico (Dynamic Dispatch)
Utiliza **Trait Objects** con la palabra clave `dyn`:
```rust
fn procesar_dinamico(item: &dyn Describible) {
    println!("{}", item.describir());
}
```
- **Memoria**: Utiliza un puntero gordo (*Fat Pointer* de 16 bytes en 64 bits):
  1. Puntero a los datos reales de la instancia.
  2. Puntero a la tabla virtual (**vtable**), que contiene las direcciones de los métodos del trait.
- **Flexibilidad**: Permite colecciones heterogéneas, por ejemplo `Vec<Box<dyn Describible>>`.
- **Rendimiento**: Implica una pequeña penalización por salto indirecto de puntero y no permite *inlining*.
