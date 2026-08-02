use super::Vector3D;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReferenceGeometry {
    #[serde(rename = "SRP")]
    pub srp: Srp,

    #[serde(rename = "ReferenceTime")]
    pub reference_time: f64,

    #[serde(rename = "SRPCODTime")]
    pub srp_cod_time: f64,

    #[serde(rename = "SRPDwellTime")]
    pub srp_dwell_time: f64,

    #[serde(rename = "Monostatic", skip_serializing_if = "Option::is_none")]
    pub monostatic: Option<Monostatic>,

    #[serde(rename = "Bistatic", skip_serializing_if = "Option::is_none")]
    pub bistatic: Option<Bistatic>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Srp {
    #[serde(rename = "ECF")]
    pub ecf: Vector3D,

    #[serde(rename = "IAC")]
    pub iac: Vector3D,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Monostatic {
    #[serde(rename = "ARPPos")]
    pub arp_pos: Vector3D,

    #[serde(rename = "ARPVel")]
    pub arp_vel: Vector3D,

    #[serde(rename = "SideOfTrack")]
    pub side_of_track: SideOfTrack,

    #[serde(rename = "SlantRange")]
    pub slant_range: f64,

    #[serde(rename = "GroundRange")]
    pub ground_range: f64,

    #[serde(rename = "DopplerConeAngle")]
    pub doppler_cone_angle: f64,

    #[serde(rename = "GrazeAngle")]
    pub graze_angle: f64,

    #[serde(rename = "IncidenceAngle")]
    pub incidence_angle: f64,

    #[serde(rename = "AzimuthAngle")]
    pub azimuth_angle: f64,

    #[serde(rename = "TwistAngle")]
    pub twist_angle: f64,

    #[serde(rename = "SlopeAngle")]
    pub slope_angle: f64,

    #[serde(rename = "LayoverAngle")]
    pub layover_angle: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Bistatic {
    #[serde(rename = "AzimuthAngle")]
    pub azimuth_angle: f64,

    #[serde(rename = "AzimuthAngleRate")]
    pub azimuth_angle_rate: f64,

    #[serde(rename = "BistaticAngle")]
    pub bistatic_angle: f64,

    #[serde(rename = "BistaticAngleRate")]
    pub bistatic_angle_rate: f64,

    #[serde(rename = "GrazeAngle")]
    pub graze_angle: f64,

    #[serde(rename = "TwistAngle")]
    pub twist_angle: f64,

    #[serde(rename = "SlopeAngle")]
    pub slope_angle: f64,

    #[serde(rename = "LayoverAngle")]
    pub layover_angle: f64,

    #[serde(rename = "TxPlatform")]
    pub tx_platform: Platform,

    #[serde(rename = "RcvPlatform")]
    pub rcv_platform: Platform,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Platform {
    #[serde(rename = "Time")]
    pub time: f64,

    #[serde(rename = "Pos")]
    pub pos: Vector3D,

    #[serde(rename = "Vel")]
    pub vel: Vector3D,

    #[serde(rename = "SideOfTrack")]
    pub side_of_track: SideOfTrack,

    #[serde(rename = "SlantRange")]
    pub slant_range: f64,

    #[serde(rename = "GroundRange")]
    pub ground_range: f64,

    #[serde(rename = "DopplerConeAngle")]
    pub doppler_cone_angle: f64,

    #[serde(rename = "GrazeAngle")]
    pub graze_angle: f64,

    #[serde(rename = "IncidenceAngle")]
    pub incidence_angle: f64,

    #[serde(rename = "AzimuthAngle")]
    pub azimuth_angle: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SideOfTrack {
    #[serde(rename = "L")]
    Left,
    #[serde(rename = "R")]
    Right,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_geometry_deserialization() {
        let xml_data = r#"<ReferenceGeometry>
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
        </ReferenceGeometry>"#;

        let ref_geom: ReferenceGeometry = quick_xml::de::from_str(xml_data).unwrap();

        assert_eq!(ref_geom.reference_time, 1.99193707028013844E+00);
        assert_eq!(ref_geom.srp.ecf.x, -2709779.4616113761);

        match ref_geom.monostatic {
            Some(mono) => {
                assert_eq!(mono.side_of_track, SideOfTrack::Right);
                assert_eq!(mono.slant_range, 6.44035227134265006E+05);
                assert_eq!(mono.arp_pos.z, 4149831.4124072385);
            }
            None => panic!("Expected monostatic variant"),
        }
        assert!(ref_geom.bistatic.is_none());
    }
}
