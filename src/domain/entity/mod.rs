
use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use serde::{Deserialize, Serialize};

#[crud_enable]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cgm {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub device: Option<String>,
    pub date: Option<i64>,
    pub date_str: Option<String>,
    pub sgv: Option<i32>,
    pub delta: Option<f32>,
    pub direction: Option<String>,
    pub type1: Option<String>,
    pub filtered: Option<f64>,
    pub unfiltered: Option<f64>,
    pub rssi: Option<i32>,
    pub noise: Option<i32>,
    pub sys_time: Option<String>,
    pub utc_offset: Option<i32>,
    pub slope: Option<f64>,
    pub intercept: Option<f64>,
    pub scale: Option<i32>,
    pub mbg: Option<f64>,
}

#[crud_enable]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeviceStatus {
    pub id: Option<i64>,
    pub user_id: Option<i64>,
    pub device: Option<String>,
    pub battery: Option<i32>,
    pub created_time: Option<NaiveDateTime>
}