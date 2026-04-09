use salvo::prelude::*;

#[handler]
pub async fn index_specialty_handler(req: &mut Request, depot: &mut Depot, res: &mut Response) {}

#[handler]
pub async fn create_specialty_handler(req: &mut Request, depot: &mut Depot, res: &mut Response) {}
