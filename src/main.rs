use norgopolis_module::{invoker_service::Service, module_communication::MessagePack, Module};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::Status;

mod ontologies;
mod ontology;

#[derive(Default)]
struct GTD {}

#[norgopolis_module::async_trait]
impl Service for GTD {
    type Stream = UnboundedReceiverStream<Result<MessagePack, Status>>;

    async fn call(
        &self,
        function: String,
        args: Option<MessagePack>,
    ) -> Result<Self::Stream, Status> {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

        match function.as_str() {
            "capture" => {}
            _ => todo!(),
        }

        Ok(UnboundedReceiverStream::new(rx))
    }
}

#[tokio::main]
async fn main() {
    Module::start(GTD::default()).await.unwrap()
}
