use std::collections::HashMap;
use std::str;
use std::io::{self, Error, ErrorKind};
use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileHeader {
    // CPHD version (e.g., "1.0")
    pub version: String,                     
    // Size of the XML block in bytes
    pub xml_block_size: u64,                
    // Offset to the XML block
    pub xml_block_byte_offset: u64,        
    // Optional: Size of the Support block
    pub support_block_size: Option<u64>,     
    // Optional: Offset to the Support block
    pub support_block_byte_offset: Option<u64>,
    // Size of the PVP block in bytes
    pub pvp_block_size: u64,                
    // Offset to the PVP block
    pub pvp_block_byte_offset: u64,        
    // Size of the Signal block in bytes
    pub signal_block_size: u64,            
    // Offset to the Signal block
    pub signal_block_byte_offset: u64,     
    // Product classification (default: "UNCLASSIFIED")
    pub classification: String,            
    // Product release info (default: "UNRESTRICTED")
    pub release_info: String,             
    // Additional optional KVPs
    pub kvp_metadata: Option<HashMap<String, String>>,
}

impl Display for FileHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out_str = String::default();
        out_str += format!("{}, ", self.version).as_ref();
        write!(f, "CPHD Header: [{out_str}]")
    }
}

pub fn parse_file_header(mmap: &[u8]) -> io::Result<FileHeader> {
    // Slice the first 1024 bytes based on the file layout offset
    let header_slice = mmap.get(..1024)
        .ok_or_else(|| Error::new(ErrorKind::UnexpectedEof, "File too short for header"))?;
    
    let raw_str = str::from_utf8(header_slice)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    // Trim trailing null bytes (\0), form feeds (\x0c), and whitespace padding
    let header_str = raw_str.trim_end_matches(['\0', '\x0c', ' ', '\n', '\r']);


    let mut version = String::new();
    let mut xml_block_size = 0;
    let mut xml_block_byte_offset = 0;
    let mut pvp_block_size = 0;
    let mut pvp_block_byte_offset = 0;
    let mut signal_block_size = 0;
    let mut signal_block_byte_offset = 0;
    let mut classification = String::from("UNCLASSIFIED");
    let mut release_info = String::from("UNRESTRICTED");
    let mut kvp_metadata = HashMap::new();

    let mut lines = header_str.lines();

    // First line contains the version string (e.g., "CPHD/1.1.0")
    if let Some(first_line) = lines.next() {
        version = first_line.trim().to_string();
    }

    // Parse subsequent lines
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some((key, value)) = line.split_once(":=") {
            let key = key.trim();
            let value = value.trim();

            match key {
                "XML_BLOCK_SIZE" => xml_block_size = value.parse().unwrap_or_default(),
                "XML_BLOCK_BYTE_OFFSET" => xml_block_byte_offset = value.parse().unwrap_or_default(),
                "PVP_BLOCK_SIZE" => pvp_block_size = value.parse().unwrap_or_default(),
                "PVP_BLOCK_BYTE_OFFSET" => pvp_block_byte_offset = value.parse().unwrap_or_default(),
                "SIGNAL_BLOCK_SIZE" => signal_block_size = value.parse().unwrap_or_default(),
                "SIGNAL_BLOCK_BYTE_OFFSET" => signal_block_byte_offset = value.parse().unwrap_or_default(),
                "CLASSIFICATION" => classification = value.to_string(),
                "RELEASE_INFO" => release_info = value.to_string(),
                other => {
                    kvp_metadata.insert(other.to_string(), value.to_string());
                }
            }
        }
    }

    let kvp_metadata_opt = if kvp_metadata.is_empty() {
        None
    } else {
       Some(kvp_metadata)
    };

    Ok(FileHeader {
        version,
        xml_block_size,
        xml_block_byte_offset,
        support_block_size: None,
        support_block_byte_offset: None,
        pvp_block_size,
        pvp_block_byte_offset,
        signal_block_size,
        signal_block_byte_offset,
        classification,
        release_info,
        kvp_metadata: kvp_metadata_opt,
    })
}
