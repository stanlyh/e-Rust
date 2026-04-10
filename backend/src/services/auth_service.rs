use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{
    config::Config,
    error::{AppError, AppResult},
    models::user::{AuthResponse, Claims, LoginRequest, RegisterRequest, UserResponse, UserRole},
    repositories::user_repo::UserRepo,
    state::AppState,
};

pub struct AuthService;

impl AuthService {
    pub async fn register(state: &AppState, req: RegisterRequest) -> AppResult<UserResponse> {
        let password_hash = Self::hash_password(&req.password)?;
        let role = req.role.unwrap_or(UserRole::SalesAgent);

        let user = UserRepo::create(
            &state.db,
            &req.email,
            &password_hash,
            &req.full_name,
            &role,
            req.phone.as_deref(),
        )
        .await?;

        Ok(UserResponse::from(user))
    }

    pub async fn login(state: &AppState, req: LoginRequest) -> AppResult<(AuthResponse, String)> {
        let user = UserRepo::find_by_email(&state.db, &req.email)
            .await?
            .ok_or(AppError::Unauthorized)?;

        Self::verify_password(&req.password, &user.password_hash)?;

        let access_token = Self::create_access_token(&user.id, &user.email, &user.role, &state.config)?;
        let refresh_token = Self::create_refresh_token();
        let refresh_token_hash = Self::hash_token(&refresh_token);

        let expires_at = Utc::now()
            + chrono::Duration::days(state.config.jwt_refresh_expiry_days);

        UserRepo::save_refresh_token(&state.db, user.id, &refresh_token_hash, expires_at).await?;

        let expires_in = state.config.jwt_access_expiry_minutes * 60;
        let response = AuthResponse {
            access_token,
            token_type: "Bearer".to_string(),
            expires_in,
            user: UserResponse::from(user),
        };

        Ok((response, refresh_token))
    }

    pub async fn refresh(state: &AppState, refresh_token: &str) -> AppResult<(AuthResponse, String)> {
        let token_hash = Self::hash_token(refresh_token);

        let (user_id, expires_at) = UserRepo::find_refresh_token(&state.db, &token_hash)
            .await?
            .ok_or(AppError::Unauthorized)?;

        if Utc::now() > expires_at {
            UserRepo::revoke_refresh_token(&state.db, &token_hash).await?;
            return Err(AppError::Unauthorized);
        }

        let user = UserRepo::find_by_id(&state.db, user_id)
            .await?
            .ok_or(AppError::Unauthorized)?;

        // Rota el refresh token
        UserRepo::revoke_refresh_token(&state.db, &token_hash).await?;
        let new_refresh_token = Self::create_refresh_token();
        let new_hash = Self::hash_token(&new_refresh_token);
        let new_expires_at = Utc::now() + chrono::Duration::days(state.config.jwt_refresh_expiry_days);
        UserRepo::save_refresh_token(&state.db, user.id, &new_hash, new_expires_at).await?;

        let access_token = Self::create_access_token(&user.id, &user.email, &user.role, &state.config)?;
        let expires_in = state.config.jwt_access_expiry_minutes * 60;

        let response = AuthResponse {
            access_token,
            token_type: "Bearer".to_string(),
            expires_in,
            user: UserResponse::from(user),
        };

        Ok((response, new_refresh_token))
    }

    pub async fn logout(state: &AppState, refresh_token: &str) -> AppResult<()> {
        let token_hash = Self::hash_token(refresh_token);
        UserRepo::revoke_refresh_token(&state.db, &token_hash).await?;
        Ok(())
    }

    pub fn validate_access_token(token: &str, config: &Config) -> AppResult<Claims> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?;

        Ok(token_data.claims)
    }

    fn create_access_token(
        user_id: &Uuid,
        email: &str,
        role: &UserRole,
        config: &Config,
    ) -> AppResult<String> {
        let now = Utc::now();
        let exp = now + chrono::Duration::minutes(config.jwt_access_expiry_minutes);

        let claims = Claims {
            sub: user_id.to_string(),
            email: email.to_string(),
            role: format!("{:?}", role).to_lowercase(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Error creando JWT: {}", e)))
    }

    fn create_refresh_token() -> String {
        use argon2::password_hash::rand_core::RngCore;
        let mut bytes = [0u8; 32];
        OsRng.fill_bytes(&mut bytes);
        hex::encode(bytes)
    }

    fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn hash_password(password: &str) -> AppResult<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Error hasheando password: {}", e)))
    }

    fn verify_password(password: &str, hash: &str) -> AppResult<()> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::Internal(anyhow::anyhow!("Hash invalido: {}", e)))?;
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::Unauthorized)
    }
}
