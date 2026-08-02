pub mod file_hdr;
pub mod collection_id;
pub mod global;
pub mod scene_coordinates;

use collection_id::CollectionId;
use global::Global;
use scene_coordinates::SceneCoordinates;

use serde;
use serde::{Deserialize, Serialize};
    
/// Represents the high-level metadata and branch tracking for the CPHD XML block.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename = "CPHD")]
pub struct CphdMeta {
    #[serde(rename = "CollectionID")]
    pub collection_id: CollectionId,
    #[serde(rename = "Global")]
    pub global: Global,
//    pub scene_coordinates: SceneCoordinates,
//    pub data: Data,
//    pub channel: Channel,
//    pub pvp: Pvp,
//    pub dwell: Dwell,
//    pub reference_geometry: ReferenceGeometry,
//    pub support_array: Option<SupportArray>,
//    pub antenna: Option<Antenna>,
//    pub tx_rcv: Option<TxRcv>,
//    pub error_parameters: Option<ErrorParameters>,
//    pub product_info: Option<ProductInfo>,
//    pub geo_info: Option<GeoInfo>,
//    pub match_info: Option<MatchInfo>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iarp {
    #[serde(rename = "ECF")]
    pub ecf: Ecf,

    #[serde(rename = "LLH")]
    pub llh: Llh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ecf {
    #[serde(rename = "X")]
    pub x: f64,

    #[serde(rename = "Y")]
    pub y: f64,

    #[serde(rename = "Z")]
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Llh {
    #[serde(rename = "Lat")]
    pub lat: f64,

    #[serde(rename = "Lon")]
    pub lon: f64,

    #[serde(rename = "HAE")]
    pub hae: f64,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Poly {
    #[serde(rename = "@order1")]
    pub order1: String,
    #[serde(rename = "Coef")]
    pub coeffs: Vec<Coef>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Coef {
    #[serde(rename = "@exponent1")]
    pub exponent1: String,
    #[serde(rename = "@exponent2", default)]
    pub exponent2: Option<String>,
    #[serde(rename = "$value")]
    pub value: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CollectType {
    #[serde(rename = "MONOSTATIC")]
    Monostatic,
    #[serde(rename = "BISTATIC")]
    Bistatic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModeType {
    #[serde(rename = "SPOTLIGHT")]
    Spotlight,
    #[serde(rename = "STRIPMAP")]
    Stripmap,
    #[serde(rename = "DYNAMIC STRIPMAP")]
    DynamicStripmap,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DomainType {
    #[serde(rename = "FX")]
    Fx,
    #[serde(rename = "TOA")]
    Toa,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RefHeight {
    #[serde(rename = "IARP")]
    Iarp,
    #[serde(rename = "ZERO")]
    Zero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EarthModel {
    #[serde(rename = "WGS_84")]
    Wgs84,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalArrayFormat {
    #[serde(rename = "CI2")]
    Ci2,
    #[serde(rename = "CI4")]
    Ci4,
    #[serde(rename = "CF8")]
    Cf8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Polarization {
    #[serde(rename = "V")]
    V,
    #[serde(rename = "H")]
    H,
    #[serde(rename = "X")]
    X,
    #[serde(rename = "Y")]
    Y,
    #[serde(rename = "S")]
    S,
    #[serde(rename = "E")]
    E,
    #[serde(rename = "RHC")]
    Rhc,
    #[serde(rename = "LHC")]
    Lhc,
    #[serde(rename = "UNSPECIFIED")]
    Unspecified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SideOfTrack {
    #[serde(rename = "L")]
    Left,
    #[serde(rename = "R")]
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Frame {
    #[serde(rename = "ECF")]
    Ecf,
    #[serde(rename = "RIC_ECF")]
    RicEcf,
    #[serde(rename = "RIC_ECI")]
    RicEci,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_cphd_meta_sub_structs() {
        let xml_str = r#"<CPHD xmlns="http://api.nsgreg.nga.mil/schema/cphd/1.1.0">
            <CollectionID>
                <CollectorName>capella-radar-3</CollectorName>
                <CoreName>29DEC21_CAPELLA-3_001_053627_ST0000R_38N122W_001X_HH_0101_SPY</CoreName>
                <CollectType>MONOSTATIC</CollectType>
                <RadarMode>
                    <ModeType>STRIPMAP</ModeType>
                </RadarMode>
                <Classification>UNCLASSIFIED</Classification>
                <ReleaseInfo>UNRESTRICTED</ReleaseInfo>
                <Parameter name="capella_collect_id">1de7df48-6bf4-464e-a962-1d2ea76b3262</Parameter>
                <Parameter name="capella_software_version">2.59.5</Parameter>
                <Parameter name="capella_software_revision">e3bdb24efc21046a6f0966ebf0f3ebbba394e2e1-dirty</Parameter>
            </CollectionID>
            <Global>
                <DomainType>FX</DomainType>
                <SGN>1</SGN>
                <Timeline>
                    <CollectionStart>2021-12-29T05:36:31.000000Z</CollectionStart>
                    <TxTime1>0.00000000000000000E+00</TxTime1>
                    <TxTime2>3.97947882666666652E+00</TxTime2>
                </Timeline>
                <FxBand>
                    <FxMin>9.54999987200000000E+09</FxMin>
                    <FxMax>9.74999987200000000E+09</FxMax>
                </FxBand>
                <TOASwath>
                    <TOAMin>-1.42239583333333346E-05</TOAMin>
                    <TOAMax>1.42239583333333346E-05</TOAMax>
                </TOASwath>
                <TropoParameters>
                    <N0>1.00000000000000000E+00</N0>
                    <RefHeight>IARP</RefHeight>
                </TropoParameters>
                <IonoParameters>
                    <TECV>0.00000000000000000E+00</TECV>
                </IonoParameters>
            </Global>
        </CPHD>"#;

        let meta: CphdMeta = quick_xml::de::from_str(xml_str).unwrap();

        // Test CollectionID sub-struct parsing
        assert_eq!(meta.collection_id.collector_name, "capella-radar-3");
        assert_eq!(meta.collection_id.collect_type, CollectType::Monostatic);
        assert_eq!(meta.collection_id.radar_mode.mode_type, ModeType::Stripmap);
        assert_eq!(meta.collection_id.classification, "UNCLASSIFIED");
        assert_eq!(meta.collection_id.parameter.len(), 3);
        assert_eq!(meta.collection_id.parameter[0].name, "capella_collect_id");

        // Test Global sub-struct parsing
        assert_eq!(meta.global.domain_type, DomainType::Fx);
        assert_eq!(meta.global.sgn, 1);
        assert_eq!(meta.global.timeline.collection_start, "2021-12-29T05:36:31.000000Z");
        assert_eq!(meta.global.fx_band.fx_min, 9549999872.0);
        
        let tropo = meta.global.tropo_parameters.as_ref().unwrap();
        assert_eq!(tropo.n0, 1.0);
        assert_eq!(tropo.ref_height, RefHeight::Iarp);
    }
}
