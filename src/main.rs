use norgopolis_module::{Module, invoker_service::Service, module_communication::MessagePack};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::Status;

#[derive(Default)]
struct GTD {
}

#[norgopolis_module::async_trait]
impl Service for GTD {
    type Stream = UnboundedReceiverStream<Result<MessagePack, Status>>;

    async fn call(
        &self,
        function: String,
        args: Option<MessagePack>,
    ) -> Result<Self::Stream, Status> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        tx.send(Ok(MessagePack::encode("ok").unwrap())).unwrap();

        Ok(UnboundedReceiverStream::new(rx))
    }
}

#[tokio::main]
async fn main() {
    Module::start(GTD::default())
        .await
        .unwrap()
}
