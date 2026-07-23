//! Scenario id / kind / generator corpus.
#![allow(clippy::too_many_lines)]

use ronin_perf_harness::{generate_scale_messages, ScenarioKind};

#[test]
fn scenario_kind_round_trips_known_ids() {
    for id in ["plain_short", "heavy_fences", "long_history"] {
        let k = ScenarioKind::parse(id).expect(id);
        assert_eq!(k.as_str(), id);
    }
}

#[test]
fn scenario_kind_rejects_unknown_matrix() {
    let bad = [
        "unknown_scenario_0",
        "unknown_scenario_1",
        "unknown_scenario_2",
        "unknown_scenario_3",
        "unknown_scenario_4",
        "unknown_scenario_5",
        "unknown_scenario_6",
        "unknown_scenario_7",
        "unknown_scenario_8",
        "unknown_scenario_9",
        "unknown_scenario_10",
        "unknown_scenario_11",
        "unknown_scenario_12",
        "unknown_scenario_13",
        "unknown_scenario_14",
        "unknown_scenario_15",
        "unknown_scenario_16",
        "unknown_scenario_17",
        "unknown_scenario_18",
        "unknown_scenario_19",
        "unknown_scenario_20",
        "unknown_scenario_21",
        "unknown_scenario_22",
        "unknown_scenario_23",
        "unknown_scenario_24",
        "unknown_scenario_25",
        "unknown_scenario_26",
        "unknown_scenario_27",
        "unknown_scenario_28",
        "unknown_scenario_29",
        "unknown_scenario_30",
        "unknown_scenario_31",
        "unknown_scenario_32",
        "unknown_scenario_33",
        "unknown_scenario_34",
        "unknown_scenario_35",
        "unknown_scenario_36",
        "unknown_scenario_37",
        "unknown_scenario_38",
        "unknown_scenario_39",
        "unknown_scenario_40",
        "unknown_scenario_41",
        "unknown_scenario_42",
        "unknown_scenario_43",
        "unknown_scenario_44",
        "unknown_scenario_45",
        "unknown_scenario_46",
        "unknown_scenario_47",
        "unknown_scenario_48",
        "unknown_scenario_49",
        "unknown_scenario_50",
        "unknown_scenario_51",
        "unknown_scenario_52",
        "unknown_scenario_53",
        "unknown_scenario_54",
        "unknown_scenario_55",
        "unknown_scenario_56",
        "unknown_scenario_57",
        "unknown_scenario_58",
        "unknown_scenario_59",
        "unknown_scenario_60",
        "unknown_scenario_61",
        "unknown_scenario_62",
        "unknown_scenario_63",
        "unknown_scenario_64",
        "unknown_scenario_65",
        "unknown_scenario_66",
        "unknown_scenario_67",
        "unknown_scenario_68",
        "unknown_scenario_69",
        "unknown_scenario_70",
        "unknown_scenario_71",
        "unknown_scenario_72",
        "unknown_scenario_73",
        "unknown_scenario_74",
        "unknown_scenario_75",
        "unknown_scenario_76",
        "unknown_scenario_77",
        "unknown_scenario_78",
        "unknown_scenario_79",
        "unknown_scenario_80",
        "unknown_scenario_81",
        "unknown_scenario_82",
        "unknown_scenario_83",
        "unknown_scenario_84",
        "unknown_scenario_85",
        "unknown_scenario_86",
        "unknown_scenario_87",
        "unknown_scenario_88",
        "unknown_scenario_89",
        "unknown_scenario_90",
        "unknown_scenario_91",
        "unknown_scenario_92",
        "unknown_scenario_93",
        "unknown_scenario_94",
        "unknown_scenario_95",
        "unknown_scenario_96",
        "unknown_scenario_97",
        "unknown_scenario_98",
        "unknown_scenario_99",
        "unknown_scenario_100",
        "unknown_scenario_101",
        "unknown_scenario_102",
        "unknown_scenario_103",
        "unknown_scenario_104",
        "unknown_scenario_105",
        "unknown_scenario_106",
        "unknown_scenario_107",
        "unknown_scenario_108",
        "unknown_scenario_109",
        "unknown_scenario_110",
        "unknown_scenario_111",
        "unknown_scenario_112",
        "unknown_scenario_113",
        "unknown_scenario_114",
        "unknown_scenario_115",
        "unknown_scenario_116",
        "unknown_scenario_117",
        "unknown_scenario_118",
        "unknown_scenario_119",
    ];
    for id in bad {
        assert!(ScenarioKind::parse(id).is_err(), "{id}");
    }
}

#[test]
fn generate_scale_messages_sizes_and_shapes() {
    for n in [0u32, 1, 2, 3, 5, 8, 13, 21, 34, 55, 80, 100, 120, 150, 200] {
        let msgs = generate_scale_messages(n as usize, true);
        assert_eq!(msgs.len(), n as usize);
        if n > 0 {
            assert!(msgs[0].contains("0") || msgs[0].contains("Message"));
        }
        let plain = generate_scale_messages(n as usize, false);
        assert_eq!(plain.len(), n as usize);
        for (i, m) in plain.iter().enumerate() {
            assert!(m.contains(&format!("Message {i}")) || n == 0);
        }
    }
}

#[test]
fn generate_scale_messages_fence_cadence() {
    for n in 1..=90 {
        let msgs = generate_scale_messages(n, true);
        for (i, m) in msgs.iter().enumerate() {
            if i % 3 == 0 {
                assert!(m.contains("```rust"), "n={n} i={i}");
            } else {
                assert!(!m.contains("```rust"), "n={n} i={i}");
            }
        }
    }
}
