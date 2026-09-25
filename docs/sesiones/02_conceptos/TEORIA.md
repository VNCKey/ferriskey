# 📖 Teoría: Sesión 02 - Conceptos Fundamentales

---

## 1. Variables, Inmutabilidad y Mutabilidad Controlada

En Rust, las variables son **inmutables por defecto**. Cuando se declara un identificador mediante `let`, su valor queda vinculado y no puede ser alterado posteriormente:

```rust
let x = 5;
// x = 6; // ❌ ERROR DE COMPILACIÓN: cannot assign twice to immutable variable `x`
```

### ¿Por qué inmutabilidad por defecto?
1. **Previsibilidad y Razonamiento del Código**: Si una variable no puede cambiar, cualquier función que la observe tiene la certeza de que su estado no ha sido corrompido por efectos secundarios concurrentes.
2. **Concurrencia Segura**: Los datos inmutables pueden ser leídos por múltiples hilos en paralelo sin necesidad de bloqueos (*locks*) ni condiciones de carrera (*data races*).
3. **Optimizaciones del Compilador**: LLVM puede propagar constantes y eliminar lecturas de memoria redundantes en registros de la CPU cuando sabe que el valor no cambiará.

### Mutabilidad Explícita con `mut`
Cuando un dato necesita evolucionar, Rust exige que el programador haga explícita su intención con la palabra clave `mut`:

```rust
let mut contador = 0;
contador += 1; // ✅ Válido: mutabilidad declarada explícitamente
```

---

## 2. Tipos de Enlace de Memoria: Comparativa

| Tipo de Enlace | Sintaxis | Tipo Explícito | Evaluado en | Espacio en Memoria | Mutabilidad |
|---|---|---|---|---|---|
| **Variable Inmutable** | `let x = 10;` | Opcional (inferido) | Tiempo de Ejecución | Stack Frame del Ámbito | No modificable |
| **Variable Mutable** | `let mut x = 10;` | Opcional (inferido) | Tiempo de Ejecución | Stack Frame del Ámbito | Modificable con `mut` |
| **Constante** | `const LIMITE: u32 = 100;` | **Obligatorio** | **Tiempo de Compilación** | Sustituida *inline* en código | Nunca modificable |
| **Estática** | `static BANNER: &str = "APP";` | **Obligatorio** | Enlace / Inicio del Programa | Sección fija `.data` / `.rodata` | Inmutable (o `unsafe mut`) |

### Constantes (`const`) vs. Estáticas (`static`)
- Las constantes (`const`) se evalúan en tiempo de compilación. El compilador copia su valor literal en cada lugar donde se utiliza (mecanismo similar a un *inline expansion*).
- Las variables estáticas (`static`) tienen una dirección de memoria fija y única a lo largo de toda la vida del programa.

---

## 3. El Mecanismo de Sombreado (Shadowing)

Rust permite declarar una nueva variable con el **mismo nombre** que una variable declarada anteriormente, utilizando la palabra clave `let`:

```rust
let espacios = "    ";          // Tipo: &str (4 espacios)
let espacios = espacios.len();   // Tipo: usize (longitud numérica = 4)
```

### Diferencia crucial entre Mutabilidad y Shadowing:
1. **Mutación (`let mut`)**: Modifica el valor contenido en la misma celda de memoria, pero **no permite cambiar el tipo de dato**. Si intentas asignar un número a una variable mutable de tipo texto, el compilador fallará.
2. **Shadowing (`let`)**: Crea una **variable completamente nueva** en el Stack que oculta (*shadows*) a la anterior dentro de ese ámbito. Permite cambiar el tipo de dato y reutilizar un nombre descriptivo sin obligar a la variable resultante a ser mutable.

---

## 4. Ámbitos Físicos (Scopes) y Stack Frames

Un ámbito (*scope*) está delimitado por un par de llaves `{}`. Cada vez que el programa entra a un bloque, se crea un marco en la pila (*Stack Frame*); al salir del bloque, las variables declaradas dentro se destruyen inmediatamente y su memoria se libera de forma determinista (filosofía RAII):

```rust
let global_al_bloque = 50;

{
    let interna = 10;
    println!("Suma interna: {}", global_al_bloque + interna); // ✅ 60
} // 💥 Aquí `interna` sale del ámbito y su memoria en el Stack es destruida.

// println!("{interna}"); // ❌ ERROR: `interna` not found in this scope
```

---

## 5. Tipos Primitivos Escalares

Un tipo escalar representa un único valor. Rust cuenta con cuatro familias de escalares primarios:

### 1. Enteros (Integers)
Representan números sin decimales:

| Tamaño | Con Signo (Rango: $-2^{n-1}$ a $2^{n-1}-1$) | Sin Signo (Rango: $0$ a $2^n - 1$) |
|---|---|---|
| **8 bits** | `i8` ($-128$ a $127$) | `u8` ($0$ a $255$) |
| **16 bits** | `i16` ($-32\,768$ a $32\,767$) | `u16` ($0$ a $65\,535$) |
| **32 bits** | `i32` (por defecto para enteros) | `u32` |
| **64 bits** | `i64` | `u64` |
| **128 bits** | `i128` | `u128` |
| **Arquitectura** | `isize` (32 o 64 bits según CPU) | `usize` (usado para indexar colecciones y tamaños) |

