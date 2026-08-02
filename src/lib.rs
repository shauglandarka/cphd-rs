pub mod dep;

use crate::dep::v1_1_0::file_hdr::FileHeader;

use std::io::{self, Error, ErrorKind};

pub fn parse_xml_block(mmap: &[u8], header: &FileHeader) -> io::Result<String> {
    let start = header.xml_block_byte_offset as usize;
    let size = header.xml_block_size as usize;
    let end = start
        .checked_add(size)
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "XML block offset/size overflow"))?;

    // Slice the exact bytes for the XML block from the memory map
    let xml_slice = mmap
        .get(start..end)
        .ok_or_else(|| Error::new(ErrorKind::UnexpectedEof, "File too short for XML block"))?;

    // Convert bytes to a UTF-8 string
    let raw_xml =
        std::str::from_utf8(xml_slice).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    // Trim the section terminator (\x0cf\n or \f\n) and padding
    let xml_instance = raw_xml.trim_end_matches(['\x0c', '\n', '\r', ' ']);

    Ok(xml_instance.to_string())
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn it_works() {
//        let result = add(2, 2);
//        assert_eq!(result, 4);
//    }
//}
