use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PvpField {
    #[serde(rename = "Offset")]
    pub offset: u32,
    #[serde(rename = "Size")]
    pub size: u32,
    #[serde(rename = "Format")]
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct AddedPvpField {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Offset")]
    pub offset: u32,
    #[serde(rename = "Size")]
    pub size: u32,
    #[serde(rename = "Format")]
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Pvp {
    #[serde(rename = "TxTime")]
    pub tx_time: PvpField,
    #[serde(rename = "TxPos")]
    pub tx_pos: PvpField,
    #[serde(rename = "TxVel")]
    pub tx_vel: PvpField,
    
    #[serde(rename = "RcvTime")]
    pub rcv_time: PvpField,
    #[serde(rename = "RcvPos")]
    pub rcv_pos: PvpField,
    #[serde(rename = "RcvVel")]
    pub rcv_vel: PvpField,
    
    #[serde(rename = "SRPPos")]
    pub srp_pos: PvpField,
    
    #[serde(rename = "AmpSF", skip_serializing_if = "Option::is_none", default)]
    pub amp_sf: Option<PvpField>,
    
    #[serde(rename = "aFDOP")]
    pub a_fdop: PvpField,
    #[serde(rename = "aFRR1")]
    pub a_frr1: PvpField,
    #[serde(rename = "aFRR2")]
    pub a_frr2: PvpField,
    
    #[serde(rename = "FX1")]
    pub fx1: PvpField,
    #[serde(rename = "FX2")]
    pub fx2: PvpField,
    
    #[serde(rename = "FXN1", skip_serializing_if = "Option::is_none", default)]
    pub fxn1: Option<PvpField>,
    #[serde(rename = "FXN2", skip_serializing_if = "Option::is_none", default)]
    pub fxn2: Option<PvpField>,
    
    #[serde(rename = "TOA1")]
    pub toa1: PvpField,
    #[serde(rename = "TOA2")]
    pub toa2: PvpField,
    
    #[serde(rename = "TOAE1", skip_serializing_if = "Option::is_none", default)]
    pub toae1: Option<PvpField>,
    #[serde(rename = "TOAE2", skip_serializing_if = "Option::is_none", default)]
    pub toae2: Option<PvpField>,
    
    #[serde(rename = "TDTropoSRP")]
    pub td_tropo_srp: PvpField,
    #[serde(rename = "TDIonoSRP", skip_serializing_if = "Option::is_none", default)]
    pub td_iono_srp: Option<PvpField>,
    
    #[serde(rename = "SC0")]
    pub sc0: PvpField,
    #[serde(rename = "SCSS")]
    pub scss: PvpField,
    
    #[serde(rename = "SIGNAL", skip_serializing_if = "Option::is_none", default)]
    pub signal: Option<PvpField>,
    
    // Support multiple or optional user-defined added PVPs
    #[serde(rename = "AddedPVP", skip_serializing_if = "Option::is_none", default)]
    pub added_pvp: Option<Vec<AddedPvpField>>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pvp_block_deserialization() {
        let xml_data = r#"<PVP>
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
        </PVP>"#;

        let pvp: Pvp = quick_xml::de::from_str(xml_data).unwrap();

        assert_eq!(pvp.tx_time.offset, 0);
        assert_eq!(pvp.tx_pos.offset, 1);
        assert_eq!(pvp.tx_pos.size, 3);
        assert_eq!(pvp.added_pvp.as_ref().unwrap()[0].name, "RefTime");
    }
}
