mod objects;

use objects::object_filter::object_filter;
use objects::object_service::ObjectService;

#[tokio::main]
async fn main() {
    let object_service = ObjectService::new();

    let api = object_filter(object_service);

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}
