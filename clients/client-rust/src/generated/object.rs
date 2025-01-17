#![allow(unused_imports)]
#![cfg_attr(rustfmt, rustfmt_skip)]
/* THIS FILE IS AUTOMATICALLY GENERATED. DO NOT EDIT */
use crate::{Client, ClientBuilder, Credentials, Retry};
use anyhow::Error;
use serde_json::Value;
use std::time::Duration;
use crate::util::urlencode;

/// Object Service
///
/// The object service provides HTTP-accessible storage for large blobs of data.
pub struct Object (Client);

#[allow(non_snake_case)]
impl Object {
    /// Create a new undefined instance, based on the given client.
    pub fn new<CB: Into<ClientBuilder>>(client_builder: CB) -> Result<Self, Error> {
        Ok(Self(client_builder
            .into()
            .path_prefix("api/object/v1/")
            .build()?))
    }

    /// Ping Server
    /// 
    /// Respond without doing anything.
    /// This endpoint is used to check that the service is up.
    pub async fn ping(&self) -> Result<(), Error> {
        let method = "GET";
        let (path, query) = Self::ping_details();
        let body = None;
        let resp = self.0.request(method, path, query, body).await?;
        resp.bytes().await?;
        Ok(())
    }

    /// Generate an unsigned URL for the ping endpoint
    pub fn ping_url(&self) -> Result<String, Error> {
        let (path, query) = Self::ping_details();
        self.0.make_url(path, query)
    }

    /// Generate a signed URL for the ping endpoint
    pub fn ping_signed_url(&self, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::ping_details();
        self.0.make_signed_url(path, query, ttl)
    }

    /// Determine the HTTP request details for ping
    fn ping_details<'a>() -> (&'static str, Option<Vec<(&'static str, &'a str)>>) {
        let path = "ping";
        let query = None;

        (path, query)
    }

    /// Upload backend data (temporary)
    /// 
    /// Upload backend data.
    pub async fn uploadObject(&self, name: &str, payload: &Value) -> Result<(), Error> {
        let method = "PUT";
        let (path, query) = Self::uploadObject_details(name);
        let body = Some(payload);
        let resp = self.0.request(method, &path, query, body).await?;
        resp.bytes().await?;
        Ok(())
    }

    /// Determine the HTTP request details for uploadObject
    fn uploadObject_details<'a>(name: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("upload/{}", urlencode(name));
        let query = None;

        (path, query)
    }

    /// Download object data
    /// 
    /// Get information on how to download an object.  Call this endpoint with a list of acceptable
    /// download methods, and the server will select a method and return the corresponding payload.
    /// Returns a 406 error if none of the given download methods are available.
    /// 
    /// See [Download Methods](https://docs.taskcluster.net/docs/reference/platform/object/download-methods) for more detail.
    pub async fn fetchObjectMetadata(&self, name: &str, payload: &Value) -> Result<Value, Error> {
        let method = "PUT";
        let (path, query) = Self::fetchObjectMetadata_details(name);
        let body = Some(payload);
        let resp = self.0.request(method, &path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Determine the HTTP request details for fetchObjectMetadata
    fn fetchObjectMetadata_details<'a>(name: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("download-object/{}", urlencode(name));
        let query = None;

        (path, query)
    }

    /// Get an object's data
    /// 
    /// Get the data in an object directly.  This method does not return a JSON body, but
    /// redirects to a location that will serve the object content directly.
    /// 
    /// URLs for this endpoint, perhaps with attached authentication (`?bewit=..`),
    /// are typically used for downloads of objects by simple HTTP clients such as
    /// web browsers, curl, or wget.
    /// 
    /// This method is limited by the common capabilities of HTTP, so it may not be
    /// the most efficient, resilient, or featureful way to retrieve an artifact.
    /// Situations where such functionality is required should ues the
    /// `fetchObjectMetadata` API endpoint.
    /// 
    /// See [Simple Downloads](https://docs.taskcluster.net/docs/reference/platform/object/simple-downloads) for more detail.
    pub async fn download(&self, name: &str) -> Result<(), Error> {
        let method = "GET";
        let (path, query) = Self::download_details(name);
        let body = None;
        let resp = self.0.request(method, &path, query, body).await?;
        resp.bytes().await?;
        Ok(())
    }

    /// Generate an unsigned URL for the download endpoint
    pub fn download_url(&self, name: &str) -> Result<String, Error> {
        let (path, query) = Self::download_details(name);
        self.0.make_url(&path, query)
    }

    /// Generate a signed URL for the download endpoint
    pub fn download_signed_url(&self, name: &str, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::download_details(name);
        self.0.make_signed_url(&path, query, ttl)
    }

    /// Determine the HTTP request details for download
    fn download_details<'a>(name: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("download/{}", urlencode(name));
        let query = None;

        (path, query)
    }
}
