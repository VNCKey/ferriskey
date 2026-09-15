# Plan de la sesión `Custom Types`

Esta sesión presenta cómo Rust permite crear tipos propios sin saturar al
estudiante con conceptos avanzados que se reutilizarán más adelante.

## Objetivo

Comprender cómo se define un tipo personalizado, cómo se protege su estado y
cómo se le agrega comportamiento.

## Contenido principal

### `struct`

Un `struct` agrupa datos relacionados bajo un tipo propio.

```rust
struct Usuario {
    nombre: String,
    edad: u8,
}
```

### Campos y visibilidad

Los campos pueden ser públicos o privados:

```rust
pub struct Usuario {
    pub nombre: String,
    edad: u8,
}
```

La visibilidad permite proteger el estado interno y controlar cómo otros
módulos pueden utilizar el tipo.

### `impl`

El bloque `impl` agrega métodos y funciones asociadas al `struct`:

```rust
impl Usuario {
    fn nuevo(nombre: String, edad: u8) -> Self {
        Self { nombre, edad }
    }

    fn edad(&self) -> u8 {
        self.edad
    }
}
```

Rust no exige un constructor especial. Una función asociada como `nuevo` puede
cumplir ese papel.

### Traits

Los traits describen comportamientos compartidos por diferentes tipos:

```rust
trait Identificable {
    fn id(&self) -> u32;
}
```

Después se implementan para el tipo mediante `impl Trait for Tipo`.

### `derive`

Los atributos `derive` generan implementaciones comunes automáticamente:

```rust
#[derive(Debug, Clone, PartialEq)]
struct Punto {
    x: i32,
    y: i32,
}
```

La sesión debe explicar que `derive` es un atributo que activa una macro
procedural de tipo derive.

### Composición

La composición construye un tipo usando otros tipos como campos:

```rust
struct Motor {
    potencia: u32,
}

struct Coche {
    motor: Motor,
}
```

La relación es “Coche tiene un Motor”, no “Coche hereda de Motor”. Rust
favorece la composición sobre la herencia de clases.

### Patrones y destructuring

Los campos de un `struct` pueden extraerse mediante patrones:

```rust
let usuario = Usuario::nuevo("Alek".to_string(), 25);
let Usuario { nombre, edad } = usuario;
```

Esto conecta los `structs` con pattern matching y destructuring.

### Serialización

La serialización transforma un tipo Rust en un formato como JSON; la
deserialización realiza el camino inverso. Bibliotecas como `serde` suelen
utilizar derives:

```rust
#[derive(serde::Serialize, serde::Deserialize)]
struct Usuario {
    nombre: String,
    edad: u8,
}
```

Este tema puede relacionarse con APIs, archivos de configuración y mensajes de
red, pero sin profundizar todavía en generics ni lifetimes.

## Enums dentro de `Custom Types`

La sesión también puede presentar enums como tipos que representan una
posibilidad entre varias:

```rust
enum Estado {
    Activo,
    Inactivo,
}
```

Temas recomendados:

- variantes simples y con datos;
- `impl` para enums;
- `match`;
- `derive`;
- relación entre enum y `Option`/`Result`.

## Temas reservados para sesiones posteriores

### Generics

No se profundiza en generics aquí porque pueden aparecer en muchos contextos:

- funciones genéricas;
- `struct<T>`;
- enums genéricos;
- traits genéricos;
- colecciones como `Vec<T>` y `HashMap<K, V>`.

### Lifetimes

Se reservan para una sesión propia porque describen cuánto tiempo son válidas
las referencias y cómo se relacionan sus vidas útiles:

- referencias con lifetime explícito;
- funciones con lifetimes;
- structs que contienen referencias;
- relaciones entre varios lifetimes;
- interacción con ownership y borrowing.

## Orden pedagógico recomendado

```text
Tipos escalares
    ↓
Tipos compuestos
    ↓
Structs y enums
    ↓
Traits
    ↓
Generics
    ↓
Lifetimes
    ↓
Ownership y borrowing avanzado
```

## Resumen para el estudiante

```text
struct → agrupa datos
impl   → agrega comportamiento propio
trait  → define comportamiento compartido
derive → genera implementaciones automáticamente
composición → combina tipos más pequeños
patrones → extrae y analiza datos
serde  → convierte tipos a otros formatos
```