- **Bases y Notaciones Literales**:
  - Decimal: `98_222` (permite guiones bajos para legibilidad visual).
  - Hexadecimal: `0xff`.
  - Octal: `0o77`.
  - Binario: `0b1111_0000`.
  - Byte (`u8` solo): `b'A'`.
- **Desbordamiento de Enteros (Integer Overflow)**:
  - En modo **Debug**, Rust incluye chequeos en tiempo de ejecución y genera un `panic!` si ocurre un desbordamiento.
  - En modo **Release** (`--release`), Rust desactiva las comprobaciones por rendimiento y aplica aritmética de complemento a dos (*two's complement wrapping*), donde `255u8 + 1` se convierte en `0`.

### 2. Números en Punto Flotante (Floating-Point)
Siguen el estándar **IEEE 754**:
- `f32`: Precisión simple (32 bits).
- `f64`: Precisión doble (64 bits, tipo por defecto para decimales debido a su mayor exactitud).

### 3. Booleanos (`bool`)
Posee exactamente dos valores: `true` o `false`. Ocupa 1 byte en memoria. Permite operaciones lógicas:
- `&&` (AND lógico con evaluación en cortocircuito).
- `||` (OR lógico con evaluación en cortocircuito).
- `!` (NOT lógico).

### 4. Caracteres (`char`)
El tipo `char` en Rust representa un **valor escalar Unicode** y ocupa **4 bytes (32 bits)** completos en memoria. Esto le permite almacenar no solo letras ASCII, sino caracteres acentuados, alfabetos chino, japonés, cirílico y emojis de forma nativa:

```rust
let letra = 'z';
let corazon = '❤️';
let cangrejo = '🦀'; // Tamaño exacto: 4 bytes
```
*(Nota: los literales `char` usan comillas simples `'`, mientras que las cadenas literales `&str` usan comillas dobles `"`).*

### Conversión de Tipos con `as` (Casting)
Rust **nunca** realiza conversiones de tipos automáticas o implícitas para evitar errores de truncamiento. Toda conversión debe ser explícita:

```rust
let entero: i32 = 42;
let flotante: f64 = entero as f64;
let byte: u8 = entero as u8;
```

---

## 6. Sentencias (Statements) vs. Expresiones (Expressions)

Rust es un lenguaje basado principalmente en **expresiones**. Comprender la diferencia es vital:

1. **Sentencias (Statements)**:
   - Son instrucciones que realizan una acción y **no devuelven ningún valor**.
   - Terminan con un punto y coma `;`.
   - Una declaración como `let x = 6;` es una sentencia.
   - Su valor de retorno evaluado es el tipo unit `()`.

2. **Expresiones (Expressions)**:
   - Evalúan y **producen un valor resultante**.
   - **No llevan punto y coma `;` al final**. Si agregas un punto y coma a una expresión, la transformas en una sentencia y su valor se descarta, retornando `()`.
   - Cualquier bloque `{}` puede actuar como una expresión:

```rust
let y = {
    let a = 3;
    let b = 4;
    a + b // 👈 Sin ';' al final: esta expresión se evalúa a 7 y se asigna a `y`
};
// y vale 7
```

---

## 7. Declaración de Funciones

Las funciones se declaran con `fn`. La convención oficial de nomenclatura es `snake_case`.

```rust
fn calcular_area_rectangulo(base: f64, altura: f64) -> f64 {
    // Retorno implícito de expresión (estilo idiomático en Rust)
    base * altura
}
```

- **Parámetros fuertemente tipados**: Todo parámetro debe declarar explícitamente su tipo (`param: Tipo`). El compilador no realiza inferencia sobre las firmas de funciones para mantener las APIs predecibles y documentadas.
- **Tipo de Retorno (`-> Tipo`)**: Si la función retorna un valor, se especifica tras la flecha `->`. Si se omite, retorna implícitamente el tipo vacío unit `()`.

---

## 8. Metaprogramación Básica: Macros y Format Strings

Las macros se distinguen de las funciones regulares por el signo de exclamación `!` al final de su identificador:

```rust
println!("Hola, {}!", "Ferris");
```

### Macros Declarativas Comunes
- `println!(...)`: Escribe en la salida estándar (*stdout*) con salto de línea.
- `print!(...)`: Escribe en *stdout* sin salto de línea.
- `eprintln!(...)`: Escribe en la salida de errores estándar (*stderr*).
- `format!(...)`: Devuelve un `String` formateado sin imprimirlo en pantalla.
- `panic!(...)`: Imprime un mensaje de error crítico y detiene la ejecución inmediatamente.
- `todo!(...)` y `unimplemented!(...)`: Marcadores para código pendiente que compilan pero causan pánico informativo si se ejecutan.

### Especificadores de Formato
- `{}`: Formato estándar mediante el trait `Display` (para usuarios finales).
- `{:?}`: Formato de depuración mediante el trait `Debug` (para desarrolladores).
- `{:#?}`: Formato de depuración con sangrías y saltos de línea legibles (*pretty-print*).
- `{:x}` / `{:b}`: Muestra el número en formato hexadecimal o binario.
- `{variable}`: Interpolación directa de variables disponibles en el ámbito actual.
