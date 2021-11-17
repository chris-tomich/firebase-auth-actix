use std::pin::Pin;

use actix_web::{HttpMessage, dev::{Service, Transform, ServiceRequest, ServiceResponse}, Error};
use firebase_auth::TokenValidator;
use futures::{Future, future::{ok, Ready}};

pub struct FirebaseAuthentication {
    pub firebase_project_id: String,
    pub firebase_project_issuer: String,
    pub firebase_public_keys_jwk_url: String,
}

impl<S, B> Transform<S> for FirebaseAuthentication
where
S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
S::Future: 'static,
B: 'static,
{
    type Request = ServiceRequest;

    type Response = ServiceResponse<B>;

    type Error = Error;

    type Transform = FirebaseAuthenticationMiddleware<S>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(FirebaseAuthenticationMiddleware {
            service,
            base_token_validator: TokenValidator {
                firebase_project_id: self.firebase_project_id.clone(),
                firebase_project_issuer: self.firebase_project_issuer.clone(),
                firebase_public_keys_jwk_url: self.firebase_public_keys_jwk_url.clone(),
            },
        })
    }
}

pub struct FirebaseAuthenticationMiddleware<S> {
    service: S,
    base_token_validator: TokenValidator,
}

impl<S, B> Service for FirebaseAuthenticationMiddleware<S>
where
S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
S::Future: 'static,
B: 'static,
{
    type Request = ServiceRequest;

    type Response = ServiceResponse<B>;

    type Error = Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let token_validator = self.base_token_validator.clone();
        req.extensions_mut().insert(token_validator);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            Ok(res)
        })
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
