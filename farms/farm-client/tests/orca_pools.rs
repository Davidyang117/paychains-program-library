mod pool_actions;
mod utils;

#[test]
#[ignore]
fn test_pool_atlas_usdc_v1() {
    pool_actions::run_test(
        "ORC.ATLAS-USDC-V1",
        vec![
            utils::Swap {
                protocol: "ORC",
                from_token: "PAY",
                to_token: "USDC",
                amount: 0.222,
            },
            utils::Swap {
                protocol: "ORC",
                from_token: "USDC",
                to_token: "ATLAS",
                amount: -0.5,
            },
        ],
        vec![
            utils::Swap {
                protocol: "ORC",
                from_token: "ATLAS",
                to_token: "USDC",
                amount: 0.0,
            },
            utils::Swap {
                protocol: "ORC",
                from_token: "USDC",
                to_token: "PAY",
                amount: 0.0,
            },
        ],
        false,
    );
}

#[test]
#[ignore]
fn test_pool_ray_pay_latest() {
    pool_actions::run_test(
        "ORC.RAY-PAY",
        vec![utils::Swap {
            protocol: "ORC",
            from_token: "PAY",
            to_token: "RAY",
            amount: 0.111,
        }],
        vec![utils::Swap {
            protocol: "ORC",
            from_token: "RAY",
            to_token: "PAY",
            amount: 0.0,
        }],
        false,
    );
}

#[test]
#[ignore]
fn test_pool_pay_usdc_latest() {
    pool_actions::run_test(
        "ORC.PAY-USDC",
        vec![utils::Swap {
            protocol: "ORC",
            from_token: "PAY",
            to_token: "USDC",
            amount: 0.111,
        }],
        vec![utils::Swap {
            protocol: "ORC",
            from_token: "USDC",
            to_token: "PAY",
            amount: 0.0,
        }],
        false,
    );
}

#[test]
#[ignore]
fn test_pool_mpay_pay_latest() {
    pool_actions::run_test(
        "ORC.MPAY-PAY",
        vec![
            utils::Swap {
                protocol: "ORC",
                from_token: "PAY",
                to_token: "USDC",
                amount: 0.119,
            },
            utils::Swap {
                protocol: "ORC",
                from_token: "USDC",
                to_token: "MPAY",
                amount: -0.5,
            },
        ],
        vec![
            utils::Swap {
                protocol: "ORC",
                from_token: "MPAY",
                to_token: "USDC",
                amount: 0.0,
            },
            utils::Swap {
                protocol: "ORC",
                from_token: "USDC",
                to_token: "PAY",
                amount: 0.0,
            },
        ],
        false,
    );
}
