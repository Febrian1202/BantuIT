use crate::common::AppError;

pub struct ReferentialIntegrityGuard;

impl ReferentialIntegrityGuard {
    /// Memastikan tidak ada data terkait yang masih menggunakan entitas ini
    /// Format parameter: &[(label, count)]
    pub fn assert_unreferenced(references: &[(&str, i64)]) -> Result<(), AppError> {
        for &(label, count) in references {
            if count > 0 {
                return Err(AppError::state_conflict(format!(
                    "{label} masih digunakan oleh {count} data terkait dan tidak dapat dihapus."
                )));
            }
        }

        Ok(())
    }
}
