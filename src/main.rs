use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

const MAX: usize = 50;

// Tipos de símbolo
const OPERANDO: u8 = 0;
const OPERADOR: u8 = 1;

// --- Tabla de símbolos (arreglos paralelos) ---
struct TablaSimbolos {
    simbolo: [char; MAX],
    tipo: [u8; MAX],
    precedencia: [i32; MAX],
    cantidad: usize,
}

impl TablaSimbolos {
    fn nueva() -> Self {
        Self {
            simbolo: ['\0'; MAX],
            tipo: [0; MAX],
            precedencia: [0; MAX],
            cantidad: 0,
        }
    }

    /// Agrega un símbolo a la tabla.
    fn agregar(&mut self, sim: char, tip: u8, prec: i32) {
        if self.cantidad >= MAX {
            eprintln!("Error: la tabla de símbolos esta llena.");
            process::exit(1);
        }
        self.simbolo[self.cantidad] = sim;
        self.tipo[self.cantidad] = tip;
        self.precedencia[self.cantidad] = prec;
        self.cantidad += 1;
    }

    /// Busca un símbolo y devuelve su índice, o None si no existe.
    fn buscar(&self, sim: char) -> Option<usize> {
        for i in 0..self.cantidad {
            if self.simbolo[i] == sim {
                return Some(i);
            }
        }
        None
    }
}

// --- Token individual ---
#[derive(Clone, Copy)]
struct Token {
    simbolo: char,
    tipo: u8,
    precedencia: i32,
}

impl Token {
    fn nuevo(simbolo: char, tipo: u8, precedencia: i32) -> Self {
        Self {
            simbolo,
            tipo,
            precedencia,
        }
    }
}

// --- Pila basada en arreglo ---
struct Pila {
    datos: [Token; MAX],
    tope: usize,
}

impl Pila {
    fn nueva() -> Self {
        Self {
            datos: [Token::nuevo('\0', 0, 0); MAX],
            tope: 0,
        }
    }

    fn vacia(&self) -> bool {
        self.tope == 0
    }

    fn meter(&mut self, token: Token) {
        self.datos[self.tope] = token;
        self.tope += 1;
    }

    fn sacar(&mut self) -> Token {
        self.tope -= 1;
        self.datos[self.tope]
    }

    fn ver_tope(&self) -> Token {
        self.datos[self.tope - 1]
    }
}

// --- Lectura del archivo ---
fn leer_archivo(ruta: &str) -> (String, TablaSimbolos) {
    let contenido = match fs::read_to_string(ruta) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: no se pudo leer el archivo '{}': {}", ruta, e);
            process::exit(1);
        }
    };

    let mut tabla = TablaSimbolos::nueva();
    let mut expresion = String::new();

    for linea in contenido.lines() {
        let linea = linea.trim();
        if linea.is_empty() {
            continue;
        }

        // Línea de expresión
        if let Some(expr) = linea.strip_prefix("EXPRESION=") {
            expresion = expr.to_string();
            continue;
        }

        // Línea de símbolo: simbolo,tipo,precedencia
        let partes: Vec<&str> = linea.split(',').collect();
        if partes.len() != 3 {
            eprintln!("Error: formato invalido en linea: '{}'", linea);
            process::exit(1);
        }

        let sim = match partes[0].chars().next() {
            Some(c) => c,
            None => {
                eprintln!("Error: simbolo vacío en linea: '{}'", linea);
                process::exit(1);
            }
        };

        let tip = match partes[1] {
            "OPERANDO" => OPERANDO,
            "OPERADOR" => OPERADOR,
            _ => {
                eprintln!(
                    "Error: tipo desconocido '{}' en linea: '{}'",
                    partes[1], linea
                );
                process::exit(1);
            }
        };

        let prec: i32 = match partes[2].parse() {
            Ok(p) => p,
            Err(_) => {
                eprintln!(
                    "Error: precedencia invalida '{}' en linea: '{}'",
                    partes[2], linea
                );
                process::exit(1);
            }
        };

        tabla.agregar(sim, tip, prec);
    }

    if expresion.is_empty() {
        eprintln!("Error: no se encontró la linea EXPRESION= en el archivo.");
        process::exit(1);
    }

    (expresion, tabla)
}

// --- Tokenizar la expresión usando la tabla de símbolos ---
fn tokenizar(expresion: &str, tabla: &TablaSimbolos) -> (Vec<Token>, usize) {
    let mut tokens = Vec::new();

    for c in expresion.chars() {
        match tabla.buscar(c) {
            Some(i) => tokens.push(Token::nuevo(c, tabla.tipo[i], tabla.precedencia[i])),
            None => {
                eprintln!("Error: simbolo '{}' no definido en la tabla.", c);
                process::exit(1);
            }
        }
    }

    let cantidad = tokens.len();
    (tokens, cantidad)
}

