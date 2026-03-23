// rustautomod
use salvo::http::{HeaderMap, HeaderValue, StatusError, header::CONTENT_TYPE};
use serde::Serialize;

pub mod user_resource;

#[derive(Serialize)]
pub struct DataResponse<T> {
    pub success: bool,
    pub data: T,
}

impl<T> DataResponse<T> {
    pub fn success(data: T) -> Self {
        DataResponse {
            success: true,
            data,
        }
    }

    pub fn error(data: T) -> Self {
        DataResponse {
            success: false,
            data,
        }
    }
}

impl<T> salvo::Scribe for DataResponse<T>
where
    T: Serialize + Send,
{
    fn render(self, res: &mut salvo::Response) {
        match serde_json::to_vec(&self) {
            Ok(bytes) => {
                let mut headers_map = HeaderMap::new();
                headers_map.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                res.set_headers(headers_map);

                let _ = res.write_body(bytes);
            }
            Err(err) => {
                println!("{err}");
                res.render(StatusError::internal_server_error());
            }
        }
    }
}
