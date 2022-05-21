pub mod lib;
use lib::entry_manager::{DefaultManager, EncryptedEntry, Entry, EntryManager, SiteEntry, WalletEntry};
fn main() {
    let mut manager = DefaultManager::new();
    manager.add_entry("gmail.com_simdi@gmail.com", Entry::Site(SiteEntry::default()));
    manager.add_entry("metamask_1", Entry::Wallet(WalletEntry::default()));

    let entry = manager.get_entry("test");
    match entry {
        Some(Entry::Site(site_entry)) => {
            let decrypted_site_entry = site_entry.decrypt("secret");
            println!("{:?}", decrypted_site_entry);
        }
        Some(Entry::Wallet(wallet_entry)) => {
            let decrypted_wallet_entry = wallet_entry.decrypt("secret");
            println!("{:?}", decrypted_wallet_entry);
        }
        _ => {
            println!("No entry found");
        }
    }
}

/// tests
#[cfg(test)]
pub mod tests {}
