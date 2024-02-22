use base64;
use bech32::{self, ToBase32, Variant};
use ripemd::{self, Ripemd160};
use serde_json::{Result, Value};
use sha2::{Digest, Sha256};

fn main() {
    let json_data = r#"
    {
        "header": {
          "chain_id": "coreum-mainnet-1",
          "timestamp": "2024-02-22T10:10:03Z"
        },
        "data": {
          "height": "16301495",
          "txhash": "3C085517AB80B6A82C45C113C27D0F039A7921AD00E1444E7056186F387AA95C",
          "codespace": "",
          "code": 0,
          "data": "12380A312F6962632E6170706C69636174696F6E732E7472616E736665722E76312E4D73675472616E73666572526573706F6E7365120308FF40",
          "logs": [
            {
              "msg_index": 0,
              "log": "",
              "events": [
                {
                  "type": "message",
                  "attributes": [
                    {
                      "key": "action",
                      "value": "/ibc.applications.transfer.v1.MsgTransfer"
                    },
                    {
                      "key": "sender",
                      "value": "core1sv08cw5z2mcn4tqt9nc6y035mpf9q3xw3q5g6a"
                    }
                  ]
                },
                {
                  "type": "coin_spent",
                  "attributes": [
                    {
                      "key": "spender",
                      "value": "core1sv08cw5z2mcn4tqt9nc6y035mpf9q3xw3q5g6a"
                    },
                    {
                      "key": "amount",
                      "value": "190000000ucore"
                    }
                  ]
                },
                {
                  "type": "coin_received",
                  "attributes": [
                    {
                      "key": "receiver",
                      "value": "core12k2pyuylm9t7ugdvz67h9pg4gmmvhn5vvgafk0"
                    },
                    {
                      "key": "amount",
                      "value": "190000000ucore"
                    }
                  ]
                },
                {
                  "type": "transfer",
                  "attributes": [
                    {
                      "key": "recipient",
                      "value": "core12k2pyuylm9t7ugdvz67h9pg4gmmvhn5vvgafk0"
                    },
                    {
                      "key": "sender",
                      "value": "core1sv08cw5z2mcn4tqt9nc6y035mpf9q3xw3q5g6a"
                    },
                    {
                      "key": "amount",
                      "value": "190000000ucore"
                    }
                  ]
                },
                {
                  "type": "message",
                  "attributes": [
                    {
                      "key": "sender",
                      "value": "core1sv08cw5z2mcn4tqt9nc6y035mpf9q3xw3q5g6a"
                    }
                  ]
                },
                {
                  "type": "send_packet",
                  "attributes": [
                    {
                      "key": "packet_data",
                      "value": "{\"amount\":\"190000000\",\"denom\":\"ucore\",\"receiver\":\"osmo1kswjuwj9pm6dngkk3c845equk5zh6nh74c2pmy\",\"sender\":\"core1sv08cw5z2mcn4tqt9nc6y035mpf9q3xw3q5g6a\"}"
                    },
                    {
                      "key": "packet_data_hex",
                      "value": "7b22616d6f756e74223a22313930303030303030222c2264656e6f6d223a2275636f7265222c227265636569766572223a226f736d6f316b73776a75776a39706d36646e676b6b33633834356571756b357a68366e6837346332706d79222c2273656e646572223a22636f726531737630386377357a326d636e34747174396e6336793033356d70663971337877337135673661227d"
                    },
                    {
                      "key": "packet_timeout_height",
                      "value": "1-13930431"
                    },
                    {
                      "key": "packet_timeout_timestamp",
                      "value": "1708597192000000000"
                    },
                    {
                      "key": "packet_sequence",
                      "value": "8319"
                    },
                    {
                      "key": "packet_src_port",
                      "value": "transfer"
                    },
                    {
                      "key": "packet_src_channel",
                      "value": "channel-2"
                    },
                    {
                      "key": "packet_dst_port",
                      "value": "transfer"
                    },
                    {
                      "key": "packet_dst_channel",
                      "value": "channel-2188"
                    },
                    {
                      "key": "packet_channel_ordering",
                      "value": "ORDER_UNORDERED"
                    },
                    {
                      "key": "packet_connection",
                      "value": "connection-3"
                    },
                    {
                      "key": "connection_id",
                      "value": "connection-3"
                    }
                  ]
                },
                {
                  "type": "message",
                  "attributes": [
                    {
                      "key": "module",
                      "value": "ibc_channel"
                    }
                  ]
                },
                {
                  "type": "ibc_transfer",
                  "attributes": [
                    {
                      "key": "sender",
                      "value": "core1sv08cw5z2mcn4tqt9nc6y035mpf9q3xw3q5g6a"
                    },
                    {
                      "key": "receiver",
                      "value": "osmo1kswjuwj9pm6dngkk3c845equk5zh6nh74c2pmy"
                    },
                    {
                      "key": "amount",
                      "value": "190000000"
                    },
                    {
                      "key": "denom",
                      "value": "ucore"
                    },
                    {
                      "key": "memo",
                      "value": ""
                    }
                  ]
                },
                {
                  "type": "message",
                  "attributes": [
                    {
                      "key": "module",
                      "value": "transfer"
                    }
                  ]
                }
              ]
            }
          ],
          "info": "",
          "gas_wanted": "178500",
          "gas_used": "119000",
          "tx": {
            "@type": "/cosmos.tx.v1beta1.Tx",
            "body": {
              "messages": [
                {
                  "@type": "/ibc.applications.transfer.v1.MsgTransfer",
                  "source_port": "transfer",
                  "source_channel": "channel-2",
                  "token": {
                    "denom": "ucore",
                    "amount": "190000000"
                  },
                  "sender": "core1sv08cw5z2mcn4tqt9nc6y035mpf9q3xw3q5g6a",
                  "receiver": "osmo1kswjuwj9pm6dngkk3c845equk5zh6nh74c2pmy",
                  "timeout_height": {
                    "revision_number": "1",
                    "revision_height": "13930431"
                  },
                  "timeout_timestamp": "1708597192000000000",
                  "memo": ""
                }
              ],
              "memo": "",
              "timeout_height": "0",
              "extension_options": [],
              "non_critical_extension_options": []
            },
            "auth_info": {
              "signer_infos": [
                {
                  "public_key": {
                    "@type": "/cosmos.crypto.secp256k1.PubKey",
                    "key": "Ah44oJha6Nkt4r3Kr8+qeuAjUdWbzpqGapCf86pvdTCv"
                  },
                  "mode_info": {
                    "single": {
                      "mode": "SIGN_MODE_DIRECT"
                    }
                  },
                  "sequence": "81"
                }
              ],
              "fee": {
                "amount": [
                  {
                    "denom": "ucore",
                    "amount": "62500"
                  }
                ],
                "gas_limit": "178500",
                "payer": "",
                "granter": ""
              },
              "tip": null
            },
            "signatures": [
              "uH1ALxB1+QZqwQdDKf8vgVXtduU6+7SAKgwVPfQq3glvkfkNibB7MiHOfydRWGXdXPjshAw3fubF2AVCDAn0Uw=="
            ]
          },
          "timestamp": "2024-02-22T10:10:03Z",
          "events": []
        }
      }
    "#;

    // Parse JSON
    let v: Value = serde_json::from_str(json_data).unwrap();

    // Access auth_info
    let auth_info = &v["data"]["tx"]["auth_info"];

    // Loop through signer_infos
    for signer_info in auth_info["signer_infos"].as_array().unwrap() {
        let public_key = signer_info["public_key"]["key"].as_str().unwrap();
        let address = decode_pub(&public_key);
    }
}
pub fn decode_pub(pub_key: &str) -> String {
    let pubkey_bytes = base64::decode(pub_key).unwrap();

    // SHA-256 Hash
    let sha256_hash = Sha256::digest(&pubkey_bytes);

    // RIPEMD-160 Hash
    let ripemd160_hash = Ripemd160::digest(&sha256_hash);

    // Convert to Bech32
    let words = bech32::convert_bits(&ripemd160_hash, 8, 5, false).unwrap();
    let address: &str = &bech32::encode("core", words.to_base32(), Variant::Bech32).unwrap();

    println!("{}", address);
    address.to_string()
}
