use std::collections::{HashMap, BTreeMap};
use tokio::sync::Mutex;
// Define the nested user structure
#[derive(Debug, Clone, PartialEq)]  // Add PartialEq trait derivation
pub struct UserData {
    name: String,
    lastname: String,
}

// MockRedisDatabase struct simulating a Redis database with nested hashmap commands
pub struct MockRedisDatabase {
    data: Mutex<HashMap<String, BTreeMap<String, UserData>>>,
}

impl MockRedisDatabase {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }

    pub async fn hset_user(&self, key: &str, field: &str, user_data: UserData) {
        let mut data = self.data.lock().await;
        let entry = data.entry(key.to_string()).or_insert_with(BTreeMap::new);
        entry.insert(field.to_string(), user_data);
    }

    pub async fn hget_user(&self, key: &str, field: &str) -> Option<UserData> {
        let data = self.data.lock().await;
        data.get(key).and_then(|entry| entry.get(field).cloned())
    }

    pub async fn hdel_user(&self, key: &str, field: &str) -> bool {
        let mut data = self.data.lock().await;
        if let Some(entry) = data.get_mut(key) {
            entry.remove(field);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_redis_hset_hget_hdel_user() {
        let mock_db = MockRedisDatabase::new();

        // Test HSET
        let user_data = UserData {
            name: "John".to_string(),
            lastname: "Doe".to_string(),
        };
        mock_db.hset_user("user_id", "field1", user_data.clone()).await;
        assert_eq!(mock_db.hget_user("user_id", "field1").await, Some(user_data.clone()));

        // Test HDEL
        assert_eq!(mock_db.hdel_user("user_id", "field1").await, true);
        assert_eq!(mock_db.hget_user("user_id", "field1").await, None);

        // Test non-existing key
        assert_eq!(mock_db.hget_user("non_existent", "field1").await, None);

        // Test non-existing field
        assert_eq!(mock_db.hget_user("user_id", "non_existent").await, None);
    }
}


