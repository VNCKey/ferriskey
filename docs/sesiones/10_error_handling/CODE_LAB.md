# 🧪 Code Lab: Sesión 10 - Manejo Idiomático de Errores

> **Entorno**: Code Lab interactivo integrado en FerrisKey ([`src/app/views/enums/mod.rs`](file:///home/alek/VNC/repos/egui_vnc/src/app/views/enums/mod.rs))  
> **Herramientas activas**: Editor interactivo Syntect, Terminal embebida y Drawer de Retos.

---

## 🎯 Descripción del Entorno de Práctica

En este Code Lab transformarás operaciones fallibles y valores ausentes sin recurrir a excepciones ni `null`. Aprenderás a encadenar combinadores en `Option` y a propagar errores limpiamente con `Result` y el operador `?`.

---

## 🏁 Ruta de Ejercicios Prácticos

### 📌 Ejercicio 1: Combinadores en `Option<T>`
- **Código Base**:
  ```rust
  fn main() {
      let config_usuario: Option<String> = None;

      // 1. Proporcionar valor por defecto con unwrap_or
      let tema = config_usuario.clone().unwrap_or(String::from("Oscuro"));
      println!("Tema activo: {tema}");

      // 2. Cálculo diferido por defecto con unwrap_or_else
      let puerto = None::<u16>.unwrap_or_else(|| 8080);
      println!("Puerto de escucha: {puerto}");

      // 3. Transformación funcional con map
      let usuario: Option<String> = Some(String::from("Ferris"));
      let longitud = usuario.map(|nombre| nombre.len());
      println!("Longitud del nombre: {:?}", longitud);
  }
  ```
- **Tu Tarea**:
  1. Diseña una función `obtener_extension(archivo: &str) -> Option<&str>` que retorne la extensión tras el punto o `None` si el archivo no tiene extensión.

---

### 📌 Ejercicio 2: Propagación de Errores con `Result` y el Operador `?`
- **Código Base**:
  ```rust
  use std::num::ParseIntError;

  fn sumar_textos_numericos(a: &str, b: &str) -> Result<i32, ParseIntError> {
      let num_a = a.parse::<i32>()?; // Si falla, retorna Err inmediatamente
      let num_b = b.parse::<i32>()?;
      Ok(num_a + num_b)
  }

  fn main() {
      match sumar_textos_numericos("15", "25") {
          Ok(total) => println!("Suma exitosa: {total}"),
          Err(error) => println!("Fallo al parsear: {error}"),
      }

      match sumar_textos_numericos("15", "abc") {
          Ok(total) => println!("Suma exitosa: {total}"),
          Err(error) => println!("Fallo controlado esperado: {error}"),
      }
  }
  ```
- **Tu Tarea**:
  1. Convierte una función que lee tres números de una cadena separada por comas `"10,20,30"` y retorna el producto multiplicativo de los tres utilizando `?`.
