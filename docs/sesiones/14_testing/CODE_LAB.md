# 🧪 Code Lab: Sesión 14 - Testing y Calidad de Código

---

## 🎯 Reto 1: Banco de Pruebas para un Validador de Usuarios

### Enunciado
Implementa una función `validar_email(email: &str) -> bool` y escribe sus tests unitarios cubriendo casos válidos e inválidos.

```rust
pub fn validar_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && !email.starts_with('@')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_valido() {
        assert!(validar_email("ferris@rust-lang.org"));
    }

    #[test]
    fn email_sin_arroba() {
        assert!(!validar_email("ferris.rust-lang.org"));
    }

    #[test]
    fn email_invalido_inicio() {
        assert!(!validar_email("@rust-lang.org"));
    }
}
```

---

## 🎯 Reto 2: Test con `Result` en vez de Panic

### Enunciado
Escribe un test que devuelva `Result<(), String>` para verificar una operación sin necesidad de usar `unwrap()`.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_resultado_exitoso() -> Result<(), String> {
        let valor: Result<i32, &str> = Ok(10);
        let num = valor.map_err(|e| e.to_string())?;
        assert_eq!(num * 2, 20);
        Ok(())
    }
}
```
