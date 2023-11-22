use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ArtifactConfig {
    pub artifacts: Vec<Artifact>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Artifact {
    pub group: String,

    pub feeds: Vec<Feed>,
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Feed {
    pub url: String,
}
