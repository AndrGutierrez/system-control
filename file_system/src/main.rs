use rand::Rng;
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::time::Instant;

const FILE_SIZE_MB: u64 = 100; // Tamaño del archivo en MB
const BUFFER_SIZE: usize = 8 * 1024; // 8KB buffer

fn main() -> io::Result<()> {
    let path = Path::new("testfile.bin");

    let write_start = Instant::now();
    create_and_write_file(path)?;
    println!("Escritura completada en: {:.2?}", write_start.elapsed());

    let read_start = Instant::now();
    read_file_sequential(path)?;
    println!(
        "Lectura secuencial completada en: {:.2?}",
        read_start.elapsed()
    );

    let random_start = Instant::now();
    random_access(path)?;
    println!(
        "Acceso aleatorio completado en: {:.2?}",
        random_start.elapsed()
    );

    Ok(())
}

fn create_and_write_file(path: &Path) -> io::Result<()> {
    let mut file = File::create(path)?;
    let mut rng = rand::thread_rng();
    let total_bytes = FILE_SIZE_MB * 1024 * 1024;
    let mut bytes_written = 0;

    while bytes_written < total_bytes {
        let remaining = (total_bytes - bytes_written) as usize;
        let chunk_size = BUFFER_SIZE.min(remaining);

        let mut buffer = vec![0u8; chunk_size];
        rng.fill(&mut buffer[..]);

        file.write_all(&buffer)?;
        bytes_written += chunk_size as u64;
    }
    file.sync_all()?;
    Ok(())
}

fn read_file_sequential(path: &Path) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = vec![0u8; BUFFER_SIZE];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        black_box(&buffer[..bytes_read]);
    }
    Ok(())
}

fn random_access(path: &Path) -> io::Result<()> {
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();
    let mut rng = rand::thread_rng();
    let mut buffer = [0u8; 512];

    for _ in 0..1000 {
        let offset = rng.gen_range(0..file_size - buffer.len() as u64);
        file.seek(SeekFrom::Start(offset))?;
        file.read_exact(&mut buffer)?;
        black_box(&buffer);
    }
    Ok(())
}

fn black_box(data: &[u8]) {
    unsafe {
        std::ptr::read_volatile(data.as_ptr());
    }
}
