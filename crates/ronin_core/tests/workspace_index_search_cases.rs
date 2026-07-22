//! Table-driven lexical search + attach gate cases (#74).

use ronin_core::{
    clamp_workspace_index_search_limit, drafts_for_workspace_index_include,
    may_inject_into_chat_request, workspace_index_hit_attachment,
    workspace_index_hit_attachment_origin, ContextOrigin, WorkspaceIndexHit,
    WorkspaceIndexHitSelection, WorkspaceIndexIncludeGate, WORKSPACE_INDEX_INCLUDE_GATE_LABEL,
    WORKSPACE_INDEX_SEARCH_DEFAULT_LIMIT, WORKSPACE_INDEX_SEARCH_MAX_LIMIT,
};

#[test]
fn search_limit_clamp_table() {
    let rows: &[(usize, usize)] = &[
        (0, 1),      // 0000
        (1, 1),      // 0001
        (2, 2),      // 0002
        (49, 49),    // 0003
        (50, 50),    // 0004
        (51, 51),    // 0005
        (199, 199),  // 0006
        (200, 200),  // 0007
        (201, 200),  // 0008
        (1000, 200), // 0009
        (9999, 200), // 0010
        (0, 1),      // 0011
        (1, 1),      // 0012
        (2, 2),      // 0013
        (49, 49),    // 0014
        (50, 50),    // 0015
        (51, 51),    // 0016
        (199, 199),  // 0017
        (200, 200),  // 0018
        (201, 200),  // 0019
        (1000, 200), // 0020
        (9999, 200), // 0021
        (0, 1),      // 0022
        (1, 1),      // 0023
        (2, 2),      // 0024
        (49, 49),    // 0025
        (50, 50),    // 0026
        (51, 51),    // 0027
        (199, 199),  // 0028
        (200, 200),  // 0029
        (201, 200),  // 0030
        (1000, 200), // 0031
        (9999, 200), // 0032
        (0, 1),      // 0033
        (1, 1),      // 0034
        (2, 2),      // 0035
        (49, 49),    // 0036
        (50, 50),    // 0037
        (51, 51),    // 0038
        (199, 199),  // 0039
        (200, 200),  // 0040
        (201, 200),  // 0041
        (1000, 200), // 0042
        (9999, 200), // 0043
        (0, 1),      // 0044
        (1, 1),      // 0045
        (2, 2),      // 0046
        (49, 49),    // 0047
        (50, 50),    // 0048
        (51, 51),    // 0049
        (199, 199),  // 0050
        (200, 200),  // 0051
        (201, 200),  // 0052
        (1000, 200), // 0053
        (9999, 200), // 0054
        (0, 1),      // 0055
        (1, 1),      // 0056
        (2, 2),      // 0057
        (49, 49),    // 0058
        (50, 50),    // 0059
        (51, 51),    // 0060
        (199, 199),  // 0061
        (200, 200),  // 0062
        (201, 200),  // 0063
        (1000, 200), // 0064
        (9999, 200), // 0065
        (0, 1),      // 0066
        (1, 1),      // 0067
        (2, 2),      // 0068
        (49, 49),    // 0069
        (50, 50),    // 0070
        (51, 51),    // 0071
        (199, 199),  // 0072
        (200, 200),  // 0073
        (201, 200),  // 0074
        (1000, 200), // 0075
        (9999, 200), // 0076
        (0, 1),      // 0077
        (1, 1),      // 0078
        (2, 2),      // 0079
        (49, 49),    // 0080
        (50, 50),    // 0081
        (51, 51),    // 0082
        (199, 199),  // 0083
        (200, 200),  // 0084
        (201, 200),  // 0085
        (1000, 200), // 0086
        (9999, 200), // 0087
        (0, 1),      // 0088
        (1, 1),      // 0089
        (2, 2),      // 0090
        (49, 49),    // 0091
        (50, 50),    // 0092
        (51, 51),    // 0093
        (199, 199),  // 0094
        (200, 200),  // 0095
        (201, 200),  // 0096
        (1000, 200), // 0097
        (9999, 200), // 0098
        (0, 1),      // 0099
        (1, 1),      // 0100
        (2, 2),      // 0101
        (49, 49),    // 0102
        (50, 50),    // 0103
        (51, 51),    // 0104
        (199, 199),  // 0105
        (200, 200),  // 0106
        (201, 200),  // 0107
        (1000, 200), // 0108
        (9999, 200), // 0109
        (0, 1),      // 0110
        (1, 1),      // 0111
        (2, 2),      // 0112
        (49, 49),    // 0113
        (50, 50),    // 0114
        (51, 51),    // 0115
        (199, 199),  // 0116
        (200, 200),  // 0117
        (201, 200),  // 0118
        (1000, 200), // 0119
    ];
    for (raw, expected) in rows {
        assert_eq!(clamp_workspace_index_search_limit(*raw), *expected);
    }
    assert_eq!(WORKSPACE_INDEX_SEARCH_DEFAULT_LIMIT, 50);
    assert_eq!(WORKSPACE_INDEX_SEARCH_MAX_LIMIT, 200);
}

