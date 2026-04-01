use salvo::http::{HeaderMap, HeaderValue, StatusError, header::CONTENT_TYPE};
use serde::Serialize;

pub mod auth_resources;
pub mod company_resources;
pub mod me_resources;
pub mod message_resource;
pub mod user_resources;
/*
* DataResponse is a struct that allows you to standardize API
* responses and avoid manually defining part-of-the-route responses.
*
* DataResponse works in conjunction with resources,
* which translate domain structures into standardized JSON responses.
*/
#[derive(Serialize)]
pub struct DataResponse<T> {
    pub success: bool,
    pub data: T,
}

impl<T> DataResponse<T> {
    /*
     * Creates a successful DataResponse with the given data.
     */
    pub fn success(data: T) -> Self {
        DataResponse {
            success: true,
            data,
        }
    }
    /*
     * Creates an error DataResponse with the given data.
     */
    pub fn error(data: T) -> Self {
        DataResponse {
            success: false,
            data,
        }
    }
}
/*
 * Implements the Scribe trait for DataResponse,
 * allowing it to be used as a response body in Salvo.
 */
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
