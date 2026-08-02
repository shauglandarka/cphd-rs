use super::{Poly, Poly2D, Vector3D, XyzPoly};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Antenna {
    #[serde(rename = "NumACFs")]
    pub num_acfs: i32,

    #[serde(rename = "NumAPCs")]
    pub num_apcs: i32,

    #[serde(rename = "NumAntPats")]
    pub num_ant_pats: i32,

    #[serde(rename = "AntCoordFrame")]
    pub ant_coord_frame: Vec<AntCoordFrame>,

    #[serde(rename = "AntPhaseCenter")]
    pub ant_phase_center: Vec<AntPhaseCenter>,

    #[serde(rename = "AntPattern")]
    pub ant_pattern: Vec<AntPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntCoordFrame {
    #[serde(rename = "Identifier")]
    pub identifier: String,

    #[serde(rename = "XAxisPoly")]
    pub x_axis_poly: XyzPoly,

    #[serde(rename = "YAxisPoly")]
    pub y_axis_poly: XyzPoly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntPhaseCenter {
    #[serde(rename = "Identifier")]
    pub identifier: String,
    #[serde(rename = "ACFId")]
    pub acf_id: String,
    #[serde(rename = "APCXYZ")]
    pub apc_xyz: Vector3D,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntPattern {
    #[serde(rename = "Identifier")]
    pub identifier: String,
    #[serde(rename = "FreqZero")]
    pub freq_zero: f64,
    #[serde(rename = "GainZero")]
    pub gain_zero: f64,
    #[serde(rename = "EBFreqShift")]
    pub eb_freq_shift: bool,
    #[serde(rename = "MLFreqDilation")]
    pub ml_freq_dilation: bool,
    #[serde(rename = "EB")]
    pub eb: Eb,
    #[serde(rename = "Array")]
    pub array: PatternDetails,
    #[serde(rename = "Element")]
    pub element: PatternDetails,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Eb {
    #[serde(rename = "DCXPoly")]
    pub dcx_poly: Poly,
    #[serde(rename = "DCYPoly")]
    pub dcy_poly: Poly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatternDetails {
    #[serde(rename = "GainPoly")]
    pub gain_poly: Poly2D,
    #[serde(rename = "PhasePoly")]
    pub phase_poly: Poly2D,
}

#[test]
fn test_antenna_deserialization() {
    let xml_data = r#"<Antenna>
        <NumACFs>1</NumACFs>
        <NumAPCs>1</NumAPCs>
        <NumAntPats>1</NumAntPats>
        <AntCoordFrame>
            <Identifier>0</Identifier>
            <XAxisPoly>
                <X order1="5">
                    <Coef exponent1="0">0.1772764491660346</Coef>
                    <Coef exponent1="1">0.00044235303753281267</Coef>
                    <Coef exponent1="2">0.0002596996054404287</Coef>
                    <Coef exponent1="3">-0.0001590834062993129</Coef>
                    <Coef exponent1="4">4.017221520460876e-05</Coef>
                    <Coef exponent1="5">-3.596595273639013e-06</Coef>
                </X>
                <Y order1="5">
                    <Coef exponent1="0">0.5998059418706748</Coef>
                    <Coef exponent1="1">0.0007171724462433608</Coef>
                    <Coef exponent1="2">-8.622407833918917e-06</Coef>
                    <Coef exponent1="3">-1.3740163719007962e-05</Coef>
                    <Coef exponent1="4">7.92166031198421e-06</Coef>
                    <Coef exponent1="5">-1.1041682623821175e-06</Coef>
                </Y>
                <Z order1="5">
                    <Coef exponent1="0">0.7802601298738472</Coef>
                    <Coef exponent1="1">-0.0006516169811630454</Coef>
                    <Coef exponent1="2">-5.362304254669119e-05</Coef>
                    <Coef exponent1="3">4.7032450021716526e-05</Coef>
                    <Coef exponent1="4">-1.5314704351393382e-05</Coef>
                    <Coef exponent1="5">1.6761859322064994e-06</Coef>
                </Z>
            </XAxisPoly>
            <YAxisPoly>
                <X order1="5">
                    <Coef exponent1="0">0.45558150354691335</Coef>
                    <Coef exponent1="1">0.00016657880969320971</Coef>
                    <Coef exponent1="2">-9.278984189712819e-05</Coef>
                    <Coef exponent1="3">5.244781032538926e-05</Coef>
                    <Coef exponent1="4">-1.30283399005913e-05</Coef>
                    <Coef exponent1="5">1.1665214821549382e-06</Coef>
                </X>
                <Y order1="5">
                    <Coef exponent1="0">-0.7527706072743596</Coef>
                    <Coef exponent1="1">0.00040472724137154717</Coef>
                    <Coef exponent1="2">-8.276697629078065e-05</Coef>
                    <Coef exponent1="3">3.7923873546885376e-05</Coef>
                    <Coef exponent1="4">-6.786016220616025e-06</Coef>
                    <Coef exponent1="5">3.6591276175065636e-07</Coef>
                </Y>
                <Z order1="5">
                    <Coef exponent1="0">0.47516514876823657</Coef>
                    <Coef exponent1="1">0.0004813540053288337</Coef>
                    <Coef exponent1="2">-4.238523631523208e-05</Coef>
                    <Coef exponent1="3">9.772811986905847e-06</Coef>
                    <Coef exponent1="4">1.733068088989393e-06</Coef>
                    <Coef exponent1="5">-5.367832684223304e-07</Coef>
                </Z>
            </YAxisPoly>
        </AntCoordFrame>
        <AntPhaseCenter>
            <Identifier>0</Identifier>
            <ACFId>0</ACFId>
            <APCXYZ>
                <X>0</X>
                <Y>0</Y>
                <Z>0</Z>
            </APCXYZ>
        </AntPhaseCenter>
        <AntPattern>
            <Identifier>0</Identifier>
            <FreqZero>9.65000000000000000E+09</FreqZero>
            <GainZero>4.40799999999999983E+01</GainZero>
            <EBFreqShift>false</EBFreqShift>
            <MLFreqDilation>false</MLFreqDilation>
            <EB>
                <DCXPoly order1="0">
                    <Coef exponent1="0">0.0</Coef>
                </DCXPoly>
                <DCYPoly order1="0">
                    <Coef exponent1="0">0.0</Coef>
                </DCYPoly>
            </EB>
            <Array>
                <GainPoly order1="2" order2="2">
                    <Coef exponent1="0" exponent2="0">-1.0514606028229732</Coef>
                    <Coef exponent1="0" exponent2="1">5.1368260756134987E-09</Coef>
                    <Coef exponent1="0" exponent2="2">-39952.047225149538</Coef>
                    <Coef exponent1="1" exponent2="0">-1.5070738419772101E-08</Coef>
                    <Coef exponent1="1" exponent2="1">-0.00067821890137584567</Coef>
                    <Coef exponent1="1" exponent2="2">-5.9178372912111324E-08</Coef>
                    <Coef exponent1="2" exponent2="0">-39153.937350953376</Coef>
                    <Coef exponent1="2" exponent2="1">-3.7709434668717851E-08</Coef>
                    <Coef exponent1="2" exponent2="2">201366160.95626393</Coef>
                </GainPoly>
                <PhasePoly order1="0" order2="0">
                    <Coef exponent1="0" exponent2="0">0</Coef>
                </PhasePoly>
            </Array>
            <Element>
                <GainPoly order1="0" order2="0">
                    <Coef exponent1="0" exponent2="0">0</Coef>
                </GainPoly>
                <PhasePoly order1="0" order2="0">
                    <Coef exponent1="0" exponent2="0">0</Coef>
                </PhasePoly>
            </Element>
        </AntPattern>
    </Antenna>"#;

    let antenna: Antenna = quick_xml::de::from_str(xml_data).unwrap();

    assert_eq!(antenna.num_acfs, 1);
    assert_eq!(antenna.num_apcs, 1);
    assert_eq!(antenna.num_ant_pats, 1);
    assert_eq!(antenna.ant_coord_frame[0].identifier, "0");
    assert_eq!(antenna.ant_phase_center[0].acf_id, "0");
    assert_eq!(antenna.ant_pattern[0].freq_zero, 9.65e9);
    assert_eq!(antenna.ant_pattern[0].array.gain_poly.coef.len(), 9);
}
