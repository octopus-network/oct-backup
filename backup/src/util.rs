use std::fmt::Debug;
use std::time::Duration;
use anyhow::{anyhow, Result};
use chrono::NaiveDateTime;
use near_jsonrpc_client::{JsonRpcClient, methods};
use near_primitives::types::{AccountId, BlockReference, Finality, FunctionArgs};
use near_primitives::views::QueryRequest;
use serde_json::json;
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use serde::{Deserialize, Serialize};
use serde_json::value::Serializer;
use crate::global::DEFAULT_NEAR_JSON_RPC_CLIENT;


pub fn naive_date_time_from_nanos_time(timestamp: u64) -> NaiveDateTime {
    let nanos_timestamp = Duration::from_nanos(timestamp);

    NaiveDateTime::from_timestamp(
        nanos_timestamp.as_secs() as i64,
        nanos_timestamp.subsec_nanos(),
    )
}

// pub async fn rpc_call<T: serde::de::DeserializeOwned>(
//     contract_account_id: AccountId,
//     method_name: String,
//     args: FunctionArgs
// ) -> anyhow::Result<T> {
//
//     let request = methods::query::RpcQueryRequest {
//         block_reference: BlockReference::Finality(Finality::Final),
//         request: QueryRequest::CallFunction {
//             account_id: contract_account_id,
//             method_name,
//             // args: FunctionArgs(json!(args).to_string().into_bytes()),
//             args
//         },
//     };
//
//     let response = DEFAULT_NEAR_JSON_RPC_CLIENT.call(request).await?;
//     // let response = client.call(request).await?;
//
//     if let QueryResponseKind::CallResult(result) = response.kind {
//         // println!("{:#?}", serde_json::from_slice::<T>(&result.result.clone())?);
//         return serde_json::from_slice(&result.result).map_err(Into::into);
//     }
//     Err(anyhow!(""))
// }


pub mod u128_dec_format {
    use serde::de;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(num: &u128, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&num.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u128, D::Error>
        where
            D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}

pub mod u64_dec_format {
    use serde::de;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(num: &u64, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(&num.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
        where
            D: Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(de::Error::custom)
    }
}