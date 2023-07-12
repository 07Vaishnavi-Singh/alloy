use futures_channel::mpsc::UnboundedReceiver;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::value::RawValue;

use std::{borrow::Cow, fmt::Debug};

use crate::{call::RpcCall, common::*, TransportError};

/// Blanket-impld trait for types that can be sent to and received via Rpc
pub trait RpcObject: Serialize + DeserializeOwned + Send + Sync + Unpin {}
impl<T> RpcObject for T where T: Serialize + DeserializeOwned + Send + Sync + Unpin {}

pub trait Connection: Debug + Send + Sync + Unpin {
    fn is_local(&self) -> bool;

    fn increment_id(&self) -> u64;

    fn next_id(&self) -> Id<'static> {
        Id::Number(self.increment_id())
    }

    fn json_rpc_request(&self, req: &Request<'_>) -> RpcFuture;

    fn batch_request(&self, reqs: &[Request<'_>]) -> BatchRpcFuture;

    fn request<Params, Resp>(
        &self,
        method: &'static str,
        params: Params,
    ) -> RpcCall<&Self, Self, Params, Resp>
    where
        Self: Sized,
        Params: RpcObject,
        Resp: RpcObject,
    {
        RpcCall::new(self, method, params, self.next_id())
    }
}

pub trait PubSubConnection: Connection {
    #[doc(hidden)]
    fn uninstall_listener(&self, id: [u8; 32]) -> Result<(), TransportError>;

    #[doc(hidden)]
    fn install_listener(
        &self,
        id: [u8; 32],
    ) -> Result<UnboundedReceiver<Cow<'_, RawValue>>, TransportError>;
}

#[cfg(test)]
mod test {
    use crate::{Connection, PubSubConnection};

    fn __compile_check() -> Box<dyn Connection> {
        todo!()
    }
    fn __compile_check_pubsub() -> Box<dyn PubSubConnection> {
        todo!()
    }
}