#[test]
fn include_gate_default_off_table() {
    let n = 80;
    for i in 0..n {
        let gate = WorkspaceIndexIncludeGate::new();
        assert!(!gate.is_enabled(), "gate {i} must default off");
        assert_eq!(gate.label(), WORKSPACE_INDEX_INCLUDE_GATE_LABEL);
        assert_eq!(gate.context_origin(), ContextOrigin::IndexSearchHit);
        assert!(!may_inject_into_chat_request(gate.context_origin()));
    }
}

#[test]
fn include_gate_enable_disable_table() {
    let toggles: &[(bool, ContextOrigin, bool)] = &[
        (false, ContextOrigin::IndexSearchHit, false), // 0000
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0001
        (false, ContextOrigin::IndexSearchHit, false), // 0002
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0003
        (false, ContextOrigin::IndexSearchHit, false), // 0004
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0005
        (false, ContextOrigin::IndexSearchHit, false), // 0006
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0007
        (false, ContextOrigin::IndexSearchHit, false), // 0008
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0009
        (false, ContextOrigin::IndexSearchHit, false), // 0010
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0011
        (false, ContextOrigin::IndexSearchHit, false), // 0012
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0013
        (false, ContextOrigin::IndexSearchHit, false), // 0014
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0015
        (false, ContextOrigin::IndexSearchHit, false), // 0016
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0017
        (false, ContextOrigin::IndexSearchHit, false), // 0018
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0019
        (false, ContextOrigin::IndexSearchHit, false), // 0020
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0021
        (false, ContextOrigin::IndexSearchHit, false), // 0022
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0023
        (false, ContextOrigin::IndexSearchHit, false), // 0024
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0025
        (false, ContextOrigin::IndexSearchHit, false), // 0026
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0027
        (false, ContextOrigin::IndexSearchHit, false), // 0028
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0029
        (false, ContextOrigin::IndexSearchHit, false), // 0030
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0031
        (false, ContextOrigin::IndexSearchHit, false), // 0032
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0033
        (false, ContextOrigin::IndexSearchHit, false), // 0034
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0035
        (false, ContextOrigin::IndexSearchHit, false), // 0036
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0037
        (false, ContextOrigin::IndexSearchHit, false), // 0038
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0039
        (false, ContextOrigin::IndexSearchHit, false), // 0040
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0041
        (false, ContextOrigin::IndexSearchHit, false), // 0042
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0043
        (false, ContextOrigin::IndexSearchHit, false), // 0044
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0045
        (false, ContextOrigin::IndexSearchHit, false), // 0046
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0047
        (false, ContextOrigin::IndexSearchHit, false), // 0048
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0049
        (false, ContextOrigin::IndexSearchHit, false), // 0050
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0051
        (false, ContextOrigin::IndexSearchHit, false), // 0052
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0053
        (false, ContextOrigin::IndexSearchHit, false), // 0054
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0055
        (false, ContextOrigin::IndexSearchHit, false), // 0056
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0057
        (false, ContextOrigin::IndexSearchHit, false), // 0058
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0059
        (false, ContextOrigin::IndexSearchHit, false), // 0060
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0061
        (false, ContextOrigin::IndexSearchHit, false), // 0062
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0063
        (false, ContextOrigin::IndexSearchHit, false), // 0064
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0065
        (false, ContextOrigin::IndexSearchHit, false), // 0066
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0067
        (false, ContextOrigin::IndexSearchHit, false), // 0068
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0069
        (false, ContextOrigin::IndexSearchHit, false), // 0070
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0071
        (false, ContextOrigin::IndexSearchHit, false), // 0072
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0073
        (false, ContextOrigin::IndexSearchHit, false), // 0074
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0075
        (false, ContextOrigin::IndexSearchHit, false), // 0076
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0077
        (false, ContextOrigin::IndexSearchHit, false), // 0078
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0079
        (false, ContextOrigin::IndexSearchHit, false), // 0080
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0081
        (false, ContextOrigin::IndexSearchHit, false), // 0082
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0083
        (false, ContextOrigin::IndexSearchHit, false), // 0084
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0085
        (false, ContextOrigin::IndexSearchHit, false), // 0086
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0087
        (false, ContextOrigin::IndexSearchHit, false), // 0088
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0089
        (false, ContextOrigin::IndexSearchHit, false), // 0090
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0091
        (false, ContextOrigin::IndexSearchHit, false), // 0092
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0093
        (false, ContextOrigin::IndexSearchHit, false), // 0094
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0095
        (false, ContextOrigin::IndexSearchHit, false), // 0096
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0097
        (false, ContextOrigin::IndexSearchHit, false), // 0098
        (true, ContextOrigin::VisiblePerSendInclude, true), // 0099
    ];
    for (enabled, origin, inject) in toggles {
        let mut gate = WorkspaceIndexIncludeGate::new();
        gate.set_enabled(*enabled);
        assert_eq!(gate.is_enabled(), *enabled);
        assert_eq!(gate.context_origin(), *origin);
        assert_eq!(may_inject_into_chat_request(gate.context_origin()), *inject);
    }
}

