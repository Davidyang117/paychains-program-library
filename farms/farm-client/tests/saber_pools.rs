mod pool_actions;
mod utils;

#[test]
#[ignore]
fn test_pool_xpay_pay_v1() {
    pool_actions::run_test(
        "SBR.XPAY-PAY-V1",
        vec![utils::Swap {
            protocol: "SBR",
            from_token: "PAY",
            to_token: "XPAY",
            amount: 0.111,
        }],
        vec![utils::Swap {
            protocol: "SBR",
            from_token: "XPAY",
            to_token: "PAY",
            amount: 0.0,
        }],
        false,
    );
}

#[test]
#[ignore]
fn test_pool_xpay_pay_latest() {
    pool_actions::run_test(
        "SBR.XPAY-PAY",
        vec![utils::Swap {
            protocol: "SBR",
            from_token: "PAY",
            to_token: "XPAY",
            amount: 0.111,
        }],
        vec![utils::Swap {
            protocol: "SBR",
            from_token: "XPAY",
            to_token: "PAY",
            amount: 0.0,
        }],
        false,
    );
}

#[test]
#[ignore]
fn test_pool_renbtc_btc_latest() {
    pool_actions::run_test(
        "SBR.RENBTC-BTC",
        vec![
            utils::Swap {
                protocol: "RDM",
                from_token: "PAY",
                to_token: "USDC",
                amount: 0.111,
            },
            utils::Swap {
                protocol: "RDM",
                from_token: "USDC",
                to_token: "BTC",
                amount: 0.0,
            },
            utils::Swap {
                protocol: "SBR",
                from_token: "BTC",
                to_token: "RENBTC",
                amount: -0.5,
            },
        ],
        vec![
            utils::Swap {
                protocol: "SBR",
                from_token: "RENBTC",
                to_token: "BTC",
                amount: 0.0,
            },
            utils::Swap {
                protocol: "RDM",
                from_token: "BTC",
                to_token: "USDC",
                amount: 0.0,
            },
        ],
        false,
    );
}

#[test]
#[ignore]
fn test_pool_usdc_wust_v1_latest() {
    pool_actions::run_test(
        "SBR.USDC-WUST_V1",
        vec![
            utils::Swap {
                protocol: "RDM",
                from_token: "PAY",
                to_token: "USDC",
                amount: 0.111,
            },
            utils::Swap {
                protocol: "SBR",
                from_token: "USDC",
                to_token: "WUST_V1",
                amount: -0.5,
            },
        ],
        vec![utils::Swap {
            protocol: "SBR",
            from_token: "WUST_V1",
            to_token: "USDC",
            amount: 0.0,
        }],
        false,
    );
}

#[test]
#[ignore]
fn test_pool_wust_usdc_latest() {
    pool_actions::run_test(
        "SBR.WUST-USDC",
        vec![
            utils::Swap {
                protocol: "RDM",
                from_token: "PAY",
                to_token: "USDC",
                amount: 0.111,
            },
            utils::Swap {
                protocol: "SBR",
                from_token: "USDC",
                to_token: "WUST",
                amount: -0.5,
            },
        ],
        vec![utils::Swap {
            protocol: "SBR",
            from_token: "WUST",
            to_token: "USDC",
            amount: 0.0,
        }],
        false,
    );
}

#[test]
#[ignore]
fn test_pool_whusd_v1_usdc_latest() {
    pool_actions::run_test(
        "SBR.WHUSD_V1-USDC",
        vec![
            utils::Swap {
                protocol: "RDM",
                from_token: "PAY",
                to_token: "USDC",
                amount: 0.111,
            },
            utils::Swap {
                protocol: "SBR",
                from_token: "USDC",
                to_token: "WHUSD_V1",
                amount: -0.5,
            },
        ],
        vec![utils::Swap {
            protocol: "SBR",
            from_token: "WHUSD_V1",
            to_token: "USDC",
            amount: 0.0,
        }],
        false,
    );
}
