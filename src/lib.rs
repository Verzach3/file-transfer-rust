use std::fs::File;
use std::io::{BufWriter, BufReader, Write, Read};
use std::net::{TcpStream, TcpListener};

const BUFFER_SIZE: usize = 1024;

pub fn client(path: &str, ip: &str) -> std::io::Result<()>{
  let mut file = BufWriter::new(File::create(path)?);

  let mut stream = TcpStream::connect(ip)?;

  let mut buffer = [0u8; BUFFER_SIZE];

  loop {
      let bytes_read = stream.read(&mut buffer)?;
      if bytes_read == 0 {
        break; // File end
      }
      file.write_all(&buffer[..bytes_read])?
  }

  Ok(())
}

pub fn server(path: &str, ip: &str) -> std::io::Result<()>{
   let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let listener = TcpListener::bind(ip)?;
    for stream in listener.incoming() {
        let mut stream = stream?;

        let mut buffer = [0; BUFFER_SIZE];
        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break; // Se alcanzó el final del archivo
            }
            stream.write_all(&buffer[..bytes_read])?;
        }
    }
    Ok(())
}