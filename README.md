# COMPENSATED PHASE HISTORY DATA (CPHD)

A rust crate for reading and writing NITF CPHD format

CPHD product is an intermediate data product. The real utility is in the products and
measurements that may be derived from it. The quality of the phase history signal arrays
(bandwidth, dwell time, etc.), along with the set of metadata provided, are critical in
generating the derived products. The sensor independence of the CPHD product refers to the
ability of the allowed signal arrays and metadata options to accurately describe the signal
data from many sensors and data processing systems. Sensor independence does not mean
that all products have the same format for the signal data arrays or the same set of metadata
parameters.

This crate doesn't support all of the optional CPHD xml fields yet.

This crate has only been tested on Capella single-channel CPHD 1.1.0

Inspired by SIX/Sarpy/Sarkit/MATLAB SAR toolbox, etc. 


See examples/read\_cphd.rs for how to use
    


