use cphd_rs::{Cphd, CphdError, read_cphd};

fn main() -> Result<(), CphdError> {
    // Pass a path string, &Path, or PathBuf
    let file_path = "/data1/u/shaugland/CAPELLA_C13_SM_CPHD_HH_20260626121031_20260626121041.cphd";
    
    // The main file reader
    let mut cphd = read_cphd(file_path.as_ref())?;

    println!("Successfully loaded CPHD file version: {:?}", cphd.version);
    dbg!(&cphd.header);

    // This library parses the XML block into a structure
    let v1_meta = cphd.meta.get_v1_1_0_meta()
           .expect("Failed to extract v1_1_0 metadata");
    let data = v1_meta.data;

    dbg!(&data);

    // No test for the opptional support block

    // The PVP block is treated as a vector of iterators. Each vector element
    // corresponds to a channel.
    println!("First PVP set for each channel:");
    for (ch_idx, pvp_iter) in cphd.pvp_iterators.iter_mut().enumerate() {
        for (i, pvp_set) in pvp_iter.enumerate().take(3) {
           println!("  Set {}: tx_time = {}", i + 1, pvp_set.tx_time); // adjust field name if nested
        }
    }

    // The signal block is handled similarly, and the iterators return
    // complex-valued ndarray::Array1.
    println!("First data set for each channel:");
    for (ch_idx, signal_iter) in cphd.signal_iterators.iter_mut().enumerate() {
        for (i, signal) in signal_iter.enumerate().take(1) {    
            println!("Len samples: {}", signal.len()); 
            for j in (0..10) {
                println!("{}", signal[j].arg().to_degrees()); 
            }
        }
    }

    Ok(())

}
