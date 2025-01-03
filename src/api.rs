use crate::types::TestObject;
use gloo_net::{http::Request, Error};

pub async fn get_items()-> Result<Vec<TestObject>, Error> {
   let res = Request::get("/constants/objects.json").send().await?;

   let items: Vec<TestObject> = res.json().await?;

   Ok(items)
}
