use crate::{
    protocol::{
        request::{Function, FunctionTypes},
        response::Response,
    },
    service::ServiceTrait,
};

struct Handler {
    service: Box<dyn ServiceTrait>,
}

impl Handler {
    pub fn new(svc: impl ServiceTrait + 'static) -> Handler {
        Handler {
            service: Box::new(svc),
        }
    }

    pub fn handle(&self, raw: &Vec<u8>) -> Vec<u8> {
        let func = match Function::parse(raw) {
            Ok(f) => f,
            Err(e) => {
                // handle err
                todo!()
            }
        };

        match func.function {
            FunctionTypes::Add => self.service.create(func).to_vec(),
            FunctionTypes::Delete => self.service.delete(func).to_vec(),
            FunctionTypes::Describe => self.service.describe(func).to_vec(),
        }
    }
}
