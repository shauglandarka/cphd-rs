pub mod file_hdr;
pub mod collection_id;
use serde;
use serde::{Deserialize, Serialize};
    
/// Represents the high-level metadata and branch tracking for the CPHD XML block.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CphdMeta {
//    pub collection_id: CollectionId,
//    pub global: Global,
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
