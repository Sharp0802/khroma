// src/client.rs
use crate::error::ChromaError;
use crate::models::*;
use reqwest::{Client as ReqwestClient, Response};
use url::Url;

/// The main client for interacting with the Chroma API.
#[derive(Debug, Clone)]
pub struct ChromaClient {
    client: ReqwestClient,
    base_url: Url,
    token: Option<String>,
}

impl ChromaClient {
    /// Creates a new Chroma client.
    ///
    /// # Arguments
    ///
    /// * `base_url` - The base URL of the Chroma server (e.g., "http://localhost:8000").
    /// * `token` - An optional authentication token for the 'x-chroma-token' header.
    pub fn new(base_url: &str, token: Option<String>) -> Result<Self, ChromaError> {
        Ok(Self {
            client: ReqwestClient::new(),
            base_url: Url::parse(base_url)?,
            token,
        })
    }

    async fn handle_response<T: serde::de::DeserializeOwned>(
        &self,
        res: Response,
    ) -> Result<T, ChromaError> {
        let status = res.status();
        if status.is_success() {
            res.json::<T>().await.map_err(|e| {
                ChromaError::Parse(format!("Failed to deserialize successful response: {}", e))
            })
        } else {
            let message = match res.json::<ErrorResponse>().await {
                Ok(err_res) => err_res.message,
                Err(_) => format!("Failed to parse error response. Status: {}", status),
            };
            Err(ChromaError::Api { status, message })
        }
    }

    async fn handle_text_response(&self, res: Response) -> Result<String, ChromaError> {
        let status = res.status();
        if status.is_success() {
            res.text().await.map_err(ChromaError::from)
        } else {
            let message = match res.json::<ErrorResponse>().await {
                Ok(err_res) => err_res.message,
                Err(_) => format!("Failed to parse error response. Status: {}", status),
            };
            Err(ChromaError::Api { status, message })
        }
    }

    fn build_request<U: AsRef<str>>(
        &self,
        method: reqwest::Method,
        path: U,
    ) -> Result<reqwest::RequestBuilder, ChromaError> {
        let url = self.base_url.join(path.as_ref())?;
        let mut builder = self.client.request(method, url);
        if let Some(token) = &self.token {
            builder = builder.header("x-chroma-token", token);
        }
        Ok(builder)
    }

