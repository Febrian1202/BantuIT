use axum::{extract::Request, middleware::Next, response::Response};

use crate::{common::AppError, interface::extractors::auth_user::AuthUser};

pub async fn ensure_password_checked(
    AuthUser(user): AuthUser,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    if user.must_change_password {
        return Err(AppError::PasswordChangeRequired);
    }

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
