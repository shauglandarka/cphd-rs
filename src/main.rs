use cphd_rs::{Cphd, CphdError, read_cphd};
use std::path::Path;

fn main() -> Result<(), CphdError> {
    // Pass a path string, &Path, or PathBuf
    let file_path = "/home/samhaug/dl/CAPELLA_C03_SM_CPHD_HH_20211229053627_20211229053631.cphd";
    
    let cphd = read_cphd(file_path.as_ref())?;

    println!("Successfully loaded CPHD file version: {:?}", cphd.version);
    
    dbg!(&cphd.header);



    Ok(())
}
