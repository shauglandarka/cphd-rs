pub mod channel;
pub mod collection_id;
pub mod data;
pub mod dwell;
pub mod file_hdr;
pub mod global;
pub mod pvp;
pub mod reference_geometry;
pub mod scene_coordinates;

use channel::Channel;
use collection_id::CollectionId;
use data::Data;
use dwell::Dwell;
use global::Global;
use pvp::Pvp;
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

    #[serde(rename = "SceneCoordinates")]
    pub scene_coordinates: SceneCoordinates,

    #[serde(rename = "Data")]
    pub data: Data,

    #[serde(rename = "Channel")]
    pub channel: Channel,

    #[serde(rename = "PVP")]
    pub pvp: Pvp,

    #[serde(rename = "Dwell")]
    pub dwell: Dwell,
    //
    //    #[serde(rename = "ReferenceGeometry")]
    //    pub reference_geometry: ReferenceGeometry,
    //
    //    #[serde(rename = "SupportArray")]
    //    pub support_array: Option<SupportArray>,
    //
    //    #[serde(rename = "Antenna")]
    //    pub antenna: Option<Antenna>,
    //
    //    #[serde(rename = "TxRcv")]
    //    pub tx_rcv: Option<TxRcv>,
    //
    //    #[serde(rename = "ErrorParameters")]
    //    pub error_parameters: Option<ErrorParameters>,
    //
    //    #[serde(rename = "ProductInfo")]
    //    pub product_info: Option<ProductInfo>,
    //
    //    #[serde(rename = "GeoInfo")]
    //    pub geo_info: Option<GeoInfo>,
    //
    //    #[serde(rename = "MatchInfo")]
    //    pub match_info: Option<MatchInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Parameter {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Iarp {
    #[serde(rename = "ECF")]
    pub ecf: Vector3D,
    #[serde(rename = "LLH")]
    pub llh: Llh,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vector3D {
    #[serde(rename = "X")]
    pub x: f64,
    #[serde(rename = "Y")]
    pub y: f64,
    #[serde(rename = "Z")]
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Llh {
    #[serde(rename = "Lat")]
    pub lat: f64,
    #[serde(rename = "Lon")]
    pub lon: f64,
    #[serde(rename = "HAE")]
    pub hae: f64,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Poly {
    #[serde(rename = "@order1")]
    pub order1: String,
    #[serde(rename = "Coef")]
    pub coeffs: Vec<Coef>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Poly2D {
    #[serde(rename = "@order1")]
    pub order1: u32,
    #[serde(rename = "@order2")]
    pub order2: u32,
    #[serde(rename = "Coef")]
    pub coef: Vec<Coef>,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
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
                <SceneCoordinates>
                    <EarthModel>WGS_84</EarthModel>
                    <IARP>
                        <ECF>
                            <X>-2709779.4616113761</X>
                            <Y>-4257549.4936505863</Y>
                            <Z>3887083.951894992</Z>
                        </ECF>
                        <LLH>
                            <Lat>37.790247983650183</Lat>
                            <Lon>-122.47530880482378</Lon>
                            <HAE>19.759685250378372</HAE>
                        </LLH>
                    </IARP>
                    <ReferenceSurface>
                        <Planar>
                            <uIAX>
                                <X>0.1777943433924461</X>
                                <Y>0.60220140554004586</Y>
                                <Z>0.77829469908463478</Z>
                            </uIAX>
                            <uIAY>
                                <X>0.88788668193492959</X>
                                <Y>-0.43919253727668595</Y>
                                <Z>0.13699326714495125</Z>
                            </uIAY>
                        </Planar>
                    </ReferenceSurface>
                    <ImageArea>
                        <X1Y1>
                            <X>-12577</X>
                            <Y>-50307</Y>
                        </X1Y1>
                        <X2Y2>
                            <X>12576</X>
                            <Y>50307</Y>
                        </X2Y2>
                    </ImageArea>
                    <ImageAreaCornerPoints>
                        <IACP index="1">
                            <Lat>37.697610922734711</Lat>
                            <Lon>-122.48356893285361</Lon>
                        </IACP>
                        <IACP index="2">
                            <Lat>37.875065080067998</Lat>
                            <Lon>-122.52299515786021</Lon>
                        </IACP>
                        <IACP index="3">
                            <Lat>37.882883003776371</Lat>
                            <Lon>-122.46702802667032</Lon>
                        </IACP>
                        <IACP index="4">
                            <Lat>37.705410406804127</Lat>
                            <Lon>-122.42773135658221</Lon>
                        </IACP>
                    </ImageAreaCornerPoints>
                    <ImageGrid>
                        <Identifier>CAPELLA_C03_SM_CPHD_HH_20211229053627_20211229053631</Identifier>
                        <IARPLocation>
                            <Line>0</Line>
                            <Sample>0</Sample>
                        </IARPLocation>
                        <IAXExtent>
                            <LineSpacing>0.19877938015992294</LineSpacing>
                            <FirstLine>-12577</FirstLine>
                            <NumLines>25153</NumLines>
                        </IAXExtent>
                        <IAYExtent>
                            <SampleSpacing>0.19877938015992294</SampleSpacing>
                            <FirstSample>-50307</FirstSample>
                            <NumSamples>100614</NumSamples>
                        </IAYExtent>
                    </ImageGrid>
                </SceneCoordinates>
                <Data>
                    <SignalArrayFormat>CI4</SignalArrayFormat>
                    <NumBytesPVP>264</NumBytesPVP>
                    <NumCPHDChannels>1</NumCPHDChannels>
                    <Channel>
                        <Identifier>0</Identifier>
                        <NumVectors>40290</NumVectors>
                        <NumSamples>8193</NumSamples>
                        <SignalArrayByteOffset>0</SignalArrayByteOffset>
                        <PVPArrayByteOffset>0</PVPArrayByteOffset>
                    </Channel>
                    <NumSupportArrays>0</NumSupportArrays>
                </Data>
                <Channel>
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
                </Channel>
                <PVP>
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
                </PVP>
                <Dwell>
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
                </Dwell>
                <ReferenceGeometry>
                    <SRP>
                        <ECF>
                            <X>-2709779.4616113761</X>
                            <Y>-4257549.4936505863</Y>
                            <Z>3887083.951894992</Z>
                        </ECF>
                        <IAC>
                            <X>0</X>
                            <Y>0</Y>
                            <Z>0</Z>
                        </IAC>
                    </SRP>
                    <ReferenceTime>1.99193707028013844E+00</ReferenceTime>
                    <SRPCODTime>1.99188770308545049E+00</SRPCODTime>
                    <SRPDwellTime>1.83137875589093269E+00</SRPDwellTime>
                    <Monostatic>
                        <ARPPos>
                            <X>-3271530.2616146682</X>
                            <Y>-4431276.8216099814</Y>
                            <Z>4149831.4124072385</Z>
                        </ARPPos>
                        <ARPVel>
                            <X>1370.0478545408178</X>
                            <Y>4615.9389623900661</Y>
                            <Z>5980.8532693009474</Z>
                        </ARPVel>
                        <SideOfTrack>R</SideOfTrack>
                        <SlantRange>6.44035227134265006E+05</SlantRange>
                        <GroundRange>3.56908715580854681E+05</GroundRange>
                        <DopplerConeAngle>8.99990010987138334E+01</DopplerConeAngle>
                        <GrazeAngle>5.31239259530187766E+01</GrazeAngle>
                        <IncidenceAngle>3.68760740469812234E+01</IncidenceAngle>
                        <AzimuthAngle>2.60017213877463917E+02</AzimuthAngle>
                        <TwistAngle>-7.85231242270209295E-02</TwistAngle>
                        <SlopeAngle>5.31239663176626209E+01</SlopeAngle>
                        <LayoverAngle>2.60115375685010122E+02</LayoverAngle>
                    </Monostatic>
                </ReferenceGeometry>
                <Antenna>
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
                </Antenna>
                <TxRcv>
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
                </TxRcv>
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
        assert_eq!(
            meta.global.timeline.collection_start,
            "2021-12-29T05:36:31.000000Z"
        );
        assert_eq!(meta.global.fx_band.fx_min, 9549999872.0);

        let tropo = meta.global.tropo_parameters.as_ref().unwrap();
        assert_eq!(tropo.n0, 1.0);
        assert_eq!(tropo.ref_height, RefHeight::Iarp);
    }
}
