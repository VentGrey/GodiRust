use std::io;

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
// * Cargar
// * Guardar
// * Buscar
// * Muestra (parámetro: int i)
// * LeerCadena (parametros: puntero char indic, s e int contador)
// * Borrar
// * Intro
// * Mostrar
// * Iniciar lista
// * buscar libre (parámetro: void)


// Función principal
fn main() {
    let opcion:i32;

    //TODO: Función iniciar Lista
    //TODO: Ciclo infinito
    //TODO: Asignar valor a una función (todavia no se como)
    //TODO: hacer un switch
    //TODO: Excepción en caso de que el switch no jale
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
    let c:i32 = s.parse().unwrap();

    loop {
        if c < 0 || c > 9 {  break;  }
    }
    return c;
}

fn LeerCadena(&indic:char, &s:char, cont:i32) -> (){

}


