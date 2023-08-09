/*
Copyright © 2023 Rust Syndicate LLC <flipchan@rustsyndi.cat>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the “Software”), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

use fixed_hash::construct_fixed_hash;
use jsonrpsee_core::Cow;
use serde::{Deserialize, Serialize};

construct_fixed_hash! {
    /// My 256 bit hash type.
    #[derive(Serialize, Deserialize)]
    pub struct H256(32);
}

impl H256 {
    /*
    fn to_string(&self) -> String {
        let stringen = format!("{:?}", self);
        stringen
    }
    */
}


#[cfg(feature = "subxthelper")]
impl From<subxt::utils::H256> for H256 {
    fn from(inputen: subxt::utils::H256) -> H256 {
        H256::from_slice(inputen.as_ref())
    }
}



/// Block Header containing the block nr
#[derive(Debug, Deserialize)]
pub struct Header {
    pub number: String,
}

/// Create a generic event in the form of:
/// event_summary {
/// pub pallet_name: "Sudo".to_string(),
/// pub pallet_method: "secret_function".to_string(),
/// }
#[derive(Debug, PartialEq)]
pub struct event_summary {
    pub pallet_name: String,
    pub pallet_method: String,
}

/// enum for defining what type of storage entry it is, is it a StorageValue, StorageMap or Unknown type
#[derive(Debug, PartialEq)]
pub enum storage_types {
    /// Substrate StorageValue
    StorageValue,
    /// Substrate StorageMap
    StorageMap,
    /// Unknown type, could not detect the right type
    Unknown,
}

/// storage value
#[derive(Debug, Clone)]
pub struct storage_value {
    name: String,
    typeid: u32,
}

// wip, parse the pallets storage types, storage values and storage maps
#[derive(Debug)]
pub struct pallet_storage_types {
    /// name of pallet
    pub pallet_prefix: String,
    /// StorageMap|StorageValue or unknown
    pub StorageType: storage_types,
    ///  Vec of storage types
    pub storage_items: Vec<storage_types>,
    /// the type id of the storage item
    pub type_id: u32, // type id of storage{Value/Map}
}

// Copied from sp_version (only available in std in the substrate version).
// https://github.com/paritytech/substrate/blob/1b3ddae9dec6e7653b5d6ef0179df1af831f46f0/primitives/version/src/lib.rs#L392-L393
mod apis_serialize {
    use super::*;
    use impl_serde::serialize as bytes;
    use serde::{de, ser::SerializeTuple, Serializer};

    #[derive(Serialize)]
    struct ApiId<'a>(
        #[serde(serialize_with = "serialize_bytesref")] &'a super::ApiId,
        &'a u32,
    );

    pub fn serialize<S>(apis: &ApisVec, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = apis.len();
        let mut seq = ser.serialize_tuple(len)?;
        for (api, ver) in &**apis {
            seq.serialize_element(&ApiId(api, ver))?;
        }
        seq.end()
    }

    pub fn serialize_bytesref<S>(&apis: &&super::ApiId, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        bytes::serialize(apis, ser)
    }

    #[derive(Deserialize)]
    struct ApiIdOwned(
        #[serde(deserialize_with = "deserialize_bytes")] super::ApiId,
        u32,
    );

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ApisVec, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = ApisVec;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                formatter.write_str("a sequence of api id and version tuples")
            }

            fn visit_seq<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
            where
                V: de::SeqAccess<'de>,
            {
                let mut apis = Vec::new();
                while let Some(value) = visitor.next_element::<ApiIdOwned>()? {
                    apis.push((value.0, value.1));
                }
                Ok(apis.into())
            }
        }
        deserializer.deserialize_seq(Visitor)
    }

    pub fn deserialize_bytes<'de, D>(d: D) -> Result<super::ApiId, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let mut arr = [0; 8];
        bytes::deserialize_check_len(d, bytes::ExpectedLen::Exact(&mut arr[..]))?;
        Ok(arr)
    }
}

/// [u8; 8]
pub type ApiId = [u8; 8];

/// A vector of pairs of `ApiId` and a `u32` for version.
pub type ApisVec = Cow<'static, [(ApiId, u32)]>;

/// RuntimeVersion ported over from substrate main repo
/// https://github.com/paritytech/substrate/blob/0cf64f8bd72d719818be2f109c0919c7c9325cd1/primitives/version/src/lib.rs#L161
#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeVersion {
    pub spec_name: String,
    pub impl_name: String,
    pub authoring_version: u32,
    pub spec_version: u32,
    pub impl_version: u32,
    #[serde(
        serialize_with = "apis_serialize::serialize",
        deserialize_with = "apis_serialize::deserialize"
    )]
    pub apis: ApisVec, //Vec<Vec<(i128, u32)>>, Cow<'static, [([u8; 8], u32)]>
    pub transaction_version: u32,
    pub state_version: u8,
}

/// generic substrate sp-runtime block
#[derive(Debug, Deserialize)]
pub struct Block<Header, Extrinsic: Serialize> {
    pub header: Header,
    pub extrinsics: Extrinsic, //Vec<String>
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct justifications(Vec<u8>);

// A generic PreBlock that contains a block and the justifications
#[derive(Debug, Deserialize)]
pub struct PreBlock {
    pub block: generic_block,

    pub justifications: Option<justifications>, //Justification can be null so lets put this in an option
}

/// A generic block format
pub type generic_block = Block<Header, Vec<String>>;
