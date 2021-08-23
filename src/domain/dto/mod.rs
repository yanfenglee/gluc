
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::domain::entity::{Cgm, DeviceStatus};
use rbatis::core::value::DateTimeNow;

/// blood glucose dto
#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BgDTO {
    pub id: Option<i64>,
    pub device: Option<String>,
    pub date: Option<i64>,
    pub dateString: Option<String>,
    pub sgv: Option<i32>,
    pub delta: Option<f32>,
    pub direction: Option<String>,
    #[serde(rename = "type")]
    pub type1: Option<String>,
    pub filtered: Option<f64>,
    pub unfiltered: Option<f64>,
    pub rssi: Option<i32>,
    pub noise: Option<i32>,
    pub sysTime: Option<String>,
    pub utcOffset: Option<i32>,
    pub slope: Option<f64>,
    pub intercept: Option<f64>,
    pub scale: Option<i32>,
    pub mbg: Option<f64>,
}

impl From<&Cgm> for BgDTO {
    fn from(arg: &Cgm) -> Self {
        BgDTO {
            id: arg.id,
            device: arg.device.clone(),
            date: arg.date,
            dateString: arg.date_str.clone(),
            sgv: arg.sgv,
            delta: arg.delta,
            direction: arg.direction.clone(),
            type1: arg.type1.clone(),
            filtered: arg.filtered,
            unfiltered: arg.unfiltered,
            rssi: arg.rssi,
            noise: arg.noise,
            sysTime: arg.sys_time.clone(),
            utcOffset: arg.utc_offset,
            slope: arg.slope,
            intercept: arg.intercept,
            scale: arg.scale,
            mbg: arg.mbg,
        }
    }
}

impl From<&BgDTO> for Cgm {
    fn from(arg: &BgDTO) -> Cgm {
        Cgm {
            id: None,
            user_id: None,
            device: arg.device.clone(),
            date: arg.date,
            date_str: arg.dateString.clone(),
            sgv: arg.sgv,
            delta: arg.delta,
            direction: arg.direction.clone(),
            type1: arg.type1.clone(),
            filtered: arg.filtered,
            unfiltered: arg.unfiltered,
            rssi: arg.rssi,
            noise: arg.noise,
            sys_time: arg.sysTime.clone(),
            utc_offset: arg.utcOffset,
            slope: arg.slope,
            intercept: arg.intercept,
            scale: arg.scale,
            mbg: arg.mbg,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Uploader {
    pub battery: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeviceStatusDTO {
    pub id: Option<i64>,
    pub device: Option<String>,
    pub uploader: Uploader,
}

impl From<&DeviceStatusDTO> for DeviceStatus {
    fn from(dto: &DeviceStatusDTO) -> Self {
        DeviceStatus {
            id: Some(rbatis::plugin::snowflake::new_snowflake_id()),
            user_id: None,
            device: dto.device.clone(),
            battery: dto.uploader.battery,
            created_time: Some(NaiveDateTime::now()),
        }
    }
}

impl From<&DeviceStatus> for DeviceStatusDTO {
    fn from(arg: &DeviceStatus) -> Self {
        DeviceStatusDTO {
            id: arg.id,
            device: arg.device.clone(),
            uploader: Uploader { battery: arg.battery },
        }
    }
}