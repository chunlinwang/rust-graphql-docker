use crate::loaders::person_loader::PersonBatcher;
use dataloader::Loader;
use juniper;
use futures::executor;

#[derive(Debug, Clone)]
pub struct Person {
    pub person_id: i32,
    pub person_name: String,
    pub cult: i32,
}

#[juniper::graphql_object(Context = Context)]
impl Person {
  pub fn id(&self) -> i32 {
    self.person_id
  }

  pub fn name(&self) -> &str {
    self.person_name.as_str()
  }
}

#[derive(Clone)]
pub struct Context {
    person_loader: Loader<i32, Person, (), PersonBatcher>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(person_loader: Loader<i32, Person, (), PersonBatcher>) -> Self {
        Self { person_loader }
    }
}

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    fn users(context: &Context, limit: Option<i32>) -> Vec<i32> {
        let vec = vec![1, 2, 3, 4];
        vec
    }
    fn person_by_id(context: &Context, id: i32) -> Person {
        let res = context.person_loader.load(id);
        executor::block_on(res).unwrap()
    }
}

pub struct Mutation;

#[juniper::graphql_object(Context = Context)]
impl Mutation {
    // not really needed, but graphiql bug if this is empty…
    pub fn nothing(name: String) -> i32 {
        0
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
