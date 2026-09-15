# Conversiones y traits estándar en Rust

Rust ofrece traits estándar para describir cómo convertir un tipo en otro. No
convierten cualquier cosa automáticamente: el programador debe definir la
conversión y garantizar que tenga sentido.

## `From` y `Into`

`From` representa una conversión que siempre puede realizarse correctamente.

```rust
let edad: u8 = 20;
let edad_ampliada: u16 = u16::from(edad);
```

Cuando implementas `From<A> for B`, Rust proporciona también `Into<B> for A`.
Por eso normalmente se recomienda implementar `From` y utilizar `into()` cuando
resulte más cómodo.

```rust
struct UsuarioId(u64);

impl From<u64> for UsuarioId {
    fn from(valor: u64) -> Self {
        UsuarioId(valor)
    }
}

let id: UsuarioId = 42u64.into();
```

La conversión debe ser infalible. Si puede fallar, no corresponde usar `From`.

## `TryFrom` y `TryInto`

Se utilizan cuando la conversión puede fallar. Devuelven un `Result`:

```rust
let valor: u16 = 300;
let resultado = u8::try_from(valor);

assert!(resultado.is_err());
```

Para un tipo propio:

```rust
use std::convert::TryFrom;

struct Edad(u8);

impl TryFrom<u16> for Edad {
    type Error = &'static str;

    fn try_from(valor: u16) -> Result<Self, Self::Error> {
        if valor <= 120 {
            Ok(Edad(valor as u8))
        } else {
            Err("la edad está fuera del rango permitido")
        }
    }
}

let edad = Edad::try_from(30u16);
```

`TryInto` ofrece la conversión como método:

```rust
use std::convert::TryInto;

let valor: u16 = 200;
let numero: u8 = valor.try_into()?;
```

El operador `?` requiere que la función actual pueda devolver un `Result`.

## Diferencia con `as`

`as` realiza una conversión directa y puede ocultar que el valor no es válido
para el tipo destino. Para conversiones que necesitan validación, es preferible
`TryFrom` o `TryInto`.

```rust
let valor: u16 = 300;
let comprobado = u8::try_from(valor); // Result<u8, Error>
```

La conversión correcta depende del dominio: convertir un identificador, una
edad o un mensaje de red puede requerir reglas diferentes.

## Conversión entre `struct` y `enum`

Estos traits son muy útiles para transformar tipos de una capa de la aplicación
en tipos de otra capa:

```rust
struct Usuario {
    nombre: String,
}

struct UsuarioResponse {
    nombre: String,
}

impl From<Usuario> for UsuarioResponse {
    fn from(usuario: Usuario) -> Self {
        Self {
            nombre: usuario.nombre,
        }
    }
}
```

Si faltan datos o existen reglas que pueden fallar, se puede implementar
`TryFrom` en lugar de `From`.

## Relación con gRPC

En una aplicación gRPC, bibliotecas como `tonic` suelen generar structs Rust a
partir de archivos `.proto`. Es habitual tener dos representaciones distintas:

```text
Modelo de dominio de la aplicación
            ↓ From / TryFrom
Mensaje generado desde .proto
```

Por ejemplo, `From` puede transformar un modelo interno en una respuesta gRPC
cuando todos los datos están disponibles. `TryFrom` es apropiado cuando el
mensaje gRPC puede contener datos inválidos, campos ausentes o valores que
necesitan validación.

Estos traits no generan una integración gRPC por sí solos. Solo organizan y
expresan de forma clara las conversiones entre los tipos de la aplicación y los
tipos usados por la comunicación.

## Resumen

```text
From<A> for B       → conversión infalible de A a B
Into<B>             → método derivado de From
TryFrom<A> for B    → conversión de A a B que puede fallar
TryInto<B>          → método derivado de TryFrom
as                  → conversión directa; usar con cuidado
```

La regla práctica es: implementa `From` cuando la conversión siempre sea válida
y `TryFrom` cuando necesites validar o informar un error.
