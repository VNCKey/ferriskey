# 🧪 Code Lab: Sesión 02 - Conceptos Fundamentales

> **Entorno**: Code Lab interactivo integrado en FerrisKey ([`src/app/views/conceptos/mod.rs`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/conceptos/mod.rs))  
> **Herramientas activas**: Editor Syntect interactivo, Consola / Terminal embebida, Drawer de Retos Guiados y Evaluador Técnico.

---

## 🎯 Descripción del Entorno de Práctica

En el Code Lab de Conceptos trabajarás directamente en el archivo principal `src/main.rs` del proyecto activo. El drawer lateral derecho te guía a través de **8 retos progresivos**, cada uno con explicaciones teóricas, fragmentos de código reproducibles y tareas prácticas verificadas mediante compilación nativa en vivo (`Ctrl + Enter` o `F5`).

---

## 🏁 Ruta de los 8 Retos Prácticos

### 📌 Reto 1: Variables Paso a Paso (`let`, `let mut` y Shadowing)
- **Nivel**: Fundamental | **Tags**: Inmutabilidad & Shadowing
- **Código Base**:
  ```rust
  fn main() {
      // 1. Variable inmutable por defecto
      let valor_base = 100;
      println!("Valor inmutable base: {valor_base}");

      // 2. Variable mutable con `mut`
      let mut contador = 0;
      contador += 10;
      contador += 5;
      println!("Contador mutable final: {contador}");

      // 3. Shadowing: cambio de tipo reutilizando el nombre
      let entrada_usuario = "42";
      let entrada_usuario: i32 = entrada_usuario.parse().expect("Debe ser un número válido");
      println!("Número parseado tras shadowing: {}", entrada_usuario * 2);
  }
  ```
- **Tu Tarea**:
  1. Comprueba el error de compilación intentando reasignar `valor_base = 200;`.
  2. Convierte una cadena de texto `"  Ferris  "` en su longitud numérica usando *Shadowing* sin declarar variables con nombres temporales como `texto_limpio_len`.

---

### 📌 Reto 2: Macros y Formato de Salida
- **Nivel**: Fundamental | **Tags**: println! & format!
- **Código Base**:
  ```rust
  fn main() {
      let lenguaje = "Rust";
      let version = 2024;
      let pi = 3.14159265;

      // Interpolación directa y posicional
      println!("¡Bienvenido a {lenguaje} {version}!");
      println!("Posicional: {0} es rápido, {0} es seguro", lenguaje);

      // Especificadores de formato numérico
      println!("Pi con 2 decimales: {:.2}", pi);
      println!("Formato binario: {:b} | Hexadecimal: {:x}", 255, 255);

      // Construcción de Strings sin imprimir en consola
      let mensaje = format!("Registro exitoso para {}", lenguaje);
      println!("Mensaje generado con format!: {mensaje}");
  }
  ```
- **Tu Tarea**:
  1. Genera una cadena formateada con `format!` que represente una dirección IP `192.168.1.1` a partir de 4 variables de tipo `u8`.
  2. Imprime un error crítico simulado en la salida de errores con `eprintln!`.

---

### 📌 Reto 3: Bloques y Ámbitos de Memoria (Blocks & Scope)
- **Nivel**: Fundamental | **Tags**: Stack Frames & RAII
- **Código Base**:
  ```rust
  fn main() {
      let exterior = 10;

      {
          let interior = 20;
          println!("Dentro del bloque: exterior = {exterior}, interior = {interior}");
          
          // Shadowing local acotado al bloque
          let exterior = 999;
          println!("Exterior sombreado localmente dentro del bloque: {exterior}");
      } // 💥 `interior` es destruido aquí de la memoria del Stack

      println!("Fuera del bloque: exterior recupera su valor = {exterior}");
      // println!("{interior}"); // ❌ Descomentar provocaría error de compilación
  }
  ```
- **Tu Tarea**:
  1. Diseña un bloque anidado que declare variables temporales para calcular el promedio de tres notas y comprueba que ninguna variable auxiliar exista al finalizar el bloque.

---

### 📌 Reto 4: Sentencias y Expresiones (Statements vs. Expressions)
- **Nivel**: Fundamental | **Tags**: ; & Retorno Implícito
- **Código Base**:
  ```rust
  fn main() {
      // Un bloque evaluado como una EXPRESIÓN asignada a una variable
      let resultado_bloque = {
          let factor_a = 5;
          let factor_b = 8;
          factor_a * factor_b // 👈 Nota: SIN ';' para que sea el valor devuelto
      };

      println!("El resultado del bloque evaluado como expresión es: {resultado_bloque}");

      // Si añades un ';', la expresión se convierte en una sentencia y devuelve ()
      let resultado_vacio = {
          let _temp = 100;
          // Sentencia vacía implícita ()
      };
      println!("Tipo unit: {:?}", resultado_vacio);
  }
  ```
