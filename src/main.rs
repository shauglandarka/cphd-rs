use cphd_rs::{Cphd, CphdError, read_cphd, PvpSet};
//use std::path::Path;

fn main() -> Result<(), CphdError> {
    // Pass a path string, &Path, or PathBuf
    let file_path = "/data1/u/shaugland/CAPELLA_C13_SM_CPHD_HH_20260626121031_20260626121041.cphd";
    
    let cphd = read_cphd(file_path.as_ref())?;

    println!("Successfully loaded CPHD file version: {:?}", cphd.version);
    
    dbg!(&cphd.header);

//    let v1_meta = cphd.meta.get_v1_1_0_meta()
//           .expect("Failed to extract v1_1_0 metadata");
//    let pvp = v1_meta.pvp;
//    dbg!(&pvp);

    println!("First PVP set:");
    for (i, pvp_set) in cphd.pvp_iterator.enumerate().take(1) {
        let pvp_set = match pvp_set {
            PvpSet::V1_1_0(set) => set,
        };
    
        println!("PVP Set {}:", i + 1);
    
        // Required scalar fields
        println!("  Required Scalar Fields:");
        println!("    TxTime: {}", pvp_set.tx_time);
        println!("    RcvTime: {}", pvp_set.rcv_time);
        println!("    aFDOP: {}", pvp_set.a_fdop);
        println!("    aFRR1: {}", pvp_set.a_frr1);
        println!("    aFRR2: {}", pvp_set.a_frr2);
        println!("    FX1: {}", pvp_set.fx1);
        println!("    FX2: {}", pvp_set.fx2);
        println!("    TOA1: {}", pvp_set.toa1);
        println!("    TOA2: {}", pvp_set.toa2);
        println!("    TDTropoSRP: {}", pvp_set.td_tropo_srp);
        println!("    SC0: {}", pvp_set.sc0);
        println!("    SCSS: {}", pvp_set.scss);
    
        // XYZ vector fields
        println!("  XYZ Vector Fields:");
        println!("    TxPos: {:?}", pvp_set.tx_pos);
        println!("    TxVel: {:?}", pvp_set.tx_vel);
        println!("    RcvPos: {:?}", pvp_set.rcv_pos);
        println!("    RcvVel: {:?}", pvp_set.rcv_vel);
        println!("    SRPPos: {:?}", pvp_set.srp_pos);
    
        // Optional scalar fields
        println!("  Optional Scalar Fields:");
        println!("    AmpSF: {:?}", pvp_set.amp_sf);
        println!("    FXN1: {:?}", pvp_set.fxn1);
        println!("    FXN2: {:?}", pvp_set.fxn2);
        println!("    TOAE1: {:?}", pvp_set.toae1);
        println!("    TOAE2: {:?}", pvp_set.toae2);
        println!("    TDIonoSRP: {:?}", pvp_set.td_iono_srp);
    
        // Optional integer field
        println!("  Optional Integer Field:");
        println!("    SIGNAL: {:?}", pvp_set.signal);
    
        // Optional transmit antenna parameters
        println!("  Optional Transmit Antenna Parameters:");
        println!("    TxACX: {:?}", pvp_set.tx_acx);
        println!("    TxACY: {:?}", pvp_set.tx_acy);
        println!("    TxEB: {:?}", pvp_set.tx_eb);
    
        // Optional receive antenna parameters
        println!("  Optional Receive Antenna Parameters:");
        println!("    RcvACX: {:?}", pvp_set.rcv_acx);
        println!("    RcvACY: {:?}", pvp_set.rcv_acy);
        println!("    RcvEB: {:?}", pvp_set.rcv_eb);
    
        // Optional added PVP fields
        println!("  Optional Added PVP Fields:");
        println!("    AddedPVP: {:?}", pvp_set.added_pvp);
    
        println!("  ---");
    }

    Ok(())
}
