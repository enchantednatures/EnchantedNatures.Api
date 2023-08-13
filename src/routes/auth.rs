use crate::error_handling::AppError;

use crate::models::UserInfo;

struct AuthService;

impl AuthService {
    // add code here
    async fn login() -> Result<Option<UserInfo>, AppError> {
        unimplemented!();
    }

    async fn logout() -> Result<(), AppError> { 
        unimplemented!()
    }
}
