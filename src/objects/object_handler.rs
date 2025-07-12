use crate::objects::object_model::CreateObjectDto;
use crate::objects::object_service::ObjectService;
use std::string::String;
use warp::http::StatusCode;

/**
 * List all objects
 */
pub async fn list_objects(
    object_service: ObjectService,
) -> Result<impl warp::Reply, warp::Rejection> {
    let objects = object_service.list_objects();

    Ok(warp::reply::with_status(
        warp::reply::json(&objects),
        StatusCode::OK,
    ))
}

/**
 * Get an object by id
 */
pub async fn get_object(
    id: String,
    object_service: ObjectService,
) -> Result<impl warp::Reply, warp::Rejection> {
    let object = object_service.get_object(&id);

    Ok(warp::reply::with_status(
        warp::reply::json(&object),
        StatusCode::OK,
    ))
}

/**
 * Create an object
 */
pub async fn create_object(
    object_service: ObjectService,
    object: CreateObjectDto,
) -> Result<impl warp::Reply, warp::Rejection> {
    let object = object_service.create_object(&object);

    Ok(warp::reply::with_status(
        warp::reply::json(&object),
        StatusCode::CREATED,
    ))
}
