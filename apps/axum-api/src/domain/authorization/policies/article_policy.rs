use crate::{
    common::AppError,
    domain::{
        enums::{ArticleStatus, RoleName},
        models::{KnowledgeArticle, User},
    },
};

pub struct ArticlePolicy;

impl ArticlePolicy {
    pub fn view_any(_user: &User) -> Result<(), AppError> {
        Ok(())
    }

    pub fn view(user: &User, article: &KnowledgeArticle) -> Result<(), AppError> {
        if !user.role_name.is_admin()
            && !user
                .role_name
                .has_role(&[RoleName::Manager, RoleName::Technician])
        {
            if article.status == ArticleStatus::Published {
                return Ok(());
            }
            return Err(AppError::NotFound);
        }

        Ok(())
    }

    pub fn create(user: &User) -> Result<(), AppError> {
        if user.role_name.is_admin()
            || user
                .role_name
                .has_role(&[RoleName::Manager, RoleName::Technician])
        {
            return Ok(());
        }
        Err(AppError::Forbidden)
    }

    pub fn update(user: &User, _article: &KnowledgeArticle) -> Result<(), AppError> {
        if user.role_name.is_admin()
            || user
                .role_name
                .has_role(&[RoleName::Manager, RoleName::Technician])
        {
            return Ok(());
        }
        Err(AppError::Forbidden)
    }

    pub fn publish(user: &User, _article: &KnowledgeArticle) -> Result<(), AppError> {
        if user.role_name.is_admin()
            || user
                .role_name
                .has_role(&[RoleName::Manager, RoleName::Technician])
        {
            return Ok(());
        }
        Err(AppError::Forbidden)
    }

    pub fn unpublish(user: &User, _article: &KnowledgeArticle) -> Result<(), AppError> {
        if user.role_name.is_admin()
            || user
                .role_name
                .has_role(&[RoleName::Manager, RoleName::Technician])
        {
            return Ok(());
        }
        Err(AppError::Forbidden)
    }

    pub fn delete(user: &User, article: &KnowledgeArticle) -> Result<(), AppError> {
        if user.role_name.is_admin() || user.role_name.has_role(&[RoleName::Manager]) {
            return Ok(());
        }

        if article.author_id == user.id {
            return Ok(());
        }

        Err(AppError::Forbidden)
    }
}
