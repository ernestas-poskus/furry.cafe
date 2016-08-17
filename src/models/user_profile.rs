use diesel;

use models::schema::user_profiles;
use models::user::User;
use database;
use models;
use error;

#[derive(Queryable, Identifiable)]
#[belongs_to(User)]
pub struct UserProfile {
    pub id: i64,
    pub user_id: i64,
    pub created_at: diesel::data_types::PgTimestamp,
    pub updated_at: diesel::data_types::PgTimestamp,
    pub bio: String,
}

impl UserProfile {
    pub fn create_from(nup: NewUserProfile) -> Result<(), error::DatabaseError> {
        use diesel;
        use diesel::prelude::*;
        use models::schema::user_profiles::dsl::*;
        diesel::insert(&nup)
            .into(user_profiles).execute(&*database::connection().get().unwrap()).map_err(|e| e.into()).map(|_| ())
    }
}

#[derive(Clone, Debug)]
#[insertable_into(user_profiles)]
pub struct NewUserProfile<'a> {
    pub user_id: i64,
    pub bio: &'a str,
}

impl<'a> NewUserProfile<'a> {
    pub fn new(user: &User, bio: &'a str) -> NewUserProfile<'a> {
        NewUserProfile {
            user_id: user.id,
            bio: bio,
        }
    }

    pub fn from(profile: &'a UserProfile) -> NewUserProfile<'a> {
        NewUserProfile {
            user_id: profile.user_id,
            bio: &profile.bio,
        }
    }
}

pub fn find_by_user_id(uid: i64) -> Result<Option<UserProfile>, error::DatabaseError> {
    use diesel::prelude::*;
    use models::schema::user_profiles::dsl::*;

    user_profiles.limit(1).filter(user_id.eq(uid)).order(created_at.desc())
         .get_result::<models::user_profile::UserProfile>(&*database::connection().get().unwrap()).optional().map_err(|e| e.into())
}
