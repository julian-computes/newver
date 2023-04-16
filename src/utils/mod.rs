pub mod ignore_before;

pub use ignore_before::*;

use std::fmt::{Display, Formatter};
use std::fs::{File};
use std::io::{Read};
use std::path::{PathBuf};
use chrono::{ DateTime, TimeZone, Utc};
use reqwest::Client;
use crate::prelude::*;

fn make_url(artifact_id: &String, group_id: &String) -> String {
    f!("https://search.maven.org/solrsearch/select?q=g:{group_id}+AND+a:{artifact_id}&core=gav&rows=1&wt=json")
}

pub struct Artifact {
    pub(crate) group_id: String,
    pub(crate) artifact_id: String,
}

pub struct ArtifactInfo {
    pub(crate) version: String,
    pub(crate) released: DateTime<Utc>,
}

impl Display for Artifact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", f!("{}:{}", self.group_id, self.artifact_id))
    }
}

impl TryFrom<&str> for Artifact {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self> {
        let mut parts = value.split(":");
        let g = parts.next();
        let a = parts.next();

        if g.is_some() && a.is_some() {
            let g = g.unwrap();
            let a = a.unwrap();
            if g.is_empty() || a.is_empty() {
                return Err(f!("invalid artifact: {value}").into());
            }
            Ok(Artifact {
                group_id: g.to_string(),
                artifact_id: a.to_string(),
            })
        } else {
            Err(f!("invalid artifact: {value}").into())
        }
    }
}

pub fn artifact_file_to_string(path: PathBuf) -> Result<String> {
    let mut buf = String::new();
    File::open(path)?.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn parse_artifact_data(config_data: String) -> Vec<Result<Artifact>> {
    config_data.lines().filter(|l| !l.is_empty()).map(|l| l.try_into()).collect()
}

pub async fn info_for(client: &Client, artifact: &Artifact) -> Result<ArtifactInfo> {
    let artifact_id = &artifact.artifact_id;
    let group_id = &artifact.group_id;

    let body = client.get(make_url(&artifact_id, &group_id)).send().await?.json::<serde_json::Value>().await?;

    let res = body.get("response").ok_or("failed to get response")?;
    let doc = res.get("docs").ok_or("Failed to get docs")?.get(0).ok_or("No docs found")?;
    let version = doc.get("v").ok_or("Failed to find version")?.as_str().ok_or("Failed to read version as string")?;
    let ts = doc.get("timestamp").unwrap().as_i64().unwrap();

    let utc = Utc.timestamp_millis_opt(ts);
    if let chrono::LocalResult::Single(date) = utc {
        Ok(ArtifactInfo {
            version: version.to_string(),
            released: date,
        })
    } else {
        Err(Error::TimeParseFail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_artifact_data() {
        let good_data = vec![
            "org.apache.commons:commons-lang3",
            "org.apache.shiro:shiro-core",
            "org.eclipse.jetty:jetty-server",
        ];

        let bad_data = vec![
            "",
            ":",
            "abc",
            "abc:",
        ];

        for data in good_data {
            let artifact: Artifact = data.try_into().unwrap();
            assert_eq!(artifact.group_id, data.split(":").next().unwrap());
            assert_eq!(artifact.artifact_id, data.split(":").last().unwrap());
        }

        for data in bad_data {
            assert!(TryInto::<Artifact>::try_into(data).is_err());
        }
    }
}