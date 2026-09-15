# Perfiles `dev` y `release` en Rust

## Idea principal

`dev` y `release` son dos perfiles diferentes para construir el mismo programa.
Ambos generan un ejecutable, pero cada uno está pensado para una etapa distinta
del desarrollo.

```text
dev     → desarrollar, modificar y depurar
release → optimizar, medir y distribuir
```

## Configuración predeterminada

No es obligatorio declarar `[profile.dev]` ni `[profile.release]` en
`Cargo.toml`. Si no existen esas secciones, Cargo utiliza sus configuraciones
predeterminadas.

```bash
cargo build              # utiliza dev por defecto
cargo run                # utiliza dev por defecto
cargo build --release    # utiliza release por defecto
cargo run --release      # utiliza release por defecto
```

Los valores principales predeterminados son aproximadamente:

```text
dev:
    opt-level = 0
    debug = true
    debug-assertions = true
    overflow-checks = true
    incremental = true

release:
    opt-level = 3
    debug = false
    debug-assertions = false
    overflow-checks = false
    incremental = false
```

Estos valores permiten que un proyecto funcione correctamente aunque su
`Cargo.toml` solo tenga `[package]` y `[dependencies]`.

## Personalizar solo lo necesario

Cuando se declara un perfil, no es necesario escribir todas sus opciones. Cargo
mantiene los valores predeterminados de las opciones que no se hayan cambiado.

Por ejemplo:

```toml
[profile.release]
lto = true
```

En este caso, `release` continúa usando `opt-level = 3`, `debug = false` y el
resto de sus valores predeterminados; solamente se activa `lto`.

Para una configuración más agresiva orientada a rendimiento y tamaño:

```toml
[profile.release]
opt-level = 3
debug = false
lto = "fat"
codegen-units = 1
strip = "symbols"
panic = "abort"
```

Esta configuración puede generar un binario más optimizado, pero aumenta el
tiempo de compilación, reduce la información disponible para depurar y cambia
la estrategia utilizada cuando ocurre un `panic!`.

## Relación con Compile Time y Run Time

`dev` y `release` no son etapas distintas de ejecución. Son configuraciones que
Cargo utiliza durante **compile time**, es decir, mientras Rust construye el
programa.

```text
Compile time
├── perfil dev     → construye target/debug/
└── perfil release → construye target/release/
```

El perfil elegido cambia cómo se compila el binario:

- `dev` prioriza una compilación rápida y una mejor experiencia de depuración;
- `release` prioriza las optimizaciones y el rendimiento del ejecutable.

Por eso los perfiles se aplican durante compile time, pero sus efectos se notan
durante run time:

```text
cargo build              → compila con dev     → ejecutable menos optimizado
cargo build --release    → compila con release → ejecutable optimizado
```

La etapa de compilación construye el programa; la etapa de ejecución utiliza el
binario que se construyó.

## Perfil `dev`

El perfil `dev` es el perfil utilizado durante el desarrollo.

```bash
cargo build
cargo run
```

El ejecutable se guarda normalmente en:

```text
target/debug/
```

Sus características principales son:

- compilación más rápida;
- pocas optimizaciones;
- información de depuración disponible;
- iteración rápida mientras se modifica el código;
- comprobaciones adicionales útiles durante el desarrollo.

Una configuración habitual es:

```toml
[profile.dev]
opt-level = 0
debug = true
```

`opt-level = 0` indica que prácticamente no se aplican optimizaciones de
rendimiento. `debug = true` conserva información que ayuda a utilizar
depuradores y a interpretar errores.

## Perfil `release`

El perfil `release` está pensado para una versión optimizada del programa.

```bash
cargo build --release
cargo run --release
```

El ejecutable se guarda normalmente en:

```text
target/release/
```

Sus características principales son:

- compilación más lenta;
- optimizaciones de rendimiento;
- menor cantidad de información de depuración;
- ejecución normalmente más rápida;
- mejor opción para distribuir el programa o medir su rendimiento.

