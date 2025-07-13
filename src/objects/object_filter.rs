use crate::objects::object_handler;
use crate::objects::object_model::CreateObjectDto;
use crate::objects::object_service::ObjectService;
use warp::Filter;

const OBJECTS_ROUTE: &str = "objects";

/**
 * Filters for the object service
 */
pub fn object_filter(
    service: ObjectService,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let list_objects = warp::path(OBJECTS_ROUTE)
        .and(warp::get())
        .and(with_service(service))
        .and_then(object_handler::list_objects);

    let create_object = warp::path(OBJECTS_ROUTE)
        .and(warp::post())
        .and(with_service(service))
        .and(json_body())
        .and_then(object_handler::create_object);

    let get_object = warp::path(OBJECTS_ROUTE)
        .and(warp::path::param::<String>())
        .and(with_service(service))
        .and(warp::get())
        .and_then(object_handler::get_object);

    list_objects.or(create_object).or(get_object)
}

fn json_body() -> impl Filter<Extract = (CreateObjectDto,), Error = warp::Rejection> + Clone {
    warp::body::json()
}

fn with_service(
    service: ObjectService,
) -> impl Filter<Extract = (ObjectService,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || service)
}
