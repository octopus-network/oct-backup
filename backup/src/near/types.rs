use near_jsonrpc_client::methods;
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::types::{AccountId, Finality};
use near_primitives::views::{CallResult, QueryRequest};
use serde_json::Value;
use crate::global::DEFAULT_NEAR_JSON_RPC_CLIENT;
use tracing::{info, instrument};

pub struct ViewResultDetails {
    /// Our result from our call into a view function.
    pub result: Vec<u8>,
    /// Logs generated from the view function.
    pub logs: Vec<String>,
}


impl ViewResultDetails {
    /// Deserialize an instance of type `T` from bytes of JSON text sourced from the
    /// execution result of this call. This conversion can fail if the structure of
    /// the internal state does not meet up with [`serde::de::DeserializeOwned`]'s
    /// requirements.
    pub fn json<T: serde::de::DeserializeOwned>(&self) -> anyhow::Result<T> {
        serde_json::from_slice(&self.result).map_err(Into::into)
    }
}

impl From<CallResult> for ViewResultDetails {
    fn from(result: CallResult) -> Self {
        ViewResultDetails {
            result: result.result,
            logs: result.logs,
        }
    }
}

#[instrument(level = "debug")]
pub async fn view(
    contract_id: AccountId,
    method_name: String,
    args: Value,
) -> anyhow::Result<ViewResultDetails> {
    let request = methods::query::RpcQueryRequest {
            block_reference: Finality::Final.into(), // Optimisitic query
            request: QueryRequest::CallFunction {
                account_id: contract_id.clone(),
                method_name: method_name.clone(),
                args: args.to_string().into_bytes().into(),
            },
        };

    let rpc_client = DEFAULT_NEAR_JSON_RPC_CLIENT.get().await;
    let response = rpc_client.call(request).await?;
    info!("view method response: {:?}", response);
    // println!("view {}.{} with arg: {}", contract_id, method_name, args.to_string());
    // dbg!(&response);

    match response.kind {
        QueryResponseKind::CallResult(result) => Ok(result.into()),
        _ => {
            anyhow::bail!("near view error result.")},
    }
}