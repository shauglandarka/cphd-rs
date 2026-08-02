use cphd_rs::dep::v1_1_0::file_hdr;
use cphd_rs::parse_xml_block;

use memmap2::Mmap;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    // Memory-map the file
    let file =
        File::open("/home/samhaug/dl/CAPELLA_C03_SM_CPHD_HH_20211229053627_20211229053631.cphd")?;
    let mmap = unsafe { Mmap::map(&file)? };

    //    let header_bytes = &mmap[..12240];
    //    println!("Header bytes: {:?}", &header_bytes[12200..12240]);
    //    let header_str = std::str::from_utf8(header_bytes);
    //    println!("header as utf 8: {:?}", header_str);

    // Parse the File Header
    let header = file_hdr::parse_file_header(&mmap)?;

    println!("File Header:");
    println!("  Version: {}", header.version);
    println!("  XML Block Size: {} bytes", header.xml_block_size);
    println!("  XML Block Offset: {} bytes", header.xml_block_byte_offset);
    println!(
        "  Support Block Size: {:?} bytes",
        header.support_block_size
    );
    println!(
        "  Support Block Offset: {:?} bytes",
        header.support_block_byte_offset
    );
    println!("  PVP Block Size: {} bytes", header.pvp_block_size);
    println!("  PVP Block Offset: {} bytes", header.pvp_block_byte_offset);
    println!("  Signal Block Size: {} bytes", header.signal_block_size);
    println!(
        "  Signal Block Offset: {} bytes",
        header.signal_block_byte_offset
    );
    println!("  Classification: {}", header.classification);
    println!("  Release Info: {}", header.release_info);
    println!("  Additional KVPs: {:?}", header.kvp_metadata);

    let xml_str = parse_xml_block(&mmap, &header)?;
    dbg!(&xml_str);

    Ok(())
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
