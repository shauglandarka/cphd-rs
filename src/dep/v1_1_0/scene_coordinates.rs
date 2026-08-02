use serde::{Deserialize, Serialize};
use super::{Iarp, Ecf, Llh, EarthModel};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneCoordinates {
    #[serde(rename = "EarthModel")]
    pub earth_model: EarthModel,

    #[serde(rename = "IARP")]
    pub iarp: Iarp,

    #[serde(rename = "ReferenceSurface")]
    pub reference_surface: ReferenceSurface,

    #[serde(rename = "ImageArea")]
    pub image_area: ImageArea,

    #[serde(rename = "ImageAreaCornerPoints")]
    pub image_area_corner_points: ImageAreaCornerPoints,

    #[serde(rename = "ExtendedArea", default)]
    pub extended_area: Option<ExtendedArea>,

    #[serde(rename = "ImageGrid", default)]
    pub image_grid: Option<ImageGrid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferenceSurface {
    #[serde(rename = "Planar", default)]
    pub planar: Option<Planar>,

    #[serde(rename = "HAE", default)]
    pub hae: Option<HaeSurface>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Planar {
    #[serde(rename = "uIAX")]
    pub u_iax: Ecf,

    #[serde(rename = "uIAY")]
    pub u_iay: Ecf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HaeSurface {
    // HAE branch parameters if populated
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageArea {
    #[serde(rename = "X1Y1")]
    pub x1_y1: PointXy,

    #[serde(rename = "X2Y2")]
    pub x2_y2: PointXy,

    #[serde(rename = "Polygon", default)]
    pub polygon: Option<Polygon>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointXy {
    #[serde(rename = "X")]
    pub x: i32,

    #[serde(rename = "Y")]
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polygon {
    #[serde(rename = "@size")]
    pub size: usize,

    #[serde(rename = "Vertex")]
    pub vertex: Vec<Vertex>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vertex {
    #[serde(rename = "@index")]
    pub index: usize,

    #[serde(rename = "X")]
    pub x: i32,

    #[serde(rename = "Y")]
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageAreaCornerPoints {
    #[serde(rename = "IACP")]
    pub iacp: Vec<Iacp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendedArea {
    #[serde(rename = "X1Y1")]
    pub x1_y1: PointXy,

    #[serde(rename = "X2Y2")]
    pub x2_y2: PointXy,

    #[serde(rename = "Polygon", default)]
    pub polygon: Option<Polygon>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iacp {
    #[serde(rename = "@index")]
    pub index: usize,

    #[serde(rename = "Lat")]
    pub lat: f64,

    #[serde(rename = "Lon")]
    pub lon: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageGrid {
    #[serde(rename = "Identifier", default)]
    pub identifier: Option<String>,

    #[serde(rename = "IARPLocation")]
    pub iarp_location: IarpLocation,

    #[serde(rename = "IAXExtent")]
    pub iax_extent: IaxExtent,

    #[serde(rename = "IAYExtent")]
    pub iay_extent: IayExtent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IarpLocation {
    #[serde(rename = "Line")]
    pub line: f64,

    #[serde(rename = "Sample")]
    pub sample: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IaxExtent {
    #[serde(rename = "LineSpacing")]
    pub line_spacing: f64,

    #[serde(rename = "FirstLine")]
    pub first_line: i32,

    #[serde(rename = "NumLines")]
    pub num_lines: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IayExtent {
    #[serde(rename = "SampleSpacing")]
    pub sample_spacing: f64,

    #[serde(rename = "FirstSample")]
    pub first_sample: i32,

    #[serde(rename = "NumSamples")]
    pub num_samples: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_scene_coordinates() {
        let xml_str = r#"
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
            </SceneCoordinates>"#;


        let sc: SceneCoordinates = quick_xml::de::from_str(xml_str).unwrap();

        // Test EarthModel & IARP
        assert_eq!(sc.earth_model, EarthModel::Wgs84);
        assert_eq!(sc.iarp.ecf.x, -2709779.4616113761);
        assert_eq!(sc.iarp.llh.lat, 37.790247983650183);

        // Test ReferenceSurface (Planar)
        let planar = sc.reference_surface.planar.as_ref().unwrap();
        assert_eq!(planar.u_iax.z, 0.77829469908463478);
        assert_eq!(planar.u_iay.x, 0.88788668193492959);

        // Test ImageArea
        assert_eq!(sc.image_area.x1_y1.x, -12577);
        assert_eq!(sc.image_area.x2_y2.y, 50307);

        // Test ImageAreaCornerPoints (IACP)
        assert_eq!(sc.image_area_corner_points.iacp.len(), 4);
        assert_eq!(sc.image_area_corner_points.iacp[0].index, 1);
        assert_eq!(sc.image_area_corner_points.iacp[0].lat, 37.697610922734711);


        // Test ImageGrid
        let image_grid = sc.image_grid.as_ref().expect("ImageGrid should be present");
        assert_eq!(
            image_grid.identifier.as_deref(),
            Some("CAPELLA_C03_SM_CPHD_HH_20211229053627_20211229053631")
        );
        assert_eq!(image_grid.iarp_location.line, 0.0);
        assert_eq!(image_grid.iax_extent.num_lines, 25153);
        assert_eq!(image_grid.iay_extent.num_samples, 100614);
    }
}
