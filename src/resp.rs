use std::fmt;

use crate::resp_results::{RESPError, RESPResult};

fn binary_extract_line(buffer: &[u8], index: &mut usize) -> RESPResult<Vec<u8>> {
    let mut output = Vec::new();

    if *index >= buffer.len(){
        return Err(RESPError::OutOfBounds(*index));
    }

    if buffer.len() - *index < 2 {
        *index = buffer.len();
        return Err(RESPError::OutOfBounds(*index));
    }

    let mut previous_elem: u8 = buffer[*index].clone();
    let mut separator_found: bool = false; 
    let mut final_index: usize = *index;

    for &elem in buffer[*index..].iter() {
        final_index += 1;

        if elem == b'\n' && previous_elem == b'\r' {
            separator_found = true;
            break;
        }
        previous_elem = elem.clone();
    }
    if !separator_found {
        *index = final_index;
        return Err(RESPError::OutOfBounds(*index));
    }

    output.extend_from_slice(&buffer[*index..final_index - 2]);
    *index = final_index;
    Ok(output)
}

pub fn binary_extract_line_as_string(buffer: &[u8], index: &mut usize) -> RESPResult<String> {
    let line = binary_extract_line(buffer, index)?;
    String::from_utf8(line).map_err(|_| RESPError::FromUtf8)
}

/// Parse a RESP array (e.g. *1\r\n$4\r\nPING\r\n) into Vec<String>.
pub fn parse_resp_array(buffer: &[u8], index: &mut usize) -> RESPResult<Vec<String>> {
    if *index >= buffer.len() {
        return Err(RESPError::OutOfBounds(*index));
    }
    if buffer[*index] != b'*' {
        return Err(RESPError::Unknown);
    }
    *index += 1;

    let count_str = binary_extract_line_as_string(buffer, index)?;
    let count: usize = count_str.parse()?;

    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        if *index >= buffer.len() {
            return Err(RESPError::OutOfBounds(*index));
        }
        if buffer[*index] != b'$' {
            return Err(RESPError::Unknown);
        }
        *index += 1;

        let len_str = binary_extract_line_as_string(buffer, index)?;
        let len: usize = len_str.parse()?;

        if buffer.len() - *index < len + 2 {
            return Err(RESPError::OutOfBounds(*index));
        }
        let data = &buffer[*index..*index + len];
        *index += len + 2; // skip \r\n

        result.push(String::from_utf8(data.to_vec()).map_err(|_| RESPError::FromUtf8)?);
    }
    Ok(result)
}

#[derive(Debug, PartialEq)]
pub enum RESP {
    BulkString(String),
    Null, 
    SimpleString(String),
}

impl fmt::Display for RESP {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = match self {
            Self::BulkString(data) => format!("${}\r\n{}\r\n", data.len(), data),
            Self::Null => String::from("$-1\r\n"),
            Self::SimpleString(data) => format!("+{}\r\n", data),
        };
        write!(f, "{}", data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_binary_extract_line_empty_buffer() {
        let buffer = "".as_bytes();
        let mut index: usize = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(index)) => assert_eq!(index, 0),
            _ => panic!(),
        }
    }

    #[test]
    fn test_binary_extract_line_single_character() {
        let buffer = "O".as_bytes();
        let mut index: usize = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(index)) => assert_eq!(index, 1),
            _ => panic!(),
        }
    }
    #[test]
    fn test_binary_extract_line_index_too_advanced() {
        let buffer = "OK".as_bytes();
        let mut index: usize = 1;

        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(idx)) => assert_eq!(idx, 2),
            _ => panic!(),
        }
    }
    #[test]
    fn test_binary_extract_line_no_separator() {
        let buffer = "OK".as_bytes();
        let mut index: usize = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(idx)) => assert_eq!(idx, 2),
            _ => panic!(),
        }
    }
    #[test]
    fn test_binary_extract_line_half_separator() {
        let buffer = "OK\r".as_bytes();
        let mut index: usize = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(idx)) => assert_eq!(idx, 3),
            _ => panic!(),
        }
    }
    #[test]
    fn test_binary_extract_line_incorrect_separator() {
        let buffer = "OK\n".as_bytes();
        let mut index: usize = 0;

        match binary_extract_line(buffer, &mut index) {
            Err(RESPError::OutOfBounds(idx)) => assert_eq!(idx, 3),
            _ => panic!(),
        }
    }
    #[test]
    fn test_binary_extract_line() {
        let buffer = "OK\r\n".as_bytes();
        let mut index: usize = 0;

        let output = binary_extract_line(buffer, &mut index).unwrap();

        assert_eq!(output, "OK".as_bytes());
        assert_eq!(index, 4);
    }

    #[test]
    fn test_binary_extract_line_as_string() {
        let buffer = "OK\r\n".as_bytes();
        let mut index: usize = 0;

        let output = binary_extract_line_as_string(buffer, &mut index).unwrap();

        assert_eq!(output, String::from("OK"));
        assert_eq!(index, 4);
    }
}