pub mod rate_repos {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;

    pub mod metrics {
        use super::*;

        pub mod bus_factor;
        pub mod correctness;
        pub mod license;
        pub mod ramp_up;
        pub mod responsive_maintainer;

        #[derive(Debug)]
        #[derive(Serialize, Deserialize)]
        pub struct MetricScores {
            net_score: f32,
            bus_factor_score: f32,
            correctness_score: f32,
            license_score: f32,
            ramp_up_score: f32,
            responsive_maintainer_score: f32,
        }

        const BUS_FACTOR_WEIGHT: f32 = 0.15;
        const CORRECTNESS_WEIGHT: f32 = 0.15;
        const LICENSE_WEIGHT: f32 = 0.15;
        const RAMP_UP_WEIGHT: f32 = 0.15;
        const RESPONSIVE_MAINTAINER_WEIGHT: f32 = 0.4;

        pub fn get_metrics(_url: &str) -> MetricScores {
            // call each metric function and return the struct
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
    #[derive(Serialize, Deserialize)]
    pub struct UrlSpecs {
        url: String,
        metric_scores: metrics::MetricScores,
    }

    pub fn parse_url_file(url_file_path: &str) {
        use std::fs;
        let file_contents = fs::read_to_string(url_file_path).expect("Should have been able to read the file");
        let urls = file_contents.lines();

        let mut url_specs: Vec<UrlSpecs> = Vec::new();

        for url in urls {
            if &url[0..22] == "https://www.npmjs.com/" {
                // requirements say we don't have to support metrics from npm modules not
                // hosted on github. the npm website has a spot for a github link
                // for many modules. we could build code that takes that link from the website
                // and uses it to get metrics but that would be a lot more work sooooo
            }
            else if &url[0..19] == "https://github.com/" {
                let url_spec = UrlSpecs {
                    url: url.to_string(),
                    metric_scores: metrics::get_metrics(&url),
                };
                url_specs.push(url_spec);
            }
            else {
                println!("Unsupported URL");
            }
        }
        // url_specs.sort_by(|a, b| a.metric_scores.net_score.partial_cmp(b.metric_scores.net_score).unwrap());
        // sort_urls_by_net_score(&url_specs);
        print_metrics(&url_specs);
    }

    // pub fn sort_urls_by_net_score(url_specs: &Vec<UrlSpecs>) {

    // }

    pub fn print_metrics(url_specs: &Vec<UrlSpecs>) {
        // println!("{:#?}", &url_specs[0]);
        // println!("{:#?}", &url_specs[1]);
        let json = serde_json::to_string(&url_specs[0]).unwrap();
        println!("{}", json);
    }
}