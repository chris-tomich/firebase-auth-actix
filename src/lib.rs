use std::{future::Ready, pin::Pin};

use actix_service::{Service, Transform};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, http::Error};
use futures::{Future, future::{ok}};

pub struct FirebaseAuthentication;

impl<S, B> Transform<S, ServiceRequest> for FirebaseAuthentication
where
S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
S::Future: 'static,
B: 'static,
{
    type Response = ServiceResponse<B>;

    type Error = Error;

    type Transform = FirebaseAuthenticationMiddleware<S>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        todo!()
    }
}

pub struct FirebaseAuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for FirebaseAuthenticationMiddleware<S>
where
S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
S::Future: 'static,
B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        todo!()
    }

    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
