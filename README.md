# COMPENSATED PHASE HISTORY DATA (CPHD)

A rust crate for reading and writing NITF CPHD format. Inspired by SIX/Sarpy/Sarkit/MATLAB SAR toolbox.

CPHD product is an intermediate data product. The real utility is in the products and
measurements that may be derived from it. The quality of the phase history signal arrays
(bandwidth, dwell time, etc.), along with the set of metadata provided, are critical in
generating the derived products. The sensor independence of the CPHD product refers to the
ability of the allowed signal arrays and metadata options to accurately describe the signal
data from many sensors and data processing systems. Sensor independence does not mean
that all products have the same format for the signal data arrays or the same set of metadata
parameters.

This crate doesn't support all of the optional CPHD xml fields yet.

This crate has only been tested on Capella and Umbra single-channel CPHD 1.1.0


```rust
// Example usage 

use cphd_rs::{CphdError, read_cphd};

fn main() -> Result<(), CphdError> {
    //let file_path = "/data1/u/shaugland/CAPELLA_C13_SM_CPHD_HH_20260626121031_20260626121041.cphd";
    let file_path = "/data1/u/shaugland/2024-06-30-01-42-52_UMBRA-05_CPHD.cphd";

    // The main file reader
    let mut cphd = read_cphd(file_path.as_ref())?;

    // Header tells you where the different blocks start and how many bytes in each
    println!("Successfully loaded CPHD file version: {:?}", cphd.version);
    dbg!(&cphd.header);

    // This library parses the XML block into a structure
    let v1_meta = cphd
        .meta
        .get_v1_1_0_meta()
        .expect("Failed to extract v1_1_0 metadata");

    let _data = v1_meta.data; // Or any sub-field of the metadata

    // Optional parsing of the support arrays if they exist
    if let Some(ref support_arrays) = cphd.support_block {
        println!("Optional support arrays");
        for array in support_arrays {
            println!("=== Array: {} ===", array.identifier);
            // .take(2) yields the first two rows. 
            // Each item yielded is now directly an Array1<f64>
            for (row_idx, row_vector) in array.clone().take(2).enumerate() {
                println!("  Row {}: length {}", row_idx, row_vector.len());
                // Print the first few parsed f64 values directly from the ndarray
                let slice = row_vector.as_slice().unwrap();
                println!("    First few values: {:?}", &slice[..slice.len().min(3)]);
            }
        }
    }

    // The PVP block is treated as a vector of iterators. Each vector element
    // corresponds to a channel.
    println!("First PVP set for each channel:");
    for (_, pvp_iter) in cphd.pvp_iterators.iter_mut().enumerate() {
        // .take(3) yields the first three rows for each channel. 
        for (i, pvp_set) in pvp_iter.enumerate().take(3) {
            println!("  Set {}: tx_time = {}", i + 1, pvp_set.tx_time); // Grab whatever you want
            println!("  Set {}: rcv_time = {}", i + 1, pvp_set.rcv_time); // Grab whatever you want
        }
    }

    // The signal block is handled similarly, and the iterators return
    // complex-valued ndarray::Array1.
    println!("Phase of first three samples of first signal vector for each channel:");
    for (_, signal_iter) in cphd.signal_iterators.iter_mut().enumerate() { // for each channel
        for (_, signal) in signal_iter.enumerate().take(1) {               // Take the first signal vector
            for j in 0..3 {                                                // Take the first three samples
                println!("{}", signal[j].arg().to_degrees());
            }
        }
    }

    Ok(())
}
```
