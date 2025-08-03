use actix::{Actor, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use futures_util::StreamExt;
use index_scheduler::IndexScheduler;
use tokio::sync::broadcast;

pub async fn task_socket(
    req: HttpRequest,
    stream: web::Payload,
    index_scheduler: web::Data<IndexScheduler>,
) -> Result<HttpResponse, Error> {
    let receiver = index_scheduler
        .task_sender
        .as_ref()
        .map(|s| s.subscribe());
    ws::start(TaskWs { receiver }, &req, stream)
}

struct TaskWs {
    receiver: Option<broadcast::Receiver<Vec<u8>>>,
}

impl Actor for TaskWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        if let Some(mut rx) = self.receiver.take() {
            ctx.add_stream(async_stream::stream! {
                while let Ok(msg) = rx.recv().await {
                    yield ws::Message::Binary(msg);
                }
            });
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for TaskWs {
    fn handle(&mut self, _item: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        // ignore incoming messages
    }
}
