use crate::SurrealDBManager;
use common::prelude::FiatIsoCode;
use surrealdb::Error;

const SERVICE_TABLE: &str = "fiat_iso_code";

impl SurrealDBManager {
    pub async fn create_fiat_iso_code(&self, data: FiatIsoCode) -> Result<bool, Error> {
        let id = data.iso_code().to_string();

        let created: Option<FiatIsoCode> = self
            .db
            .update((SERVICE_TABLE, id))
            .merge(data)
            .await
            .expect("Failed to create service");

        match created {
            None => Ok(false),
            Some(_) => Ok(true),
        }
    }

    pub async fn check_if_fiat_iso_code_exists(&self, iso_code: u16) -> Result<bool, Error> {
        let res = self
            .read_record_by_id(iso_code)
            .await
            .expect("Failed to check if service id exists");

        match res {
            None => Ok(false),
            Some(_) => Ok(true),
        }
    }

    pub async fn get_currency_code(&self, iso_code: u16) -> Result<Option<String>, Error> {
        let id = iso_code.to_string();
        let q = format!(
            "SELECT VALUE alphabetic_code FROM {}:{};",
            SERVICE_TABLE, id
        );

        let mut res = self
            .db
            .query(q)
            .await
            .expect("Failed to check if service id exists");

        let res = res.take(0).expect("Failed to get online status");

        match res {
            None => Ok(None),
            Some(res) => Ok(res),
        }
    }

    pub async fn read_fiat_iso_code(&self, iso_code: u16) -> Result<Option<FiatIsoCode>, Error> {
        let res = self
            .db
            .select((SERVICE_TABLE, &iso_code.to_string()))
            .await
            .expect("Failed to read FiatIsoCode by id");

        Ok(res)
    }

    pub async fn update_fiat_iso_code(
        &self,
        data: FiatIsoCode,
    ) -> Result<Option<FiatIsoCode>, Error> {
        let id = data.iso_code().to_string();

        let updated = self
            .db
            .update((SERVICE_TABLE, id))
            .content(data)
            .await
            .expect("Failed to update service");

        Ok(updated)
    }

    pub async fn delete_fiat_iso_code(&self, id: u16) -> Result<bool, Error> {
        let deleted: Option<FiatIsoCode> = self
            .db
            .delete((SERVICE_TABLE, &id.to_string()))
            .await
            .expect("Failed to delete service");

        match deleted {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}

impl SurrealDBManager {
    async fn read_record_by_id(&self, id: u16) -> Result<Option<FiatIsoCode>, Error> {
        let res = self
            .db
            .select((SERVICE_TABLE, &id.to_string()))
            .await
            .expect("Failed to read service by id");

        Ok(res)
    }
}
