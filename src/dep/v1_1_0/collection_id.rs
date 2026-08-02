use serde::{Deserialize, Serialize};
use super::{ModeType, CollectType};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CollectionId {
    #[serde(rename = "CollectorName")]
    pub collector_name: String,

    #[serde(rename = "IlluminatorName", default)]
    pub illuminator_name: Option<String>,

    #[serde(rename = "CoreName")]
    pub core_name: String,

    #[serde(rename = "CollectType")]
    pub collect_type: CollectType,

    #[serde(rename = "RadarMode")]
    pub radar_mode: RadarMode,

    #[serde(rename = "Classification", default = "default_classification")]
    pub classification: String,

    #[serde(rename = "ReleaseInfo", default = "default_release_info")]
    pub release_info: String,

    #[serde(rename = "CountryCode", default)]
    pub country_code: Option<Vec<String>>,

    #[serde(rename = "Parameter", default)]
    pub parameter: Vec<Parameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RadarMode {
    #[serde(rename = "ModeType")]
    pub mode_type: ModeType,

    #[serde(rename = "ModeID", default)]
    pub mode_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Parameter {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    pub value: String,
}

fn default_classification() -> String {
    "UNCLASSIFIED".to_string()
}

fn default_release_info() -> String {
    "UNRESTRICTED".to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_collection_id() {
        let xml_str = r#"
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
            </CollectionID>"#;

        let collection_id: CollectionId = quick_xml::de::from_str(xml_str).unwrap();
        
        assert_eq!(collection_id.collector_name, "capella-radar-3");
        assert_eq!(collection_id.collect_type, CollectType::Monostatic);
        assert_eq!(collection_id.radar_mode.mode_type, ModeType::Stripmap);
        assert_eq!(collection_id.classification, "UNCLASSIFIED");
        assert_eq!(collection_id.parameter.len(), 1);
        assert_eq!(collection_id.parameter[0].name, "capella_collect_id");
        assert_eq!(collection_id.parameter[0].value, "1de7df48-6bf4-464e-a962-1d2ea76b3262");
    }
}









