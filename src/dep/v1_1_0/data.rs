use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "SignalArrayFormat")]
    pub signal_array_format: SignalArrayFormat,

    #[serde(rename = "NumBytesPVP")]
    pub num_bytes_pvp: u64,

    #[serde(rename = "NumCPHDChannels")]
    pub num_cphd_channels: usize,

    #[serde(rename = "SignalCompressionID", default)]
    pub signal_compression_id: Option<String>,

    #[serde(rename = "Channel")]
    pub channel: Vec<ChannelData>,

    #[serde(rename = "NumSupportArrays")]
    pub num_support_arrays: usize,

    #[serde(rename = "SupportArray", default)]
    pub support_array: Vec<SupportArray>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelData {
    #[serde(rename = "Identifier")]
    pub identifier: String,

    #[serde(rename = "NumVectors")]
    pub num_vectors: u64,

    #[serde(rename = "NumSamples")]
    pub num_samples: u64,

    #[serde(rename = "SignalArrayByteOffset")]
    pub signal_array_byte_offset: u64,

    #[serde(rename = "PVPArrayByteOffset")]
    pub pvp_array_byte_offset: u64,

    #[serde(rename = "CompressedSignalSize", default)]
    pub compressed_signal_size: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportArray {
    #[serde(rename = "Identifier")]
    pub identifier: String,

    #[serde(rename = "NumRows")]
    pub num_rows: u64,

    #[serde(rename = "NumCols")]
    pub num_cols: u64,

    #[serde(rename = "BytesPerElement")]
    pub bytes_per_element: u64,

    #[serde(rename = "ArrayByteOffset")]
    pub array_byte_offset: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SignalArrayFormat {
    #[serde(rename = "CI2")]
    CI2,
    #[serde(rename = "CI4")]
    CI4,
    #[serde(rename = "CF8")]
    CF8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_data_block() {
        let xml_str = r#"
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
                  <NumSupportArrays>2</NumSupportArrays>
                  <SupportArray>
                      <Identifier>SA_01</Identifier>
                      <NumRows>100</NumRows>
                      <NumCols>50</NumCols>
                      <BytesPerElement>4</BytesPerElement>
                      <ArrayByteOffset>0</ArrayByteOffset>
                  </SupportArray>
                  <SupportArray>
                      <Identifier>SA_02</Identifier>
                      <NumRows>200</NumRows>
                      <NumCols>50</NumCols>
                      <BytesPerElement>4</BytesPerElement>
                      <ArrayByteOffset>20000</ArrayByteOffset>
                  </SupportArray>
              </Data>"#;

        let data: Data = quick_xml::de::from_str(xml_str).expect("Failed to parse Data block");

        assert_eq!(data.signal_array_format, SignalArrayFormat::CI4);
        assert_eq!(data.num_bytes_pvp, 264);
        assert_eq!(data.num_cphd_channels, 1);
        
        assert_eq!(data.channel.len(), 1);
        assert_eq!(data.channel[0].identifier, "0");
        assert_eq!(data.channel[0].num_vectors, 40290);
        assert_eq!(data.channel[0].num_samples, 8193);
        assert_eq!(data.channel[0].signal_array_byte_offset, 0);
        assert_eq!(data.channel[0].pvp_array_byte_offset, 0);
        assert_eq!(data.channel[0].compressed_signal_size, None);

        assert_eq!(data.num_support_arrays, 2);
        assert_eq!(data.support_array.len(), 2);
        
        assert_eq!(data.support_array[0].identifier, "SA_01");
        assert_eq!(data.support_array[0].num_rows, 100);
        assert_eq!(data.support_array[0].num_cols, 50);
        assert_eq!(data.support_array[0].bytes_per_element, 4);
        assert_eq!(data.support_array[0].array_byte_offset, 0);

        assert_eq!(data.support_array[1].identifier, "SA_02");
        assert_eq!(data.support_array[1].num_rows, 200);
        assert_eq!(data.support_array[1].num_cols, 50);
        assert_eq!(data.support_array[1].bytes_per_element, 4);
        assert_eq!(data.support_array[1].array_byte_offset, 20000);

    }
}
