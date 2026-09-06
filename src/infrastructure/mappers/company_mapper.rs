use crate::domain::entities::company::Company as DomainCompany;
use crate::infrastructure::models::company::Model as CompanyModel;

impl TryFrom<CompanyModel> for DomainCompany {
    type Error = String;

    fn try_from(model: CompanyModel) -> Result<Self, Self::Error> {
        DomainCompany::restore(
            model.id,
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
