# 📖 Teoría: Sesión 09 - Tipos Personalizados (Structs & Enums)

---

## 1. Estructuras (`struct`)

Las estructuras son tipos compuestos personalizados que permiten agrupar datos bajo un nombre significativo.

### A. Structs Clásicos con Campos Nombrados
```rust
struct Usuario {
    nombre: String,
    activo: bool,
    visitas: u32,
}

let u = Usuario {
    nombre: String::from("Ana"),
    activo: true,
    visitas: 1,
};
```
*Shorthand syntax:* Si la variable y el campo tienen el mismo nombre, se escribe `Usuario { nombre, activo, visitas }`.

### B. Tuple Structs (El Patrón Newtype)
Tienen campos anónimos accesibles por índice (`.0`, `.1`):
```rust
struct Color(u8, u8, u8);
struct Metros(f64); // Newtype: previene sumar Metros con Segundos por error en compilación
```

### C. Unit-like Structs
No contienen campos y ocupan 0 bytes en memoria (*Zero-Sized Types* / ZST). Ideales para implementar traits o actuar como testigos de estado:
```rust
struct SoloLectura;
```

---

## 2. Métodos y Funciones Asociadas (`impl`)

El bloque `impl` inyecta comportamiento a la estructura:

```rust
struct Rectangulo {
    ancho: u32,
    alto: u32,
}

impl Rectangulo {
    // 1. Función Asociada (Constructor): no recibe `self`
    pub fn new(ancho: u32, alto: u32) -> Self {
        Self { ancho, alto }
    }

    // 2. Método de solo lectura: recibe `&self`
    pub fn area(&self) -> u32 {
        self.ancho * self.alto
    }

    // 3. Método mutable: recibe `&mut self`
    pub fn escalar(&mut self, factor: u32) {
        self.ancho *= factor;
        self.alto *= factor;
    }
}
```

---

## 3. Enumeraciones Algebraicas (`enum`)

Los enums en Rust son tipos suma (*Tagged Unions*) de máxima potencia: cada variante puede almacenar tipos y estructuras de datos completamente dispares:

```rust
enum MensajeRed {
    Desconectar,                         // Sin datos (unitario)
    Mover { x: i32, y: i32 },            // Campos nombrados
    Escribir(String),                    // Con un String asociado
    CambiarColor(u8, u8, u8),            // Tres valores u8 (tupla)
}
```

### Representación en Memoria
En hardware, un enum se almacena como:
1. Una **etiqueta (*Discriminant Tag*)**: un entero pequeño que indica qué variante está activa.
2. Un bloque contiguo de memoria con el tamaño de la **variante más pesada**, más cualquier *padding* necesario para alineación de la CPU.

---

## 4. Desestructuración de Enums con `match` e `if let`

```rust
let msg = MensajeRed::Mover { x: 10, y: 20 };

match msg {
    MensajeRed::Desconectar => println!("Cerrando conexión"),
    MensajeRed::Mover { x, y } => println!("Mover a ({x}, {y})"),
    MensajeRed::Escribir(texto) => println!("Texto: {texto}"),
    MensajeRed::CambiarColor(r, g, b) => println!("RGB: {r}, {g}, {b}"),
}

// Azúcar sintáctico con `if let` cuando solo interesa una única variante:
if let MensajeRed::Mover { x, y } = msg {
    println!("Destino: x={x}, y={y}");
}
```
