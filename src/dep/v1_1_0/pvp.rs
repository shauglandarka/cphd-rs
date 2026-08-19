use serde::{Deserialize, Serialize};
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use crate::Result;
use memmap2::Mmap;
use std::sync::Arc;


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PvpField {
    #[serde(rename = "Offset")]
    pub offset: u32,
    #[serde(rename = "Size")]
    pub size: u32,
    #[serde(rename = "Format")]
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AddedPvpField {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Offset")]
    pub offset: u32,
    #[serde(rename = "Size")]
    pub size: u32,
    #[serde(rename = "Format")]
    pub format: String,
}

// The PxP xml string in the CPHD header
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Pvp {
    #[serde(rename = "TxTime")]
    pub tx_time: PvpField,
    #[serde(rename = "TxPos")]
    pub tx_pos: PvpField,
    #[serde(rename = "TxVel")]
    pub tx_vel: PvpField,

    #[serde(rename = "RcvTime")]
    pub rcv_time: PvpField,
    #[serde(rename = "RcvPos")]
    pub rcv_pos: PvpField,
    #[serde(rename = "RcvVel")]
    pub rcv_vel: PvpField,

    #[serde(rename = "SRPPos")]
    pub srp_pos: PvpField,

    #[serde(rename = "AmpSF", skip_serializing_if = "Option::is_none", default)]
    pub amp_sf: Option<PvpField>,

    #[serde(rename = "aFDOP")]
    pub a_fdop: PvpField,
    #[serde(rename = "aFRR1")]
    pub a_frr1: PvpField,
    #[serde(rename = "aFRR2")]
    pub a_frr2: PvpField,

    #[serde(rename = "FX1")]
    pub fx1: PvpField,
    #[serde(rename = "FX2")]
    pub fx2: PvpField,

    #[serde(rename = "FXN1", skip_serializing_if = "Option::is_none", default)]
    pub fxn1: Option<PvpField>,
    #[serde(rename = "FXN2", skip_serializing_if = "Option::is_none", default)]
    pub fxn2: Option<PvpField>,

    #[serde(rename = "TOA1")]
    pub toa1: PvpField,
    #[serde(rename = "TOA2")]
    pub toa2: PvpField,

    #[serde(rename = "TOAE1", skip_serializing_if = "Option::is_none", default)]
    pub toae1: Option<PvpField>,
    #[serde(rename = "TOAE2", skip_serializing_if = "Option::is_none", default)]
    pub toae2: Option<PvpField>,

    #[serde(rename = "TDTropoSRP")]
    pub td_tropo_srp: PvpField,
    #[serde(rename = "TDIonoSRP", skip_serializing_if = "Option::is_none", default)]
    pub td_iono_srp: Option<PvpField>,

    #[serde(rename = "SC0")]
    pub sc0: PvpField,
    #[serde(rename = "SCSS")]
    pub scss: PvpField,
    
    #[serde(rename = "TxACX", skip_serializing_if = "Option::is_none", default)]
    pub tx_acx: Option<PvpField>,

    #[serde(rename = "TxACY", skip_serializing_if = "Option::is_none", default)]
    pub tx_acy: Option<PvpField>,

    #[serde(rename = "TxEB", skip_serializing_if = "Option::is_none", default)]
    pub tx_eb: Option<PvpField>,

    #[serde(rename = "RcvACX", skip_serializing_if = "Option::is_none", default)]
    pub rcv_acx: Option<PvpField>,

    #[serde(rename = "RcvACY", skip_serializing_if = "Option::is_none", default)]
    pub rcv_acy: Option<PvpField>,

    #[serde(rename = "RcvEB", skip_serializing_if = "Option::is_none", default)]
    pub rcv_eb: Option<PvpField>,

    #[serde(rename = "SIGNAL", skip_serializing_if = "Option::is_none", default)]
    pub signal: Option<PvpField>,

    // Support multiple or optional user-defined added PVPs
    #[serde(rename = "AddedPVP", skip_serializing_if = "Option::is_none", default)] pub added_pvp: Option<Vec<AddedPvpField>>,
}

// Table 2-2 Defined Per Vector Parameters. 
// NGA.STND.0068-1_1.1.0_CPHD p. 25
#[derive(Debug, Clone)]
pub struct PvpSet {
    // Required scalar fields
    pub tx_time: f64,
    pub rcv_time: f64,
    pub a_fdop: f64,
    pub a_frr1: f64,
    pub a_frr2: f64,
    pub fx1: f64,
    pub fx2: f64,
    pub toa1: f64,
    pub toa2: f64,
    pub td_tropo_srp: f64,
    pub sc0: f64,
    pub scss: f64,

    // XYZ vector fields
    pub tx_pos: [f64; 3],
    pub tx_vel: [f64; 3],
    pub rcv_pos: [f64; 3],
    pub rcv_vel: [f64; 3],
    pub srp_pos: [f64; 3],

    // Optional scalar fields
    pub amp_sf: Option<f64>,
    pub fxn1: Option<f64>,
    pub fxn2: Option<f64>,
    pub toae1: Option<f64>,
    pub toae2: Option<f64>,
    pub td_iono_srp: Option<f64>,

    // Optional integer field
    pub signal: Option<i64>,

    // Optional transmit antenna parameters
    pub tx_acx: Option<[f64; 3]>,
    pub tx_acy: Option<[f64; 3]>,
    pub tx_eb: Option<[f64; 2]>,

    // Optional receive antenna parameters
    pub rcv_acx: Option<[f64; 3]>,
    pub rcv_acy: Option<[f64; 3]>,
    pub rcv_eb: Option<[f64; 2]>,

    // Optional added PVP fields
    pub added_pvp: Option<Vec<AddedPvpValue>>,
}

#[derive(Debug, Clone)]
pub struct AddedPvpValue {
    pub name: String,
    pub value: f64,
}
// Iterator for reading pvp from cphd

#[derive(Debug)]
pub struct PvpIterator {
    mmap: Arc<Mmap>,
    pvp: Pvp,
    current_vector: usize,
    total_vectors: usize,
    pvp_set_size: usize,
    pvp_block_offset: usize,
}

impl PvpIterator {
    pub fn new(mmap: Arc<Mmap>, 
               pvp: &Pvp, 
               pvp_block_offset: usize,
               total_vectors: usize,
               num_bytes_pvp: usize) -> Self {

        let pvp_set_size = calculate_pvp_set_size(pvp);

        //assert_eq!(pvp_set_size, num_bytes_pvp);

        Self {
            mmap,
            pvp: pvp.clone(),
            current_vector: 0,
            total_vectors,
            pvp_set_size: num_bytes_pvp, // pvp_set_size,
            pvp_block_offset,
        }
    }
}


impl Iterator for PvpIterator {
    type Item = PvpSet;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_vector >= self.total_vectors {
            return None;
        }

        let pvp_set = parse_single_pvp_set(
            &self.mmap,
            &self.pvp,
            self.current_vector,
            self.pvp_set_size,
            self.pvp_block_offset,
        ).expect("Failed to parse PVP set");

        self.current_vector += 1;
        Some(pvp_set)
    }
}

