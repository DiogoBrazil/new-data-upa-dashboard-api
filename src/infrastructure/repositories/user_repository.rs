use async_trait::async_trait;
use sqlx::{PgPool};
use uuid::Uuid;
use crate::domain::models::user::{CreateUserDto, UpdateUserDto, AddApplicationDto, CreateFeedbackRespiratoryDiseasesDto, FeedbackRespiratoryDiseasesResponse, User, CreateFeedbackTuberculosisDto, FeedbackTuberculosisResponse, UpdateEnabledUserDto};
use crate::domain::repositories::user::UserRepository;

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query!(
            r#"
            SELECT
                id,
                full_name,
                email,
                password,
                profile,
                allowed_applications as "allowed_applications!: Vec<String>",
                enabled
            FROM users_api
            WHERE enabled = true
            ORDER BY full_name
            "#
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(users.into_iter().map(|row| User {
            id: row.id,
            full_name: row.full_name,
            email: row.email,
            password: row.password,
            profile: row.profile,
            allowed_applications: row.allowed_applications,
            enabled: row.enabled,
        }).collect())
    }

    async fn find_all_feedbacks_respiratory_diseases(&self) -> Result<Vec<FeedbackRespiratoryDiseasesResponse>, sqlx::Error> {
        let feedbacks = sqlx::query!(
            r#"
            SELECT
                id,
                user_name,
                feedback,
                prediction_made,
                correct_prediction
            FROM feedbacks
            ORDER BY user_name
            "#
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(feedbacks.into_iter().map(|row| FeedbackRespiratoryDiseasesResponse {
            id: row.id,
            user_name: row.user_name,
            feedback: row.feedback,
            prediction_made: row.prediction_made,
            correct_prediction: row.correct_prediction
        }).collect())
    }

    async fn find_all_feedbacks_tuberculosis(&self) -> Result<Vec<FeedbackTuberculosisResponse>, sqlx::Error> {
        let feedbacks_tuberculosis = sqlx::query!(
            r#"
            SELECT
               id,
               user_name,
               feedback
            FROM feedback_tuberculosis
            ORDER BY user_name
            "#
        )
            .fetch_all(&self.pool)
            .await?;

        Ok(feedbacks_tuberculosis.into_iter().map(|row | FeedbackTuberculosisResponse {
            id: row.id,
            user_name: row.user_name,
            feedback: row.feedback
        }).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query!(
            r#"
            SELECT
                id,
                full_name,
                email,
                password,
                profile,
                allowed_applications as "allowed_applications!: Vec<String>",
                enabled
            FROM users_api
            WHERE id = $1
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(user.map(|row| User {
            id: row.id,
            full_name: row.full_name,
            email: row.email,
            password: row.password,
            profile: row.profile,
            allowed_applications: row.allowed_applications,
            enabled: row.enabled,
        }))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query!(
            r#"
            SELECT
                id,
                full_name,
                email,
                password,
                profile,
                allowed_applications as "allowed_applications!: Vec<String>",
                enabled
            FROM users_api
            WHERE email = $1
            "#,
            email
        )
            .fetch_optional(&self.pool)
            .await?;

        Ok(user.map(|row| User {
            id: row.id,
            full_name: row.full_name,
            email: row.email,
            password: row.password,
            profile: row.profile,
            allowed_applications: row.allowed_applications,
            enabled: row.enabled,
        }))
    }

    async fn create(&self, user: CreateUserDto) -> Result<User, sqlx::Error> {
        let id = Uuid::new_v4();
    
        let user = sqlx::query!(
            r#"
            INSERT INTO users_api (
                id,
                full_name,
                email,
                password,
                profile,
                allowed_applications,
                enabled
            )
            VALUES ($1, $2, $3, $4, $5, $6, true)
            RETURNING
                id,
                full_name,
                email,
                password,
                profile,
                allowed_applications as "allowed_applications!: Vec<String>",
                enabled
            "#,
            id,
            user.full_name,
            user.email,
            user.password, 
            user.profile,
            &user.allowed_applications as &[String],
        )
        .fetch_one(&self.pool)
        .await?;
    
        Ok(User {
            id: user.id,
            full_name: user.full_name,
            email: user.email,
            password: user.password,
            profile: user.profile,
            allowed_applications: user.allowed_applications,
            enabled: user.enabled,
        })
    }

    async fn update(&self, id: Uuid, user: UpdateUserDto) -> Result<Option<User>, sqlx::Error> {
        let current_user = self.find_by_id(id).await?;

        if let Some(_) = current_user {
            let updated_user = sqlx::query!(
                r#"
                UPDATE users_api
                SET
                    full_name = $1,
                    email = $2,
                    profile = $3,
                    allowed_applications = $4
                WHERE id = $5
                RETURNING
                    id,
                    full_name,
                    email,
                    password,
                    profile,
                    allowed_applications as "allowed_applications!: Vec<String>",
                    enabled
                "#,
                user.full_name,
                user.email,
                user.profile,
                &user.allowed_applications as &[String],
                id
            )
                .fetch_one(&self.pool)
                .await?;

            Ok(Some(User {
                id: updated_user.id,
                full_name: updated_user.full_name,
                email: updated_user.email,
                password: updated_user.password,
                profile: updated_user.profile,
                allowed_applications: updated_user.allowed_applications,
                enabled: updated_user.enabled,
            }))
        } else {
            Ok(None)
        }
    }

    async fn update_password(&self, id: Uuid, new_password: String) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE users_api
            SET password = $1
            WHERE id = $2
            "#,
            new_password,
            id
        )
        .execute(&self.pool)
        .await?;
    
        Ok(result.rows_affected() > 0)
    }

    async fn update_enabled(&self, id: Uuid, enabled: UpdateEnabledUserDto) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE users_api
            SET enabled = $1
            WHERE id = $2
            "#,
            enabled.enabled,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    async fn delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM users_api
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
    
        Ok(result.rows_affected() > 0)
    }

    async fn delete_application(&self, id: Uuid, application_name: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE users_api
            SET allowed_applications = array_remove(allowed_applications, $1)
            WHERE id = $2
            "#,
            application_name,
            id
        )
        .execute(&self.pool)
        .await?;
    
        Ok(result.rows_affected() > 0)
    }

    async fn add_application(&self, id: Uuid, applications: AddApplicationDto) -> Result<Option<User>, sqlx::Error> {
        let current_user = self.find_by_id(id).await?;
    
        if let Some(_) = current_user {
            let result = sqlx::query!(
                r#"
                UPDATE users_api
                SET allowed_applications = array_cat(allowed_applications, $1)
                WHERE id = $2
                RETURNING
                    id,
                    full_name,
                    email,
                    password,
                    profile,
                    allowed_applications as "allowed_applications!: Vec<String>",
                    enabled
                "#,
                &applications.applications_name as &[String],
                id
            )
            .fetch_one(&self.pool)
            .await?;
        
            Ok(Some(User {
                id: result.id,
                full_name: result.full_name,
                email: result.email,
                password: result.password,
                profile: result.profile,
                allowed_applications: result.allowed_applications,
                enabled: result.enabled,
            }))
        } else {
            Ok(None)
        }
    }

    async fn create_feedback_respiratory_diseases(
        &self, 
        feedback: CreateFeedbackRespiratoryDiseasesDto
    ) -> Result<Option<FeedbackRespiratoryDiseasesResponse>, sqlx::Error> {
        let id = Uuid::new_v4();
        let created_at = chrono::Utc::now().naive_utc();
        
        sqlx::query!(
            r#"
            INSERT INTO feedbacks (
                id,
                user_name,
                feedback,
                prediction_made,
                correct_prediction,
                created_at
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id
            "#,
            id,
            feedback.user_name,
            feedback.feedback,
            feedback.prediction_made,
            feedback.correct_prediction,
            created_at
        )
        .fetch_one(&self.pool)
        .await?;
    
        Ok(Some(FeedbackRespiratoryDiseasesResponse {
            id,
            user_name: feedback.user_name,
            feedback: feedback.feedback,
            prediction_made: feedback.prediction_made,
            correct_prediction: feedback.correct_prediction,
        }))
    }

    async fn create_feedback_tuberculosis(
        &self,
        feedback_tuberculosis: CreateFeedbackTuberculosisDto
    ) -> Result<Option<FeedbackTuberculosisResponse>, sqlx::Error> {
        let id = Uuid::new_v4();
        let created_at = chrono::Utc::now().naive_utc();

        sqlx::query!(
            r#"
            INSERT INTO feedback_tuberculosis (
                id,
                user_name,
                feedback,
                created_at
            )
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
            id,
            feedback_tuberculosis.user_name,
            feedback_tuberculosis.feedback,
            created_at
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(Some(FeedbackTuberculosisResponse {
            id,
            user_name: feedback_tuberculosis.user_name,
            feedback: feedback_tuberculosis.feedback,
        }))
    }
}
