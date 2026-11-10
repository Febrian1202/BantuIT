use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

use crate::common::{response_json, setup_test_app};

#[tokio::test]
async fn test_health_check_endpoint_returns_200_and_database_connected() {
    let (_state, app) = setup_test_app().await;

    let request = Request::builder()
        .uri("/api/health")
        .method("GET")
        .body(Body::empty())
        .expect("Gagal membuat request health");

    let response = app
        .oneshot(request)
        .await
        .expect("Request health gagal dieksekusi");

    assert_eq!(response.status(), StatusCode::OK);

    let json = response_json(response).await;

    assert_eq!(json["success"], true);
    assert_eq!(json["message"], "Sistem Beroperasi Normal");
    assert_eq!(json["data"]["status"], "OK");
    assert_eq!(json["data"]["database"]["status"], "CONNECTED");
    assert!(json["data"]["uptime_seconds"].as_u64().is_some());
}
