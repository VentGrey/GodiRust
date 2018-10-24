pub fn guardar() -> std::io::Result<()> {
    let mut archivo = File::create("clientes.grs")?;
    //file.write_all(b)?;
    Ok(())
}

pub fn cargar() {
    let mut archivo = File::open("clientes.grs")?;
    let mut contenido = String::new();
    file.read_to_string(&mut contenido)?;
    Ok(())
}
