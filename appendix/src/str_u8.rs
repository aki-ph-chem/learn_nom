use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    // &str -> &[u8]
    let s = "abc_def";
    let _s_as_bytes = s.as_bytes();

    // &[u8] -> &str
    let bytes:[u8; 7] = [b'a', b'b', b'c', b'_', b'd', b'e', b'f'];
    let bytes_as_str = std::str::from_utf8(&bytes)?;
    println!("bytes_as_str = {}", bytes_as_str);

    // &[u8] -> &str
    let bytes = b"abc_def";
    let bytes_as_str = std::str::from_utf8(bytes)?;
    println!("bytes_as_str = {}", bytes_as_str);

    // Vec<u8> -> &str
    let byte_vector = vec![b'a', b'b', b'c', b'_', b'd', b'e', b'f'];
    let byte_vector_as_str = std::str::from_utf8(&byte_vector)?;
    println!("byte_vector_as_str = {}", byte_vector_as_str);

    Ok(())
}
