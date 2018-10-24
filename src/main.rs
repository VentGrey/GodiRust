use std::io;
use std::process;
use std::fs::File;
use std::io::prelude::*;

// Externas a la Biblioteca standard

struct cliente {
    ID: String,
    nombre: String,
    telefono: String,
    dia: u64,
    mes: u64,
    anio: u64,
}

// Estas son las funciones básicas para que jale el programa
// Funciones tipo void:
// * Buscar
// * Muestra (parámetro: int i)
// * LeerCadena (parametros: puntero char indic, s e int contador)
// * Borrar
// * Intro
// * Mostrar
// * buscar libre (parámetro: void)

//Funcion acerca de:
fn about() {
    println!("GodiRust vX.X.X - Pre-pre Alpha");
    println!("Licencia: GPL-2");
    println!("Usando Rust Stable");
    println!("Bibliotecas externas usadas:");

    main();
}

// Función principal
fn main() {
    let mut opcion: i32;

    iniciar_lista();
    loop {
        opcion = menu();

        match opcion {
            1 => intro(),
            2 => borrar(),
            3 => mostrar(),
            4 => buscar(),
            5 => guardar(),
            6 => cargar(),
            7 => ordenar(),
            8 => process::exit(1),
            9 => about(),
            _ => panic!("¡ERROR, VOY A EXPLOTAR!"),
        }
    }
}

fn iniciar_lista() -> () {
    /* Sobres, vamos a calar los iteradores de Rust aquí */
    //TODO: Iterador sobre un arreglo de estructuras
}

fn menu() -> i32 {
    let mut s = String::new();

    println!("1- Introducir un cliente");
    println!("2- Eliminar un cliente");
    println!("3- Listar los clientes");
    println!("4- Buscar dentro del archivo");
    println!("5- Guardar archivo");
    println!("6- Cargar archivo");
    println!("7- Ordenar archivo por nombre");
    println!("8- Salir");
    println!("9- Acerca de:");
    println!("Introduzca la opción deseada");

    io::stdin().read_line(&mut s).unwrap();
    let c: i32 = s.parse().unwrap();

    loop {
        if c < 0 || c > 9 {
            break;
        }
    }
    return c;
}

fn guardar() -> std::io::Result<()> {
    let mut archivo = File::create("clientes.grs")?;
    //file.write_all(b)?;
    Ok(())
}

fn cargar() -> std::io::Result<()> {
    let mut archivo = File::open("clientes.grs")?;
    let mut contenido = String::new();
    file.read_to_string(&mut contenido)?;
    Ok(())
}
