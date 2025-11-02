// Integration tests for the API
use actix_web::{test, web, App};
use classtop_management_server::{handlers, models, routes};

#[actix_web::test]
async fn test_health_check() {
    let app =
        test::init_service(App::new().route("/api/health", web::get().to(handlers::health_check)))
            .await;

    let req = test::TestRequest::get().uri("/api/health").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
}

#[actix_web::test]
async fn test_root_endpoint() {
    let app = test::init_service(App::new().route("/", web::get().to(routes::root))).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let body: models::RootResponse = test::read_body_json(resp).await;
    assert_eq!(body.message, "ClassTop Management Server");
}

#[cfg(test)]
mod model_tests {
    use classtop_management_server::models;

    #[test]
    fn test_pagination_params() {
        let params = models::PaginationParams {
            page: 2,
            page_size: 10,
        };

        assert_eq!(params.offset(), 10);
        assert_eq!(params.limit(), 10);
    }

    #[test]
    fn test_pagination_info() {
        let info = models::PaginationInfo::new(1, 20, 100);

        assert_eq!(info.page, 1);
        assert_eq!(info.page_size, 20);
        assert_eq!(info.total_items, 100);
        assert_eq!(info.total_pages, 5);
    }

    #[test]
    fn test_user_info_from_user() {
        let user = models::User {
            id: 1,
            uuid: "test-uuid".to_string(),
            username: "testuser".to_string(),
            password_hash: "hashed".to_string(),
            email: Some("test@example.com".to_string()),
            role: "user".to_string(),
            is_active: true,
            created_at: "2024-01-01".to_string(),
            updated_at: "2024-01-01".to_string(),
        };

        let info: models::UserInfo = user.into();
        assert_eq!(info.username, "testuser");
        assert_eq!(info.email, Some("test@example.com".to_string()));
    }
}

#[cfg(test)]
mod auth_tests {
    use classtop_management_server::auth;
    use uuid::Uuid;

    #[test]
    fn test_password_hashing_and_verification() {
        let password = "super_secret_password_123!";
        let hash = auth::hash_password(password).unwrap();

        assert!(auth::verify_password(password, &hash).unwrap());
        assert!(!auth::verify_password("wrong_password", &hash).unwrap());
    }

    #[test]
    fn test_jwt_token_generation() {
        let user_id = Uuid::new_v4();
        let username = "testuser".to_string();
        let secret = "test_secret_key_for_testing_purposes";

        let token = auth::generate_token(user_id, username.clone(), secret).unwrap();
        assert!(!token.is_empty());

        let claims = auth::validate_token(&token, secret).unwrap();
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.username, username);
    }

    #[test]
    fn test_jwt_token_with_wrong_secret() {
        let user_id = Uuid::new_v4();
        let username = "testuser".to_string();
        let secret = "correct_secret";
        let wrong_secret = "wrong_secret";

        let token = auth::generate_token(user_id, username, secret).unwrap();
        let result = auth::validate_token(&token, wrong_secret);

        assert!(result.is_err());
    }

    #[test]
    fn test_claims_creation() {
        let user_id = Uuid::new_v4();
        let username = "testuser".to_string();

        let claims = auth::Claims::new(user_id, username.clone(), 24);

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.username, username);
        assert!(claims.exp > claims.iat);
    }
}
