use salvo::prelude::*;

pub struct LogMiddleware;

#[async_trait::async_trait]
impl Handler for LogMiddleware {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        tracing::info!("{} {}", req.method(), req.uri());
        ctrl.call_next(req, depot, res).await;
    }
}
