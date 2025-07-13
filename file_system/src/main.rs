use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::time::Instant;

const NOMBRE_ARCHIVO: &str = "prueba_rendimiento.dat";
const TAMANO_BLOQUE: usize = 1024; // 1 KB
const NUM_BLOQUES: usize = 10240; //10 MB

fn main() -> io::Result<()> {
    println!("Iniciando prueba de escritura en '{}'...", NOMBRE_ARCHIVO);
    let inicio_escritura = Instant::now();

    let archivo_escritura = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(NOMBRE_ARCHIVO)?;
    let mut writer = BufWriter::new(archivo_escritura);

    let buffer = vec![b'A'; TAMANO_BLOQUE];

    for _ in 0..NUM_BLOQUES {
        writer.write_all(&buffer)?;
    }

    writer.flush()?;

    let duracion_escritura = inicio_escritura.elapsed();
    println!(
        "Escritura completada en {:.2?}. Total escrito: {} KB.",
        duracion_escritura,
        (TAMANO_BLOQUE * NUM_BLOQUES) / 1024
    );

    println!("\nIniciando prueba de lectura de '{}'...", NOMBRE_ARCHIVO);
    let inicio_lectura = Instant::now();

    let archivo_lectura = File::open(NOMBRE_ARCHIVO)?;
    let mut reader = BufReader::new(archivo_lectura);
    let mut buffer_lectura = [0u8; TAMANO_BLOQUE];

    while reader.read(&mut buffer_lectura)? > 0 {}

    let duracion_lectura = inicio_lectura.elapsed();
    println!("Lectura completada en {:.2?}.", duracion_lectura);

    println!("\nPrograma finalizado con éxito.");
    Ok(())
}
