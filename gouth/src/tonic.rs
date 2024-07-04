use crate::Token;
use tonic::{metadata::MetadataValue, Interceptor, Request, Status};

macro_rules! map_err {
    ($res:expr) => {
        $res.map_err(|e| Status::unknown(e.to_string()))
    };
}

pub fn interceptor() -> impl Into<Interceptor> {
    let token = Token::new().expect("Token::new()");
    move |mut req: Request<()>| {
        // let res = futures::executor::block_on(async move { token.header_value().await });
        // let token = map_err!(res)?;
        // let meta = map_err!(MetadataValue::from_str(&*token))?;
        // req.metadata_mut().insert("authorization", meta);
        // Ok(req)
        todo!()
    }
}
