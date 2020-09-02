#![allow(unused_variables)]

use std::{collections::HashSet, sync::Arc};

use async_trait::async_trait;
use indexeddb::{IndexedDb, TransactionMode};

use matrix_sdk_common::{
    identifiers::{DeviceId, RoomId, UserId},
    locks::Mutex,
};
use matrix_sdk_crypto::{
    olm::{Account, InboundGroupSession, PickledAccount, PicklingMode, Session},
    CryptoStore, CryptoStoreError, ReadOnlyDevice, ReadOnlyUserDevices, UserIdentities,
};

const VERSION: u32 = 1;

pub type Result<T> = std::result::Result<T, CryptoStoreError>;

#[derive(Debug)]
pub struct IndexedDbCryptostore(IndexedDb);

impl IndexedDbCryptostore {
    pub async fn open() -> Self {
        let db = IndexedDb::open("matrix-cryptostore", VERSION, |old_version, db| {
            if old_version == 0 {
                db.create_object_store("account")
                    .expect("Can't create account store");
            }
        })
        .await
        .expect("Can't open database");

        IndexedDbCryptostore(db)
    }
}

#[async_trait(?Send)]
impl CryptoStore for IndexedDbCryptostore {
    async fn save_account(&self, account: Account) -> Result<()> {
        let pickled = account.pickle(PicklingMode::Unencrypted).await;
        let key = "account".to_owned();

        let transaction = self.0.transaction(TransactionMode::ReadWrite);
        let store = transaction
            .object_store("account")
            .expect("Can't get object store");
        store
            .add(&key, &pickled)
            .await
            .expect("Can't store account");

        Ok(())
    }

    async fn load_account(&self) -> Result<Option<Account>> {
        let key = "account".to_owned();

        let transaction = self.0.transaction(TransactionMode::Readonly);
        let store = transaction
            .object_store("account")
            .expect("Can't get object store");

        let pickle: Option<PickledAccount> = store
            .get(&key)
            .await
            .expect("Can't store account")
            .expect("Can't get account from store");

        if let Some(p) = pickle {
            Ok(Some(Account::from_pickle(p, PicklingMode::Unencrypted)?))
        } else {
            Ok(None)
        }
    }

    async fn save_sessions(&self, sessions: &[Session]) -> Result<()> {
        todo!()
    }

    async fn get_sessions(&self, sender_key: &str) -> Result<Option<Arc<Mutex<Vec<Session>>>>> {
        todo!()
    }

    async fn save_inbound_group_session(&self, session: InboundGroupSession) -> Result<bool> {
        todo!()
    }

    async fn get_inbound_group_session(
        &self,
        room_id: &RoomId,
        sender_key: &str,
        session_id: &str,
    ) -> Result<Option<InboundGroupSession>> {
        todo!()
    }

    fn users_for_key_query(&self) -> HashSet<UserId> {
        todo!()
    }

    fn is_user_tracked(&self, user_id: &UserId) -> bool {
        todo!()
    }

    fn has_users_for_key_query(&self) -> bool {
        todo!()
    }

    async fn update_tracked_user(&self, user: &UserId, dirty: bool) -> Result<bool> {
        todo!()
    }

    async fn get_device(
        &self,
        user_id: &UserId,
        device_id: &DeviceId,
    ) -> Result<Option<ReadOnlyDevice>> {
        todo!()
    }

    async fn delete_device(&self, device: ReadOnlyDevice) -> Result<()> {
        todo!()
    }

    async fn get_user_devices(&self, user_id: &UserId) -> Result<ReadOnlyUserDevices> {
        todo!()
    }

    async fn save_devices(&self, devices: &[ReadOnlyDevice]) -> Result<()> {
        todo!()
    }

    async fn get_user_identity(&self, user_id: &UserId) -> Result<Option<UserIdentities>> {
        todo!()
    }

    async fn save_user_identities(&self, identities: &[UserIdentities]) -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::IndexedDbCryptostore;
    use matrix_sdk_common::identifiers::{user_id, DeviceIdBox};
    use matrix_sdk_crypto::{olm::Account, CryptoStore};

    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn save_account() {
        let user_id = user_id!("@alice:example.org");
        let device_id: DeviceIdBox = "ADEVICE".into();

        let db = IndexedDbCryptostore::open().await;
        let account = Account::new(&user_id, &device_id);

        db.save_account(account.clone())
            .await
            .expect("Couldn't save the account");
        let loaded_account = db
            .load_account()
            .await
            .expect("Couldn't load the account")
            .expect("Didn't find an account in the store");

        assert_eq!(account, loaded_account);
    }
}