pub fn calculate_pvp_set_size(pvp: &Pvp) -> usize {
    // Calculate the total size of a single PVP set in 8-byte words
    let mut size = 0;

    // Required fields
    size += pvp.tx_time.size;
    size += pvp.tx_pos.size;
    size += pvp.tx_vel.size;
    size += pvp.rcv_time.size;
    size += pvp.rcv_pos.size;
    size += pvp.rcv_vel.size;
    size += pvp.srp_pos.size;
    size += pvp.a_fdop.size;
    size += pvp.a_frr1.size;
    size += pvp.a_frr2.size;
    size += pvp.fx1.size;
    size += pvp.fx2.size;
    size += pvp.toa1.size;
    size += pvp.toa2.size;
    size += pvp.td_tropo_srp.size;
    size += pvp.sc0.size;
    size += pvp.scss.size;

    // Optional fields
    if pvp.amp_sf.is_some() {
        size += pvp.amp_sf.as_ref().unwrap().size;
    }
    if pvp.fxn1.is_some() {
        size += pvp.fxn1.as_ref().unwrap().size;
    }
    if pvp.fxn2.is_some() {
        size += pvp.fxn2.as_ref().unwrap().size;
    }
    if pvp.toae1.is_some() {
        size += pvp.toae1.as_ref().unwrap().size;
    }
    if pvp.toae2.is_some() {
            size += pvp.toae2.as_ref().unwrap().size;
    }
    if pvp.td_iono_srp.is_some() {
        size += pvp.td_iono_srp.as_ref().unwrap().size;
    }
    if pvp.signal.is_some() {
        size += pvp.signal.as_ref().unwrap().size;
    }

    // Added PVP fields
    if let Some(added_pvp) = &pvp.added_pvp {
        for field in added_pvp {
            size += field.size;
        }
    }

    (size * 8).try_into().unwrap() // Convert 8-byte words to bytes
}