#[test]
fn search_hit_origin_never_injects_table() {
    let paths: &[&str] = &[
        "src/file_0000.rs",
        "src/file_0001.rs",
        "src/file_0002.rs",
        "src/file_0003.rs",
        "src/file_0004.rs",
        "src/file_0005.rs",
        "src/file_0006.rs",
        "src/file_0007.rs",
        "src/file_0008.rs",
        "src/file_0009.rs",
        "src/file_0010.rs",
        "src/file_0011.rs",
        "src/file_0012.rs",
        "src/file_0013.rs",
        "src/file_0014.rs",
        "src/file_0015.rs",
        "src/file_0016.rs",
        "src/file_0017.rs",
        "src/file_0018.rs",
        "src/file_0019.rs",
        "src/file_0020.rs",
        "src/file_0021.rs",
        "src/file_0022.rs",
        "src/file_0023.rs",
        "src/file_0024.rs",
        "src/file_0025.rs",
        "src/file_0026.rs",
        "src/file_0027.rs",
        "src/file_0028.rs",
        "src/file_0029.rs",
        "src/file_0030.rs",
        "src/file_0031.rs",
        "src/file_0032.rs",
        "src/file_0033.rs",
        "src/file_0034.rs",
        "src/file_0035.rs",
        "src/file_0036.rs",
        "src/file_0037.rs",
        "src/file_0038.rs",
        "src/file_0039.rs",
        "src/file_0040.rs",
        "src/file_0041.rs",
        "src/file_0042.rs",
        "src/file_0043.rs",
        "src/file_0044.rs",
        "src/file_0045.rs",
        "src/file_0046.rs",
        "src/file_0047.rs",
        "src/file_0048.rs",
        "src/file_0049.rs",
        "src/file_0050.rs",
        "src/file_0051.rs",
        "src/file_0052.rs",
        "src/file_0053.rs",
        "src/file_0054.rs",
        "src/file_0055.rs",
        "src/file_0056.rs",
        "src/file_0057.rs",
        "src/file_0058.rs",
        "src/file_0059.rs",
        "src/file_0060.rs",
        "src/file_0061.rs",
        "src/file_0062.rs",
        "src/file_0063.rs",
        "src/file_0064.rs",
        "src/file_0065.rs",
        "src/file_0066.rs",
        "src/file_0067.rs",
        "src/file_0068.rs",
        "src/file_0069.rs",
        "src/file_0070.rs",
        "src/file_0071.rs",
        "src/file_0072.rs",
        "src/file_0073.rs",
        "src/file_0074.rs",
        "src/file_0075.rs",
        "src/file_0076.rs",
        "src/file_0077.rs",
        "src/file_0078.rs",
        "src/file_0079.rs",
        "src/file_0080.rs",
        "src/file_0081.rs",
        "src/file_0082.rs",
        "src/file_0083.rs",
        "src/file_0084.rs",
        "src/file_0085.rs",
        "src/file_0086.rs",
        "src/file_0087.rs",
        "src/file_0088.rs",
        "src/file_0089.rs",
        "src/file_0090.rs",
        "src/file_0091.rs",
        "src/file_0092.rs",
        "src/file_0093.rs",
        "src/file_0094.rs",
        "src/file_0095.rs",
        "src/file_0096.rs",
        "src/file_0097.rs",
        "src/file_0098.rs",
        "src/file_0099.rs",
        "src/file_0100.rs",
        "src/file_0101.rs",
        "src/file_0102.rs",
        "src/file_0103.rs",
        "src/file_0104.rs",
        "src/file_0105.rs",
        "src/file_0106.rs",
        "src/file_0107.rs",
        "src/file_0108.rs",
        "src/file_0109.rs",
        "src/file_0110.rs",
        "src/file_0111.rs",
        "src/file_0112.rs",
        "src/file_0113.rs",
        "src/file_0114.rs",
        "src/file_0115.rs",
        "src/file_0116.rs",
        "src/file_0117.rs",
        "src/file_0118.rs",
        "src/file_0119.rs",
        "src/file_0120.rs",
        "src/file_0121.rs",
        "src/file_0122.rs",
        "src/file_0123.rs",
        "src/file_0124.rs",
        "src/file_0125.rs",
        "src/file_0126.rs",
        "src/file_0127.rs",
        "src/file_0128.rs",
        "src/file_0129.rs",
        "src/file_0130.rs",
        "src/file_0131.rs",
        "src/file_0132.rs",
        "src/file_0133.rs",
        "src/file_0134.rs",
        "src/file_0135.rs",
        "src/file_0136.rs",
        "src/file_0137.rs",
        "src/file_0138.rs",
        "src/file_0139.rs",
        "src/file_0140.rs",
        "src/file_0141.rs",
        "src/file_0142.rs",
        "src/file_0143.rs",
        "src/file_0144.rs",
        "src/file_0145.rs",
        "src/file_0146.rs",
        "src/file_0147.rs",
        "src/file_0148.rs",
        "src/file_0149.rs",
    ];
    for path in paths {
        let hit = WorkspaceIndexHit {
            relative_path: (*path).to_string(),
            snippet: format!("match in {path}"),
        };
        assert_eq!(hit.context_origin(), ContextOrigin::IndexSearchHit);
        assert!(!may_inject_into_chat_request(hit.context_origin()));
    }
}

