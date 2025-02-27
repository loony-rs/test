use std::fs::File;
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::Path;

fn read_file<P>(path: P) -> std::io::Result<String> 
where P: AsRef<Path>
{
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn create_file<P>(path: P, data: &Vec<u8>) -> std::io::Result<()> 
where P: AsRef<Path>
{
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

fn file_seek<P>(path: P, seek_from: u64, buffer: &mut [u8; 10]) -> io::Result<String> where P: AsRef<Path> {
    let mut file = File::open(path)?;
    file.seek(SeekFrom::Start(seek_from))?;
    file.read_exact(buffer)?;
    Ok(String::from_utf8_lossy(buffer).to_string())
}

#[cfg(test)]
mod test {
    
    use super::*;
    
    #[test]
    fn test_file_seek() {
        let mut buffer: [u8; 10] = [0; 10];
        let data = file_seek("./files/example.txt", 0, &mut buffer).unwrap();
        assert_eq!(data, String::from("What is Lo"));
    }
}