Una configuración habitual es:

```toml
[profile.release]
opt-level = 3
debug = false
lto = true
```

`opt-level = 3` solicita un nivel alto de optimización. `debug = false` evita
conservar información de depuración completa y `lto = true` permite optimizar
entre distintos módulos durante el enlazado.

## Opciones de configuración de los perfiles

Los perfiles se configuran en `Cargo.toml`. Cada opción cambia algún aspecto de
la compilación o del binario generado.

### `opt-level`

Define el nivel de optimización:

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

Valores habituales:

| Valor | Significado |
| --- | --- |
| `0` | Sin optimización; compila rápido |
| `1` | Optimización ligera |
| `2` | Optimización intermedia |
| `3` | Optimización máxima para velocidad |
| `"s"` | Intenta reducir el tamaño |
| `"z"` | Prioriza aún más la reducción del tamaño |

`opt-level = "z"` puede ser útil para binarios pequeños, pero no siempre
produce el programa más rápido.

### `debug`

Controla si se conserva información para depuración:

```toml
[profile.dev]
debug = true

[profile.release]
debug = false
```

Con `debug = true` es más fácil utilizar depuradores como GDB o LLDB y leer
*stack traces*. Desactivarlo puede reducir el tamaño del binario.

### Información de depuración en `release`

La información de depuración también puede ser útil en un binario optimizado:

```toml
[profile.release]
opt-level = 3
debug = true
```

Esto conserva información para investigar errores en una versión `release`,
aunque puede aumentar el tamaño del binario. El programa sigue estando
optimizado; `debug = true` no lo convierte en un perfil `dev`.

También existe una opción intermedia:

```toml
[profile.release]
debug = "line-tables-only"
```

`line-tables-only` conserva información mínima para relacionar un error con el
archivo y la línea correspondiente, sin guardar todos los detalles de variables
y tipos. Puede ser útil para obtener *backtraces* informativos sin aumentar
tanto el tamaño del artefacto final.

Los valores conceptualmente más importantes son:

```text
debug = false                  → sin información de depuración
debug = "line-tables-only"    → archivos y líneas para backtraces
debug = true                   → información completa de depuración
```

### Flujo habitual en equipos profesionales

Una organización puede utilizar distintas configuraciones según la etapa:

```text
Desarrollo       → dev + debug completo
Pruebas internas → release + información de depuración
Producción       → release optimizado
```

En producción, los símbolos de depuración pueden conservarse por separado en
los sistemas internos de la organización. Así se pueden analizar errores y
*crash reports* sin entregar toda esa información junto con el binario público.

La decisión depende del equilibrio entre tamaño, rendimiento, privacidad y
capacidad para investigar errores.

### `lto`

LTO significa *Link Time Optimization*. Permite optimizar código de diferentes
módulos durante el enlazado:

```toml
[profile.release]
lto = true
```

Puede mejorar la velocidad o reducir el tamaño, pero aumenta el tiempo de
compilación. El modo `"fat"` solicita una optimización global más profunda.

### `codegen-units`

Indica en cuántas unidades se divide la generación de código:

```toml
[profile.release]
codegen-units = 1
```

Un valor bajo puede permitir mejores optimizaciones, pero normalmente hace que
la compilación sea más lenta. El valor `1` se utiliza cuando se prioriza la
optimización final.

### `strip`

Elimina símbolos e información que no son necesarios para ejecutar el programa:

```toml
[profile.release]
strip = "symbols"
```

Esto puede reducir el tamaño del ejecutable, pero dificulta la depuración de la
versión resultante.

### `panic`

Define qué ocurre cuando el programa entra en `panic!`:

```toml
[profile.release]
panic = "abort"
```

Los valores principales son:

- `"unwind"`: intenta desenrollar la pila y recuperarse del pánico;
- `"abort"`: termina el proceso inmediatamente y puede producir un binario más pequeño.

