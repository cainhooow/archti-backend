use salvo::{
    http::{
        HeaderMap, HeaderValue,
        header::{ACCEPT, CONTENT_TYPE, FORWARDED},
    },
    prelude::*,
};

pub struct AppMiddleware;

#[async_trait::async_trait]
impl Handler for AppMiddleware {
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    ) {
        let mut headers = HeaderMap::new();
        _ = req.add_header(CONTENT_TYPE, HeaderValue::from_static("application/json"), true);
        _ = req.add_header(ACCEPT, HeaderValue::from_static("application/json"), true);
        headers.insert(FORWARDED, HeaderValue::from_static("ArchTI"));
        res.set_headers(headers);

        _ = ctrl.call_next(req, depot, res).await;
    }
}
