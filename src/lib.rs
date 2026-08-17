pub mod dep;
use serde::{Deserialize};
use thiserror::Error;
use memmap2::Mmap;
use std::sync::Arc;

use crate::dep::v1_1_0;

use quick_xml::DeError;
use std::io::{Error, ErrorKind};
use std::collections::HashMap;
use std::str::{Utf8Error, from_utf8};
use std::fs::File;
use std::path::Path;
use std::fmt::Display;

use byteorder::{LittleEndian, BigEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Error, Debug)]
pub enum CphdError {
    #[error("unknown cphd version {0}")]
    VersionError(String),
    #[error("metadata for version {0} is not implemented")]
    Unimpl(String),
    #[error("file does not appear to be a CPHD")]
    NotASidd,
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    UTF8(#[from] Utf8Error),
    #[error(transparent)]
    DESER(#[from] DeError),
}

pub type Result<T> = std::result::Result<T, CphdError>;

pub fn read_cphd(path: &Path) -> Result<Cphd> {
    let file = File::open(path)?;
    Cphd::from_file(file)
}

#[derive(Debug)]
pub struct Cphd {
    pub header: CphdHeader,
    pub version: CphdVersion,
    pub meta: CphdMeta,
    pub mmap: Arc<Mmap>,
    pub support_block: Option<Vec<u8>>, // not implemented yet
    pub pvp_iterators: Vec<v1_1_0::pvp::PvpIterator>,
}

impl Cphd {
    pub fn from_file(file: File) -> Result<Self> {
        let mmap = unsafe { Mmap::map(&file)? };
        let mmap_arc = Arc::new(mmap);

        let header = parse_file_header(&mmap_arc)?;

        let version_str = match &header {
            CphdHeader::V1_1_0(h) => &h.version,
        };

        let version = match version_str.as_str() {
            "CPHD/1.1.0" | "1.1.0" => CphdVersion::V1_1_0,
            other => return Err(CphdError::VersionError(other.to_string())),
        };
        
        let offset = header.xml_block_byte_offset() as usize;
        let size = header.xml_block_size() as usize;

        let xml_slice = mmap_arc.get(offset..offset + size)
            .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;
        let xml_str = from_utf8(xml_slice)?;

        let meta = match version {
            CphdVersion::V1_1_0 => {
                let parsed_meta: v1_1_0::CphdMeta = quick_xml::de::from_str(xml_str)?;
                CphdMeta::V1_1_0(parsed_meta)
            }
        };
  
        // Optional support block
        let support_block = match &header {
            CphdHeader::V1_1_0(h) => {
                if let Some(support_block_size) = h.support_block_size {
                    let support_offset = h.support_block_byte_offset.unwrap() as usize;
                    let support_size = support_block_size as usize;
                    let support_slice = mmap_arc.get(support_offset..support_offset + support_size)
                        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;
                    Some(support_slice.to_vec())

                } else {
                    None
                }
            }
        };
        let global_pvp_offset = header.pvp_block_byte_offset() as usize;
        let pvp_size = header.pvp_block_size() as usize;

        let pvp_iterators = match version {
            CphdVersion::V1_1_0 => {
                let v1_meta = match &meta {
                    CphdMeta::V1_1_0(m) => m,
                    _ => panic!("Metadata version mismatch"),
                };

                v1_meta.data.channel
                    .iter()
                    .map(|ch| {
                       v1_1_0::pvp::PvpIterator::new(
                           mmap_arc.clone(),
                           &v1_meta.pvp,
                           global_pvp_offset + (ch.pvp_array_byte_offset as usize),
                           ch.num_vectors as usize,
                           v1_meta.data.num_bytes_pvp as usize,
                       )
                })
                .collect()
            }
        };



//        let signal_offset = header.signal_block_size() as usize;
//        let signal_size =  header.signal_block_byte_offset() as usize;

        Ok(Cphd {
               header,
               version,
               meta,
               mmap: mmap_arc,
               support_block,
               pvp_iterators,
        })
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum CphdVersion {
    #[serde(rename = "1.1.0")]
    V1_1_0,
    //#[serde(rename = "1.2.0")]
    //V1_2_0,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum CphdMeta {
    V1_1_0(v1_1_0::CphdMeta),
    //V1_2_0(v1_1_0::CphdMeta),
}

impl CphdMeta {
    pub fn get_v1_1_0_meta(self) -> Option<v1_1_0::CphdMeta> {
        match self {
            Self::V1_1_0(meta) => Some(meta),
        }
    }
}
 
#[derive(Debug, PartialEq, Deserialize)]
pub enum CphdHeader {
    V1_1_0(v1_1_0::CphdHeader),
}

impl CphdHeader {
    pub fn xml_block_byte_offset(&self) -> u64 {
        match self {
            CphdHeader::V1_1_0(h) => h.xml_block_byte_offset,
            // CphdHeader::V1_2_0(h) => h.xml_block_byte_offset, // Future versions
        }
    }

    pub fn xml_block_size(&self) -> u64 {
        match self {
            CphdHeader::V1_1_0(h) => h.xml_block_size,
            // CphdHeader::V1_2_0(h) => h.xml_block_size, // Future versions
        }
    }

    pub fn pvp_block_byte_offset(&self) -> u64 {
        match self {
            CphdHeader::V1_1_0(h) => h.pvp_block_byte_offset,
            // CphdHeader::V1_2_0(h) => h.xml_block_byte_offset, // Future versions
        }
    }

    pub fn pvp_block_size(&self) -> u64 {
        match self {
            CphdHeader::V1_1_0(h) => h.pvp_block_size,
            // CphdHeader::V1_2_0(h) => h.xml_block_size, // Future versions
        }
    }

    pub fn signal_block_byte_offset(&self) -> u64 {
        match self {
            CphdHeader::V1_1_0(h) => h.signal_block_byte_offset,
            // CphdHeader::V1_2_0(h) => h.xml_block_byte_offset, // Future versions
        }
    }

    pub fn signal_block_size(&self) -> u64 {
        match self {
            CphdHeader::V1_1_0(h) => h.signal_block_size,
            // CphdHeader::V1_2_0(h) => h.xml_block_size, // Future versions
        }
    }

}

impl Display for CphdHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let meta = match self {
            CphdHeader::V1_1_0(meta) => meta,
            // CphdHeader::V1_2_0(h) => h.xml_block_byte_offset, // Future versions
        };
        let mut out_str = String::default();
        out_str += format!("{}, ", meta.version).as_ref();
        write!(f, "CPHD Header: [{out_str}]")
    }
}

pub fn parse_file_header(mmap: &[u8]) -> Result<CphdHeader> {
    // Slice the first 1024 bytes based on the file layout offset
    let header_slice = mmap
        .get(..1024)
        .ok_or_else(|| Error::new(ErrorKind::UnexpectedEof, "File too short for header"))?;

    let raw_str =
        str::from_utf8(header_slice).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;

    // Trim trailing null bytes (\0), form feeds (\x0c), and whitespace padding
    let header_str = raw_str.trim_end_matches(['\0', '\x0c', ' ', '\n', '\r']);

    let mut version = String::new();
    let mut xml_block_size = 0;
    let mut xml_block_byte_offset = 0;
    let mut support_block_size: Option<u64> = None;
    let mut support_block_byte_offset: Option<u64> = None;
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
                "XML_BLOCK_BYTE_OFFSET" => {
                    xml_block_byte_offset = value.parse().unwrap_or_default()
                }
                "SUPPORT_BLOCK_SIZE" => support_block_size = Some(value.parse().unwrap_or_default()),
                "SUPPORT_BLOCK_BYTE_OFFSET" => {
                    support_block_byte_offset = Some(value.parse().unwrap_or_default())
                }
                "PVP_BLOCK_SIZE" => pvp_block_size = value.parse().unwrap_or_default(),
                "PVP_BLOCK_BYTE_OFFSET" => {
                    pvp_block_byte_offset = value.parse().unwrap_or_default()
                }
                "SIGNAL_BLOCK_SIZE" => signal_block_size = value.parse().unwrap_or_default(),
                "SIGNAL_BLOCK_BYTE_OFFSET" => {
                    signal_block_byte_offset = value.parse().unwrap_or_default()
                }
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

    if version.contains("1.1.0") {
        let inner_header = v1_1_0::CphdHeader {
            version, // moved here safely
            xml_block_size,
            xml_block_byte_offset,
            support_block_size,
            support_block_byte_offset,
            pvp_block_size,
            pvp_block_byte_offset,
            signal_block_size,
            signal_block_byte_offset,
            classification,
            release_info,
            kvp_metadata: kvp_metadata_opt,
        };
        Ok(CphdHeader::V1_1_0(inner_header))
    } else if version.contains("2.0.0") { // Future version placeholder
        Err(CphdError::Unimpl(version))
    } else {
        Err(CphdError::VersionError(version))
    }

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
