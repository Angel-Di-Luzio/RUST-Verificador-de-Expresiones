# Conversor de Expresiones en Rust

Programa de consola escrito en Rust que lee una expresión matemática infija y una tabla de símbolos desde un archivo, realiza la tokenización y la traduce a notación **postfija** (polaca inversa) y **prefija** (polaca).

## Funcionamiento

1. El programa lee un archivo de entrada (por defecto `entrada.txt`).
2. Extrae la expresión infija a partir de la línea `EXPRESION=...` y registra los operandos y operadores con sus respectivas precedencias en una tabla de símbolos.
3. Valida y tokeniza cada carácter de la expresión.
4. Realiza la conversión de la expresión utilizando pilas (stacks) locales implementadas con arreglos de tamaño estático.
5. Muestra en consola la tabla de tokens clasificados, la expresión original, y los resultados convertidos.

## Requisitos

### Lenguaje
- **Rust** (Edición 2024 o superior) y su compilador/gestor de paquetes **Cargo**.
  (Puedes instalarlo desde: https://www.rust-lang.org/tools/install)

### Estructura de Entrada (`entrada.txt`)

Crea un archivo llamado `entrada.txt` en la raíz del proyecto con el siguiente formato:
```
EXPRESION=A+B*C
A,OPERANDO,0
B,OPERANDO,0
C,OPERANDO,0
+,OPERADOR,1
-,OPERADOR,1
*,OPERADOR,2
/,OPERADOR,2
```
*   La línea `EXPRESION=` define la fórmula matemática a traducir.
*   Las líneas siguientes asocian cada carácter a su tipo (`OPERANDO` u `OPERADOR`) y su nivel de precedencia matemática (número entero).

## Uso

Para compilar y ejecutar el programa, abre la consola en la carpeta del proyecto y ejecuta:
```bash
cargo run
```
