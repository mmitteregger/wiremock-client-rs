use reqwest::header::{self, HeaderMap, HeaderValue};

pub trait ClientAuthenticator {
    fn generate_auth_headers(&self) -> HeaderMap;
}

pub struct NoClientAuthenticator;

pub struct ClientBasicAuthenticator {
    headers: HeaderMap,
}

impl ClientBasicAuthenticator {
    pub fn new(username: String, password: String) -> ClientBasicAuthenticator {
        let credentials = format!("{}:{}", username, password);
        let encoded_credentials = base64::encode(&credentials);
        let header_value = format!("Basic {}", encoded_credentials);

        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, header_value.parse().unwrap());

        ClientBasicAuthenticator {
            headers,
        }
    }
}

pub struct ClientTokenAuthenticator {
    headers: HeaderMap,
}

impl ClientTokenAuthenticator {
    pub fn new(token: String) -> ClientTokenAuthenticator {
        let header_value = format!("Token {}", token);

        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, header_value.parse().unwrap());

        ClientTokenAuthenticator {
            headers,
        }
    }
}

impl ClientAuthenticator for NoClientAuthenticator {
    fn generate_auth_headers(&self) -> HeaderMap<HeaderValue> {
        HeaderMap::new()
    }
}

impl ClientAuthenticator for ClientBasicAuthenticator {
    fn generate_auth_headers(&self) -> HeaderMap<HeaderValue> {
        self.headers.clone()
    }
}

impl ClientAuthenticator for ClientTokenAuthenticator {
    fn generate_auth_headers(&self) -> HeaderMap<HeaderValue> {
        self.headers.clone()
    }
}
