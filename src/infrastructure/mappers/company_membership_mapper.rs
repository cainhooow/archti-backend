use crate::domain::entities::company_membership::CompanyMembership as DomainCompanyMembership;
use crate::domain::entities::company_membership::{
    CompanyMembership, MembershipStatus, MembershipType,
};
use crate::infrastructure::models::company_membership::Model as CompanyMembershipModel;

impl TryFrom<CompanyMembershipModel> for DomainCompanyMembership {
    type Error = String;

    fn try_from(model: CompanyMembershipModel) -> Result<Self, Self::Error> {
        Ok(CompanyMembership::restore(
            model.id,
            model.company_id,
            model.user_id,
            MembershipType::try_from(model.membership_type.as_str()).unwrap(),
            MembershipStatus::try_from(model.status_key.as_str()).unwrap(),
            model.display_name,
            model.invited_at,
            model.accepted_at,
            model.last_seen_at,
            model.created_at,
            model.updated_at,
        ))
    }
}
