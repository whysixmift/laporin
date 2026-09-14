use regex::Regex;
use std::net::{IpAddr, ToSocketAddrs};
use std::time::Duration;
use url::Url;

#[derive(Clone)]
pub struct CrawlerService {
    client: reqwest::Client,
}

impl CrawlerService {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .redirect(reqwest::redirect::Policy::limited(3))
                .build()
                .unwrap_or_default(),
        }
    }

    pub fn validate_url(&self, raw_url: &str) -> Result<Url, String> {
        let parsed = Url::parse(raw_url).map_err(|e| format!("Invalid URL: {}", e))?;

        if parsed.scheme() != "https" {
            return Err("Only HTTPS scheme is permitted".into());
        }

        let host = parsed
            .host_str()
            .ok_or_else(|| "URL missing hostname".to_string())?;

        // Resolve DNS and check against private/disallowed IP ranges
        let socket_addr_str = format!("{}:443", host);
        let addrs = socket_addr_str
            .to_socket_addrs()
            .map_err(|e| format!("DNS resolution failed for {}: {}", host, e))?;

        for addr in addrs {
            let ip = addr.ip();
            if Self::is_private_or_restricted(&ip) {
                return Err(format!(
                    "Access to private/restricted IP ({}) is forbidden",
                    ip
                ));
            }
        }

        Ok(parsed)
    }

    pub fn is_private_or_restricted(ip: &IpAddr) -> bool {
        match ip {
            IpAddr::V4(ipv4) => {
                let octets = ipv4.octets();
                // 127.0.0.0/8 (Loopback)
                if octets[0] == 127 {
                    return true;
                }
                // 10.0.0.0/8 (Private)
                if octets[0] == 10 {
                    return true;
                }
                // 172.16.0.0/12 (Private)
                if octets[0] == 172 && (16..=31).contains(&octets[1]) {
                    return true;
                }
                // 192.168.0.0/16 (Private)
                if octets[0] == 192 && octets[1] == 168 {
                    return true;
                }
                // 169.254.0.0/16 (Link-local)
                if octets[0] == 169 && octets[1] == 254 {
                    return true;
                }
                // 0.0.0.0/8
                if octets[0] == 0 {
                    return true;
                }
                // Multicast & broadcast
                if octets[0] >= 224 {
                    return true;
                }
                false
            }
            IpAddr::V6(ipv6) => {
                if ipv6.is_loopback() || ipv6.is_unspecified() {
                    return true;
                }
                let segments = ipv6.segments();
                // Unique local address fc00::/7
                if (segments[0] & 0xfe00) == 0xfc00 {
                    return true;
                }
                // Link-local unicast fe80::/10
                if (segments[0] & 0xffc0) == 0xfe80 {
                    return true;
                }
                // Multicast ff00::/8
                if (segments[0] & 0xff00) == 0xff00 {
                    return true;
                }
                false
            }
        }
    }

    pub async fn fetch_and_sanitize(
        &self,
        url_str: &str,
    ) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync>> {
        let validated_url = self
            .validate_url(url_str)
            .map_err(|e| format!("SSRF Error: {}", e))?;

        let response = self.client.get(validated_url.as_str()).send().await?;
        if !response.status().is_success() {
            return Err(format!("HTTP request failed with status: {}", response.status()).into());
        }

        // Limit response size to 2 MiB
        let max_bytes = 2 * 1024 * 1024;
        let bytes = response.bytes().await?;
        if bytes.len() > max_bytes {
            return Err("Response exceeded maximum size limit of 2 MiB".into());
        }

        let body_str = String::from_utf8_lossy(&bytes).to_string();
        let title = Self::extract_title(&body_str).unwrap_or_else(|| url_str.to_string());
        let sanitized_text = Self::sanitize_html(&body_str);

        Ok((title, sanitized_text))
    }

    fn extract_title(html: &str) -> Option<String> {
        let title_re = Regex::new(r"(?i)<title[^>]*>(.*?)</title>").ok()?;
        title_re.captures(html).map(|cap| {
            cap[1]
                .trim()
                .replace('\n', " ")
                .replace('\r', "")
                .chars()
                .take(120)
                .collect()
        })
    }

    pub fn sanitize_html(html: &str) -> String {
        let mut clean = html.to_string();

        // Remove script, style, iframe, noscript, svg
        if let Ok(script_re) = Regex::new(r"(?is)<script[^>]*>.*?</script>") {
            clean = script_re.replace_all(&clean, " ").to_string();
        }
        if let Ok(style_re) = Regex::new(r"(?is)<style[^>]*>.*?</style>") {
            clean = style_re.replace_all(&clean, " ").to_string();
        }
        if let Ok(iframe_re) = Regex::new(r"(?is)<iframe[^>]*>.*?</iframe>") {
            clean = iframe_re.replace_all(&clean, " ").to_string();
        }
        if let Ok(noscript_re) = Regex::new(r"(?is)<noscript[^>]*>.*?</noscript>") {
            clean = noscript_re.replace_all(&clean, " ").to_string();
        }
        if let Ok(svg_re) = Regex::new(r"(?is)<svg[^>]*>.*?</svg>") {
            clean = svg_re.replace_all(&clean, " ").to_string();
        }
        if let Ok(comment_re) = Regex::new(r"(?s)<!--.*?-->") {
            clean = comment_re.replace_all(&clean, " ").to_string();
        }

        // Remove HTML tags
        if let Ok(tags_re) = Regex::new(r"<[^>]+>") {
            clean = tags_re.replace_all(&clean, " ").to_string();
        }

        // Normalize whitespace
        if let Ok(ws_re) = Regex::new(r"\s+") {
            clean = ws_re.replace_all(&clean, " ").trim().to_string();
        }

        // Limit to 10,000 characters for LLM prompt context
        clean.chars().take(10000).collect()
    }

    pub async fn fetch_url(
        &self,
        url_str: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let (_, text) = self.fetch_and_sanitize(url_str).await?;
        Ok(text)
    }
}

