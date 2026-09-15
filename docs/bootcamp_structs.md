# 📘 Bootcamp de Arquitectura en Rust

Este documento resume los conceptos aprendidos durante el entrenamiento de estructuración de datos, modelado de comportamiento y organización de proyectos.

---

## 1. Structs y Privacidad
* **Struct:** Agrupación de datos. Por defecto, en Rust todos los campos son privados.
* **Privacidad de Módulos (`pub`):** La palabra reservada `pub` no hace que algo sea público para "cualquier clase" como en Java, sino que lo hace público **fuera de su archivo (módulo)**. Dentro del mismo archivo, el código puede acceder a campos privados.
* **Field Init Shorthand:** Si inicializas un struct y la variable local se llama igual que el campo, puedes omitir la repetición: `Jugador { nombre, nivel: 1 }` en lugar de `nombre: nombre`.

## 2. Comportamiento (`impl`)
Separa los datos de la lógica. Los métodos pueden tener 4 niveles de permisos respecto a la instancia actual:
1. **Sin `self` (Función Asociada):** Como `Jugador::new()`. No necesita una instancia para existir.
2. **`&self` (Mirar sin tocar):** Solo lectura.
3. **`&mut self` (Permiso de mutación):** Permite alterar los datos del struct.
4. **`self` (Ownership / Consumo):** Destruye el struct, moviendo su propiedad a la función.

> **Tip Idiomático (`Self`):** Usar `Self` (con mayúscula) dentro de un `impl` es un alias para el nombre del struct. Facilita refactorizaciones futuras.

## 3. Destructuring y Ownership
* **Destructuring:** Desarmar un struct usando llaves. *Regla: Se desarma con la misma forma con la que se arma.*
  * Ejemplo: `let Jugador { nombre, nivel, .. } = jugador;`
* **Cuidado con el Move:** Al desarmar tipos complejos (como `String`), estos se **mueven** a la nueva variable, dejando al struct original inutilizable. La solución es desarmar una referencia (`&jugador`) o clonar.

## 4. Polimorfismo y Contratos (Traits)
* Los **Traits** son contratos de comportamiento.
* Si defines `trait Atacable { fn recibir_damage(&mut self, cantidad: u32); }`, obligas a que cualquier struct que firme el contrato implemente esa lógica.
* Se implementan en un bloque separado: `impl Atacable for Jugador { ... }`.

## 5. Módulos y Organización
* Cada archivo en Rust es un módulo cerrado.
* Para conectar archivos, el archivo principal (`main.rs`) debe declarar la existencia del módulo con `pub mod nombre_archivo;`.
* Luego, se importan las herramientas con `use nombre_archivo::{Herramienta1, Herramienta2};`.

## 6. Enums con Súper-poderes y Match
* A diferencia de otros lenguajes, los Enums en Rust pueden almacenar datos dentro de sus variantes (Ej: `Asesino(String)`).
* La herramienta definitiva para evaluarlos es el bloque **`match`**, el cual obliga exhaustividad (tienes que cubrir todas las opciones o usar el comodín `_`).

## 7. Macros y Ecosistema (Serde)
* **Macros Procedurales (`#[derive(...)]`):** Son atributos que funcionan como pequeños programas que escriben código de forma automática durante la compilación.
* **Serde:** La librería estándar de facto para convertir structs a texto (JSON, YAML, etc.) y viceversa. Requiere activar el *feature* `derive` en Cargo.
* **Unit Type `()`:** El tipo de retorno de las funciones vacías (el `void` de Rust).
