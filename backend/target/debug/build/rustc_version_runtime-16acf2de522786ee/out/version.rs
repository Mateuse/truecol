
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 75,
                        patch: 0,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "aarch64-apple-darwin".to_owned(),
                    short_version_string: "rustc 1.75.0 (82e1608df 2023-12-21)".to_owned(),
                    commit_hash: Some("82e1608dfa6e0b5569232559e3d385fea5a93112".to_owned()),
                    commit_date: Some("2023-12-21".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            