// --- Mostrar tabla de tokens ---
fn mostrar_tabla_tokens(tokens: &[Token], cantidad: usize) {
    println!("=== Tabla de tokens ===");
    println!("{:<8}{:<11}{}", "Token", "Tipo", "Precedencia");
    for i in 0..cantidad {
        let tipo_str = if tokens[i].tipo == OPERANDO {
            "OPERANDO"
        } else {
            "OPERADOR"
        };
        println!(
            "{:<8}{:<11}{}",
            tokens[i].simbolo, tipo_str, tokens[i].precedencia
        );
    }
    println!();
}

// --- Conversión infija → postfija ---
fn infija_a_postfija(tokens: &[Token], cantidad: usize) -> String {
    let mut salida = String::new();
    let mut pila = Pila::nueva();

    for i in 0..cantidad {
        let token = tokens[i];
        if token.tipo == OPERANDO {
            salida.push(token.simbolo);
        } else {
            // Mientras la pila no esté vacía y el tope tenga >= precedencia
            while !pila.vacia() && pila.ver_tope().precedencia >= token.precedencia {
                salida.push(pila.sacar().simbolo);
            }
            pila.meter(token);
        }
    }

    // Vaciar la pila
    while !pila.vacia() {
        salida.push(pila.sacar().simbolo);
    }

    salida
}

// --- Conversión infija → prefija ---
fn infija_a_prefija(tokens: &[Token], cantidad: usize) -> String {
    // 1. Invertir los tokens
    let mut invertidos: Vec<Token> = Vec::new();
    for i in (0..cantidad).rev() {
        invertidos.push(tokens[i]);
    }

    // 2. Aplicar algoritmo similar a postfija pero con precedencia estricta (>)
    let mut salida = String::new();
    let mut pila = Pila::nueva();
    let total = invertidos.len();

    for i in 0..total {
        let token = invertidos[i];
        if token.tipo == OPERANDO {
            salida.push(token.simbolo);
        } else {
            // En prefija: sacar solo si precedencia del tope es ESTRICTAMENTE mayor
            while !pila.vacia() && pila.ver_tope().precedencia > token.precedencia {
                salida.push(pila.sacar().simbolo);
            }
            pila.meter(token);
        }
    }

    while !pila.vacia() {
        salida.push(pila.sacar().simbolo);
    }

    // 3. Invertir el resultado
    salida.chars().rev().collect()
}

/// Busca archivos .txt en el directorio actual.
fn buscar_archivos_txt() -> Vec<String> {
    let mut archivos: Vec<String> = Vec::new();
    if let Ok(entradas) = fs::read_dir(".") {
        for entrada in entradas.flatten() {
            if let Some(nombre) = entrada.file_name().to_str() {
                if nombre.ends_with(".txt") {
                    archivos.push(nombre.to_string());
                }
            }
        }
    }
    archivos.sort();
    archivos
}

/// Determina qué archivo .txt usar: argumento de CLI, selección del usuario, o el único disponible.
fn obtener_archivo() -> String {
    let args: Vec<String> = env::args().collect();

    // Si se pasó un argumento, usarlo directamente
    if args.len() > 1 {
        let ruta = &args[1];
        if !ruta.ends_with(".txt") {
            eprintln!("Advertencia: el archivo '{}' no tiene extension .txt.", ruta);
        }
        return ruta.clone();
    }

    // Buscar archivos .txt en el directorio actual
    let archivos = buscar_archivos_txt();

    match archivos.len() {
        0 => {
            eprintln!("Error: no se encontraron archivos .txt en el directorio actual.");
            eprintln!("Uso: cargo run -- <archivo.txt>");
            process::exit(1);
        }
        1 => {
            println!("Archivo encontrado: {}", archivos[0]);
            println!();
            archivos[0].clone()
        }
        _ => {
            println!("Se encontraron varios archivos .txt:");
            println!();
            for (i, archivo) in archivos.iter().enumerate() {
                println!("  [{}] {}", i + 1, archivo);
            }
            println!();
            print!("Seleccione un archivo (1-{}): ", archivos.len());
            io::stdout().flush().unwrap();

            let mut entrada = String::new();
            io::stdin().read_line(&mut entrada).unwrap();

            let seleccion: usize = match entrada.trim().parse() {
                Ok(n) if n >= 1 && n <= archivos.len() => n,
                _ => {
                    eprintln!("Error: seleccion invalida.");
                    process::exit(1);
                }
            };

            println!();
            archivos[seleccion - 1].clone()
        }
    }
}

fn main() {
    let archivo = obtener_archivo();
    let (expresion, tabla) = leer_archivo(&archivo);

    let (tokens, cantidad) = tokenizar(&expresion, &tabla);

    mostrar_tabla_tokens(&tokens, cantidad);

    println!("Expresion infija:   {}", expresion);

    let postfija = infija_a_postfija(&tokens, cantidad);
    println!("Expresion postfija: {}", postfija);

    let prefija = infija_a_prefija(&tokens, cantidad);
    println!("Expresion prefija:  {}", prefija);
}

