use crate::config::Config;
use crate::services::CrawlerService;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResultItem {
    pub title: String,
    pub url: String,
    pub snippet: String,
}

#[derive(Clone)]
pub struct SearchEngineService {
    client: reqwest::Client,
    config: Config,
    crawler: CrawlerService,
}

impl SearchEngineService {
    pub fn new(config: Config) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(12))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36")
            .build()
            .unwrap_or_default();

        Self {
            client,
            config,
            crawler: CrawlerService::new(),
        }
    }

    pub async fn search_company(
        &self,
        company_name: &str,
        role: Option<&str>,
        limit: usize,
    ) -> Result<Vec<SearchResultItem>, Box<dyn std::error::Error + Send + Sync>> {
        let clean_name = company_name.trim();
        if clean_name.is_empty() {
            return Ok(Vec::new());
        }

        // Build Indonesian vocational search query
        let query = if let Some(r) = role {
            format!("{} {} profil visi misi indonesia", clean_name, r)
        } else {
            format!("{} profil perusahaan visi misi indonesia", clean_name)
        };

        match self.config.search_provider.as_str() {
            "serper" if !self.config.search_api_key.is_empty() => {
                self.search_serper(&query, limit).await
            }
            "brave" if !self.config.search_api_key.is_empty() => {
                self.search_brave(&query, limit).await
            }
            "duckduckgo_html" | "ddg" => {
                self.search_duckduckgo_html(&query, clean_name, limit).await
            }
            "mock" => {
                self.search_mock(clean_name, limit)
            }
            _ => {
                // Default fallback: Try DuckDuckGo, fall back to mock if network fails
                match self.search_duckduckgo_html(&query, clean_name, limit).await {
                    Ok(results) if !results.is_empty() => Ok(results),
                    _ => self.search_mock(clean_name, limit),
                }
            }
        }
    }

    async fn search_duckduckgo_html(
        &self,
        query: &str,
        company_name: &str,
        limit: usize,
    ) -> Result<Vec<SearchResultItem>, Box<dyn std::error::Error + Send + Sync>> {
        let url = "https://html.duckduckgo.com/html/";
        let params = [("q", query), ("b", ""), ("kl", "id-id")];

        let resp = match self.client.post(url).form(&params).send().await {
            Ok(r) => r,
            Err(e) => {
                tracing::warn!(error = %e, "DuckDuckGo HTML search failed, using fallback");
                return self.search_mock(company_name, limit);
            }
        };

        if !resp.status().is_success() {
            return self.search_mock(company_name, limit);
        }

        let html = resp.text().await.unwrap_or_default();
        let document = scraper::Html::parse_document(&html);
        let result_selector = scraper::Selector::parse(".result").unwrap();
        let title_selector = scraper::Selector::parse(".result__title a").unwrap();
        let snippet_selector = scraper::Selector::parse(".result__snippet").unwrap();

        let mut results = Vec::new();

        for element in document.select(&result_selector) {
            if results.len() >= limit {
                break;
            }

            if let Some(title_elem) = element.select(&title_selector).next() {
                let raw_href = title_elem.value().attr("href").unwrap_or_default();
                let title = title_elem.text().collect::<Vec<_>>().join(" ").trim().to_string();

                // Extract actual redirect URL from uddg= parameter if present
                let target_url = if raw_href.contains("uddg=") {
                    if let Some(pos) = raw_href.find("uddg=") {
                        let encoded = &raw_href[pos + 5..];
                        let clean_encoded = encoded.split('&').next().unwrap_or(encoded);
                        urlencoding::decode(clean_encoded)
                            .unwrap_or_else(|_| raw_href.into())
                            .to_string()
                    } else {
                        raw_href.to_string()
                    }
                } else if raw_href.starts_with("http") {
                    raw_href.to_string()
                } else {
                    continue;
                };

                // SSRF and HTTPS check
                if self.crawler.validate_url(&target_url).is_err() {
                    continue;
                }

                let snippet = element
                    .select(&snippet_selector)
                    .next()
                    .map(|s| s.text().collect::<Vec<_>>().join(" ").trim().to_string())
                    .unwrap_or_default();

                results.push(SearchResultItem {
                    title: if title.is_empty() { format!("Profil {}", company_name) } else { title },
                    url: target_url,
                    snippet,
                });
            }
        }

        if results.is_empty() {
            return self.search_mock(company_name, limit);
        }

        Ok(results)
    }

    async fn search_serper(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResultItem>, Box<dyn std::error::Error + Send + Sync>> {
        #[derive(Serialize)]
        struct SerperReq<'a> {
            q: &'a str,
            gl: &'a str,
            hl: &'a str,
            num: usize,
        }

        #[derive(Deserialize)]
        struct SerperOrganic {
            title: Option<String>,
            link: Option<String>,
            snippet: Option<String>,
        }

        #[derive(Deserialize)]
        struct SerperResp {
            organic: Option<Vec<SerperOrganic>>,
        }

        let resp = self.client
            .post("https://google.serper.dev/search")
            .header("X-API-KEY", &self.config.search_api_key)
            .json(&SerperReq {
                q: query,
                gl: "id",
                hl: "id",
                num: limit,
            })
            .send()
            .await?;

        let body: SerperResp = resp.json().await?;
        let mut results = Vec::new();

        if let Some(organic) = body.organic {
            for item in organic {
                if results.len() >= limit {
                    break;
                }
                if let Some(url) = item.link {
                    if self.crawler.validate_url(&url).is_ok() {
                        results.push(SearchResultItem {
                            title: item.title.unwrap_or_else(|| "Informasi Publik".into()),
                            url,
                            snippet: item.snippet.unwrap_or_default(),
                        });
                    }
                }
            }
        }

        Ok(results)
    }

    async fn search_brave(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResultItem>, Box<dyn std::error::Error + Send + Sync>> {
        #[derive(Deserialize)]
        struct BraveResult {
            title: Option<String>,
            url: Option<String>,
            description: Option<String>,
        }

        #[derive(Deserialize)]
        struct BraveWeb {
            results: Option<Vec<BraveResult>>,
        }

        #[derive(Deserialize)]
        struct BraveResp {
            web: Option<BraveWeb>,
        }

        let resp = self.client
            .get("https://api.search.brave.com/res/v1/web/search")
            .header("X-Subscription-Token", &self.config.search_api_key)
            .query(&[("q", query), ("country", "id"), ("count", &limit.to_string())])
            .send()
            .await?;

        let body: BraveResp = resp.json().await?;
        let mut results = Vec::new();

        if let Some(web) = body.web {
            if let Some(items) = web.results {
                for item in items {
                    if results.len() >= limit {
                        break;
                    }
                    if let Some(url) = item.url {
                        if self.crawler.validate_url(&url).is_ok() {
                            results.push(SearchResultItem {
                                title: item.title.unwrap_or_else(|| "Profil Perusahaan".into()),
                                url,
                                snippet: item.description.unwrap_or_default(),
                            });
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    fn search_mock(
        &self,
        company_name: &str,
        limit: usize,
    ) -> Result<Vec<SearchResultItem>, Box<dyn std::error::Error + Send + Sync>> {
        let slug = company_name.to_lowercase().replace(' ', "-");
        let results = vec![
            SearchResultItem {
                title: format!("Profil Resmi & Informasi {}", company_name),
                url: format!("https://profil-instansi.id/company/{}", slug),
                snippet: format!(
                    "Profil lengkap {} yang bergerak di bidang layanan profesional, teknologi, dan industri di Indonesia.",
                    company_name
                ),
            },
            SearchResultItem {
                title: format!("Visi Misi dan Struktur Kerja - {}", company_name),
                url: format!("https://profil-instansi.id/company/{}/visi-misi", slug),
                snippet: format!(
                    "Struktur organisasi, sejarah berdiri, dan tata kelola operasional kerja pada {}.",
                    company_name
                ),
            },
        ];

        Ok(results.into_iter().take(limit).collect())
    }
}
