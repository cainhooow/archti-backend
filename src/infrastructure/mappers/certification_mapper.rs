use crate::domain::entities::certification::Certification as DomainCertification;
use crate::infrastructure::models::certification::Model as CertificationModel;

impl From<CertificationModel> for DomainCertification {
    fn from(model: CertificationModel) -> Self {
        DomainCertification::restore(
            model.id.to_string(),
            model.company_id.to_string(),
            model.name,
            model.valid_until,
            model.status_label,
            Some(model.created_at),
        )
    }
}
