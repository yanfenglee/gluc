use actix_http::Response;
use actix_web::HttpResponse;
use chrono::NaiveDateTime;
use rbatis::crud::CRUDEnable;
use rbatis::core::Error;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;


/// http接口返回模型结构，提供基础的 code，msg，data 等json数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RespVO<T> {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T> where T: Serialize + DeserializeOwned + Clone {
    pub fn from_result(arg: &Result<T, Error>) -> Self {
        if arg.is_ok() {
            Self {
                code: Some("SUCCESS".to_string()),
                msg: None,
                data: arg.clone().ok(),
            }
        } else {
            Self {
                code: Some("FAIL".to_string()),
                msg: Some(arg.clone().err().unwrap().to_string()),
                data: None,
            }
        }
    }

    pub fn from(arg: &T) -> Self {
        Self {
            code: Some("SUCCESS".to_string()),
            msg: None,
            data: Some(arg.clone()),
        }
    }

    pub fn from_error(code: &str, arg: &Error) -> Self {
        let mut code_str = code.to_string();
        if code_str.is_empty() {
            code_str = "FAIL".to_string();
        }
        Self {
            code: Some(code_str),
            msg: Some(arg.to_string()),
            data: None,
        }
    }

    pub fn from_error_info(code: &str, info: &str) -> Self {
        let mut code_str = code.to_string();
        if code_str.is_empty() {
            code_str = "FAIL".to_string();
        }
        Self {
            code: Some(code_str),
            msg: Some(info.to_string()),
            data: None,
        }
    }

    pub fn resp(&self) -> Response {
        return HttpResponse::Ok().content_type("json").body(self.to_string());
    }

}

pub fn resp<T>(ret: &Result<T, Error>) -> Response where T: Serialize + DeserializeOwned + Clone {
    let vo = RespVO::from_result(ret).resp();
    vo
}

impl<T> ToString for RespVO<T> where T: Serialize + DeserializeOwned + Clone {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}