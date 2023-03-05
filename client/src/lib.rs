extern crate core;

use serde::{Deserialize, Serialize};

use std::fmt;

use crate::types::RestError;
use std::fmt::{Pointer, Write};

mod rest;
mod types;

#[derive(Serialize, Deserialize, Debug)]
pub struct PartitionQoS {
    // Default 2k; one of 2k or 4k; the MTU of the services.
    pub mtu: i32,
    // Default is None, value can be range from 0-15
    pub service_level: i32,
    // Default is None, can be one of the following: 2.5, 10, 30, 5, 20, 40, 60, 80, 120, 14, 56, 112, 168, 25, 100, 200, or 300
    pub rate_limit: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PortBinding {
    // The GUID of Port.
    pub guid: String,
    // Default false; store the PKey at index 0 of the PKey table of the GUID.
    pub index0: bool,
    // Default is full:
    //   "full" - members with full membership can communicate with all hosts (members) within the network/partition
    //   "limited" - members with limited membership cannot communicate with other members with limited membership. However, communication is allowed between every other combination of membership types.
    pub membership: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Partition {
    // The name of Partition.
    pub name: String,
    // The pkeys of Partition.
    pub pkey: i32,
    // Default false
    pub ipoib: bool,
    // The QoS of Partition.
    pub qos: PartitionQoS,
    // The Ports belong to the partition
    pub guids: Vec<PortBinding>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Port {
    pub guid: String,
    pub name: String,
    pub system_id: String,
    pub lid: i32,
    pub dname: String,
    pub system_name: String,
    pub physical_state: String,
    pub logical_state: String,
}

pub struct UFM {
    client: rest::RestClient,
}

pub enum UFMError {
    Unknown,
}

impl From<types::RestError> for UFMError {
    fn from(e: types::RestError) -> Self {
        match e {
            _ => UFMError::Unknown,
        }
    }
}

impl fmt::Debug for UFMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UFMError").finish()
    }
}

impl UFM {
    pub fn new() -> Result<UFM, UFMError> {
        let restclient = rest::RestClient::new();

        match restclient {
            Ok(c) => Ok(Self { client: c }),
            Err(e) => Err(UFMError::Unknown),
        }
    }

    pub async fn create_partition(self: &mut Self, p: Partition) -> Result<(), UFMError> {
        Ok(())
    }
    pub async fn get_partition(self: &mut Self, pkey: &String) -> Result<Partition, UFMError> {
        let mut path = String::new();
        write!(
            path,
            "/ufmRest/resources/pkeys/{}?guids_data=true&qos_conf=true",
            pkey
        )
        .unwrap();

        let ps = self.client.get(&path).await?;
        let p: Partition = serde_json::from_str(&ps[..]).unwrap();

        Ok(p)
    }

    pub fn delete_partition(left: usize, right: usize) -> usize {
        left + right
    }
    pub fn patch_partition(left: usize, right: usize) -> usize {
        left + right
    }

    pub fn get_port(left: usize, right: usize) -> usize {
        left + right
    }
}

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
