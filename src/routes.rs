use warp::Filter;

// Define your route handlers here
// Example: GET /manga/{id}

pub fn manga_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("manga" / String)
        .map(|id: String| {
            format!("Fetching manga with ID: {}", id)
        })
}