use serde::{Deserialize, Serialize};

use crate::{config::parameter, error::service_error::ServiceError};

pub struct RedisClient {
    redis_connection: redis::Connection,
}

impl RedisClient {
    pub fn new() -> Result<Self, ServiceError> {
        let redis_conn_url = parameter::get("REDIS_CONNECTION_URL");
        let redis_client = redis::Client::open(redis_conn_url).map_err(ServiceError::RedisError)?;

        let redis_connection = redis_client
            .get_connection()
            .map_err(ServiceError::RedisError)?;

        Ok(Self { redis_connection })
    }
}
pub trait RedisClientExt {
    #[allow(dead_code)]
    fn execute_cmd(&mut self, cmd: &RedisCommand) -> Result<(), ServiceError>;
    fn store_refresh_token_with_expiry(
        &mut self,
        token: &str,
        expiry_secs: u64,
    ) -> Result<(), ServiceError>;
    fn rotate_refresh_token_with_expiry(
        &mut self,
        old_refresh_token: &str,
        new_refresh_token: &str,
        expiry_secs: u64,
    ) -> Result<(), ServiceError>;
    fn get_refresh_token(&mut self, key: &str) -> Result<Option<String>, ServiceError>;
}

impl RedisClientExt for RedisClient {
    fn execute_cmd(&mut self, cmd: &RedisCommand) -> Result<(), ServiceError> {
        let redis_connection = &mut self.redis_connection;
        // Remove sensitive data from logs
        log::debug!("Executing Redis command: {} {}", cmd.cmd, cmd.key);
        redis::cmd(&cmd.cmd)
            .arg(&cmd.key)
            .arg(&cmd.value)
            .exec(redis_connection)
            .map_err(ServiceError::RedisError)
    }

    fn store_refresh_token_with_expiry(
        &mut self,
        token: &str,
        expiry_secs: u64,
    ) -> Result<(), ServiceError> {
        let key = &format!("refresh_token:{token}");

        redis::cmd("SETEX")
            .arg(key)
            .arg(expiry_secs)
            .arg(token)
            .exec(&mut self.redis_connection)
            .map_err(ServiceError::RedisError)?;
        Ok(())
    }

    fn rotate_refresh_token_with_expiry(
        &mut self,
        old_refresh_token: &str,
        new_refresh_token: &str,
        expiry_secs: u64,
    ) -> Result<(), ServiceError> {
        let old_key = format!("refresh_token:{old_refresh_token}");
        let new_key = format!("refresh_token:{new_refresh_token}");

        // Use atomic transaction for rotation
        redis::pipe()
            .atomic()
            .del(&old_key)
            .set_ex(&new_key, new_refresh_token, expiry_secs)
            .query::<()>(&mut self.redis_connection)
            .map_err(ServiceError::RedisError)?;

        Ok(())
    }

    fn get_refresh_token(&mut self, key: &str) -> Result<Option<String>, ServiceError> {
        let result: Option<String> = redis::cmd("GET")
            .arg(key)
            .query(&mut self.redis_connection)
            .map_err(ServiceError::RedisError)?;
        Ok(result)
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct RedisCommand {
    pub cmd: String,
    pub value: String,
    pub key: String,
}

#[derive(Serialize, Deserialize, Default)]
struct RedisCommandBuilder {
    cmd: String,
    value: String,
    key: String,
}

#[allow(dead_code)]
impl RedisCommandBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn cmd(mut self, cmd: &str) -> Self {
        self.cmd = cmd.to_string();
        self
    }

    pub fn key(mut self, key: &str) -> Self {
        self.key = key.to_string();
        self
    }

    #[allow(unused)]
    pub fn value(mut self, value: &str) -> Self {
        self.value = value.to_string();
        self
    }

    pub fn build(&self) -> RedisCommand {
        RedisCommand {
            cmd: self.cmd.to_string(),
            value: self.value.to_string(),
            key: self.key.to_string(),
        }
    }
}
