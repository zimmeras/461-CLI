pub mod rate_repos {
    pub mod metrics {
        pub mod bus_factor;
        pub mod correctness;
        pub mod license;
        pub mod ramp_up;
        pub mod responsive_maintainer;

        #[derive(Debug)]
        pub struct MetricScores {
            pub net_score: f32,
            pub bus_factor_score: f32,
            pub correctness_score: f32,
            pub license_score: f32,
            pub ramp_up_score: f32,
            pub responsive_maintainer_score: f32,
        }

        const BUS_FACTOR_WEIGHT: f32 = 0.15;
        const CORRECTNESS_WEIGHT: f32 = 0.15;
        const LICENSE_WEIGHT: f32 = 0.15;
        const RAMP_UP_WEIGHT: f32 = 0.15;
        const RESPONSIVE_MAINTAINER_WEIGHT: f32 = 0.4;

        pub fn get_metrics(_url: &str) -> MetricScores {
            // each of these 0.5's will be a call each metric function in their file
            let mut scores = MetricScores {
                net_score: 0.0,
                bus_factor_score: 0.5,
                correctness_score: 0.5,
                license_score: 0.5,
                ramp_up_score: 0.5,
                responsive_maintainer_score: 0.5,
            };

            scores.net_score =  scores.bus_factor_score * BUS_FACTOR_WEIGHT +
                                scores.correctness_score * CORRECTNESS_WEIGHT +
                                scores.license_score * LICENSE_WEIGHT +
                                scores.ramp_up_score * RAMP_UP_WEIGHT +
                                scores.responsive_maintainer_score * RESPONSIVE_MAINTAINER_WEIGHT;

            return scores;
        }
    }

    #[derive(Debug)]
    pub struct UrlSpecs {
        url: String,
        metric_scores: metrics::MetricScores,
    }

    fn get_github_url_for_npm(npm_url: &str) -> Result<String, ureq::Error>{
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
        }
        else {
            return Ok("".to_string());
        }
    }

    pub fn rate_repos(url_file_path: &str) {
        use std::fs;
        let file_contents = fs::read_to_string(url_file_path).expect("Should have been able to read the file");
        let urls = file_contents.lines();

        let mut url_specs: Vec<UrlSpecs> = Vec::new();

        for url in urls {
            if &url[0..22] == "https://www.npmjs.com/" {
                let github_url = get_github_url_for_npm(&url).unwrap();
                if &github_url[0..19] == "https://github.com/" {
                    let url_spec = UrlSpecs {
                        url: url.to_string(),
                        metric_scores: metrics::get_metrics(&github_url),
                    };
                    url_specs.push(url_spec);
                }
            }
            else if &url[0..19] == "https://github.com/" {
                let url_spec = UrlSpecs {
                    url: url.to_string(),
                    metric_scores: metrics::get_metrics(&url),
                };
                url_specs.push(url_spec);
            }
        }

        // sort the repos in decreasing order
        url_specs.sort_by(|a, b| b.metric_scores.net_score.partial_cmp(&a.metric_scores.net_score).unwrap());
        
        print_metrics(&url_specs);
    }

    pub fn print_metrics(url_specs: &Vec<UrlSpecs>) {
        println!("URL NET_SCORE RAMP_UP_SCORE CORRECTNESS_SCORE BUS_FACTOR_SCORE RESPONSIVE_MAINTAINER_SCORE LICENSE_SCORE");
        for repo in url_specs {
            print!("{}", repo.url);
            print!(" {}", repo.metric_scores.net_score);
            print!(" {}", repo.metric_scores.ramp_up_score);
            print!(" {}", repo.metric_scores.correctness_score);
            print!(" {}", repo.metric_scores.bus_factor_score);
            print!(" {}", repo.metric_scores.responsive_maintainer_score);
            println!(" {}", repo.metric_scores.license_score);
        }
    }
}