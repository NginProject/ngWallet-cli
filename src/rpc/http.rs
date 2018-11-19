//! # Send JSON encoded HTTP requests

use hyper::Url;
use hyper::client::IntoUrl;
use jsonrpc_core::Params;
use reqwest::Client;
use serde_json::Value;
use cmd::Error;

lazy_static! {
    static ref CLIENT: Client = Client::new().expect("Expect to create an HTTP client");
}

/// RPC methods
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum ClientMethod {
    /// [ngin_gasPrice](https://github.com/ethereum/wiki/wiki/JSON-RPC#ngin_gasprice)
    EthGasPrice,

    /// [ngin_estimatePrice](
    /// https://github.com/ethereumproject/go-ethereum/wiki/JSON-RPC#ngin_estimategas)
    EthEstimateGas,

    /// [ngin_getTransactionCount](
    /// https://github.com/ethereum/wiki/wiki/JSON-RPC#ngin_gettransactioncount)
    EthGetTxCount,

    /// [ngin_getTransactionByHash](
    /// https://github.com/ethereumproject/wiki/wiki/JSON-RPC#ngin_gettransactionbyhash)
    EthGetTxByHash,

    /// [ngin_sendRawTransaction](
    /// https://github.com/paritytech/parity/wiki/JSONRPC-eth-module#ngin_sendrawtransaction)
    EthSendRawTransaction,

    /// [ngin_getBalance](
    /// https://github.com/ethereumproject/go-ethereum/wiki/JSON-RPC#ngin_getbalance)
    EthGetBalance,
}

/// RPC method's parameters
#[derive(Clone, Debug, PartialEq)]
pub struct MethodParams<'a>(pub ClientMethod, pub &'a Params);

pub struct RpcConnector {
    pub url: Url,
}

impl RpcConnector {
    pub fn new<U: IntoUrl>(url: U) -> Result<RpcConnector, Error> {
        let url = url.into_url()?;

        Ok(RpcConnector { url })
    }

    /// Send and JSON RPC HTTP post request
    pub fn send_post(&self, params: &MethodParams) -> Result<Value, Error> {
        let mut res = CLIENT.post(self.url.clone()).json(params).send()?;
        let json: Value = res.json()?;

        Ok(json["result"].clone())
    }
}
