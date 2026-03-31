use crate::domain::entities::company::Company as DomainCompany;
use crate::infrastructure::entities::company::Model as CompanyModel;

impl From<CompanyModel> for DomainCompany {
    fn from(model: CompanyModel) -> Self {
        DomainCompany::restore(
            model.id.to_string(),
            model.legal_name,
            model.trade_name,
            model.service_type,
            model.document,
            model.contact_name,
            model.primary_phone,
            model.secondary_phone,
            model.operational_base,
            model.notes,
            Some(model.created_at),
            Some(model.updated_at),
        )
    }
}
