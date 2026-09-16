# 🧪 Code Lab: 10 - Error Handling

## Ejercicio 01: Familia de Combinadores en `Option`
```rust
fn main() {
    let config_usuario: Option<String> = None;

    // 1. .unwrap_or con valor estático
    let tema = config_usuario.clone().unwrap_or(String::from("Oscuro"));
    println!("Tema seleccionado: {}", tema);

    // 2. .unwrap_or_else con evaluación perezosa
    let puerto = None::<u16>.unwrap_or_else(|| 8080);
    println!("Puerto activo: {}", puerto);

    // 3. .map para transformar sin desempaquetar
    let nombre = Some(String::from("Ferris"));
    let longitud = nombre.map(|n| n.len());
    println!("Longitud opcional: {:?}", longitud); // Some(6)
}
```

---

## Ejercicio 02: Propagación de Errores con `Result` y `?`
```rust
use std::num::ParseIntError;

fn sumar_numeros_str(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let num_a = a.parse::<i32>()?; // Si falla, retorna Err de inmediato
    let num_b = b.parse::<i32>()?;
    Ok(num_a + num_b)
}

fn main() {
    match sumar_numeros_str("15", "25") {
        Ok(total) => println!("Suma exitosa: {}", total),
        Err(e) => println!("Error de parseo: {}", e),
    }

    if let Err(err) = sumar_numeros_str("quince", "25") {
        println!("Error capturado limpiamente: {}", err);
    }
}
```
