extern crate google_drive3 as drive3;
extern crate hyper;
extern crate hyper_rustls;
use async_trait::async_trait;
use drive3::DriveHub;
use yup_oauth2;
use yup_oauth2::{read_application_secret, InstalledFlowAuthenticator, InstalledFlowReturnMethod};

pub struct Drive {
    pub hub: DriveHub,
    api_key: String,
}

#[async_trait]
pub trait GoogleDrive {
    async fn init(secret_file: &str, api_key: &str, id: &str) -> Self;
    async fn get_files_in_folder(&self, id: &str) -> drive3::api::FileList;
    async fn get_direct_link(&self, file: &drive3::api::File) -> String;
    async fn get_shared_drives(&self) -> drive3::api::DriveList;
}
#[async_trait]
impl GoogleDrive for Drive {
    async fn init(secret_file: &str, api_key: &str, id: &str) -> Self {
        let secret = read_application_secret(secret_file)
            .await
            .expect("failed to read gsuite credential file");
        let auth =
            InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::HTTPRedirect)
                .persist_tokens_to_disk(&format!("tokencache-{}.json", id))
                .build()
                .await
                .unwrap();
        let hub = DriveHub::new(
            hyper::Client::builder().build(hyper_rustls::HttpsConnector::with_native_roots()),
            auth,
        );
        Self {
            hub: hub,
            api_key: api_key.to_string(),
        }
    }
    async fn get_files_in_folder(&self, id: &str) -> drive3::api::FileList {
        self.hub
            .files()
            .list()
            .supports_all_drives(true)
            .include_items_from_all_drives(true)
            .q(&format!("'{}' in parents and trashed=false", id))
            .include_team_drive_items(false)
            .spaces("drive")
            .doit()
            .await
            .unwrap()
            .1
    }
    async fn get_direct_link(&self, file: &drive3::api::File) -> String {
        format!(
            "https://www.googleapis.com/drive/v3/files/{}/?key={}&alt=media",
            file.id.as_ref().unwrap(),
            self.api_key
        )
    }
    async fn get_shared_drives(&self) -> drive3::api::DriveList {
        self.hub
            .drives()
            .list()
            .page_size(90)
            .doit()
            .await
            .unwrap()
            .1
    }
}
