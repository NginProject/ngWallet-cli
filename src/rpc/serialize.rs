//! # Serialize JSON RPC parameters

use super::{ClientMethod, MethodParams};
use serde::{Serialize, Serializer};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use jsonrpc_core::Params;

lazy_static! {
    static ref REQ_ID: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(1));
}

#[derive(Clone, Debug, Serialize)]
struct JsonData<'a> {
    jsonrpc: &'static str,
    method: &'static str,
    params: &'a Params,
    id: usize,
}

impl<'a> Serialize for MethodParams<'a> {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.0 {
            ClientMethod::EthGasPrice => serialize("ngin_gasPrice", self.1, s),
            ClientMethod::EthEstimateGas => serialize("ngin_etsimateGas", self.1, s),
            ClientMethod::EthGetTxCount => serialize("ngin_getTransactionCount", self.1, s),
            ClientMethod::EthGetTxByHash => serialize("ngin_getTransactionByHash", self.1, s),
            ClientMethod::EthSendRawTransaction => serialize("ngin_sendRawTransaction", self.1, s),
            ClientMethod::EthGetBalance => serialize("ngin_getBalance", self.1, s),
        }
    }
}

fn serialize<S>(method: &'static str, params: &Params, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    to_json_data(method, params).serialize(serializer)
}

fn to_json_data<'a>(method: &'static str, params: &'a Params) -> JsonData<'a> {
    let id = REQ_ID.fetch_add(1, Ordering::SeqCst);

    JsonData {
        jsonrpc: "2.0",
        method,
        params,
        id,
    }
}