`panic = "abort"` no convierte un error en uno manejable; solo cambia la forma
en que termina el proceso.

### `overflow-checks`

Controla las comprobaciones de desbordamiento de enteros:

```toml
[profile.release]
overflow-checks = true
```

En `dev` están activas por defecto. En `release` normalmente están desactivadas
para priorizar el rendimiento. Activarlas puede ayudar a detectar errores
aritméticos también en la versión optimizada.

### `incremental`

Controla la compilación incremental:

```toml
[profile.dev]
incremental = true

[profile.release]
incremental = false
```

En desarrollo permite recompilar solo partes que cambiaron. En una compilación
final normalmente se desactiva para priorizar un resultado reproducible y
optimizado.

### `debug-assertions`

Controla las aserciones de depuración, como `debug_assert!`:

```toml
[profile.dev]
debug-assertions = true

[profile.release]
debug-assertions = false
```

Una llamada a `debug_assert!` se comprueba en `dev` y normalmente se elimina en
`release`.

## Ejemplos de perfiles completos

Configuración orientada al desarrollo:

```toml
[profile.dev]
opt-level = 0
debug = true
debug-assertions = true
overflow-checks = true
incremental = true
```

Configuración orientada a velocidad:

```toml
[profile.release]
opt-level = 3
debug = false
lto = "fat"
codegen-units = 1
strip = "symbols"
debug-assertions = false
overflow-checks = false
incremental = false
```

Configuración orientada a tamaño:

```toml
[profile.release]
opt-level = "z"
debug = false
lto = "fat"
codegen-units = 1
strip = "symbols"
panic = "abort"
incremental = false
```

No es necesario activar todas las opciones desde el principio. La configuración
por defecto suele ser suficiente; estas opciones se ajustan cuando existe una
razón concreta, como mejorar el rendimiento, reducir el tamaño o facilitar la
depuración.

## Comparación

| Característica | `dev` | `release` |
| --- | --- | --- |
| Comando | `cargo build` / `cargo run` | `cargo build --release` / `cargo run --release` |
| Carpeta | `target/debug/` | `target/release/` |
| Velocidad de compilación | Más rápida | Más lenta |
| Optimización | Baja o ninguna | Alta |
| Información de depuración | Incluida | Desactivada por defecto |
| Uso principal | Desarrollo | Publicación y rendimiento |

## Overflow de enteros

En el perfil `dev`, las comprobaciones de overflow de enteros están activas por
defecto. Esto puede producir un `panic!` si una operación desborda el tipo:

```rust
let valor: u8 = 255;
let resultado = valor + 1;
```

En `release`, estas comprobaciones normalmente están desactivadas para priorizar
el rendimiento. La operación puede envolver el valor según las reglas de
aritmética de enteros de Rust.

Se puede activar la comprobación explícitamente:

```toml
[profile.release]
overflow-checks = true
```

## `build` frente a `run`

Los perfiles y los comandos están relacionados, pero no son lo mismo:

```bash
cargo build --release
```

Compila el programa en modo `release`, pero no lo ejecuta.

```bash
cargo run --release
```

Compila si es necesario y después ejecuta el programa optimizado.

Es aproximadamente equivalente a:

```bash
cargo build --release
./target/release/mi_programa
```

## Analogía

Imagina que estás construyendo un coche:

```text
dev     → coche de pruebas: fácil de revisar y modificar
release → coche final: optimizado para funcionar eficientemente
```

Los dos coches funcionan, igual que los dos binarios son ejecutables. La
diferencia está en el objetivo de la construcción.

## Regla para recordar

```text
cargo run              → desarrollo
cargo run --release    → versión optimizada
```

Durante el aprendizaje y la programación diaria conviene usar `dev`. Cuando se
quiera distribuir el programa, comparar rendimiento o generar una versión final,
se debe utilizar `release`.
