use crate::domain::entities::specialty::Specialty as DomainSpecialty;
use crate::infrastructure::models::specialty::Model as SpecialtyModel;

impl From<SpecialtyModel> for DomainSpecialty {
    fn from(model: SpecialtyModel) -> Self {
        DomainSpecialty::restore(
            model.id.to_string(),
            model.company_id.to_string(),
            model.name,
            Some(model.created_at),
        )
    }
}
