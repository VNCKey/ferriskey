pub const PLAYGROUND_CODE: &str = r#"fn main() {
    println!("¡Hola desde el Editor de Egui!");
}
"#;

pub const DATATYPES_CODE: &str = r#"// Tipos Compuestos en Rust: Arrays, Slices y Tuplas
fn main() {
    let numeros: [i32; 5] = [10, 20, 30, 40, 50];
    let slice_nums: &[i32] = &numeros[1..4];
    let tupla: (&str, u8) = ("Rust", 10);

    println!("Array: {:?}", numeros);
    println!("Slice: {:?}", slice_nums);
    println!("Tupla: {}, {}", tupla.0, tupla.1);
}
"#;

pub const PLAYGROUND_NUBE_CODE: &str = r#"use std::collections::HashMap;
// ¡Importamos serde sin haberlo configurado en Cargo.toml!
// (Esto compila en la nube del Rust Playground oficial)

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Yellow", 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}"#;

pub const CONTROL_FLUJO_CODE: &str = r#"fn main() {
    let numero = 7;

    // 'if' usado como expresión
    let estado = if numero % 2 == 0 { "par" } else { "impar" };
    println!("El número {} es {}", numero, estado);

    // Bucle 'for' sobre un rango inclusivo
    print!("Conteo: ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();
}
"#;

pub const OWNERSHIP_CODE: &str = r#"fn main() {
    // &str: vista (a menudo al binario o prestada)
    let saludo: &str = "Hola";

    // String: dueño en el Heap (puede crecer)
    let mut s1 = String::from("Rust");
    s1.push_str(" Ownership");

    // MOVE: s1 deja de ser válido
    let s2 = s1;
    // println!("{}", s1); // error: value moved

    // BORROW: prestamos sin regalar el dueño
    imprimir_len(&s2);
    println!("saludo={}, s2={}", saludo, s2);
}

fn imprimir_len(texto: &str) {
    println!("len = {}", texto.len());
}
"#;

pub const STRUCTS_CODE: &str = r#"struct ServidorWeb {
    puerto: u16,
    host: String,
    activo: bool,
}

impl ServidorWeb {
    fn new(puerto: u16, host: &str) -> Self {
        Self {
            puerto,
            host: host.to_string(),
            activo: false,
        }
    }

    fn iniciar(&mut self) {
        self.activo = true;
        println!("Servidor iniciado en http://{}:{}", self.host, self.puerto);
    }
}

fn main() {
    let mut mi_servidor = ServidorWeb::new(8080, "127.0.0.1");
    mi_servidor.iniciar();
}
"#;

pub const ENUMS_CODE: &str = r#"enum EstadoPedido {
    Pendiente,
    Enviado { guia: String },
    Entregado,
}

fn procesar(estado: EstadoPedido) {
    match estado {
        EstadoPedido::Pendiente => println!("⏳ El pedido está pendiente"),
        EstadoPedido::Enviado { guia } => println!("🚚 En camino. Guía: {}", guia),
        EstadoPedido::Entregado => println!("✅ Pedido entregado"),
    }
}

fn main() {
    let pedido = EstadoPedido::Enviado { guia: "RUST-9921".to_string() };
    procesar(pedido);
}
"#;

pub const COLLECTIONS_CODE: &str = r#"use std::collections::HashMap;

fn main() {
    // Vector dinámico
    let mut numeros = vec![10, 20, 30];
    numeros.push(40);
    println!("Vector: {:?}, len: {}, cap: {}", numeros, numeros.len(), numeros.capacity());

    // HashMap
    let mut puntajes = HashMap::new();
    puntajes.insert("Rustaceans", 100);
    puntajes.insert("Gophers", 85);

    for (equipo, puntos) in &puntajes {
        println!("Equipo {}: {} pts", equipo, puntos);
    }
}
"#;

pub const ERRORS_CODE: &str = r#"fn dividir(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("No se puede dividir entre cero".to_string())
    } else {
        Ok(a / b)
    }
}

fn calcular() -> Result<f64, String> {
    let res1 = dividir(100.0, 2.0)?;
    let res2 = dividir(res1, 5.0)?;
    Ok(res2)
}