    /// GET /api/v2/auth/identity - Retrieves the current user's identity, tenant, and databases.
    pub async fn get_user_identity(&self) -> Result<GetUserIdentityResponse, ChromaError> {
        let req = self.build_request(reqwest::Method::GET, "/api/v2/auth/identity")?;
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// GET /api/v2/healthcheck - Health check endpoint.
    pub async fn healthcheck(&self) -> Result<String, ChromaError> {
        let req = self.build_request(reqwest::Method::GET, "/api/v2/healthcheck")?;
        let res = req.send().await?;
        self.handle_text_response(res).await
    }

    /// GET /api/v2/heartbeat - Heartbeat endpoint.
    pub async fn heartbeat(&self) -> Result<HeartbeatResponse, ChromaError> {
        let req = self.build_request(reqwest::Method::GET, "/api/v2/heartbeat")?;
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// GET /api/v2/pre-flight-checks - Pre-flight checks endpoint.
    pub async fn pre_flight_checks(&self) -> Result<ChecklistResponse, ChromaError> {
        let req = self.build_request(reqwest::Method::GET, "/api/v2/pre-flight-checks")?;
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// POST /api/v2/reset - Reset the database.
    pub async fn reset(&self) -> Result<bool, ChromaError> {
        let req = self.build_request(reqwest::Method::POST, "/api/v2/reset")?;
        let res = req.send().await?;
        let text = self.handle_text_response(res).await?;
        text.parse::<bool>().map_err(|e| ChromaError::Parse(e.to_string()))
    }

    /// GET /api/v2/version - Returns the version of the server.
    pub async fn version(&self) -> Result<String, ChromaError> {
        let req = self.build_request(reqwest::Method::GET, "/api/v2/version")?;
        let res = req.send().await?;
        self.handle_text_response(res).await
    }

    /// POST /api/v2/tenants - Creates a new tenant.
    pub async fn create_tenant(&self, payload: &CreateTenantPayload) -> Result<CreateTenantResponse, ChromaError> {
        let req = self.build_request(reqwest::Method::POST, "/api/v2/tenants")?.json(payload);
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// GET /api/v2/tenants/{tenant_name} - Returns an existing tenant by name.
    pub async fn get_tenant(&self, tenant_name: &str) -> Result<GetTenantResponse, ChromaError> {
        let path = format!("/api/v2/tenants/{}", tenant_name);
        let req = self.build_request(reqwest::Method::GET, &path)?;
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// GET /api/v2/tenants/{tenant}/databases - Lists all databases for a given tenant.
    pub async fn list_databases(&self, tenant: &str, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<Database>, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases", tenant);
        let mut req = self.build_request(reqwest::Method::GET, &path)?;
        let mut query_params = Vec::new();
        if let Some(l) = limit { query_params.push(("limit", l.to_string())); }
        if let Some(o) = offset { query_params.push(("offset", o.to_string())); }
        if !query_params.is_empty() {
            req = req.query(&query_params);
        }
        let res = req.send().await?;
        // The spec uses a generic `Vec` schema name, but the items are Databases.
        self.handle_response(res).await
    }

    /// POST /api/v2/tenants/{tenant}/databases - Creates a new database for a given tenant.
    pub async fn create_database(&self, tenant: &str, payload: &CreateDatabasePayload) -> Result<CreateDatabaseResponse, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases", tenant);
        let req = self.build_request(reqwest::Method::POST, &path)?.json(payload);
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// GET /api/v2/tenants/{tenant}/databases/{database} - Retrieves a specific database by name.
    pub async fn get_database(&self, tenant: &str, database: &str) -> Result<Database, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases/{}", tenant, database);
        let req = self.build_request(reqwest::Method::GET, &path)?;
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// DELETE /api/v2/tenants/{tenant}/databases/{database} - Deletes a specific database.
    pub async fn delete_database(&self, tenant: &str, database: &str) -> Result<DeleteDatabaseResponse, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases/{}", tenant, database);
        let req = self.build_request(reqwest::Method::DELETE, &path)?;
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// GET /api/v2/tenants/{tenant}/databases/{database}/collections - Lists all collections in the specified database.
    pub async fn list_collections(&self, tenant: &str, database: &str, limit: Option<i32>, offset: Option<i32>) -> Result<Vec<Collection>, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases/{}/collections", tenant, database);
        let mut req = self.build_request(reqwest::Method::GET, &path)?;
        let mut query_params = Vec::new();
        if let Some(l) = limit { query_params.push(("limit", l.to_string())); }
        if let Some(o) = offset { query_params.push(("offset", o.to_string())); }
        if !query_params.is_empty() {
            req = req.query(&query_params);
        }
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// POST /api/v2/tenants/{tenant}/databases/{database}/collections - Creates a new collection.
    pub async fn create_collection(&self, tenant: &str, database: &str, payload: &CreateCollectionPayload) -> Result<Collection, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases/{}/collections", tenant, database);
        let req = self.build_request(reqwest::Method::POST, &path)?.json(payload);
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// GET /api/v2/tenants/{tenant}/databases/{database}/collections/{collection_id} - Retrieves a collection.
    pub async fn get_collection(&self, tenant: &str, database: &str, collection_id: &str) -> Result<Collection, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases/{}/collections/{}", tenant, database, collection_id);
        let req = self.build_request(reqwest::Method::GET, &path)?;
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// PUT /api/v2/tenants/{tenant}/databases/{database}/collections/{collection_id} - Updates a collection.
    pub async fn update_collection(&self, tenant: &str, database: &str, collection_id: &str, payload: &UpdateCollectionPayload) -> Result<UpdateCollectionResponse, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases/{}/collections/{}", tenant, database, collection_id);
        let req = self.build_request(reqwest::Method::PUT, &path)?.json(payload);
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// DELETE /api/v2/tenants/{tenant}/databases/{database}/collections/{collection_id} - Deletes a collection.
    pub async fn delete_collection(&self, tenant: &str, database: &str, collection_id: &str) -> Result<UpdateCollectionResponse, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases/{}/collections/{}", tenant, database, collection_id);
        let req = self.build_request(reqwest::Method::DELETE, &path)?;
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// POST /api/v2/tenants/{tenant}/databases/{database}/collections/{collection_id}/add - Adds records to a collection.
    pub async fn collection_add(&self, tenant: &str, database: &str, collection_id: &str, payload: &AddCollectionRecordsPayload) -> Result<AddCollectionRecordsResponse, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases/{}/collections/{}/add", tenant, database, collection_id);
        let req = self.build_request(reqwest::Method::POST, &path)?.json(payload);
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// GET /api/v2/tenants/{tenant}/databases/{database}/collections/{collection_id}/count - Retrieves the number of records in a collection.
    pub async fn collection_count(&self, tenant: &str, database: &str, collection_id: &str) -> Result<u32, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases/{}/collections/{}/count", tenant, database, collection_id);
        let req = self.build_request(reqwest::Method::GET, &path)?;
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// POST /api/v2/tenants/{tenant}/databases/{database}/collections/{collection_id}/delete - Deletes records in a collection.
    pub async fn collection_delete(&self, tenant: &str, database: &str, collection_id: &str, payload: &DeleteCollectionRecordsPayload) -> Result<DeleteCollectionRecordsResponse, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases/{}/collections/{}/delete", tenant, database, collection_id);
        let req = self.build_request(reqwest::Method::POST, &path)?.json(payload);
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// POST /api/v2/tenants/{tenant}/databases/{database}/collections/{collection_id}/fork - Forks an existing collection.
    pub async fn fork_collection(&self, tenant: &str, database: &str, collection_id: &str, payload: &ForkCollectionPayload) -> Result<Collection, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases/{}/collections/{}/fork", tenant, database, collection_id);
        let req = self.build_request(reqwest::Method::POST, &path)?.json(payload);
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// POST /api/v2/tenants/{tenant}/databases/{database}/collections/{collection_id}/get - Retrieves records from a collection.
    pub async fn collection_get(&self, tenant: &str, database: &str, collection_id: &str, payload: &GetRequestPayload) -> Result<GetResponse, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases/{}/collections/{}/get", tenant, database, collection_id);
        let req = self.build_request(reqwest::Method::POST, &path)?.json(payload);
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// POST /api/v2/tenants/{tenant}/databases/{database}/collections/{collection_id}/query - Query a collection.
    pub async fn collection_query(&self, tenant: &str, database: &str, collection_id: &str, limit: Option<i32>, offset: Option<i32>, payload: &QueryRequestPayload) -> Result<QueryResponse, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases/{}/collections/{}/query", tenant, database, collection_id);
        let mut req = self.build_request(reqwest::Method::POST, &path)?.json(payload);
        let mut query_params = Vec::new();
        if let Some(l) = limit { query_params.push(("limit", l.to_string())); }
        if let Some(o) = offset { query_params.push(("offset", o.to_string())); }
        if !query_params.is_empty() {
            req = req.query(&query_params);
        }
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// POST /api/v2/tenants/{tenant}/databases/{database}/collections/{collection_id}/update - Updates records in a collection.
    pub async fn collection_update(&self, tenant: &str, database: &str, collection_id: &str, payload: &UpdateCollectionRecordsPayload) -> Result<UpdateCollectionRecordsResponse, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases/{}/collections/{}/update", tenant, database, collection_id);
        let req = self.build_request(reqwest::Method::POST, &path)?.json(payload);
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// POST /api/v2/tenants/{tenant}/databases/{database}/collections/{collection_id}/upsert - Upserts records in a collection.
    pub async fn collection_upsert(&self, tenant: &str, database: &str, collection_id: &str, payload: &UpsertCollectionRecordsPayload) -> Result<UpsertCollectionRecordsResponse, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases/{}/collections/{}/upsert", tenant, database, collection_id);
        let req = self.build_request(reqwest::Method::POST, &path)?.json(payload);
        let res = req.send().await?;
        self.handle_response(res).await
    }

    /// GET /api/v2/tenants/{tenant}/databases/{database}/collections_count - Retrieves the total number of collections.
    pub async fn count_collections(&self, tenant: &str, database: &str) -> Result<u32, ChromaError> {
        let path = format!("/api/v2/tenants/{}/databases/{}/collections_count", tenant, database);
        let req = self.build_request(reqwest::Method::GET, &path)?;
        let res = req.send().await?;
        self.handle_response(res).await
    }
}