- **Tu Tarea**:
  1. Modifica la expresión del bloque para que retorne `true` si la multiplicación supera el valor de `30`, asignando el resultado directamente a una variable `es_mayor: bool`.

---

### 📌 Reto 5: Dominio de Tipos Primitivos Escalares (Data Types)
- **Nivel**: Fundamental | **Tags**: Enteros, Floats, Bool, Char
- **Código Base**:
  ```rust
  fn main() {
      // Enteros con notación legible
      let salario_anual: u32 = 45_000;
      let byte_mascara: u8 = 0b1111_0000;
      let codigo_hex: u32 = 0xDEAD_BEEF;

      // Punto flotante
      let temp_celsius: f32 = 24.5;
      let gravedad: f64 = 9.80665;

      // Booleanos y Caracteres Unicode (4 bytes)
      let activo: bool = true;
      let inicial: char = '🦀';

      // Casting explícito seguro con `as`
      let temp_redondeada: i32 = temp_celsius as i32;

      println!("Datos: Salario=${salario_anual}, Hex={codigo_hex:#x}, Temp={temp_redondeada}°C, Emoji={inicial}");
  }
  ```
- **Tu Tarea**:
  1. Comprueba el límite de `u8::MAX` (255) y observa la diferencia de comportamiento ante el desbordamiento ejecutando en modo normal (`cargo run`) vs release (`cargo run --release`).

---

### 📌 Reto 6: Comentarios y Documentación (`cargo doc`)
- **Nivel**: Fundamental | **Tags**: Docs & Markdown
- **Código Base**:
  ```rust
  // Comentario de una sola línea

  /*
     Comentario de bloque
     para explicaciones extensas.
  */

  /// Calcula el precio total aplicando un impuesto porcentual.
  ///
  /// # Ejemplos
  /// ```
  /// let final_price = calcular_total(100.0, 18.0);
  /// assert_eq!(final_price, 118.0);
  /// ```
  fn calcular_total(precio_base: f64, impuesto_porcentaje: f64) -> f64 {
      precio_base + (precio_base * (impuesto_porcentaje / 100.0))
  }

  fn main() {
      let total = calcular_total(250.0, 16.0);
      println!("Precio con impuestos: ${total:.2}");
  }
  ```
- **Tu Tarea**:
  1. Ejecuta en la terminal de FerrisKey el comando `cargo doc` para comprobar cómo el compilador genera un sitio web completo con la documentación de tu función y los ejemplos en Markdown.

---

### 📌 Reto 7: Funciones Idiomáticas (Functions)
- **Nivel**: Fundamental | **Tags**: fn, Parámetros y Retorno
- **Código Base**:
  ```rust
  fn es_par(numero: i32) -> bool {
      numero % 2 == 0 // Retorno implícito de expresión booleana
  }

  fn dividir(dividendo: f64, divisor: f64) -> f64 {
      if divisor == 0.0 {
          println!("Advertencia: división por cero");
          return 0.0; // Retorno anticipado explícito
      }
      dividendo / divisor
  }

  fn main() {
      let n = 42;
      println!("¿El número {n} es par?: {}", es_par(n));
      println!("División: {}", dividir(100.0, 4.0));
  }
  ```
- **Tu Tarea**:
  1. Escribe una función `convertir_celsius_a_fahrenheit(celsius: f64) -> f64` utilizando exclusivamente la expresión de retorno implícita: $(C \times 9/5) + 32$.

---

### 📌 Reto 8: Evaluación Técnica Interactiva (Questions)
- **Nivel**: Autoevaluación | **Tags**: Quiz de 15 Preguntas
- En este último paso del drawer de FerrisKey, pondrás a prueba tu comprensión a través de 15 preguntas técnicas sobre:
  1. La inmutabilidad de `let` y cuándo usar `mut`.
  2. La diferencia física entre `const` y `static`.
  3. Las reglas de `shadowing` respecto a la mutabilidad.
  4. La diferencia entre sentencias y expresiones.
  5. Tamaños en bytes de tipos escalares (especialmente `char` de 4 bytes).
  6. Comportamiento del desbordamiento en debug vs release.
  7. Formateo de cadenas y retornos de funciones.
