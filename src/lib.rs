pub mod rate_repos {
    use serde::{Deserialize, Serialize};
    pub mod metrics {
        use super::*;

        pub mod bus_factor;
        pub mod correctness;
        pub mod license;
        pub mod ramp_up;
        pub mod responsive_maintainer;
        use bus_factor::bus_factor_score;
        use responsive_maintainer::responsive_maintainer_score;

        #[derive(Serialize, Deserialize, Debug)]
        #[serde(rename_all = "UPPERCASE")]
        pub struct MetricScores {
            pub net_score: f32,
            pub ramp_up_score: i32,
            pub correctness_score: f32,
            pub bus_factor_score: f32,
            pub responsive_maintainer_score: f32,
            pub license_score: f32,
        }

        const BUS_FACTOR_WEIGHT: f32 = 0.2;
        const CORRECTNESS_WEIGHT: f32 = 0.2;
        const LICENSE_WEIGHT: f32 = 0.2;
        const RAMP_UP_WEIGHT: i32 = 0;
        const RESPONSIVE_MAINTAINER_WEIGHT: f32 = 0.4;

        pub fn get_metrics(_url: &str) -> MetricScores {
            // each of these 0.5's will be a call each metric function in their file
            let mut scores = MetricScores {
                net_score: 0.0,
                ramp_up_score: -1,
                correctness_score: 0.5,
                bus_factor_score: bus_factor_score(_url),
                responsive_maintainer_score: responsive_maintainer_score(_url),
                license_score: 0.5,
            };

            scores.net_score = scores.bus_factor_score * BUS_FACTOR_WEIGHT
                + scores.correctness_score * CORRECTNESS_WEIGHT
                + scores.license_score * LICENSE_WEIGHT
                + scores.responsive_maintainer_score * RESPONSIVE_MAINTAINER_WEIGHT
                + (scores.ramp_up_score * RAMP_UP_WEIGHT) as f32;

            return scores;
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "UPPERCASE")]
    pub struct UrlSpecs {
        url: String,
        #[serde(flatten)]
        metric_scores: metrics::MetricScores,
    }

    fn get_github_url_for_npm(npm_url: &str) -> Result<String, ureq::Error> {
        let url = format!("https://registry.npmjs.org/{}", &npm_url[30..]);
        let json: serde_json::Value = ureq::get(&url).call()?.into_json()?;
        let repo_info = &json["repository"];

        if repo_info["type"] == "git" {
            let mut github_url = repo_info["url"].as_str().unwrap()[4..].to_string();
            if &github_url[..10] == "ssh://git@" {
                github_url = github_url[10..].to_string();
                github_url = format!("https://{github_url}");
            }
            for _i in 1..5 {
                github_url.pop();
            }
            return Ok(github_url);
        } else {
            return Ok("".to_string());
        }
    }

    fn validate_github_url(url: &str) -> Result<bool, ureq::Error> {
        let repo_full_name = &url[19..];
        let http_url = format!("https://api.github.com/repos/{}", &repo_full_name);
        let response = ureq::get(&http_url).call();
        match response {
            Ok(_r) => return Ok(true),
            Err(_e) => return Ok(false),
        };
    }

    pub fn rate_repos(url_file_path: &str) {
        use std::fs;
        simple_log::info!("Parsing url file.");
        let file_contents =
            fs::read_to_string(url_file_path).expect("Should have been able to read the file");
        let urls = file_contents.lines();

        let mut url_specs: Vec<UrlSpecs> = Vec::new();

        simple_log::info!("Obtaining github urls.");
        simple_log::info!("Calling metric score calculation functions.");
        for url in urls {
            if &url[0..22] == "https://www.npmjs.com/" {
                let github_url = get_github_url_for_npm(&url).unwrap();
                if &github_url[0..19] == "https://github.com/" {
                    if validate_github_url(&github_url).unwrap() {
                        let url_spec = UrlSpecs {
                            url: url.to_string(),
                            metric_scores: metrics::get_metrics(&github_url),
                        };
                        url_specs.push(url_spec);
                    } else {
                        let url_spec = UrlSpecs {
                            url: url.to_string(),
                            metric_scores: metrics::MetricScores {
                                net_score: 0.0,
                                ramp_up_score: -1,
                                correctness_score: 0.0,
                                bus_factor_score: 0.0,
                                responsive_maintainer_score: 0.0,
                                license_score: 0.0,
                            },
                        };
                        url_specs.push(url_spec);
                    }
                }
            } else if &url[0..19] == "https://github.com/" {
                if validate_github_url(&url).unwrap() {
                    let url_spec = UrlSpecs {
                        url: url.to_string(),
                        metric_scores: metrics::get_metrics(&url),
                    };
                    url_specs.push(url_spec);
                } else {
                    let url_spec = UrlSpecs {
                        url: url.to_string(),
                        metric_scores: metrics::MetricScores {
                            net_score: 0.0,
                            ramp_up_score: -1,
                            correctness_score: 0.0,
                            bus_factor_score: 0.0,
                            responsive_maintainer_score: 0.0,
                            license_score: 0.0,
                        },
                    };
                    url_specs.push(url_spec);
                }
            }
        }

        // sort the repos in decreasing order
        simple_log::info!("Sorting repos in decreasing order.");
        url_specs.sort_by(|a, b| {
            b.metric_scores
                .net_score
                .partial_cmp(&a.metric_scores.net_score)
                .unwrap()
        });

        simple_log::info!("Printing final score calculations.");
        print_url_specs(&url_specs);
    }

    pub fn print_url_specs(url_specs: &Vec<UrlSpecs>) {
        for repo_info in url_specs {
            println!("{}", serde_json::to_string(&repo_info).unwrap());
        }
    }
}
