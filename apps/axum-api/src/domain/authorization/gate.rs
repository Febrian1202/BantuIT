use uuid::Uuid;

use crate::{
    common::AppError,
    domain::{authorization::AbilityMatrix, enums::Ability, models::User},
};

pub struct Gate;

impl Gate {
    /// Evaluasi apakah pengguna diizinkan untuk menjalankan ability tersebut
    pub fn check(user: &User, ability: Ability) -> Result<(), AppError> {
        let role = user.role_name;

        // Admin Ability Exceptio
        if role.is_admin() && !AbilityMatrix::ADMIN_GATE_EXCEPTIONS.contains(&ability) {
            return Ok(());
        }

        // Evaluasi RBAC Matrix
        if AbilityMatrix::allows_role(role, ability) {
            return Ok(());
        }

        // 403 Kalau tidak memenuhi
        Err(AppError::Forbidden)
    }

    /// Memeriksa supaya admin tidak bisa menghapus atau deactivate akunnya
    /// sendiri
    pub fn check_target_user(
        user: &User,
        target_user_id: Uuid,
        ability: Ability,
    ) -> Result<(), AppError> {
        if (ability == Ability::UserDeactivate || ability == Ability::UserDelete)
            && user.id == target_user_id
        {
            return Err(AppError::Forbidden);
        }

        Self::check(user, ability)
    }
}
