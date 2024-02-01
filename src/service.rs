use crate::protocol::{request::Function, response::Response};

pub trait ServiceTrait {
    fn create(&self, function: Function) -> Response;
    fn delete(&self, function: Function) -> Response;
    fn describe(&self, function: Function) -> Response;
}
