use super::Polarization as PolarizationEnum;
use super::scene_coordinates::ImageArea;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Channel {
    #[serde(rename = "RefChId")]
    pub ref_ch_id: String,

    #[serde(rename = "FXFixedCPHD")]
    pub fx_fixed_cphd: bool,

    #[serde(rename = "TOAFixedCPHD")]
    pub toa_fixed_cphd: bool,

    #[serde(rename = "SRPFixedCPHD")]
    pub srp_fixed_cphd: bool,

    #[serde(rename = "Parameters")]
    pub parameters: Vec<ChannelParameters>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelParameters {
    #[serde(rename = "Identifier")]
    pub identifier: String,

    #[serde(rename = "RefVectorIndex")]
    pub ref_vector_index: u32,

    #[serde(rename = "FXFixed")]
    pub fx_fixed: bool,

    #[serde(rename = "TOAFixed")]
    pub toa_fixed: bool,

    #[serde(rename = "SRPFixed")]
    pub srp_fixed: bool,

    #[serde(
        rename = "SignalNormal",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub signal_normal: Option<bool>,

    #[serde(rename = "Polarization")]
    pub polarization: Polarization,

    #[serde(rename = "TxPolRef", skip_serializing_if = "Option::is_none", default)]
    pub tx_pol_ref: Option<TxPolRef>,

    #[serde(rename = "RcvPolRef", skip_serializing_if = "Option::is_none", default)]
    pub rcv_pol_ref: Option<RcvPolRef>,

    #[serde(rename = "FxC")]
    pub fx_c: f64,

    #[serde(rename = "FxBW")]
    pub fx_bw: f64,

    #[serde(rename = "FxBWNoise", skip_serializing_if = "Option::is_none", default)]
    pub fx_bw_noise: Option<f64>,

    #[serde(rename = "TOASaved")]
    pub toa_saved: f64,

    #[serde(
        rename = "TOAExtended",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub toa_extended: Option<ToaExtended>,

    #[serde(
        rename = "LFMEclipse",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub lfm_eclipse: Option<LfmEclipse>,

    #[serde(rename = "DwellTimes")]
    pub dwell_times: DwellTimes,

    #[serde(rename = "ImageArea", skip_serializing_if = "Option::is_none", default)]
    pub image_area: Option<ImageArea>,

    #[serde(rename = "Antenna", skip_serializing_if = "Option::is_none", default)]
    pub antenna: Option<ChannelAntenna>,

    #[serde(rename = "TxRcv", skip_serializing_if = "Option::is_none", default)]
    pub tx_rcv: Option<ChannelTxRcv>,

    #[serde(
        rename = "TgtRefLevel",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub tgt_ref_level: Option<TgtRefLevel>,

    #[serde(
        rename = "NoiseLevel",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub noise_level: Option<NoiseLevel>,

    #[serde(
        rename = "FxNoiseProfile",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub fx_noise_profile: Option<FxNoiseProfile>,

    #[serde(
        rename = "AddedParameters",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub added_parameters: Option<AddedParameters>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Polarization {
    #[serde(rename = "TxPol")]
    pub tx_pol: PolarizationEnum,

    #[serde(rename = "RcvPol")]
    pub rcv_pol: PolarizationEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TxPolRef {
    #[serde(rename = "AmpH")]
    pub amp_h: f64,

    #[serde(rename = "AmpV")]
    pub amp_v: f64,

    #[serde(rename = "PhaseV")]
    pub phase_v: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RcvPolRef {
    #[serde(rename = "AmpH")]
    pub amp_h: f64,

    #[serde(rename = "AmpV")]
    pub amp_v: f64,

    #[serde(rename = "PhaseV")]
    pub phase_v: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToaExtended {
    #[serde(rename = "TOAExtSaved")]
    pub toa_ext_saved: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LfmEclipse {
    #[serde(rename = "FxEarlyLow")]
    pub fx_early_low: f64,

    #[serde(rename = "FxEarlyHigh")]
    pub fx_early_high: f64,

    #[serde(rename = "FxLateLow")]
    pub fx_late_low: f64,

    #[serde(rename = "FxLateHigh")]
    pub fx_late_high: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DwellTimes {
    #[serde(rename = "CODId")]
    pub cod_id: String,

    #[serde(rename = "DwellId")]
    pub dwell_id: String,

    #[serde(rename = "DTAId", skip_serializing_if = "Option::is_none", default)]
    pub dta_id: Option<String>,

    #[serde(rename = "UseDTA", skip_serializing_if = "Option::is_none", default)]
    pub use_dta: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelAntenna {
    #[serde(rename = "TxAPCId")]
    pub tx_apc_id: String,

    #[serde(rename = "TxAPATId")]
    pub tx_apat_id: String,

    #[serde(rename = "RcvAPCId")]
    pub rcv_apc_id: String,

    #[serde(rename = "RcvAPATId")]
    pub rcv_apat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChannelTxRcv {
    #[serde(rename = "TxWFId")]
    pub tx_wf_id: Vec<String>,

    #[serde(rename = "RcvId")]
    pub rcv_id: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TgtRefLevel {
    #[serde(rename = "PTRef")]
    pub pt_ref: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NoiseLevel {
    #[serde(rename = "PNRef")]
    pub pn_ref: f64,

    #[serde(rename = "BNRef")]
    pub bn_ref: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FxNoiseProfile {
    #[serde(rename = "Point")]
    pub point: Vec<FxNoisePoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FxNoisePoint {
    #[serde(rename = "Fx")]
    pub fx: f64,

    #[serde(rename = "PN")]
    pub pn: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AddedParameters {
    #[serde(rename = "Parameter")]
    pub parameter: Vec<super::Parameter>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_block_deserialization() {
        let xml_data = r#"<Channel>
            <RefChId>0</RefChId>
            <FXFixedCPHD>true</FXFixedCPHD>
            <TOAFixedCPHD>false</TOAFixedCPHD>
            <SRPFixedCPHD>false</SRPFixedCPHD>
            <Parameters>
                <Identifier>0</Identifier>
                <RefVectorIndex>20145</RefVectorIndex>
                <FXFixed>true</FXFixed>
                <TOAFixed>false</TOAFixed>
                <SRPFixed>false</SRPFixed>
                <SignalNormal>false</SignalNormal>
                <Polarization>
                    <TxPol>H</TxPol>
                    <RcvPol>H</RcvPol>
                </Polarization>
                <FxC>9.64999987200000000E+09</FxC>
                <FxBW>2.00000000000000000E+08</FxBW>
                <TOASaved>2.84479166666666692E-05</TOASaved>
                <TOAExtended>
                    <TOAExtSaved>2.84479166666666692E-05</TOAExtSaved>
                </TOAExtended>
                <DwellTimes>
                    <CODId>0</CODId>
                    <DwellId>0</DwellId>
                </DwellTimes>
                <Antenna>
                    <TxAPCId>0</TxAPCId>
                    <TxAPATId>0</TxAPATId>
                    <RcvAPCId>0</RcvAPCId>
                    <RcvAPATId>0</RcvAPATId>
                </Antenna>
                <TxRcv>
                    <TxWFId>0</TxWFId>
                    <RcvId>0</RcvId>
                </TxRcv>
            </Parameters>
        </Channel>"#;

        let channel: Channel = quick_xml::de::from_str(xml_data).unwrap();

        assert_eq!(channel.ref_ch_id, "0");
        assert!(channel.fx_fixed_cphd);
        assert!(!channel.toa_fixed_cphd);
        assert!(!channel.srp_fixed_cphd);
        assert_eq!(channel.parameters.len(), 1);

        let params = &channel.parameters[0];
        assert_eq!(params.identifier, "0");
        assert_eq!(params.ref_vector_index, 20145);
        assert!(params.fx_fixed);
        assert_eq!(params.polarization.tx_pol, PolarizationEnum::H);
        assert_eq!(params.polarization.rcv_pol, PolarizationEnum::H);
        assert_eq!(params.fx_c, 9.64999987200000000E+09);
        assert_eq!(params.fx_bw, 200000000.0);
        assert_eq!(params.toa_saved, 2.84479166666666692E-05);
        assert_eq!(
            params.toa_extended.as_ref().unwrap().toa_ext_saved,
            2.84479166666666692E-05
        );
    }
}
