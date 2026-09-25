# 🧪 Code Lab: Sesión 03 - Memoria y Ownership

> **Entorno**: Code Lab interactivo integrado en FerrisKey ([`src/app/views/memoria/mod.rs`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/memoria/mod.rs))  
> **Herramientas activas**: Editor interactivo Syntect, Terminal embebida, Visualizador de Memoria y Drawer de Retos.

---

## 🎯 Descripción del Entorno de Práctica

En este Code Lab experimentarás de primera mano las reglas del **Borrow Checker**. Cada ejercicio está diseñado para compilar en vivo y demostrar la diferencia física entre copias en el Stack, movimientos de propiedad en el Heap y exclusión mutua de referencias.

---

## 🏁 Ruta de los 5 Retos Prácticos

### 📌 Reto 1: Stack & Copy Semantics
- **Nivel**: Fundamental | **Tags**: Stack & Trait Copy
- **Código Base**:
  ```rust
  fn main() {
      // Tipos primitivos enteros viven 100% en el Stack
      let x = 42;
      let y = x; // Copy automático bit a bit

      println!("Valor original de x: {x}");
      println!("Copia independiente en y: {y}");

      // Modificar y no altera x
      let y = y + 10;
      println!("Tras modificar y: x={x}, y={y}");
  }
  ```
- **Tu Tarea**:
  1. Comprueba que las tuplas fijas compuestas de escalares (ej. `let punto = (10, 20);`) también implementan `Copy`.
  2. Verifica que ambas variables permanecen utilizables sin restricciones.

---

### 📌 Reto 2: String, Heap & Move Semantics
- **Nivel**: Fundamental | **Tags**: Heap & Move
- **Código Base**:
  ```rust
  fn main() {
      // String aloja un buffer dinámico en el Heap
      let mut texto = String::from("Rust");
      texto.push_str(" en el Heap");

      println!("Texto antes del move: {texto}");

      // Transferencia de Ownership (Move Semantics)
      let movido = texto; 
      
      println!("El nuevo propietario es: {movido}");

      // ❌ Intenta descomentar la línea inferior para experimentar el Borrow Checker:
      // println!("Intentando leer texto original: {texto}");
  }
  ```
- **Tu Tarea**:
  1. Descomenta la línea de impresión de `texto` y observa el mensaje de diagnóstico del compilador: `borrow of moved value: texto`.
  2. Soluciona el error utilizando el método `.clone()` para duplicar explícitamente los datos en el Heap y compara el coste de memoria.

---

### 📌 Reto 3: Reglas de Ownership y Ciclo de Vida (Drop)
- **Nivel**: Fundamental | **Tags**: Ownership & RAII
- **Código Base**:
  ```rust
  fn tomar_propiedad(cadena: String) {
      println!("La función recibió la propiedad de: {cadena}");
  } // 💥 Aquí `cadena` sale del ámbito y se libera su memoria en el Heap

  fn main() {
      let original = String::from("Ferris");

      // Al pasar la variable como argumento sin referencia, se transfiere la propiedad
      tomar_propiedad(original);

      // println!("{original}"); // ❌ ERROR: `original` ya no es dueño de nada
  }
  ```
- **Tu Tarea**:
  1. Modifica la función `tomar_propiedad` para que reciba una referencia `&String` en lugar del valor completo, permitiendo que `main` siga usando `original` tras la invocación.

---

### 📌 Reto 4: Reglas de Borrowing y Exclusividad
- **Nivel**: Fundamental | **Tags**: &T vs &mut T & NLL
- **Código Base**:
  ```rust
  fn main() {
      let mut datos = String::from("Concurrencia");

      // 1. Múltiples referencias inmutables simultáneas permitidas
      {
          let ref_lectura1 = &datos;
          let ref_lectura2 = &datos;
          println!("Lecturas compartidas: {ref_lectura1} y {ref_lectura2}");
      } // Los préstamos inmutables finalizan aquí

      // 2. Exactamente UNA referencia mutable exclusiva
      let ref_escritura = &mut datos;
      ref_escritura.push_str(" Segura");
      println!("Dato modificado exclusivamente: {ref_escritura}");
  }
  ```
- **Tu Tarea**:
  1. Intenta crear una referencia inmutable `let r = &datos;` mientras `ref_escritura` sigue activa y analiza cómo el compilador impide posibles carreras de datos.

---

### 📌 Reto 5: Dominio de String frente a `&str` (Slices)
- **Nivel**: Intermedio | **Tags**: Deref Coercion & Slices
- **Código Base**:
  ```rust
  // Función idiomática que acepta cualquier vista de texto UTF-8
  fn procesar_texto(slice: &str) {
      println!("Vista: '{}' | Longitud en bytes: {}", slice, slice.len());
  }

  fn main() {
      let texto_heap: String = String::from("FerrisKey Desktop App");
      let texto_literal: &str = "Hola desde .rodata";

      // 1. Pasar un literal directo (&str)
      procesar_texto(texto_literal);

      // 2. Pasar un String completo mediante Deref Coercion (&String -> &str)
      procesar_texto(&texto_heap);

      // 3. Pasar una rodaja parcial (Slice)
      let palabra = &texto_heap[0..9]; // "FerrisKey"
      procesar_texto(palabra);
  }
  ```
- **Tu Tarea**:
  1. Construye una función `primera_palabra(s: &str) -> &str` que busque el primer espacio en blanco y retorne únicamente el slice de la primera palabra sin realizar asignaciones en el Heap.
