# 🧪 Code Lab: Sesión 05 - Tipos Compuestos y Colecciones

> **Entorno**: Code Lab interactivo integrado en FerrisKey ([`src/app/views/tipos_compuestos/mod.rs`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/tipos_compuestos/mod.rs))  
> **Herramientas activas**: Editor interactivo Syntect, Terminal embebida, Simulador interactivo de vectores y Drawer de Retos.

---

## 🎯 Descripción del Entorno de Práctica

En este Code Lab operarás con tipos de datos compuestos en el Stack (Arrays y Tuplas) y colecciones dinámicas en el Heap (`Vec` y `HashMap`). Aprenderás a navegar colecciones sin copias de memoria mediante Slices prestados y a aplicar patrones de desestructuración (*destructuring*).

---

## 🏁 Ruta de los 5 Retos Prácticos

### 📌 Reto 1: Arreglos de Tamaño Fijo (Array)
- **Nivel**: Fundamental | **Tags**: [T; N] & Destructuring
- **Código Base**:
  ```rust
  fn main() {
      // 1. Declaración con tipo y tamaño explícito
      let mut notas: [u8; 4] = [10, 12, 15, 18];

      // 2. Sintaxis de repetición
      let ceros = [0; 4];

      // 3. Consulta e indexación
      let segunda = notas[1];
      println!("Segunda nota: {segunda}");

      // 4. Modificación de posición mutable
      notas[0] = 20;

      // 5. Destructuring Pattern con `_` y `..`
      let [primera, _, .., ultima] = notas;
      println!("Primera: {primera}, Última: {ultima}");
  }
  ```
- **Tu Tarea**:
  1. Experimenta qué ocurre si intentas consultar `notas[4]`. Observa el mensaje de pánico seguro de Rust por *index out of bounds*.

---

### 📌 Reto 2: Tuplas Heterogéneas (Tuples)
- **Nivel**: Fundamental | **Tags**: Heterogéneo & .0
- **Código Base**:
  ```rust
  fn main() {
      // Tupla con 3 tipos dispares: &str, i32, bool
      let datos = ("Alicia", 26, true);

      // Acceso directo con punto
      let nombre = datos.0;
      let edad = datos.1;
      println!("Usuario: {nombre}, Edad: {edad}");

      // Tupla mutable
      let mut punto = (10, 20);
      punto.0 = 15;
      println!("Punto modificado: ({}, {})", punto.0, punto.1);

      // Desestructuración completa y parcial
      let (nombre, edad, activo) = datos;
      let (primero, .., ultimo) = (10, 20, 30, 40);
      println!("Primero: {primero}, Último: {ultimo}");
  }
  ```
- **Tu Tarea**:
  1. Diseña una función `calcular_min_max(numeros: &[i32]) -> (i32, i32)` que devuelva una tupla con el valor mínimo y máximo de una secuencia.

---

### 📌 Reto 3: Colecciones Dinámicas (Vec y HashMap)
- **Nivel**: Intermedio | **Tags**: Vec<T>, HashMap & Heap
- **Código Base**:
  ```rust
  use std::collections::HashMap;

  fn main() {
      // Vector dinámico
      let mut niveles = vec![1, 2, 4];
      niveles.push(8);
      niveles.insert(1, 3);
      niveles.remove(0);
      println!("Vector niveles: {:?}", niveles);

      // Tabla Hash
      let mut puntos = HashMap::new();
      puntos.insert("Rust", 100);
      puntos.insert("Python", 80);

      let existe = puntos.contains_key("Rust");
      println!("¿Existe Rust?: {existe}");

      if let Some(valor) = puntos.get("Rust") {
          println!("Puntaje obtenido: {valor}");
      }
  }
  ```
- **Tu Tarea**:
  1. Consulta la pestaña teórica del simulador de vectores y observa cómo cambia `cap` conforme añades elementos con `push`.

---

### 📌 Reto 4: Slices y Vistas Prestadas (Slices)
- **Nivel**: Intermedio | **Tags**: &[T], &mut [T], &str
- **Código Base**:
  ```rust
  fn main() {
      // Slice de un Array inmutable
      let datos = [10, 20, 30, 40, 50];
      let vista: &[i32] = &datos[1..4]; // [20, 30, 40]
      println!("Slice inmutable: {:?}", vista);

      // Slice mutable
      let mut valores = [10, 20, 30, 40];
      let parte: &mut [i32] = &mut valores[1..3];
      parte[0] = 99; // Modifica el array original en el índice 1
      println!("Valores tras modificar slice: {:?}", valores);

      // String Slice (&str)
      let texto = String::from("Lenguaje Rust");
      let palabra: &str = &texto[0..8];
      println!("Slice de texto: '{palabra}'");
  }
  ```
- **Tu Tarea**:
  1. Toma un slice completo con `&datos[..]` y pásalo a una función que sume sus elementos con un bucle.

---

### 📌 Reto 5: Evaluación Técnica (Questions)
- **Nivel**: Autoevaluación | **Tags**: 10 Preguntas
- Cuestionario de 10 preguntas interactivas sobre:
  1. Diferencias de memoria entre `[T; N]` y `Vec<T>`.
  2. Indexación en tuplas mediante `.0`.
  3. Comportamiento del desbordamiento de índices en arreglos.
  4. La duplicación geométrica de capacidad en `Vec`.
  5. Semántica de préstamos en slices inmutables y mutables.
