use crate::alias::param_alias::{FromLang, TargetLang};
use crate::sdk::crypto::Crypto;
use anyhow::Result;
use hex;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde::Deserialize;
use serde_json::json;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

const REGION_DISTANCE_FIRST_HOST: &str = "tmt.tencentcloudapi.com";
const SERVICE_NAME: &str = "tmt";
const SERVICE_VERSION: &str = "2018-03-21";
const SERVICE_REGION: &str = "ap-guangzhou";
const SEC_ALGORITHM: &str = "TC3-HMAC-SHA256";
const API_CONTENT_TYPE: &str = "application/json; charset=utf-8";
const SIGNED_HEADERS: &str = "content-type;host;x-tc-action";
pub struct TencentCloudTranslateSDK {
    secure_id: String,
    secure_key: String,
}
impl TencentCloudTranslateSDK {
    pub fn from_env() -> Result<Self> {
        let secure_id = env::var("TCC_SECRET_ID")?;
        let secure_key = env::var("TCC_SECRET_KEY")?;
        Ok(Self { secure_id, secure_key })
    }
    pub async fn call_service_with_json(&self, action: &str, json: &str) -> Result<String> {
        //time data
        let start = SystemTime::now();
        let timestamp = start.duration_since(UNIX_EPOCH)?.as_secs();
        let date = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let credential_scope = format!("{}/{}/tc3_request", date, SERVICE_NAME);
        //call func
        let canonical_request = canonical_request_str(action, json);
        let str_to_sign = string_to_sign(&canonical_request, timestamp, &date);
        let signature_d = signature(&date, &self.secure_key, &str_to_sign);
        let authorization = format!(
            "{} Credential={}/{}, SignedHeaders={}, Signature={}",
            SEC_ALGORITHM, self.secure_id, credential_scope, SIGNED_HEADERS, signature_d
        );
        Ok(send_request(action, json, timestamp, &authorization).await?)
    }
    pub async fn translate_text(&self, text: &str, from_lang: &FromLang, target_lang: &TargetLang) -> Result<String> {
        let action = "TextTranslate";
        let req_payload = json!({
             "SourceText": text,
             "Source": from_lang,
             "Target": target_lang,
             "ProjectId": 0
        }).to_string();
        let res_json_str = self.call_service_with_json(action, &req_payload).await?;
        match  serde_json::from_str::<TranslateResponse>(&res_json_str) {
            Ok(deserialized)=>Ok(deserialized.response.target_text),
            Err(_)=>Err(anyhow::anyhow!("Failed to translate text,response is {}", res_json_str)),
        }
    }
}
fn canonical_request_str(action: &str, payload: &str) -> String {
    let http_request_method = "POST";
    let canonical_uri = "/";
    let canonical_query_string = "";
    let canonical_headers = format!(
        "content-type:{}\nhost:{}\nx-tc-action:{}\n",
        API_CONTENT_TYPE, REGION_DISTANCE_FIRST_HOST, action.to_lowercase()
    );
    let hashed_request_payload = Crypto::sha256hex(payload);
    let canonical_request = format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        http_request_method,
        canonical_uri,
        canonical_query_string,
        canonical_headers,
        SIGNED_HEADERS,
        hashed_request_payload
    );
    canonical_request
}
fn string_to_sign(canonical_request: &str, timestamp: u64, date: &str) -> String {
    let credential_scope = format!("{}/{}/tc3_request", date, SERVICE_NAME);
    let hashed_canonical_request = Crypto::sha256hex(&canonical_request);
    let string_to_sign = format!(
        "{}\n{}\n{}\n{}",
        SEC_ALGORITHM, timestamp, credential_scope, hashed_canonical_request
    );
    string_to_sign
}

fn signature(date: &str, secret_key: &str, str_to_sign: &str) -> String {
    let secret_date = Crypto::hmacsha256(date.as_bytes(), format!("TC3{}", secret_key).as_bytes());
    let secret_service = Crypto::hmacsha256(SERVICE_NAME.as_bytes(), &secret_date);
    let secret_signing = Crypto::hmacsha256("tc3_request".as_bytes(), &secret_service);
    let signature = Crypto::hmacsha256(str_to_sign.as_bytes(), &secret_signing);
    hex::encode(signature)
}
async fn send_request(action: &str, payload: &str, timestamp: u64, authorization: &str) -> Result<String> {
    let url = format!("https://{}", REGION_DISTANCE_FIRST_HOST);
    let mut headers = HeaderMap::new();
    headers.insert("Host", HeaderValue::from_str(REGION_DISTANCE_FIRST_HOST)?);
    headers.insert("X-TC-Action", HeaderValue::from_str(action)?);
    headers.insert("X-TC-Version", HeaderValue::from_str(SERVICE_VERSION)?);
    headers.insert("X-TC-Timestamp", HeaderValue::from_str(&timestamp.to_string())?);
    headers.insert(CONTENT_TYPE, HeaderValue::from_str(API_CONTENT_TYPE)?);
    headers.insert("Authorization", HeaderValue::from_str(authorization)?);
    headers.insert("X-TC-Region", HeaderValue::from_str(SERVICE_REGION)?);

    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .headers(headers)
        .body(payload.to_string())
        .send()
        .await?;
    let body = res.text().await?;
    Ok(body)
}
#[derive(Deserialize, Debug)]
struct TranslateResponse {
    #[serde(rename = "Response")]
    response: TranslateResponseInfo,
}
#[derive(Deserialize, Debug)]
struct TranslateResponseInfo {
    #[serde(rename = "TargetText")]
    target_text: String,
}