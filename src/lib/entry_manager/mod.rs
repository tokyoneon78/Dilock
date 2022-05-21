
pub struct EntryManager {
    entries: HashMap<String, Entry>,
}

pub enum Entry {
    Site(SiteEntry),
    Wallet(WalletEntry),
}


pub struct SiteEntry {
    pub user_name: String,
    pub password: String,
    pub url: String,
}


pub struct WalletEntry {
    pub phrase: String,
    /// e.g metamask
    pub name: String
}

trait EncryptedEntry {
    fn encrypt_value(&self, secret: &str) -> String;
}