pub fn parse_single_pvp_set(
    mmap: &Arc<Mmap>,
    pvp: &Pvp,
    vector_index: usize,
    pvp_set_size: usize,
    pvp_block_offset: usize,
) -> Result<PvpSet> {
    let pvp_set_offset = pvp_block_offset + vector_index * pvp_set_size;
    let pvp_set_slice = mmap.get(pvp_set_offset..pvp_set_offset + pvp_set_size)
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::UnexpectedEof))?;

    let mut cursor = Cursor::new(pvp_set_slice);

    // Parse these in order of Table 2-2 pg 25
    let tx_time = cursor.read_f64::<BigEndian>()?;

    let tx_pos = [
        cursor.read_f64::<BigEndian>()?,
        cursor.read_f64::<BigEndian>()?,
        cursor.read_f64::<BigEndian>()?,
    ];

    let tx_vel = [
        cursor.read_f64::<BigEndian>()?,
        cursor.read_f64::<BigEndian>()?,
        cursor.read_f64::<BigEndian>()?,
    ];

    let rcv_time = cursor.read_f64::<BigEndian>()?;

    let rcv_pos = [
        cursor.read_f64::<BigEndian>()?,
        cursor.read_f64::<BigEndian>()?,
        cursor.read_f64::<BigEndian>()?,
    ];

    let rcv_vel = [
        cursor.read_f64::<BigEndian>()?,
        cursor.read_f64::<BigEndian>()?,
        cursor.read_f64::<BigEndian>()?,
    ];

    let srp_pos = [
        cursor.read_f64::<BigEndian>()?,
        cursor.read_f64::<BigEndian>()?,
        cursor.read_f64::<BigEndian>()?,
    ];

    // Parse optional scalar fields
    let amp_sf = if pvp.amp_sf.is_some() {
        Some(cursor.read_f64::<BigEndian>()?)
    } else {
        None
    };

    let a_fdop = cursor.read_f64::<BigEndian>()?;
    let a_frr1 = cursor.read_f64::<BigEndian>()?;
    let a_frr2 = cursor.read_f64::<BigEndian>()?;

    let fx1 = cursor.read_f64::<BigEndian>()?;
    let fx2 = cursor.read_f64::<BigEndian>()?;

    let fxn1 = if pvp.fxn1.is_some() {
        Some(cursor.read_f64::<BigEndian>()?)
    } else {
        None
    };

    let fxn2 = if pvp.fxn2.is_some() {
        Some(cursor.read_f64::<BigEndian>()?)
    } else {
        None
    };

    let toa1 = cursor.read_f64::<BigEndian>()?;
    let toa2 = cursor.read_f64::<BigEndian>()?;

    let toae1 = if pvp.toae1.is_some() {
        Some(cursor.read_f64::<BigEndian>()?)
    } else {
        None
    };

    let toae2 = if pvp.toae2.is_some() {
        Some(cursor.read_f64::<BigEndian>()?)
    } else {
        None
    };

    let td_tropo_srp = cursor.read_f64::<BigEndian>()?;

    let td_iono_srp = if pvp.td_iono_srp.is_some() {
        Some(cursor.read_f64::<BigEndian>()?)
    } else {
        None
    };

    let sc0 = cursor.read_f64::<BigEndian>()?;
    let scss = cursor.read_f64::<BigEndian>()?;

    // Parse optional integer field
    let signal = if pvp.signal.is_some() {
        Some(cursor.read_i64::<BigEndian>()?)
    } else {
        None
    };

    // Parse optional transmit antenna parameters
    let tx_acx = if pvp.tx_acx.is_some() {
        Some([
            cursor.read_f64::<BigEndian>()?, // Xmt_ACX_X
            cursor.read_f64::<BigEndian>()?, // Xmt_ACX_Y
            cursor.read_f64::<BigEndian>()?, // Xmt_ACX_Z
        ])
    } else {
        None
    };
    
    let tx_acy = if pvp.tx_acy.is_some() {
        Some([
            cursor.read_f64::<BigEndian>()?, // Xmt_ACY_X
            cursor.read_f64::<BigEndian>()?, // Xmt_ACY_Y
            cursor.read_f64::<BigEndian>()?, // Xmt_ACY_Z
        ])
    } else {
        None
    };
    
    let tx_eb = if pvp.tx_eb.is_some() {
        Some([
            cursor.read_f64::<BigEndian>()?, // Xmt_EB_DCX
            cursor.read_f64::<BigEndian>()?, // Xmt_EB_DCY
        ])
    } else {
        None
    };
    
    // Parse optional receive antenna parameters
    let rcv_acx = if pvp.rcv_acx.is_some() {
        Some([
            cursor.read_f64::<BigEndian>()?, // Rcv_ACX_X
            cursor.read_f64::<BigEndian>()?, // Rcv_ACX_Y
            cursor.read_f64::<BigEndian>()?, // Rcv_ACX_Z
        ])
    } else {
        None
    };
    
    let rcv_acy = if pvp.rcv_acy.is_some() {
        Some([
            cursor.read_f64::<BigEndian>()?, // Rcv_ACY_X
            cursor.read_f64::<BigEndian>()?, // Rcv_ACY_Y
            cursor.read_f64::<BigEndian>()?, // Rcv_ACY_Z
        ])
    } else {
        None
    };
    
    let rcv_eb = if pvp.rcv_eb.is_some() {
        Some([
            cursor.read_f64::<BigEndian>()?, // Rcv_EB_DCX
            cursor.read_f64::<BigEndian>()?, // Rcv_EB_DCY
        ])
    } else {
        None
    };

    // Parse added PVP fields
    let added_pvp = if let Some(added_pvp_fields) = &pvp.added_pvp {
        let mut added_values = Vec::new();
        for field in added_pvp_fields {
            let value = cursor.read_f64::<BigEndian>()?;
            added_values.push(AddedPvpValue {
                name: field.name.clone(),
                value,
            });
        }
        Some(added_values)
    } else {
        None
    };

    Ok(PvpSet {
        // Required scalar fields
        tx_time,
        rcv_time,
        a_fdop,
        a_frr1,
        a_frr2,
        fx1,
        fx2,
        toa1,
        toa2,
        td_tropo_srp,
        sc0,
        scss,

        // XYZ vector fields
        tx_pos,
        tx_vel,
        rcv_pos,
        rcv_vel,
        srp_pos,

        // Optional scalar fields
        amp_sf,
        fxn1,
        fxn2,
        toae1,
        toae2,
        td_iono_srp,

        // Optional integer field
        signal,

        // Optional transmit antenna parameters
        tx_acx,
        tx_acy,
        tx_eb,

        // Optional receive antenna parameters
        rcv_acx,
        rcv_acy,
        rcv_eb,

        // Optional added PVP fields
        added_pvp,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pvp_block_deserialization() {
        let xml_data = r#"<PVP>
            <TxTime><Offset>0</Offset><Size>1</Size><Format>F8</Format></TxTime>
            <TxPos><Offset>1</Offset><Size>3</Size><Format>X=F8;Y=F8;Z=F8;</Format></TxPos>
            <TxVel><Offset>4</Offset><Size>3</Size><Format>X=F8;Y=F8;Z=F8;</Format></TxVel>
            <RcvTime><Offset>7</Offset><Size>1</Size><Format>F8</Format></RcvTime>
            <RcvPos><Offset>8</Offset><Size>3</Size><Format>X=F8;Y=F8;Z=F8;</Format></RcvPos>
            <RcvVel><Offset>11</Offset><Size>3</Size><Format>X=F8;Y=F8;Z=F8;</Format></RcvVel>
            <SRPPos><Offset>14</Offset><Size>3</Size><Format>X=F8;Y=F8;Z=F8;</Format></SRPPos>
            <AmpSF><Offset>17</Offset><Size>1</Size><Format>F8</Format></AmpSF>
            <aFDOP><Offset>18</Offset><Size>1</Size><Format>F8</Format></aFDOP>
            <aFRR1><Offset>19</Offset><Size>1</Size><Format>F8</Format></aFRR1>
            <aFRR2><Offset>20</Offset><Size>1</Size><Format>F8</Format></aFRR2>
            <FX1><Offset>21</Offset><Size>1</Size><Format>F8</Format></FX1>
            <FX2><Offset>22</Offset><Size>1</Size><Format>F8</Format></FX2>
            <TOA1><Offset>23</Offset><Size>1</Size><Format>F8</Format></TOA1>
            <TOA2><Offset>24</Offset><Size>1</Size><Format>F8</Format></TOA2>
            <TOAE1><Offset>25</Offset><Size>1</Size><Format>F8</Format></TOAE1>
            <TOAE2><Offset>26</Offset><Size>1</Size><Format>F8</Format></TOAE2>
            <TDTropoSRP><Offset>27</Offset><Size>1</Size><Format>F8</Format></TDTropoSRP>
            <TDIonoSRP><Offset>28</Offset><Size>1</Size><Format>F8</Format></TDIonoSRP>
            <SC0><Offset>29</Offset><Size>1</Size><Format>F8</Format></SC0>
            <SCSS><Offset>30</Offset><Size>1</Size><Format>F8</Format></SCSS>
            <SIGNAL><Offset>31</Offset><Size>1</Size><Format>I8</Format></SIGNAL>
            <AddedPVP><Name>RefTime</Name><Offset>32</Offset><Size>1</Size><Format>F8</Format></AddedPVP>
        </PVP>"#;

        let pvp: Pvp = quick_xml::de::from_str(xml_data).unwrap();

        assert_eq!(pvp.tx_time.offset, 0);
        assert_eq!(pvp.tx_pos.offset, 1);
        assert_eq!(pvp.tx_pos.size, 3);
        assert_eq!(pvp.added_pvp.as_ref().unwrap()[0].name, "RefTime");
    }
}
