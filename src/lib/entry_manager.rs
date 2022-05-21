/// Entry Manager is the
pub trait EntryManager {
    fn add_entry(&mut self, id: &str, entry: Entry) {}
    fn get_entry(&self, name: &str) -> Option<&Entry> {
        None
    }

    fn id_from_entry(&self, entry: &Entry) -> &str {
        ""
    }
}

struct DefaultManager;

impl DefaultManager {
    pub fn new() -> Self {
        DefaultManager
    }
}

impl EntryManager for DefaultManager {}

#[derive(Debug, PartialEq)]
pub enum Entry {
    Site(SiteEntry),
    Wallet(WalletEntry),
}
#[derive(Debug, PartialEq)]
pub struct SiteEntry {
    pub user_name: String,
    pub password: String,
    pub url: String,
}
#[derive(Debug, PartialEq)]
pub struct WalletEntry {
    pub phrase: String,
    /// e.g metamask
    pub name: String,
}

pub trait EncryptedEntry {
    fn encrypt(&self, secret: &str) -> String;
    fn decrypt(&self, secret: &str) -> Self;
}

impl EncryptedEntry for SiteEntry {
    fn encrypt(&self, secret: &str) -> String {
        let mut encrypted_user_name = self.user_name.clone();
        let mut encrypted_password = self.password.clone();
        let mut encrypted_url = self.url.clone();

        encrypted_user_name.push_str(secret);
        encrypted_password.push_str(secret);
        encrypted_url.push_str(secret);
        encrypted_user_name
    }

    fn decrypt(&self, secret: &str) -> Self {
        let mut decrypted_user_name = self.user_name.clone();
        let mut decrypted_password = self.password.clone();
        let mut decrypted_url = self.url.clone();

        SiteEntry {
            user_name: decrypted_user_name,
            password: decrypted_password,
            url: decrypted_url,
        }
    }
}

/// Tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_entry_manager_add_and_get_entry() {
        let mut entry_manager = DefaultManager::new();
        let entry = Entry::Site(SiteEntry {
            user_name: "test".to_string(),
            password: "test".to_string(),
            url: "test".to_string(),
        });
        entry_manager.add_entry("test", entry);
        let entry = entry_manager.get_entry("test").unwrap();
        let found_entry = Entry::Site(SiteEntry {
            user_name: "test".to_string(),
            password: "test".to_string(),
            url: "test".to_string(),
        });
        assert_eq!(*entry, found_entry);
    }
}