fn main() {
    match calcular() {
        Ok(val) => println!("Resultado exitoso: {}", val),
        Err(err) => println!("Error en cálculo: {}", err),
    }
}
"#;

pub const TRAITS_CODE: &str = r#"trait Dibujable {
    fn dibujar(&self);
}

struct Circulo { radio: f64 }
struct Rectangulo { ancho: f64, alto: f64 }

impl Dibujable for Circulo {
    fn dibujar(&self) {
        println!("🔴 Dibujando círculo de radio {}", self.radio);
    }
}

impl Dibujable for Rectangulo {
    fn dibujar(&self) {
        println!("🟦 Dibujando rectángulo {}x{}", self.ancho, self.alto);
    }
}

fn renderizar(item: &impl Dibujable) {
    item.dibujar();
}

fn main() {
    let c = Circulo { radio: 5.0 };
    let r = Rectangulo { ancho: 10.0, alto: 4.0 };
    renderizar(&c);
    renderizar(&r);
}
"#;

pub const GENERICS_CODE: &str = r#"fn identidad<T>(valor: T) -> T {
    valor
}

fn main() {
    let n = identidad(42);
    let s = identidad("Hola Rust");
    println!("n: {}, s: {}", n, s);
}
"#;

pub const FUNCTIONS_CODE: &str = r#"fn main() {
    let a = 15;
    let b = 25;

    // Llamada + retorno implícito (sin ';')
    let suma = calcular_suma(a, b);
    println!("La suma de {} + {} es: {}", a, b, suma);

    // Paso por referencia mutable
    let mut contador = 0;
    incrementar(&mut contador);
    println!("Contador incrementado: {}", contador);

    // Closure: captura el entorno
    let factor = 3;
    let multiplicar = |x: i32| x * factor;
    println!("10 * factor = {}", multiplicar(10));
}

fn calcular_suma(x: i32, y: i32) -> i32 {
    x + y // última expresión = retorno
}

fn incrementar(val: &mut i32) {
    *val += 1;
}
"#;

pub const ITERATORS_CODE: &str = r#"fn main() {
    let numeros = vec![1, 2, 3, 4, 5, 6];

    println!("--- 1. Iterar por referencia (&T) con .iter() ---");
    for n in numeros.iter() {
        print!("{} ", n);
    }
    println!();

    println!("--- 2. Pipeline Lazy: filter -> map -> collect ---");
    let pares_cuadrados: Vec<i32> = numeros
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();

    println!("Pares al cuadrado: {:?}", pares_cuadrados);
}
"#;

pub const ARRAYS_CODE: &str = r#"fn main() {
    let arr: [i32; 5] = [100, -500, 2048, 42, 0];
    println!("Arreglo completo: {:?}", arr);
    println!("Longitud (arr.len()): {}", arr.len());
    println!("Tamaño en Stack: {} bytes", std::mem::size_of_val(&arr));

    for (idx, elem) in arr.iter().enumerate() {
        println!("arr[{}] = {}", idx, elem);
    }
}
"#;

pub const SLICES_CODE: &str = r#"fn main() {
    let arreglo = [10, 20, 30, 40, 50, 60];
    let slice: &[i32] = &arreglo[1..4];

    println!("Array base: {:?}", arreglo);
    println!("Slice &arreglo[1..4]: {:?}", slice);
    println!("Longitud del slice: {}", slice.len());
    println!("Primer elemento del slice: {:?}", slice.first());
}
"#;

pub const TUPLES_CODE: &str = r#"fn main() {
    let mi_tupla: (i32, bool, f64) = (100, true, 3.1415);

    println!("Tupla completa: {:?}", mi_tupla);
    println!("Campos: .0 = {}, .1 = {}, .2 = {}", mi_tupla.0, mi_tupla.1, mi_tupla.2);

    let (entero, booleano, flotante) = mi_tupla;
    println!("Desestructurado: {}, {}, {}", entero, booleano, flotante);
}
"#;

pub const MODULES_CODE: &str = r#"// Módulos en Rust
mod redes {
    pub fn conectar() {
        println!("Conectado a la red!");
    }
}

fn main() {
    redes::conectar();
}
"#;
