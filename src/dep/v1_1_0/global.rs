use super::{DomainType, RefHeight};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Global {
    #[serde(rename = "DomainType")]
    pub domain_type: DomainType,

    #[serde(rename = "SGN")]
    pub sgn: i32,

    #[serde(rename = "Timeline")]
    pub timeline: Timeline,

    #[serde(rename = "FxBand")]
    pub fx_band: FxBand,

    #[serde(rename = "TOASwath")]
    pub toa_swath: ToaSwath,

    #[serde(rename = "TropoParameters", default)]
    pub tropo_parameters: Option<TropoParameters>,

    #[serde(rename = "IonoParameters", default)]
    pub iono_parameters: Option<IonoParameters>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Timeline {
    #[serde(rename = "CollectionStart")]
    pub collection_start: String,

    #[serde(rename = "RcvCollectionStart", default)]
    pub rcv_collection_start: Option<String>,

    #[serde(rename = "TxTime1")]
    pub tx_time1: f64,

    #[serde(rename = "TxTime2")]
    pub tx_time2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FxBand {
    #[serde(rename = "FxMin")]
    pub fx_min: f64,

    #[serde(rename = "FxMax")]
    pub fx_max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToaSwath {
    #[serde(rename = "TOAMin")]
    pub toa_min: f64,

    #[serde(rename = "TOAMax")]
    pub toa_max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TropoParameters {
    #[serde(rename = "N0")]
    pub n0: f64,

    #[serde(rename = "RefHeight")]
    pub ref_height: RefHeight,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IonoParameters {
    #[serde(rename = "TECV")]
    pub tecv: f64,

    #[serde(rename = "F2Height", default)]
    pub f2_height: Option<f64>,
}


#[cfg(test)]
    mod tests {
    use super::*;
    
    #[test]
    fn test_deserialize_global() {
        let xml_str = r#"
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
            </Global>"#;

        let global: Global = quick_xml::de::from_str(xml_str).unwrap();

        assert_eq!(global.domain_type, DomainType::Fx);
        assert_eq!(global.sgn, 1);
        assert_eq!(global.timeline.collection_start, "2021-12-29T05:36:31.000000Z");
        assert_eq!(global.timeline.tx_time1, 0.0);
        assert_eq!(global.timeline.tx_time2, 3.97947882666666652);
        assert_eq!(global.fx_band.fx_min, 9549999872.0);
        assert_eq!(global.fx_band.fx_max, 9749999872.0);
        assert_eq!(global.toa_swath.toa_min, -0.000014223958333333334);
        assert_eq!(global.toa_swath.toa_max, 0.000014223958333333334);

        let tropo = global.tropo_parameters.as_ref().unwrap();
        assert_eq!(tropo.n0, 1.0);
        assert_eq!(tropo.ref_height, RefHeight::Iarp);

        let iono = global.iono_parameters.as_ref().unwrap();
        assert_eq!(iono.tecv, 0.0);
    }
}



