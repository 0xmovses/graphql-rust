use juniper::{graphql_object, FieldResult, FieldError};

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

    fn get_user_by_id(context: &Database, id: i32) -> FieldResult<User> {
        let user = context.get_user_by_id(&id);
        match user {
            Some(user) => Ok(User { id: user.id, name: user.name.clone() }),
            None => Err(FieldError::from("could not find the user"))
        }

    }
}
