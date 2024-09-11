use std::env;
use hmac::{Hmac, Mac};
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::Write;
use anyhow::Result;
use hex;
use crate::alias::param_alias::{FromLang, TargetLang};
use crate::sdk::crypto::Crypto;

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
    pub async fn call_service_with_payload(&self, action: &str, payload: &str) -> Result<String> {
        let service = "tmt";
        let version = "2018-03-21";
        let region = "ap-guangzhou";
        let secret_id = &self.secure_id;
        let secret_key = &self.secure_key;
        let token = "";
        let host = "tmt.tencentcloudapi.com";
        let algorithm = "TC3-HMAC-SHA256";
        // 获取当前时间戳
        let start = SystemTime::now();
        let timestamp = start.duration_since(UNIX_EPOCH)?.as_secs();

        // ************* 步骤 1：拼接规范请求串 *************
        let http_request_method = "POST";
        let canonical_uri = "/";
        let canonical_query_string = "";
        let content_type = "application/json; charset=utf-8";
        let canonical_headers = format!(
            "content-type:{}\nhost:{}\nx-tc-action:{}\n",
            content_type, host, action.to_lowercase()
        );
        let signed_headers = "content-type;host;x-tc-action";
        let hashed_request_payload = Crypto::sha256hex(payload);
        let canonical_request = format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            http_request_method,
            canonical_uri,
            canonical_query_string,
            canonical_headers,
            signed_headers,
            hashed_request_payload
        );
        println!("Canonical Request: {}", canonical_request);

        // ************* 步骤 2：拼接待签名字符串 *************
        let date = chrono::Utc::now().format("%Y-%m-%d").to_string();
        let credential_scope = format!("{}/{}/tc3_request", date, service);
        let hashed_canonical_request = Crypto::sha256hex(&canonical_request);
        let string_to_sign = format!(
            "{}\n{}\n{}\n{}",
            algorithm, timestamp, credential_scope, hashed_canonical_request
        );
        println!("String to Sign: {}", string_to_sign);

        // ************* 步骤 3：计算签名 *************
        let secret_date = Crypto::hmacsha256(date.as_ref(), format!("TC3{}", secret_key).as_ref());
        let secret_service = Crypto::hmacsha256(service.as_ref(), &secret_date);
        let secret_signing = Crypto::hmacsha256("tc3_request".as_ref(), &secret_service);
        let signature_bin = Crypto::hmacsha256(string_to_sign.as_ref(), &secret_signing);
        let signature = hex::encode(&signature_bin);
        println!("Signature: {:?}", signature);

        // ************* 步骤 4：拼接 Authorization *************
        let authorization = format!(
            "{} Credential={}/{}, SignedHeaders={}, Signature={}",
            algorithm, secret_id, credential_scope, signed_headers, signature
        );
        println!("Authorization: {}", authorization);

        // ************* 步骤 5：构造并发起请求 *************
        let url = format!("https://{}", host);
        let mut headers = HeaderMap::new();
        headers.insert("Host", HeaderValue::from_str(host)?);
        headers.insert("X-TC-Action", HeaderValue::from_str(action)?);
        headers.insert("X-TC-Version", HeaderValue::from_str(version)?);
        headers.insert("X-TC-Timestamp", HeaderValue::from_str(&timestamp.to_string())?);
        headers.insert(CONTENT_TYPE, HeaderValue::from_str(content_type)?);
        headers.insert("Authorization", HeaderValue::from_str(&authorization)?);
        if !region.is_empty() {
            headers.insert("X-TC-Region", HeaderValue::from_str(region)?);
        }
        if !token.is_empty() {
            headers.insert("X-TC-Token", HeaderValue::from_str(token)?);
        }

        let client = reqwest::Client::new();
        let res = client
            .post(&url)
            .headers(headers)
            .body(payload.to_string())
            .send()
            .await?;
        let body = res.text().await?;
        println!("Response: {}", body);
        Ok(body)
    }
    pub async fn translate_text(&self, text: &str, from_lang: &FromLang, target_lang: &TargetLang) -> Result<String> {
        let action = "TextTranslate";
        let req_payload = json!({
             "SourceText": text,
             "Source": from_lang,
             "Target": target_lang,
             "ProjectId": 0
        }).to_string();
        let res_json_str = self.call_service_with_payload(action, &req_payload).await?;
        println!("{}", res_json_str);
        Ok(String::from('s'))
    }
}
