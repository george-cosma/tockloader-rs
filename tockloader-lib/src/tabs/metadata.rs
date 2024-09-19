// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright OXIDOS AUTOMOTIVE 2024.

use crate::errors::TockloaderError;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub(super) struct Metadata {
    #[serde(rename = "tab-version")]
    pub tab_version: i64,
    pub name: String,
    #[serde(rename = "minimum-tock-kernel-version")]
    pub minimum_tock_kernel_version: TockKernelVersion,
    #[serde(rename = "build-date")]
    pub build_date: toml::value::Datetime,
    #[serde(
        default,
        deserialize_with = "deserialize_boards",
        rename = "only-for-boards"
    )]
    pub only_for_boards: Option<Vec<String>>,
}

impl Metadata {
    pub fn new(metadata: String) -> Result<Self, TockloaderError> {
        toml::from_str(&metadata).map_err(TockloaderError::InvalidMetadata)
    }
}

fn deserialize_boards<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    Ok(opt.map(|s| s.split(',').map(|s| s.trim().to_string()).collect()))
}

#[allow(dead_code)]
#[derive(Debug)]
pub(super) struct TockKernelVersion {
    pub major: u32,
    pub minor: u32,
}

impl<'de> Deserialize<'de> for TockKernelVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 2 {
            return Err(serde::de::Error::custom(
                "Invalid version string. It needs to contain exactly one dot.",
            ));
        }

        let major = parts[0]
            .parse::<u32>()
            .map_err(|_| serde::de::Error::custom("Invalid Major Version"))?;
        let minor = parts[1]
            .parse::<u32>()
            .map_err(|_| serde::de::Error::custom("Invalid Minor Version"))?;

        Ok(TockKernelVersion { major, minor })
    }
}