#[test]
fn hit_attachment_explicit_origin_table() {
    let rows: &[(&str, &str)] = &[
        ("path/file_0000.rs", "fn item_0000() {}"),
        ("path/file_0001.rs", "fn item_0001() {}"),
        ("path/file_0002.rs", "fn item_0002() {}"),
        ("path/file_0003.rs", "fn item_0003() {}"),
        ("path/file_0004.rs", "fn item_0004() {}"),
        ("path/file_0005.rs", "fn item_0005() {}"),
        ("path/file_0006.rs", "fn item_0006() {}"),
        ("path/file_0007.rs", "fn item_0007() {}"),
        ("path/file_0008.rs", "fn item_0008() {}"),
        ("path/file_0009.rs", "fn item_0009() {}"),
        ("path/file_0010.rs", "fn item_0010() {}"),
        ("path/file_0011.rs", "fn item_0011() {}"),
        ("path/file_0012.rs", "fn item_0012() {}"),
        ("path/file_0013.rs", "fn item_0013() {}"),
        ("path/file_0014.rs", "fn item_0014() {}"),
        ("path/file_0015.rs", "fn item_0015() {}"),
        ("path/file_0016.rs", "fn item_0016() {}"),
        ("path/file_0017.rs", "fn item_0017() {}"),
        ("path/file_0018.rs", "fn item_0018() {}"),
        ("path/file_0019.rs", "fn item_0019() {}"),
        ("path/file_0020.rs", "fn item_0020() {}"),
        ("path/file_0021.rs", "fn item_0021() {}"),
        ("path/file_0022.rs", "fn item_0022() {}"),
        ("path/file_0023.rs", "fn item_0023() {}"),
        ("path/file_0024.rs", "fn item_0024() {}"),
        ("path/file_0025.rs", "fn item_0025() {}"),
        ("path/file_0026.rs", "fn item_0026() {}"),
        ("path/file_0027.rs", "fn item_0027() {}"),
        ("path/file_0028.rs", "fn item_0028() {}"),
        ("path/file_0029.rs", "fn item_0029() {}"),
        ("path/file_0030.rs", "fn item_0030() {}"),
        ("path/file_0031.rs", "fn item_0031() {}"),
        ("path/file_0032.rs", "fn item_0032() {}"),
        ("path/file_0033.rs", "fn item_0033() {}"),
        ("path/file_0034.rs", "fn item_0034() {}"),
        ("path/file_0035.rs", "fn item_0035() {}"),
        ("path/file_0036.rs", "fn item_0036() {}"),
        ("path/file_0037.rs", "fn item_0037() {}"),
        ("path/file_0038.rs", "fn item_0038() {}"),
        ("path/file_0039.rs", "fn item_0039() {}"),
        ("path/file_0040.rs", "fn item_0040() {}"),
        ("path/file_0041.rs", "fn item_0041() {}"),
        ("path/file_0042.rs", "fn item_0042() {}"),
        ("path/file_0043.rs", "fn item_0043() {}"),
        ("path/file_0044.rs", "fn item_0044() {}"),
        ("path/file_0045.rs", "fn item_0045() {}"),
        ("path/file_0046.rs", "fn item_0046() {}"),
        ("path/file_0047.rs", "fn item_0047() {}"),
        ("path/file_0048.rs", "fn item_0048() {}"),
        ("path/file_0049.rs", "fn item_0049() {}"),
        ("path/file_0050.rs", "fn item_0050() {}"),
        ("path/file_0051.rs", "fn item_0051() {}"),
        ("path/file_0052.rs", "fn item_0052() {}"),
        ("path/file_0053.rs", "fn item_0053() {}"),
        ("path/file_0054.rs", "fn item_0054() {}"),
        ("path/file_0055.rs", "fn item_0055() {}"),
        ("path/file_0056.rs", "fn item_0056() {}"),
        ("path/file_0057.rs", "fn item_0057() {}"),
        ("path/file_0058.rs", "fn item_0058() {}"),
        ("path/file_0059.rs", "fn item_0059() {}"),
        ("path/file_0060.rs", "fn item_0060() {}"),
        ("path/file_0061.rs", "fn item_0061() {}"),
        ("path/file_0062.rs", "fn item_0062() {}"),
        ("path/file_0063.rs", "fn item_0063() {}"),
        ("path/file_0064.rs", "fn item_0064() {}"),
        ("path/file_0065.rs", "fn item_0065() {}"),
        ("path/file_0066.rs", "fn item_0066() {}"),
        ("path/file_0067.rs", "fn item_0067() {}"),
        ("path/file_0068.rs", "fn item_0068() {}"),
        ("path/file_0069.rs", "fn item_0069() {}"),
        ("path/file_0070.rs", "fn item_0070() {}"),
        ("path/file_0071.rs", "fn item_0071() {}"),
        ("path/file_0072.rs", "fn item_0072() {}"),
        ("path/file_0073.rs", "fn item_0073() {}"),
        ("path/file_0074.rs", "fn item_0074() {}"),
        ("path/file_0075.rs", "fn item_0075() {}"),
        ("path/file_0076.rs", "fn item_0076() {}"),
        ("path/file_0077.rs", "fn item_0077() {}"),
        ("path/file_0078.rs", "fn item_0078() {}"),
        ("path/file_0079.rs", "fn item_0079() {}"),
        ("path/file_0080.rs", "fn item_0080() {}"),
        ("path/file_0081.rs", "fn item_0081() {}"),
        ("path/file_0082.rs", "fn item_0082() {}"),
        ("path/file_0083.rs", "fn item_0083() {}"),
        ("path/file_0084.rs", "fn item_0084() {}"),
        ("path/file_0085.rs", "fn item_0085() {}"),
        ("path/file_0086.rs", "fn item_0086() {}"),
        ("path/file_0087.rs", "fn item_0087() {}"),
        ("path/file_0088.rs", "fn item_0088() {}"),
        ("path/file_0089.rs", "fn item_0089() {}"),
        ("path/file_0090.rs", "fn item_0090() {}"),
        ("path/file_0091.rs", "fn item_0091() {}"),
        ("path/file_0092.rs", "fn item_0092() {}"),
        ("path/file_0093.rs", "fn item_0093() {}"),
        ("path/file_0094.rs", "fn item_0094() {}"),
        ("path/file_0095.rs", "fn item_0095() {}"),
        ("path/file_0096.rs", "fn item_0096() {}"),
        ("path/file_0097.rs", "fn item_0097() {}"),
        ("path/file_0098.rs", "fn item_0098() {}"),
        ("path/file_0099.rs", "fn item_0099() {}"),
        ("path/file_0100.rs", "fn item_0100() {}"),
        ("path/file_0101.rs", "fn item_0101() {}"),
        ("path/file_0102.rs", "fn item_0102() {}"),
        ("path/file_0103.rs", "fn item_0103() {}"),
        ("path/file_0104.rs", "fn item_0104() {}"),
        ("path/file_0105.rs", "fn item_0105() {}"),
        ("path/file_0106.rs", "fn item_0106() {}"),
        ("path/file_0107.rs", "fn item_0107() {}"),
        ("path/file_0108.rs", "fn item_0108() {}"),
        ("path/file_0109.rs", "fn item_0109() {}"),
        ("path/file_0110.rs", "fn item_0110() {}"),
        ("path/file_0111.rs", "fn item_0111() {}"),
        ("path/file_0112.rs", "fn item_0112() {}"),
        ("path/file_0113.rs", "fn item_0113() {}"),
        ("path/file_0114.rs", "fn item_0114() {}"),
        ("path/file_0115.rs", "fn item_0115() {}"),
        ("path/file_0116.rs", "fn item_0116() {}"),
        ("path/file_0117.rs", "fn item_0117() {}"),
        ("path/file_0118.rs", "fn item_0118() {}"),
        ("path/file_0119.rs", "fn item_0119() {}"),
    ];
    assert_eq!(
        workspace_index_hit_attachment_origin(),
        ContextOrigin::ExplicitAttachment
    );
    assert!(may_inject_into_chat_request(
        workspace_index_hit_attachment_origin()
    ));
    for (path, body) in rows {
        let draft = workspace_index_hit_attachment(path, body);
        assert!(draft.context_block.contains("Attached workspace file"));
        assert!(draft.context_block.contains(path));
        assert!(draft.context_block.contains(body));
        assert_eq!(draft.content.as_deref(), Some(*body));
    }
}

