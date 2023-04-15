use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use chrono::{Datelike, TimeZone};
use reqwest::Client;
use crate::prelude::*;

fn make_url(artifact_id: &String, group_id: &String) -> String {
    f!("https://search.maven.org/solrsearch/select?q=g:{group_id}+AND+a:{artifact_id}&core=gav&rows=1&wt=json")
}

pub struct Artifact {
    pub(crate) group_id: String,
    pub(crate) artifact_id: String,
}

impl Display for Artifact {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", f!("{}:{}", self.group_id, self.artifact_id))
    }
}

impl TryFrom<String> for Artifact {
    type Error = Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        let mut parts = value.split(":");
        let g = parts.next().ok_or("Malformed artifact")?;
        let a = parts.next().ok_or("Malformed group")?;
        Ok(Artifact {
            group_id: g.to_string(),
            artifact_id: a.to_string(),
        })
    }
}

pub fn read_data() -> Result<Vec<Artifact>> {
    let reader = BufReader::new(File::open("data/artifacts")?);

    Ok(reader.lines().map(|line| line.unwrap().try_into().unwrap()).collect())
}

pub async fn info_for(client: &Client, artifact: &Artifact) -> Result<String> {

    let artifact_id = &artifact.artifact_id;
    let group_id = &artifact.group_id;

    let body = client.get(make_url(&artifact_id, &group_id)).send().await?.json::<serde_json::Value>().await?;

    let res = body.get("response").ok_or("failed to get response")?;
    let doc = res.get("docs").ok_or("Failed to get docs")?.get(0).ok_or("No docs found")?;
    let version = doc.get("v").ok_or("Failed to find version")?.as_str().ok_or("Failed to read version as string")?;
    let ts = doc.get("timestamp").unwrap().as_i64().unwrap();

    let utc = chrono::Utc.timestamp_millis_opt(ts);
    if let chrono::LocalResult::Single(date) = utc {

        Ok(f!("{} version {}\nreleased {}-{}-{}", artifact_id, version, date.month(), date.day(), date.year()))
    } else {
        Err(Error::TimeParseFail)
    }
}