pub mod dep;
use memmap2::Mmap;
use serde::Deserialize;
use std::sync::Arc;
use thiserror::Error;

use crate::dep::v1_1_0;
use crate::dep::v1_1_0::data::SignalArrayFormat;

use quick_xml::DeError;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::str::{Utf8Error, from_utf8};

use ndarray::Array1;
use num_complex::Complex;

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
    pub support_block: Option<Vec<u8>>, // not tested
    pub pvp_iterators: Vec<v1_1_0::pvp::PvpIterator>,
    pub signal_iterators: Vec<SignalIterator>,
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

        let xml_slice = mmap_arc
            .get(offset..offset + size)
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
                    let support_slice = mmap_arc
                        .get(support_offset..support_offset + support_size)
                        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;
                    Some(support_slice.to_vec())
                } else {
                    None
                }
            }
        };

        let global_pvp_offset = header.pvp_block_byte_offset() as usize;

        let pvp_iterators = match version {
            CphdVersion::V1_1_0 => {
                let v1_meta = match &meta {
                    CphdMeta::V1_1_0(m) => m,
                };

                v1_meta
                    .data
                    .channel
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

        let global_signal_offset = header.signal_block_byte_offset() as usize;

        let signal_iterators = match version {
            CphdVersion::V1_1_0 => {
                let v1_meta = match &meta {
                    CphdMeta::V1_1_0(m) => m,
                };

                v1_meta
                    .data
                    .channel
                    .iter()
                    .map(|ch| {
                        SignalIterator::new(
                            mmap_arc.clone(),
                            v1_meta.data.signal_array_format,
                            global_signal_offset + (ch.signal_array_byte_offset as usize),
                            ch.signal_array_byte_offset as usize,
                            ch.num_vectors as usize,
                            ch.num_samples as usize,
                        )
                    })
                    .collect()
            }
        };

        Ok(Cphd {
            header,
            version,
            meta,
            mmap: mmap_arc,
            support_block,
            pvp_iterators,
            signal_iterators,
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
                "SUPPORT_BLOCK_SIZE" => {
                    support_block_size = Some(value.parse().unwrap_or_default())
                }
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
    } else if version.contains("2.0.0") {
        // Future version placeholder
        Err(CphdError::Unimpl(version))
    } else {
        Err(CphdError::VersionError(version))
    }
}

#[derive(Debug)]
pub struct SignalIterator {
    mmap: Arc<Mmap>,
    signal_block_offset: usize,
    channel_offset: usize,
    num_vectors: usize,
    num_samples: usize,
    current_vector: usize,
    signal_format: SignalArrayFormat,
}

impl SignalIterator {
    pub fn new(
        mmap: Arc<Mmap>,
        signal_format: SignalArrayFormat,
        signal_block_offset: usize,
        channel_offset: usize,
        num_vectors: usize,
        num_samples: usize,
    ) -> Self {
        Self {
            mmap,
            signal_block_offset,
            channel_offset,
            num_vectors,
            num_samples,
            current_vector: 0,
            signal_format,
        }
    }
}

impl Iterator for SignalIterator {
    type Item = Array1<Complex<f32>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_vector >= self.num_vectors {
            return None;
        }

        let bytes_per_sample = match self.signal_format {
            SignalArrayFormat::CI2 => 2,
            SignalArrayFormat::CI4 => 4,
            SignalArrayFormat::CF8 => 8,
        };

        let bytes_per_vector = self.num_samples * bytes_per_sample;

        // Use the absolute start offset + stride for the current vector
        let absolute_offset = self.signal_block_offset
            + self.channel_offset
            + (self.current_vector * bytes_per_vector);

        let vector_slice = self
            .mmap
            .get(absolute_offset..absolute_offset + bytes_per_vector)?;

        let mut samples = Vec::with_capacity(self.num_samples);

        match self.signal_format {
            SignalArrayFormat::CI2 => {
                for chunk in vector_slice.chunks_exact(2) {
                    let real = chunk[0] as i8 as f32;
                    let imag = chunk[1] as i8 as f32;
                    samples.push(num_complex::Complex::new(real, imag));
                }
            }
            SignalArrayFormat::CI4 => {
                for chunk in vector_slice.chunks_exact(4) {
                    let real = i16::from_be_bytes([chunk[0], chunk[1]]) as f32;
                    let imag = i16::from_be_bytes([chunk[2], chunk[3]]) as f32;
                    samples.push(num_complex::Complex::new(real, imag));
                }
            }
            SignalArrayFormat::CF8 => {
                for chunk in vector_slice.chunks_exact(8) {
                    let real = f32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                    let imag = f32::from_be_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
                    samples.push(num_complex::Complex::new(real, imag));
                }
            }
        }
        self.current_vector += 1;
        Some(ndarray::Array1::from(samples))
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
