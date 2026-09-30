# 📖 Teoría Exhaustiva: Sesión 14 - Testing y Calidad de Código

---

## 1. El Ecosistema de Pruebas de Rust

Rust incluye un arnés de pruebas integrado de primer nivel. No necesitas instalar librerías de terceros (como JUnit, Jest o Mocha); basta con invocar `cargo test`.

---

## 2. Anatomía de un Test Unitario

Los tests unitarios residen en el mismo archivo que el código fuente, dentro de un submódulo marcado con `#[cfg(test)]`:

```rust
pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

pub fn dividir(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        Err("División por cero")
    } else {
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sumar() {
        assert_eq!(sumar(2, 3), 5);
        assert_ne!(sumar(2, 2), 5);
    }

    #[test]
    fn test_dividir_valido() {
        let res = dividir(10.0, 2.0);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), 5.0);
    }

    #[test]
    fn test_dividir_por_cero() {
        assert!(dividir(5.0, 0.0).is_err());
    }
}
```

---

## 3. Aserciones y Panics Esperados

| Macro / Atributo | Propósito |
| :--- | :--- |
| `assert!(condicion)` | Verifica que una expresión booleana sea `true`. |
| `assert_eq!(a, b)` | Comprueba igualdad e imprime ambos valores en caso de fallo. |
| `assert_ne!(a, b)` | Comprueba que ambos valores sean diferentes. |
| `#[should_panic(expected = "...")]` | El test pasa únicamente si el código entra en pánico con el mensaje esperado. |

```rust
pub struct Adivinanza {
    valor: i32,
}

impl Adivinanza {
    pub fn new(valor: i32) -> Self {
        if valor < 1 || valor > 100 {
            panic!("El valor debe estar entre 1 y 100, se recibió {}", valor);
        }
        Self { valor }
    }
}

#[test]
#[should_panic(expected = "debe estar entre 1 y 100")]
fn test_adivinanza_invalida() {
    Adivinanza::new(200);
}
```

---

## 4. Tests de Integración (`tests/`)

Los tests de integración viven en un directorio separado `tests/` en la raíz del proyecto. Cada archivo `.rs` en `tests/` es tratado como una crate independiente que solo puede acceder a la interfaz pública `pub` de tu librería:

```rust
// tests/integracion_calculadora.rs
use mi_crate::sumar;

#[test]
fn test_flujo_completo() {
    assert_eq!(sumar(100, 200), 300);
}
```

---

## 5. Doc-Tests (Documentación Ejecutable)

Los ejemplos en los comentarios de documentación (`///`) son compilados y probados automáticamente por `cargo test`:

```rust
/// Suma dos enteros y retorna el total.
///
/// # Ejemplos
///
/// ```
/// use mi_crate::sumar;
/// assert_eq!(sumar(2, 2), 4);
/// ```
pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}
```
