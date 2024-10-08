use common_enums::EntityType;
use common_utils::id_type;
use diesel::{AsChangeset, Identifiable, Insertable, Queryable, Selectable};
use time::PrimitiveDateTime;

use crate::{enums, schema::user_roles};

#[derive(Clone, Debug, Identifiable, Queryable, Selectable)]
#[diesel(table_name = user_roles, check_for_backend(diesel::pg::Pg))]
pub struct UserRole {
    pub id: i32,
    pub user_id: String,
    pub merchant_id: Option<id_type::MerchantId>,
    pub role_id: String,
    pub org_id: Option<id_type::OrganizationId>,
    pub status: enums::UserStatus,
    pub created_by: String,
    pub last_modified_by: String,
    pub created_at: PrimitiveDateTime,
    pub last_modified: PrimitiveDateTime,
    pub profile_id: Option<String>,
    pub entity_id: Option<String>,
    pub entity_type: Option<EntityType>,
    pub version: enums::UserRoleVersion,
}

#[derive(router_derive::Setter, Clone, Debug, Insertable, router_derive::DebugAsDisplay)]
#[diesel(table_name = user_roles)]
pub struct UserRoleNew {
    pub user_id: String,
    pub merchant_id: Option<id_type::MerchantId>,
    pub role_id: String,
    pub org_id: Option<id_type::OrganizationId>,
    pub status: enums::UserStatus,
    pub created_by: String,
    pub last_modified_by: String,
    pub created_at: PrimitiveDateTime,
    pub last_modified: PrimitiveDateTime,
    pub profile_id: Option<String>,
    pub entity_id: Option<String>,
    pub entity_type: Option<EntityType>,
    pub version: enums::UserRoleVersion,
}

#[derive(Clone, Debug, AsChangeset, router_derive::DebugAsDisplay)]
#[diesel(table_name = user_roles)]
pub struct UserRoleUpdateInternal {
    role_id: Option<String>,
    status: Option<enums::UserStatus>,
    last_modified_by: Option<String>,
    last_modified: PrimitiveDateTime,
}

#[derive(Clone)]
pub enum UserRoleUpdate {
    UpdateStatus {
        status: enums::UserStatus,
        modified_by: String,
    },
    UpdateRole {
        role_id: String,
        modified_by: String,
    },
}

impl From<UserRoleUpdate> for UserRoleUpdateInternal {
    fn from(value: UserRoleUpdate) -> Self {
        let last_modified = common_utils::date_time::now();
        match value {
            UserRoleUpdate::UpdateRole {
                role_id,
                modified_by,
            } => Self {
                role_id: Some(role_id),
                last_modified_by: Some(modified_by),
                status: None,
                last_modified,
            },
            UserRoleUpdate::UpdateStatus {
                status,
                modified_by,
            } => Self {
                status: Some(status),
                last_modified,
                last_modified_by: Some(modified_by),
                role_id: None,
            },
        }
    }
}