#[test]
fn include_gate_draft_release_matrix() {
    let draft = workspace_index_hit_attachment("a.rs", "fn a() {}");
    let cases: &[(bool, usize, usize)] = &[
        (true, 0, 0),  // 0000
        (false, 1, 0), // 0001
        (true, 2, 2),  // 0002
        (false, 0, 0), // 0003
        (true, 1, 1),  // 0004
        (false, 2, 0), // 0005
        (true, 0, 0),  // 0006
        (false, 1, 0), // 0007
        (true, 2, 2),  // 0008
        (false, 0, 0), // 0009
        (true, 1, 1),  // 0010
        (false, 2, 0), // 0011
        (true, 0, 0),  // 0012
        (false, 1, 0), // 0013
        (true, 2, 2),  // 0014
        (false, 0, 0), // 0015
        (true, 1, 1),  // 0016
        (false, 2, 0), // 0017
        (true, 0, 0),  // 0018
        (false, 1, 0), // 0019
        (true, 2, 2),  // 0020
        (false, 0, 0), // 0021
        (true, 1, 1),  // 0022
        (false, 2, 0), // 0023
        (true, 0, 0),  // 0024
        (false, 1, 0), // 0025
        (true, 2, 2),  // 0026
        (false, 0, 0), // 0027
        (true, 1, 1),  // 0028
        (false, 2, 0), // 0029
        (true, 0, 0),  // 0030
        (false, 1, 0), // 0031
        (true, 2, 2),  // 0032
        (false, 0, 0), // 0033
        (true, 1, 1),  // 0034
        (false, 2, 0), // 0035
        (true, 0, 0),  // 0036
        (false, 1, 0), // 0037
        (true, 2, 2),  // 0038
        (false, 0, 0), // 0039
        (true, 1, 1),  // 0040
        (false, 2, 0), // 0041
        (true, 0, 0),  // 0042
        (false, 1, 0), // 0043
        (true, 2, 2),  // 0044
        (false, 0, 0), // 0045
        (true, 1, 1),  // 0046
        (false, 2, 0), // 0047
        (true, 0, 0),  // 0048
        (false, 1, 0), // 0049
        (true, 2, 2),  // 0050
        (false, 0, 0), // 0051
        (true, 1, 1),  // 0052
        (false, 2, 0), // 0053
        (true, 0, 0),  // 0054
        (false, 1, 0), // 0055
        (true, 2, 2),  // 0056
        (false, 0, 0), // 0057
        (true, 1, 1),  // 0058
        (false, 2, 0), // 0059
        (true, 0, 0),  // 0060
        (false, 1, 0), // 0061
        (true, 2, 2),  // 0062
        (false, 0, 0), // 0063
        (true, 1, 1),  // 0064
        (false, 2, 0), // 0065
        (true, 0, 0),  // 0066
        (false, 1, 0), // 0067
        (true, 2, 2),  // 0068
        (false, 0, 0), // 0069
        (true, 1, 1),  // 0070
        (false, 2, 0), // 0071
        (true, 0, 0),  // 0072
        (false, 1, 0), // 0073
        (true, 2, 2),  // 0074
        (false, 0, 0), // 0075
        (true, 1, 1),  // 0076
        (false, 2, 0), // 0077
        (true, 0, 0),  // 0078
        (false, 1, 0), // 0079
    ];
    for (enabled, selected, expected) in cases {
        let mut gate = WorkspaceIndexIncludeGate::new();
        gate.set_enabled(*enabled);
        let selected_drafts = vec![draft.clone(); *selected];
        let out = drafts_for_workspace_index_include(&gate, &selected_drafts);
        assert_eq!(out.len(), *expected);
    }
}

