use super::Polarization;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TxRcv {
    #[serde(rename = "NumTxWFs")]
    pub num_tx_wfs: i32,

    #[serde(rename = "TxWFParameters")]
    pub tx_wf_parameters: Vec<TxWfParameters>,

    #[serde(rename = "NumRcvs")]
    pub num_rcvs: i32,

    #[serde(rename = "RcvParameters")]
    pub rcv_parameters: Vec<RcvParameters>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TxWfParameters {
    #[serde(rename = "Identifier")]
    pub identifier: String,

    #[serde(rename = "PulseLength")]
    pub pulse_length: f64,

    #[serde(rename = "RFBandwidth")]
    pub rf_bandwidth: f64,

    #[serde(rename = "FreqCenter")]
    pub freq_center: f64,

    #[serde(rename = "LFMRate", default)]
    pub lfm_rate: Option<f64>,

    #[serde(rename = "Polarization")]
    pub polarization: Polarization,

    #[serde(rename = "Power", default)]
    pub power: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RcvParameters {
    #[serde(rename = "Identifier")]
    pub identifier: String,

    #[serde(rename = "WindowLength")]
    pub window_length: f64,

    #[serde(rename = "SampleRate")]
    pub sample_rate: f64,

    #[serde(rename = "IFFilterBW")]
    pub if_filter_bw: f64,

    #[serde(rename = "FreqCenter")]
    pub freq_center: f64,

    #[serde(rename = "LFMRate", default)]
    pub lfm_rate: Option<f64>,

    #[serde(rename = "Polarization")]
    pub polarization: Polarization,

    #[serde(rename = "PathGain", default)]
    pub path_gain: Option<f64>,
}

#[test]
fn test_tx_rcv_deserialization() {
    let xml_data = r#"<TxRcv>
        <NumTxWFs>1</NumTxWFs>
        <TxWFParameters>
            <Identifier>0</Identifier>
            <PulseLength>1.97546666666666657E-05</PulseLength>
            <RFBandwidth>2.00000000000000000E+08</RFBandwidth>
            <FreqCenter>9.64999987200000000E+09</FreqCenter>
            <LFMRate>1.01241900647948164E+13</LFMRate>
            <Polarization>H</Polarization>
            <Power>2.94972087775335297E+01</Power>
        </TxWFParameters>
        <NumRcvs>1</NumRcvs>
        <RcvParameters>
            <Identifier>0</Identifier>
            <WindowLength>3.41375000000000031E-05</WindowLength>
            <SampleRate>2.40000000000000000E+08</SampleRate>
            <IFFilterBW>6.55000000000000000E+08</IFFilterBW>
            <FreqCenter>9.64999987200000000E+09</FreqCenter>
            <Polarization>H</Polarization>
            <PathGain>-3.70657947139393571E+02</PathGain>
        </RcvParameters>
    </TxRcv>"#;

    let tx_rcv: TxRcv = quick_xml::de::from_str(xml_data).unwrap();

    assert_eq!(tx_rcv.num_tx_wfs, 1);
    assert_eq!(tx_rcv.tx_wf_parameters[0].identifier, "0");
    assert_eq!(tx_rcv.tx_wf_parameters[0].polarization, Polarization::H);
    assert_eq!(tx_rcv.tx_wf_parameters[0].power, Some(29.49720877753353));

    assert_eq!(tx_rcv.num_rcvs, 1);
    assert_eq!(tx_rcv.rcv_parameters[0].identifier, "0");
    assert_eq!(
        tx_rcv.rcv_parameters[0].path_gain,
        Some(-370.65794713939357)
    );
}
