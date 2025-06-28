use crate::client::ChromaClient;
use crate::error::ChromaError;
use crate::models;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Collection {
    pub id: Uuid,
    pub name: String,
    pub tenant_name: String,
    pub database_name: String,
    client: Arc<ChromaClient>,
}

impl Collection {
    pub(crate) fn from(value: models::Collection, client: Arc<ChromaClient>) -> Self {
        Self {
            id: value.id,
            name: value.name,
            tenant_name: value.tenant,
            database_name: value.database,
            client,
        }
    }
}

impl Collection {
    pub async fn add(
        &self,
        payload: &models::AddCollectionRecordsPayload,
    ) -> Result<(), ChromaError> {
        self.client
            .collection_add(
                &self.tenant_name,
                &self.database_name,
                &self.id.to_string(),
                payload,
            )
            .await?;
        Ok(())
    }

    pub async fn upsert(
        &self,
        payload: &models::UpsertCollectionRecordsPayload,
    ) -> Result<(), ChromaError> {
        self.client
            .collection_upsert(
                &self.tenant_name,
                &self.database_name,
                &self.id.to_string(),
                payload,
            )
            .await?;
        Ok(())
    }

    pub async fn query(
        &self,
        payload: &models::QueryRequestPayload,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<models::QueryResponse, ChromaError> {
        self.client
            .collection_query(
                &self.tenant_name,
                &self.database_name,
                &self.id.to_string(),
                limit,
                offset,
                payload,
            )
            .await
    }

    pub async fn get(
        &self,
        payload: &models::GetRequestPayload,
    ) -> Result<models::GetResponse, ChromaError> {
        self.client
            .collection_get(
                &self.tenant_name,
                &self.database_name,
                &self.id.to_string(),
                payload,
            )
            .await
    }

    pub async fn delete(
        &self,
        payload: &models::DeleteCollectionRecordsPayload,
    ) -> Result<(), ChromaError> {
        self.client
            .collection_delete(
                &self.tenant_name,
                &self.database_name,
                &self.id.to_string(),
                payload,
            )
            .await?;
        Ok(())
    }

    pub async fn update_records(
        &self,
        payload: &models::UpdateCollectionRecordsPayload,
    ) -> Result<(), ChromaError> {
        self.client
            .collection_update(
                &self.tenant_name,
                &self.database_name,
                &self.id.to_string(),
                payload,
            )
            .await?;
        Ok(())
    }

    pub async fn update(
        &self,
        payload: &models::UpdateCollectionPayload,
    ) -> Result<(), ChromaError> {
        self.client
            .update_collection(
                &self.tenant_name,
                &self.database_name,
                &self.id.to_string(),
                payload,
            )
            .await?;
        Ok(())
    }

    pub async fn count(&self) -> Result<u32, ChromaError> {
        self.client
            .collection_count(&self.tenant_name, &self.database_name, &self.id.to_string())
            .await
    }
}

#[derive(Debug, Clone)]
pub struct Database {
    pub name: String,
    pub tenant_name: String,
    client: Arc<ChromaClient>,
}

impl Database {
    pub(crate) fn from(value: models::Database, client: Arc<ChromaClient>) -> Self {
        Self {
            name: value.name,
            tenant_name: value.tenant,
            client,
        }
    }
}

impl Database {
    pub async fn create_collection(
        &self,
        payload: &models::CreateCollectionPayload,
    ) -> Result<Collection, ChromaError> {
        let collection_model = self
            .client
            .create_collection(&self.tenant_name, &self.name, payload)
            .await?;
        Ok(Collection::from(collection_model, self.client.clone()))
    }

    pub async fn get_collection(&self, collection_id: &str) -> Result<Collection, ChromaError> {
        let collection_model = self
            .client
            .get_collection(&self.tenant_name, &self.name, collection_id)
            .await?;
        Ok(Collection::from(collection_model, self.client.clone()))
    }

    pub async fn get_or_create_collection(
        &self,
        payload: models::CreateCollectionPayload,
    ) -> Result<Collection, ChromaError> {
        let mut create_payload = payload;
        create_payload.get_or_create = Some(true);
        self.create_collection(&create_payload).await
    }

    pub async fn list_collections(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Collection>, ChromaError> {
        Ok(self
            .client
            .list_collections(&self.tenant_name, &self.name, limit, offset)
            .await?
            .into_iter()
            .map(|i| Collection::from(i, self.client.clone()))
            .collect())
    }

    pub async fn delete_collection(&self, collection_id: &str) -> Result<(), ChromaError> {
        self.client
            .delete_collection(&self.tenant_name, &self.name, collection_id)
            .await?;
        Ok(())
    }

    pub async fn count_collections(&self) -> Result<u32, ChromaError> {
        self.client
            .count_collections(&self.tenant_name, &self.name)
            .await
    }
}

#[derive(Debug, Clone)]
pub struct Tenant {
    pub name: String,
    client: Arc<ChromaClient>,
}

impl Tenant {
    fn database(&self, name: &str) -> Database {
        Database {
            name: name.to_string(),
            tenant_name: self.name.clone(),
            client: self.client.clone(),
        }
    }

    pub async fn get_database(&self, name: &str) -> Result<Database, ChromaError> {
        self.client
            .get_database(&self.name, name)
            .await
            .map(|d| Database::from(d, self.client.clone()))
    }

    pub async fn create_database(&self, name: &str) -> Result<Database, ChromaError> {
        let payload = models::CreateDatabasePayload {
            name: name.to_string(),
        };
        self.client.create_database(&self.name, &payload).await?;
        Ok(self.database(name))
    }

    pub async fn list_databases(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Database>, ChromaError> {
        Ok(self
            .client
            .list_databases(&self.name, limit, offset)
            .await?
            .into_iter()
            .map(|i| Database::from(i, self.client.clone()))
            .collect())
    }
}

#[derive(Debug, Clone)]
pub struct Chroma {
    client: Arc<ChromaClient>,
}

impl Chroma {
    pub fn new(base_url: &str, token: Option<String>) -> Result<Self, ChromaError> {
        Ok(Self {
            client: Arc::new(ChromaClient::new(base_url, token)?),
        })
    }

    fn tenant(&self, name: &str) -> Tenant {
        Tenant {
            name: name.to_string(),
            client: self.client.clone(),
        }
    }

    pub async fn create_tenant(&self, name: &str) -> Result<Tenant, ChromaError> {
        let payload = models::CreateTenantPayload {
            name: name.to_string(),
        };
        self.client.create_tenant(&payload).await?;
        Ok(self.tenant(name))
    }

    pub async fn get_tenant(&self, name: &str) -> Result<Tenant, ChromaError> {
        self.client.get_tenant(name).await?;
        Ok(self.tenant(name))
    }

    pub async fn version(&self) -> Result<String, ChromaError> {
        self.client.version().await
    }

    pub async fn heartbeat(&self) -> Result<models::HeartbeatResponse, ChromaError> {
        self.client.heartbeat().await
    }

    pub async fn healthcheck(&self) -> Result<String, ChromaError> {
        self.client.healthcheck().await
    }

    pub async fn reset(&self) -> Result<bool, ChromaError> {
        self.client.reset().await
    }
}