#[test]
fn hit_selection_dedupe_and_clear_table() {
    let batches: &[&[&str]] = &[
        &["a000.rs", "a000.rs", "b000.rs"],
        &["a001.rs", "a001.rs", "b001.rs"],
        &["a002.rs", "a002.rs", "b002.rs"],
        &["a003.rs", "a003.rs", "b003.rs"],
        &["a004.rs", "a004.rs", "b004.rs"],
        &["a005.rs", "a005.rs", "b005.rs"],
        &["a006.rs", "a006.rs", "b006.rs"],
        &["a007.rs", "a007.rs", "b007.rs"],
        &["a008.rs", "a008.rs", "b008.rs"],
        &["a009.rs", "a009.rs", "b009.rs"],
        &["a010.rs", "a010.rs", "b010.rs"],
        &["a011.rs", "a011.rs", "b011.rs"],
        &["a012.rs", "a012.rs", "b012.rs"],
        &["a013.rs", "a013.rs", "b013.rs"],
        &["a014.rs", "a014.rs", "b014.rs"],
        &["a015.rs", "a015.rs", "b015.rs"],
        &["a016.rs", "a016.rs", "b016.rs"],
        &["a017.rs", "a017.rs", "b017.rs"],
        &["a018.rs", "a018.rs", "b018.rs"],
        &["a019.rs", "a019.rs", "b019.rs"],
        &["a020.rs", "a020.rs", "b020.rs"],
        &["a021.rs", "a021.rs", "b021.rs"],
        &["a022.rs", "a022.rs", "b022.rs"],
        &["a023.rs", "a023.rs", "b023.rs"],
        &["a024.rs", "a024.rs", "b024.rs"],
        &["a025.rs", "a025.rs", "b025.rs"],
        &["a026.rs", "a026.rs", "b026.rs"],
        &["a027.rs", "a027.rs", "b027.rs"],
        &["a028.rs", "a028.rs", "b028.rs"],
        &["a029.rs", "a029.rs", "b029.rs"],
        &["a030.rs", "a030.rs", "b030.rs"],
        &["a031.rs", "a031.rs", "b031.rs"],
        &["a032.rs", "a032.rs", "b032.rs"],
        &["a033.rs", "a033.rs", "b033.rs"],
        &["a034.rs", "a034.rs", "b034.rs"],
        &["a035.rs", "a035.rs", "b035.rs"],
        &["a036.rs", "a036.rs", "b036.rs"],
        &["a037.rs", "a037.rs", "b037.rs"],
        &["a038.rs", "a038.rs", "b038.rs"],
        &["a039.rs", "a039.rs", "b039.rs"],
        &["a040.rs", "a040.rs", "b040.rs"],
        &["a041.rs", "a041.rs", "b041.rs"],
        &["a042.rs", "a042.rs", "b042.rs"],
        &["a043.rs", "a043.rs", "b043.rs"],
        &["a044.rs", "a044.rs", "b044.rs"],
        &["a045.rs", "a045.rs", "b045.rs"],
        &["a046.rs", "a046.rs", "b046.rs"],
        &["a047.rs", "a047.rs", "b047.rs"],
        &["a048.rs", "a048.rs", "b048.rs"],
        &["a049.rs", "a049.rs", "b049.rs"],
        &["a050.rs", "a050.rs", "b050.rs"],
        &["a051.rs", "a051.rs", "b051.rs"],
        &["a052.rs", "a052.rs", "b052.rs"],
        &["a053.rs", "a053.rs", "b053.rs"],
        &["a054.rs", "a054.rs", "b054.rs"],
        &["a055.rs", "a055.rs", "b055.rs"],
        &["a056.rs", "a056.rs", "b056.rs"],
        &["a057.rs", "a057.rs", "b057.rs"],
        &["a058.rs", "a058.rs", "b058.rs"],
        &["a059.rs", "a059.rs", "b059.rs"],
    ];
    for (i, batch) in batches.iter().enumerate() {
        let mut sel = WorkspaceIndexHitSelection::new();
        sel.set_paths(batch.iter().copied());
        assert_eq!(sel.paths().len(), 2, "batch {i}");
        assert_eq!(sel.paths()[0], batch[0]);
        assert_eq!(sel.paths()[1], batch[2]);
        sel.deselect(batch[0]);
        assert_eq!(sel.paths(), &[batch[2].to_string()]);
        sel.clear();
        assert!(sel.is_empty());
    }
}

