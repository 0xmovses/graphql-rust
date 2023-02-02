use juniper::{graphql_object, FieldResult};

use crate::schema::{User, UserInput, DatabaseContext};

pub struct MutationRoot;

#[graphql_object(context = DatabaseContext)]
impl MutationRoot {
    fn create_user(context: &DatabaseContext, user: UserInput) -> FieldResult<User>{
        let mut write = context.0.write().expect("could not access the database");
        let user = User { id: user.id, name: user.name };
        let user_to_return = user.clone();
        write.insert(user);
        Ok(user_to_return)
    }
}
