use std::fmt::Debug;

use redis::ToRedisArgs;
use serde::{Serialize, de::DeserializeOwned};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct RedisMessage<T>
where
    T: Serialize + DeserializeOwned + Debug,
{
    identifier: Uuid,
    payload: T,
}

impl<T> ToRedisArgs for RedisMessage<T>
where
    T: Serialize + DeserializeOwned + Debug,
{
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        todo!()
    }

    fn to_redis_args(&self) -> Vec<Vec<u8>> {
        let mut out = Vec::new();
        self.write_redis_args(&mut out);
        out
    }

    fn describe_numeric_behavior(&self) -> redis::NumericBehavior {
        redis::NumericBehavior::NonNumeric
    }

    fn num_of_args(&self) -> usize {
        1
    }
}
