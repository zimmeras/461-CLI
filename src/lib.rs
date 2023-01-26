pub mod rate_repos {
    pub mod metrics {
        pub mod bus_factor;
        pub mod correctness;
        pub mod license;
        pub mod ramp_up;
        pub mod responsive_maintainer;
    }

    pub fn parse_url_file(_url_file_path: &str) {
        use crate::rate_repos::metrics::bus_factor;
        let bf_score = bus_factor::get_bf();
        println!("Bus factor score {bf_score}");
    }
}