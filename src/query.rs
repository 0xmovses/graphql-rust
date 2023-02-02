use juniper::{graphql_object, FieldResult};

use crate::schema::{Database, User};

pub struct QueryRoot;

#[graphql_object(context = Database)]
impl QueryRoot {
    fn get_all_users(context: &Database) -> FieldResult<Vec<User>>{
        let users = context.get_all_users();
        let mut result = Vec::<User>::new();
        result.reserve(users.len());

        for user in users {
            result.push(User { id: user.id, name: user.name.clone() })
        }

        Ok(result)
    }
}
