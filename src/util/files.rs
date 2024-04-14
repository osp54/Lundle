use encoding_rs::Encoding;
use std::io::Read;

pub fn read_file_in_defined_encoding(path: &str) -> Result<String, std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let first_line = buffer.split(|&c| c == b'\n' || c == b'\r').next().unwrap();

    if first_line.starts_with(b"--#") {
        let encoding = Encoding::for_label(&first_line[3..]).expect("Invalid encoding");
        let (string, _, _) = encoding.decode(&buffer);
        Ok(string.to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "Invalid file encoding. Define encoding with --#<encoding>",
        ))
    }
}

pub fn read_file_in_encoding(path: &str, encoding: &'static encoding_rs::Encoding) -> Result<String, std::io::Error> {
    if encoding == encoding_rs::UTF_8 {
        return std::fs::read_to_string(path);
    }
    
    let file = std::fs::read(path)?;
    let (string, _, _) = encoding.decode(&file);

    Ok(string.to_string())
}

pub fn write_file_in_encoding(path: &str, content: &str, encoding: &'static encoding_rs::Encoding) -> Result<(), std::io::Error> {
    let (bytes, _, _) = encoding.encode(content);
    std::fs::write(path, bytes)?;

    Ok(())
}