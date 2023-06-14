use crate::error::ErrorEvent;
//use crate::types::H256;
use serde::{Deserialize, Serialize};

//use crate::error::Error;
/*
pub fn decode_char(data: &[u8], indent: u32) -> anhow::Result<DecodedOut, Error> {
    let mut slice_to_char = data.get(0..4).ok_or(ParserDecodingError::DataTooShort)?;
    let a = <u32>::decode(&mut slice_to_char)
        .map_err(|_| ParserDecodingError::PrimitiveFailure("char".to_string()))?;
    let b = char::from_u32(a)
        .ok_or_else(|| ParserDecodingError::PrimitiveFailure("char".to_string()))?;
    let fancy_out = vec![OutputCard {
        card: ParserCard::Default(b.to_string()),
        indent,
    }];
    let remaining_vector = (data[4..]).to_vec();
    Ok(DecodedOut {
        remaining_vector,
        fancy_out,
    })
}

*/
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeVersionEvent {
    /// The runtime version.
    pub spec: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum RuntimeEvent {
    /// The runtime version of this block.
    Valid(RuntimeVersionEvent),
    /// The runtime could not be obtained due to an error.
    Invalid(ErrorEvent),
}

/*
#[derive(Clone)]
pub enum ParserCard {
    Pallet(String), // pallet name
    Method {
        method_name: String,
        docs: String,
    },
    Varname(String),
    Default(String),
    Text(String),
    Id {
        id: AccountId32,
        base58prefix: u16,
    },
    Id20 {
        id: [u8; 20],
        base58prefix: u16,
    },
    None,
    IdentityField(String),
    BitVec(String), // String from printing `BitVec`
    Balance {
        number: String,
        units: String,
    },
    FieldName {
        name: String,
        docs_field_name: String,
        path_type: String,
        docs_type: String,
    },
    FieldNumber {
        number: usize,
        docs_field_number: String,
        path_type: String,
        docs_type: String,
    },
    EnumVariantName {
        name: String,
        docs_enum_variant: String,
    },
    Era(Era),
    Nonce(String),
    BlockHash(H256),
    Tip {
        number: String,
        units: String,
    },
    NetworkNameVersion {
        name: String,
        version: String,
    },
    TxVersion(String),
}

impl ParserCard {
    pub fn show_no_docs(&self, indent: u32) -> String {
        match &self {
            ParserCard::Pallet(pallet_name) => readable(indent, "pallet", pallet_name),
            ParserCard::Method {
                method_name,
                docs: _,
            } => readable(indent, "method", method_name),
            ParserCard::Varname(varname) => readable(indent, "varname", varname),
            ParserCard::Default(decoded_string) => readable(indent, "default", decoded_string),
            ParserCard::Text(decoded_text) => readable(indent, "text", decoded_text),
            ParserCard::Id { id, base58prefix } => readable(
                indent,
                "Id",
                &id.to_ss58check_with_version(Ss58AddressFormat::custom(*base58prefix)),
            ),
            ParserCard::Id20 {
                id,
                base58prefix: _,
            } => readable(indent, "Id", &format!("0x{}", hex::encode(id))),
            ParserCard::None => readable(indent, "none", ""),
            ParserCard::IdentityField(variant) => readable(indent, "identity_field", variant),
            ParserCard::BitVec(bv) => readable(indent, "bitvec", bv),
            ParserCard::Balance { number, units } => {
                readable(indent, "balance", &format!("{number} {units}"))
            }
            ParserCard::FieldName {
                name,
                docs_field_name: _,
                path_type: _,
                docs_type: _,
            } => readable(indent, "field_name", name),
            ParserCard::FieldNumber {
                number,
                docs_field_number: _,
                path_type: _,
                docs_type: _,
            } => readable(indent, "field_number", &number.to_string()),
            ParserCard::EnumVariantName {
                name,
                docs_enum_variant: _,
            } => readable(indent, "enum_variant_name", name),
            ParserCard::Era(era) => match era {
                Era::Immortal => readable(indent, "era", "Immortal"),
                Era::Mortal(period, phase) => readable(
                    indent,
                    "era",
                    &format!("Mortal, phase: {phase}, period: {period}"),
                ),
            },
            ParserCard::Nonce(nonce) => readable(indent, "nonce", nonce),
            ParserCard::BlockHash(block_hash) => {
                readable(indent, "block_hash", &hex::encode(block_hash))
            }
            ParserCard::Tip { number, units } => {
                readable(indent, "tip", &format!("{number} {units}"))
            }
            ParserCard::NetworkNameVersion { name, version } => {
                readable(indent, "network", &format!("{name}{version}"))
            }
            ParserCard::TxVersion(x) => readable(indent, "tx_version", x),
        }
    }
}

fn readable(indent: u32, card_type: &str, card_payload: &str) -> String {
    format!(
        "{}{}: {}",
        "  ".repeat(indent as usize),
        card_type,
        card_payload
    )
}
*/
