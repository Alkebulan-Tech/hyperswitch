use actix_web::test::TestRequest;
use serde_json::value::{Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MasterData{
  pub merchant_id : Option<String>,
  pub api_key : Option<String>,
  pub api_key_id : Option<String>,
  pub payment_id : Option<String>,
  pub customer_id : Option<String>,
// fields in test data
  pub admin_api_key : String,
  pub merchant_account : Value,
  pub merchant_account_update : Option<Value>,
  pub merchant_account_delete : Option<Value>,
  pub merchant_account_retreive : Option<Value>,
  pub customers : Option<Value>,
  pub connector_create : Option<Value>,
  pub api_key_create : Option<Value>,
  pub api_key_update : Option<Value>,
  pub api_key_delete : Option<Value>,
  pub api_key_retreive : Option<Value>,
  pub payments_create : Option<Value>,
  pub payments_retrieve : Option<Value>,
  pub payment_confirm : Option<Value>,
  pub payment_capture : Option<Value>,
}

pub trait RequestBuilder{
  fn make_request_body(data : &MasterData) -> Option<TestRequest>;
  fn verify_success_response(response : &Value, data : &MasterData) -> Self;
  fn verify_failure_response(response : &Value, data : &MasterData) -> Self;
  fn update_master_data(&self,data : &mut MasterData, resp : &Value);
}