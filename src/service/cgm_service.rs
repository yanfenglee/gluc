use chrono::NaiveDateTime;
use rbatis::crud::CRUD;
use rbatis::core::Error;
use rbatis::core::Result;
use rbatis::core::value::DateTimeNow;

use crate::dao::RB;
use crate::domain::dto::BgDTO;
use crate::domain::entity::Cgm;
use actix_web::guard::Guard;
use std::ops::Deref;

///Cgm service
pub struct CgmService {}

impl CgmService {
    pub async fn add(&self, arg: &Vec<BgDTO>, user_id: i64) -> Result<u64> {
        let entries: Vec<Cgm> = arg.iter().map(|item| {
            let mut cgm: Cgm = item.into();
            cgm.id = Some(rbatis::plugin::snowflake::new_snowflake_id());
            cgm.user_id = Some(user_id);
            cgm
        }).collect();

        Ok(RB.save_batch("", &entries).await?.rows_affected)
    }


    pub async fn list(&self, ts: i64, cnt: i64, user_id: i64) -> Result<Vec<BgDTO>> {

        #[py_sql(RB, "SELECT * FROM cgm WHERE user_id = #{user_id} and `date` < #{ts} order by `date` desc LIMIT #{cc}")]
        async fn select_entries(user_id: i64, ts: i64, cc: i64) -> Vec<Cgm> {}

        let cgms = select_entries(user_id, ts, cnt).await?;

        Ok(cgms.iter().map(|x| x.into()).collect())
    }
}
