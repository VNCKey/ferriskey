# 🧪 Code Lab: 06 - Control de Flujo

## Ejercicio 01: Devuelvo de Valores con `loop` y `break`
```rust
fn main() {
    let mut contador = 0;

    let resultado = loop {
        contador += 1;
        if contador == 10 {
            break contador * 2; // Devuelve 20 al salir del bucle
        }
    };

    println!("Resultado devuelto por el bucle loop: {}", resultado);
}
```

---

## Ejercicio 02: Pattern Matching con Rangos y Match Guards
```rust
fn clasificar_edad(edad: u32) {
    match edad {
        0..=12 => println!("Niño ({})", edad),
        13..=17 => println!("Adolescente ({})", edad),
        n if n >= 18 && n < 65 => println!("Adulto ({})", n),
        _ => println!("Adulto mayor"),
    }
}

fn main() {
    clasificar_edad(10);
    clasificar_edad(15);
    clasificar_edad(30);
}
```

---

## Ejercicio 03: Extracción concisa con `if let`
```rust
fn main() {
    let configuracion_opcional: Option<&str> = Some("Modo Oscuro");

    if let Some(config) = configuracion_opcional {
        println!("Configuración activa: {}", config);
    } else {
        println!("Sin configuración predeterminada");
    }
}
```
