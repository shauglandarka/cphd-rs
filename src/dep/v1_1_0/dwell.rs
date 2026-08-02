use serde::{Deserialize, Serialize};
use super::{Poly2D};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dwell {
    #[serde(rename = "NumCODTimes")]
    pub num_cod_times: u32,

    #[serde(rename = "CODTime")]
    pub cod_time: Vec<CodTimeEntry>,

    #[serde(rename = "NumDwellTimes")]
    pub num_dwell_times: u32,

    #[serde(rename = "DwellTime")]
    pub dwell_time: Vec<DwellTimeEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodTimeEntry {
    #[serde(rename = "Identifier")]
    pub identifier: String,

    #[serde(rename = "CODTimePoly")]
    pub cod_time_poly: Poly2D,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DwellTimeEntry {
    #[serde(rename = "Identifier")]
    pub identifier: String,

    #[serde(rename = "DwellTimePoly")]
    pub dwell_time_poly: Poly2D,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dwell_block_deserialization() {
        let xml_data = r#"<Dwell>
            <NumCODTimes>1</NumCODTimes>
            <CODTime>
                <Identifier>0</Identifier>
                <CODTimePoly order1="1" order2="0">
                    <Coef exponent1="0" exponent2="0">1.9918877030854505</Coef>
                    <Coef exponent1="1" exponent2="0">9.1568990193021885E-05</Coef>
                </CODTimePoly>
            </CODTime>
            <NumDwellTimes>1</NumDwellTimes>
            <DwellTime>
                <Identifier>0</Identifier>
                <DwellTimePoly order1="1" order2="1">
                    <Coef exponent1="0" exponent2="0">1.8313787558909327</Coef>
                    <Coef exponent1="0" exponent2="1">1.3140501921389402E-08</Coef>
                    <Coef exponent1="1" exponent2="0">-3.7338977230569272E-06</Coef>
                    <Coef exponent1="1" exponent2="1">7.4301160417127226E-12</Coef>
                </DwellTimePoly>
            </DwellTime>
        </Dwell>"#;

        let dwell: Dwell = quick_xml::de::from_str(xml_data).unwrap();

        assert_eq!(dwell.num_cod_times, 1);
        assert_eq!(dwell.cod_time.len(), 1);
        assert_eq!(dwell.cod_time[0].identifier, "0");
        assert_eq!(dwell.cod_time[0].cod_time_poly.order1, 1);
        assert_eq!(dwell.cod_time[0].cod_time_poly.order2, 0);
        assert_eq!(dwell.cod_time[0].cod_time_poly.coef.len(), 2);
        assert_eq!(dwell.cod_time[0].cod_time_poly.coef[0].value, 1.9918877030854505);

        assert_eq!(dwell.num_dwell_times, 1);
        assert_eq!(dwell.dwell_time.len(), 1);
        assert_eq!(dwell.dwell_time[0].identifier, "0");
        assert_eq!(dwell.dwell_time[0].dwell_time_poly.order1, 1);
        assert_eq!(dwell.dwell_time[0].dwell_time_poly.order2, 1);
        assert_eq!(dwell.dwell_time[0].dwell_time_poly.coef.len(), 4);
        assert_eq!(dwell.dwell_time[0].dwell_time_poly.coef[0].value, 1.8313787558909327);
    }
}