#[test]
fn hit_attachment_scrubs_secret_shapes_table() {
    let secrets: &[&str] = &[
        "api_key=sk-abc123",
        "token=secret-token-value",
        "password=hunter2",
        "bearer sk-live-xyz",
        "access_token=tok_999",
        "key=supersecret",
        "api_key=sk-abc123",
        "token=secret-token-value",
        "password=hunter2",
        "bearer sk-live-xyz",
        "access_token=tok_999",
        "key=supersecret",
        "api_key=sk-abc123",
        "token=secret-token-value",
        "password=hunter2",
        "bearer sk-live-xyz",
        "access_token=tok_999",
        "key=supersecret",
        "api_key=sk-abc123",
        "token=secret-token-value",
        "password=hunter2",
        "bearer sk-live-xyz",
        "access_token=tok_999",
        "key=supersecret",
        "api_key=sk-abc123",
        "token=secret-token-value",
        "password=hunter2",
        "bearer sk-live-xyz",
        "access_token=tok_999",
        "key=supersecret",
        "api_key=sk-abc123",
        "token=secret-token-value",
        "password=hunter2",
        "bearer sk-live-xyz",
        "access_token=tok_999",
        "key=supersecret",
        "api_key=sk-abc123",
        "token=secret-token-value",
        "password=hunter2",
        "bearer sk-live-xyz",
        "access_token=tok_999",
        "key=supersecret",
        "api_key=sk-abc123",
        "token=secret-token-value",
        "password=hunter2",
        "bearer sk-live-xyz",
        "access_token=tok_999",
        "key=supersecret",
        "api_key=sk-abc123",
        "token=secret-token-value",
        "password=hunter2",
        "bearer sk-live-xyz",
        "access_token=tok_999",
        "key=supersecret",
        "api_key=sk-abc123",
        "token=secret-token-value",
        "password=hunter2",
        "bearer sk-live-xyz",
        "access_token=tok_999",
        "key=supersecret",
        "api_key=sk-abc123",
        "token=secret-token-value",
        "password=hunter2",
        "bearer sk-live-xyz",
        "access_token=tok_999",
        "key=supersecret",
        "api_key=sk-abc123",
        "token=secret-token-value",
        "password=hunter2",
        "bearer sk-live-xyz",
        "access_token=tok_999",
        "key=supersecret",
        "api_key=sk-abc123",
        "token=secret-token-value",
        "password=hunter2",
        "bearer sk-live-xyz",
        "access_token=tok_999",
        "key=supersecret",
        "api_key=sk-abc123",
        "token=secret-token-value",
        "password=hunter2",
        "bearer sk-live-xyz",
        "access_token=tok_999",
        "key=supersecret",
        "api_key=sk-abc123",
        "token=secret-token-value",
        "password=hunter2",
        "bearer sk-live-xyz",
        "access_token=tok_999",
        "key=supersecret",
    ];
    for (i, secret) in secrets.iter().enumerate() {
        let draft = workspace_index_hit_attachment(&format!("s{i}.env"), secret);
        assert!(
            draft.context_block.contains("REDACTED"),
            "secret shape {i} should scrub: {secret}"
        );
        // Raw secret value fragment should not remain verbatim when keyed.
        if secret.contains('=') {
            let raw = secret.split('=').nth(1).unwrap_or("");
            if !raw.is_empty() {
                assert!(!draft.context_block.contains(raw));
            }
        }
    }
}
