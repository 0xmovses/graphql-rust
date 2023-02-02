use std::collections::HashMap;

use juniper::{EmptySubscription, RootNode, EmptyMutation, GraphQLObject};

use crate::query::QueryRoot;

pub type Schema = RootNode<'static, QueryRoot,EmptyMutation<Database>, EmptySubscription<Database>>;

pub fn create_schema() -> Schema {
    Schema::new(
        QueryRoot {},
        EmptyMutation::<Database>::default(),
        EmptySubscription::<Database>::default(),
    )
}

#[derive(GraphQLObject)]
pub struct User {
    pub id: i32,
    pub name: String
}

pub struct Database {
    users: HashMap<i32, User>
}

impl Database {
    pub fn new() -> Self{
        let mut users = HashMap::new();
        users.insert(0, User { id: 0, name: String::from("Alice")});
        users.insert(1, User { id: 0, name: String::from("Bob")});
        Database { users }
    }

    pub fn get_all_users(&self) -> Vec<&User> {
        Vec::from_iter(self.users.values())
    }
}


