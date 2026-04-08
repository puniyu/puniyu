use std::any::Any;

use async_trait::async_trait;
use puniyu_adapter_types::SendMsgType;
use puniyu_contact::ContactType;
use puniyu_message::Message;
use puniyu_error::Result;

#[async_trait]
pub trait Runtime: Any + Send + Sync {
    async fn send_message(
        &self,
        contact: &ContactType<'_>,
        message: &Message,
    ) -> Result<SendMsgType>;
}
