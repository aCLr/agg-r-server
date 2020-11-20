use actix_web::dev::*;
use actix_web::http::header::Header;
use actix_web::{
    dev::Payload, error::ErrorUnauthorized, Error, FromRequest, HttpMessage, HttpRequest,
};
use futures::{future, task, FutureExt};
use std::rc::Rc;

use actix_web::dev::ServiceRequest;

use crate::db::models::User;
use crate::db::queries;
use crate::db::Pool;
use actix_web::web::Data;
use actix_web_httpauth::headers::authorization;
use futures::future::{ok, LocalBoxFuture};
use futures::task::Poll;
use std::cell::RefCell;

impl FromRequest for User {
    type Config = ();
    type Error = Error;
    type Future = future::Ready<Result<User, self::Error>>;

    fn from_request(req: &HttpRequest, _pl: &mut Payload) -> Self::Future {
        ok(req.extensions().get::<User>().unwrap().clone())
    }
}

// It makes Middleware. It's Intermediate Object.
#[derive(Default)]
pub struct Authorization;

impl<S, B> Transform<S> for Authorization
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = future::Ready<Result<Self::Transform, Self::InitError>>;

    // New Middlware Instance
    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware(Rc::new(RefCell::new(service))))
    }
}

/// The actual Flash middleware
pub struct AuthMiddleware<S>(Rc<RefCell<S>>);

impl<S, B> Service for AuthMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<ServiceResponse<B>, Error>>;

    fn poll_ready(&mut self, ctx: &mut task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let service = Rc::clone(&self.0);
        let db_pool = req.app_data::<Data<Pool>>().unwrap().clone();
        let token = match authorization::Authorization::<authorization::Bearer>::parse(&req) {
            Ok(bearer) => bearer.into_scheme().token().to_string(),
            Err(err) => return Box::pin(async { Err(ErrorUnauthorized(err)) }),
        };

        async move {
            match queries::get_user_by_token(&db_pool, token).await {
                None => Err(ErrorUnauthorized("unauthorized")),
                Some(user) => {
                    req.extensions_mut().insert(user);
                    service.borrow_mut().call(req).await
                }
            }
        }
        .boxed_local()
    }
}
