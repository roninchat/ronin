//! Dense public-seam assertions for search/attach gate (#74).

use ronin_core::{
    clamp_workspace_index_search_limit, drafts_for_workspace_index_include,
    may_inject_into_chat_request, workspace_index_hit_attachment, ContextOrigin, WorkspaceIndexHit,
    WorkspaceIndexIncludeGate, WORKSPACE_INDEX_INCLUDE_GATE_LABEL,
};

#[test]
fn dense_hit_fields_and_origins() {
    let hits: &[WorkspaceIndexHit] = &[
        WorkspaceIndexHit {
            relative_path: "p/0000.rs".into(),
            snippet: "sn0000".into(),
        }, // 0000
        WorkspaceIndexHit {
            relative_path: "p/0001.rs".into(),
            snippet: "sn0001".into(),
        }, // 0001
        WorkspaceIndexHit {
            relative_path: "p/0002.rs".into(),
            snippet: "sn0002".into(),
        }, // 0002
        WorkspaceIndexHit {
            relative_path: "p/0003.rs".into(),
            snippet: "sn0003".into(),
        }, // 0003
        WorkspaceIndexHit {
            relative_path: "p/0004.rs".into(),
            snippet: "sn0004".into(),
        }, // 0004
        WorkspaceIndexHit {
            relative_path: "p/0005.rs".into(),
            snippet: "sn0005".into(),
        }, // 0005
        WorkspaceIndexHit {
            relative_path: "p/0006.rs".into(),
            snippet: "sn0006".into(),
        }, // 0006
        WorkspaceIndexHit {
            relative_path: "p/0007.rs".into(),
            snippet: "sn0007".into(),
        }, // 0007
        WorkspaceIndexHit {
            relative_path: "p/0008.rs".into(),
            snippet: "sn0008".into(),
        }, // 0008
        WorkspaceIndexHit {
            relative_path: "p/0009.rs".into(),
            snippet: "sn0009".into(),
        }, // 0009
        WorkspaceIndexHit {
            relative_path: "p/0010.rs".into(),
            snippet: "sn0010".into(),
        }, // 0010
        WorkspaceIndexHit {
            relative_path: "p/0011.rs".into(),
            snippet: "sn0011".into(),
        }, // 0011
        WorkspaceIndexHit {
            relative_path: "p/0012.rs".into(),
            snippet: "sn0012".into(),
        }, // 0012
        WorkspaceIndexHit {
            relative_path: "p/0013.rs".into(),
            snippet: "sn0013".into(),
        }, // 0013
        WorkspaceIndexHit {
            relative_path: "p/0014.rs".into(),
            snippet: "sn0014".into(),
        }, // 0014
        WorkspaceIndexHit {
            relative_path: "p/0015.rs".into(),
            snippet: "sn0015".into(),
        }, // 0015
        WorkspaceIndexHit {
            relative_path: "p/0016.rs".into(),
            snippet: "sn0016".into(),
        }, // 0016
        WorkspaceIndexHit {
            relative_path: "p/0017.rs".into(),
            snippet: "sn0017".into(),
        }, // 0017
        WorkspaceIndexHit {
            relative_path: "p/0018.rs".into(),
            snippet: "sn0018".into(),
        }, // 0018
        WorkspaceIndexHit {
            relative_path: "p/0019.rs".into(),
            snippet: "sn0019".into(),
        }, // 0019
        WorkspaceIndexHit {
            relative_path: "p/0020.rs".into(),
            snippet: "sn0020".into(),
        }, // 0020
        WorkspaceIndexHit {
            relative_path: "p/0021.rs".into(),
            snippet: "sn0021".into(),
        }, // 0021
        WorkspaceIndexHit {
            relative_path: "p/0022.rs".into(),
            snippet: "sn0022".into(),
        }, // 0022
        WorkspaceIndexHit {
            relative_path: "p/0023.rs".into(),
            snippet: "sn0023".into(),
        }, // 0023
        WorkspaceIndexHit {
            relative_path: "p/0024.rs".into(),
            snippet: "sn0024".into(),
        }, // 0024
        WorkspaceIndexHit {
            relative_path: "p/0025.rs".into(),
            snippet: "sn0025".into(),
        }, // 0025
        WorkspaceIndexHit {
            relative_path: "p/0026.rs".into(),
            snippet: "sn0026".into(),
        }, // 0026
        WorkspaceIndexHit {
            relative_path: "p/0027.rs".into(),
            snippet: "sn0027".into(),
        }, // 0027
        WorkspaceIndexHit {
            relative_path: "p/0028.rs".into(),
            snippet: "sn0028".into(),
        }, // 0028
        WorkspaceIndexHit {
            relative_path: "p/0029.rs".into(),
            snippet: "sn0029".into(),
        }, // 0029
        WorkspaceIndexHit {
            relative_path: "p/0030.rs".into(),
            snippet: "sn0030".into(),
        }, // 0030
        WorkspaceIndexHit {
            relative_path: "p/0031.rs".into(),
            snippet: "sn0031".into(),
        }, // 0031
        WorkspaceIndexHit {
            relative_path: "p/0032.rs".into(),
            snippet: "sn0032".into(),
        }, // 0032
        WorkspaceIndexHit {
            relative_path: "p/0033.rs".into(),
            snippet: "sn0033".into(),
        }, // 0033
        WorkspaceIndexHit {
            relative_path: "p/0034.rs".into(),
            snippet: "sn0034".into(),
        }, // 0034
        WorkspaceIndexHit {
            relative_path: "p/0035.rs".into(),
            snippet: "sn0035".into(),
        }, // 0035
        WorkspaceIndexHit {
            relative_path: "p/0036.rs".into(),
            snippet: "sn0036".into(),
        }, // 0036
        WorkspaceIndexHit {
            relative_path: "p/0037.rs".into(),
            snippet: "sn0037".into(),
        }, // 0037
        WorkspaceIndexHit {
            relative_path: "p/0038.rs".into(),
            snippet: "sn0038".into(),
        }, // 0038
        WorkspaceIndexHit {
            relative_path: "p/0039.rs".into(),
            snippet: "sn0039".into(),
        }, // 0039
        WorkspaceIndexHit {
            relative_path: "p/0040.rs".into(),
            snippet: "sn0040".into(),
        }, // 0040
        WorkspaceIndexHit {
            relative_path: "p/0041.rs".into(),
            snippet: "sn0041".into(),
        }, // 0041
        WorkspaceIndexHit {
            relative_path: "p/0042.rs".into(),
            snippet: "sn0042".into(),
        }, // 0042
        WorkspaceIndexHit {
            relative_path: "p/0043.rs".into(),
            snippet: "sn0043".into(),
        }, // 0043
        WorkspaceIndexHit {
            relative_path: "p/0044.rs".into(),
            snippet: "sn0044".into(),
        }, // 0044
        WorkspaceIndexHit {
            relative_path: "p/0045.rs".into(),
            snippet: "sn0045".into(),
        }, // 0045
        WorkspaceIndexHit {
            relative_path: "p/0046.rs".into(),
            snippet: "sn0046".into(),
        }, // 0046
        WorkspaceIndexHit {
            relative_path: "p/0047.rs".into(),
            snippet: "sn0047".into(),
        }, // 0047
        WorkspaceIndexHit {
            relative_path: "p/0048.rs".into(),
            snippet: "sn0048".into(),
        }, // 0048
        WorkspaceIndexHit {
            relative_path: "p/0049.rs".into(),
            snippet: "sn0049".into(),
        }, // 0049
        WorkspaceIndexHit {
            relative_path: "p/0050.rs".into(),
            snippet: "sn0050".into(),
        }, // 0050
        WorkspaceIndexHit {
            relative_path: "p/0051.rs".into(),
            snippet: "sn0051".into(),
        }, // 0051
        WorkspaceIndexHit {
            relative_path: "p/0052.rs".into(),
            snippet: "sn0052".into(),
        }, // 0052
        WorkspaceIndexHit {
            relative_path: "p/0053.rs".into(),
            snippet: "sn0053".into(),
        }, // 0053
        WorkspaceIndexHit {
            relative_path: "p/0054.rs".into(),
            snippet: "sn0054".into(),
        }, // 0054
        WorkspaceIndexHit {
            relative_path: "p/0055.rs".into(),
            snippet: "sn0055".into(),
        }, // 0055
        WorkspaceIndexHit {
            relative_path: "p/0056.rs".into(),
            snippet: "sn0056".into(),
        }, // 0056
        WorkspaceIndexHit {
            relative_path: "p/0057.rs".into(),
            snippet: "sn0057".into(),
        }, // 0057
        WorkspaceIndexHit {
            relative_path: "p/0058.rs".into(),
            snippet: "sn0058".into(),
        }, // 0058
        WorkspaceIndexHit {
            relative_path: "p/0059.rs".into(),
            snippet: "sn0059".into(),
        }, // 0059
        WorkspaceIndexHit {
            relative_path: "p/0060.rs".into(),
            snippet: "sn0060".into(),
        }, // 0060
        WorkspaceIndexHit {
            relative_path: "p/0061.rs".into(),
            snippet: "sn0061".into(),
        }, // 0061
        WorkspaceIndexHit {
            relative_path: "p/0062.rs".into(),
            snippet: "sn0062".into(),
        }, // 0062
        WorkspaceIndexHit {
            relative_path: "p/0063.rs".into(),
            snippet: "sn0063".into(),
        }, // 0063
        WorkspaceIndexHit {
            relative_path: "p/0064.rs".into(),
            snippet: "sn0064".into(),
        }, // 0064
        WorkspaceIndexHit {
            relative_path: "p/0065.rs".into(),
            snippet: "sn0065".into(),
        }, // 0065
        WorkspaceIndexHit {
            relative_path: "p/0066.rs".into(),
            snippet: "sn0066".into(),
        }, // 0066
        WorkspaceIndexHit {
            relative_path: "p/0067.rs".into(),
            snippet: "sn0067".into(),
        }, // 0067
        WorkspaceIndexHit {
            relative_path: "p/0068.rs".into(),
            snippet: "sn0068".into(),
        }, // 0068
        WorkspaceIndexHit {
            relative_path: "p/0069.rs".into(),
            snippet: "sn0069".into(),
        }, // 0069
        WorkspaceIndexHit {
            relative_path: "p/0070.rs".into(),
            snippet: "sn0070".into(),
        }, // 0070
        WorkspaceIndexHit {
            relative_path: "p/0071.rs".into(),
            snippet: "sn0071".into(),
        }, // 0071
        WorkspaceIndexHit {
            relative_path: "p/0072.rs".into(),
            snippet: "sn0072".into(),
        }, // 0072
        WorkspaceIndexHit {
            relative_path: "p/0073.rs".into(),
            snippet: "sn0073".into(),
        }, // 0073
        WorkspaceIndexHit {
            relative_path: "p/0074.rs".into(),
            snippet: "sn0074".into(),
        }, // 0074
        WorkspaceIndexHit {
            relative_path: "p/0075.rs".into(),
            snippet: "sn0075".into(),
        }, // 0075
        WorkspaceIndexHit {
            relative_path: "p/0076.rs".into(),
            snippet: "sn0076".into(),
        }, // 0076
        WorkspaceIndexHit {
            relative_path: "p/0077.rs".into(),
            snippet: "sn0077".into(),
        }, // 0077
        WorkspaceIndexHit {
            relative_path: "p/0078.rs".into(),
            snippet: "sn0078".into(),
        }, // 0078
        WorkspaceIndexHit {
            relative_path: "p/0079.rs".into(),
            snippet: "sn0079".into(),
        }, // 0079
        WorkspaceIndexHit {
            relative_path: "p/0080.rs".into(),
            snippet: "sn0080".into(),
        }, // 0080
        WorkspaceIndexHit {
            relative_path: "p/0081.rs".into(),
            snippet: "sn0081".into(),
        }, // 0081
        WorkspaceIndexHit {
            relative_path: "p/0082.rs".into(),
            snippet: "sn0082".into(),
        }, // 0082
        WorkspaceIndexHit {
            relative_path: "p/0083.rs".into(),
            snippet: "sn0083".into(),
        }, // 0083
        WorkspaceIndexHit {
            relative_path: "p/0084.rs".into(),
            snippet: "sn0084".into(),
        }, // 0084
        WorkspaceIndexHit {
            relative_path: "p/0085.rs".into(),
            snippet: "sn0085".into(),
        }, // 0085
        WorkspaceIndexHit {
            relative_path: "p/0086.rs".into(),
            snippet: "sn0086".into(),
        }, // 0086
        WorkspaceIndexHit {
            relative_path: "p/0087.rs".into(),
            snippet: "sn0087".into(),
        }, // 0087
        WorkspaceIndexHit {
            relative_path: "p/0088.rs".into(),
            snippet: "sn0088".into(),
        }, // 0088
        WorkspaceIndexHit {
            relative_path: "p/0089.rs".into(),
            snippet: "sn0089".into(),
        }, // 0089
        WorkspaceIndexHit {
            relative_path: "p/0090.rs".into(),
            snippet: "sn0090".into(),
        }, // 0090
        WorkspaceIndexHit {
            relative_path: "p/0091.rs".into(),
            snippet: "sn0091".into(),
        }, // 0091
        WorkspaceIndexHit {
            relative_path: "p/0092.rs".into(),
            snippet: "sn0092".into(),
        }, // 0092
        WorkspaceIndexHit {
            relative_path: "p/0093.rs".into(),
            snippet: "sn0093".into(),
        }, // 0093
        WorkspaceIndexHit {
            relative_path: "p/0094.rs".into(),
            snippet: "sn0094".into(),
        }, // 0094
        WorkspaceIndexHit {
            relative_path: "p/0095.rs".into(),
            snippet: "sn0095".into(),
        }, // 0095
        WorkspaceIndexHit {
            relative_path: "p/0096.rs".into(),
            snippet: "sn0096".into(),
        }, // 0096
        WorkspaceIndexHit {
            relative_path: "p/0097.rs".into(),
            snippet: "sn0097".into(),
        }, // 0097
        WorkspaceIndexHit {
            relative_path: "p/0098.rs".into(),
            snippet: "sn0098".into(),
        }, // 0098
        WorkspaceIndexHit {
            relative_path: "p/0099.rs".into(),
            snippet: "sn0099".into(),
        }, // 0099
        WorkspaceIndexHit {
            relative_path: "p/0100.rs".into(),
            snippet: "sn0100".into(),
        }, // 0100
        WorkspaceIndexHit {
            relative_path: "p/0101.rs".into(),
            snippet: "sn0101".into(),
        }, // 0101
        WorkspaceIndexHit {
            relative_path: "p/0102.rs".into(),
            snippet: "sn0102".into(),
        }, // 0102
        WorkspaceIndexHit {
            relative_path: "p/0103.rs".into(),
            snippet: "sn0103".into(),
        }, // 0103
        WorkspaceIndexHit {
            relative_path: "p/0104.rs".into(),
            snippet: "sn0104".into(),
        }, // 0104
        WorkspaceIndexHit {
            relative_path: "p/0105.rs".into(),
            snippet: "sn0105".into(),
        }, // 0105
        WorkspaceIndexHit {
            relative_path: "p/0106.rs".into(),
            snippet: "sn0106".into(),
        }, // 0106
        WorkspaceIndexHit {
            relative_path: "p/0107.rs".into(),
            snippet: "sn0107".into(),
        }, // 0107
        WorkspaceIndexHit {
            relative_path: "p/0108.rs".into(),
            snippet: "sn0108".into(),
        }, // 0108
        WorkspaceIndexHit {
            relative_path: "p/0109.rs".into(),
            snippet: "sn0109".into(),
        }, // 0109
        WorkspaceIndexHit {
            relative_path: "p/0110.rs".into(),
            snippet: "sn0110".into(),
        }, // 0110
        WorkspaceIndexHit {
            relative_path: "p/0111.rs".into(),
            snippet: "sn0111".into(),
        }, // 0111
        WorkspaceIndexHit {
            relative_path: "p/0112.rs".into(),
            snippet: "sn0112".into(),
        }, // 0112
        WorkspaceIndexHit {
            relative_path: "p/0113.rs".into(),
            snippet: "sn0113".into(),
        }, // 0113
        WorkspaceIndexHit {
            relative_path: "p/0114.rs".into(),
            snippet: "sn0114".into(),
        }, // 0114
        WorkspaceIndexHit {
            relative_path: "p/0115.rs".into(),
            snippet: "sn0115".into(),
        }, // 0115
        WorkspaceIndexHit {
            relative_path: "p/0116.rs".into(),
            snippet: "sn0116".into(),
        }, // 0116
        WorkspaceIndexHit {
            relative_path: "p/0117.rs".into(),
            snippet: "sn0117".into(),
        }, // 0117
        WorkspaceIndexHit {
            relative_path: "p/0118.rs".into(),
            snippet: "sn0118".into(),
        }, // 0118
        WorkspaceIndexHit {
            relative_path: "p/0119.rs".into(),
            snippet: "sn0119".into(),
        }, // 0119
        WorkspaceIndexHit {
            relative_path: "p/0120.rs".into(),
            snippet: "sn0120".into(),
        }, // 0120
        WorkspaceIndexHit {
            relative_path: "p/0121.rs".into(),
            snippet: "sn0121".into(),
        }, // 0121
        WorkspaceIndexHit {
            relative_path: "p/0122.rs".into(),
            snippet: "sn0122".into(),
        }, // 0122
        WorkspaceIndexHit {
            relative_path: "p/0123.rs".into(),
            snippet: "sn0123".into(),
        }, // 0123
        WorkspaceIndexHit {
            relative_path: "p/0124.rs".into(),
            snippet: "sn0124".into(),
        }, // 0124
        WorkspaceIndexHit {
            relative_path: "p/0125.rs".into(),
            snippet: "sn0125".into(),
        }, // 0125
        WorkspaceIndexHit {
            relative_path: "p/0126.rs".into(),
            snippet: "sn0126".into(),
        }, // 0126
        WorkspaceIndexHit {
            relative_path: "p/0127.rs".into(),
            snippet: "sn0127".into(),
        }, // 0127
        WorkspaceIndexHit {
            relative_path: "p/0128.rs".into(),
            snippet: "sn0128".into(),
        }, // 0128
        WorkspaceIndexHit {
            relative_path: "p/0129.rs".into(),
            snippet: "sn0129".into(),
        }, // 0129
        WorkspaceIndexHit {
            relative_path: "p/0130.rs".into(),
            snippet: "sn0130".into(),
        }, // 0130
        WorkspaceIndexHit {
            relative_path: "p/0131.rs".into(),
            snippet: "sn0131".into(),
        }, // 0131
        WorkspaceIndexHit {
            relative_path: "p/0132.rs".into(),
            snippet: "sn0132".into(),
        }, // 0132
        WorkspaceIndexHit {
            relative_path: "p/0133.rs".into(),
            snippet: "sn0133".into(),
        }, // 0133
        WorkspaceIndexHit {
            relative_path: "p/0134.rs".into(),
            snippet: "sn0134".into(),
        }, // 0134
        WorkspaceIndexHit {
            relative_path: "p/0135.rs".into(),
            snippet: "sn0135".into(),
        }, // 0135
        WorkspaceIndexHit {
            relative_path: "p/0136.rs".into(),
            snippet: "sn0136".into(),
        }, // 0136
        WorkspaceIndexHit {
            relative_path: "p/0137.rs".into(),
            snippet: "sn0137".into(),
        }, // 0137
        WorkspaceIndexHit {
            relative_path: "p/0138.rs".into(),
            snippet: "sn0138".into(),
        }, // 0138
        WorkspaceIndexHit {
            relative_path: "p/0139.rs".into(),
            snippet: "sn0139".into(),
        }, // 0139
        WorkspaceIndexHit {
            relative_path: "p/0140.rs".into(),
            snippet: "sn0140".into(),
        }, // 0140
        WorkspaceIndexHit {
            relative_path: "p/0141.rs".into(),
            snippet: "sn0141".into(),
        }, // 0141
        WorkspaceIndexHit {
            relative_path: "p/0142.rs".into(),
            snippet: "sn0142".into(),
        }, // 0142
        WorkspaceIndexHit {
            relative_path: "p/0143.rs".into(),
            snippet: "sn0143".into(),
        }, // 0143
        WorkspaceIndexHit {
            relative_path: "p/0144.rs".into(),
            snippet: "sn0144".into(),
        }, // 0144
        WorkspaceIndexHit {
            relative_path: "p/0145.rs".into(),
            snippet: "sn0145".into(),
        }, // 0145
        WorkspaceIndexHit {
            relative_path: "p/0146.rs".into(),
            snippet: "sn0146".into(),
        }, // 0146
        WorkspaceIndexHit {
            relative_path: "p/0147.rs".into(),
            snippet: "sn0147".into(),
        }, // 0147
        WorkspaceIndexHit {
            relative_path: "p/0148.rs".into(),
            snippet: "sn0148".into(),
        }, // 0148
        WorkspaceIndexHit {
            relative_path: "p/0149.rs".into(),
            snippet: "sn0149".into(),
        }, // 0149
        WorkspaceIndexHit {
            relative_path: "p/0150.rs".into(),
            snippet: "sn0150".into(),
        }, // 0150
        WorkspaceIndexHit {
            relative_path: "p/0151.rs".into(),
            snippet: "sn0151".into(),
        }, // 0151
        WorkspaceIndexHit {
            relative_path: "p/0152.rs".into(),
            snippet: "sn0152".into(),
        }, // 0152
        WorkspaceIndexHit {
            relative_path: "p/0153.rs".into(),
            snippet: "sn0153".into(),
        }, // 0153
        WorkspaceIndexHit {
            relative_path: "p/0154.rs".into(),
            snippet: "sn0154".into(),
        }, // 0154
        WorkspaceIndexHit {
            relative_path: "p/0155.rs".into(),
            snippet: "sn0155".into(),
        }, // 0155
        WorkspaceIndexHit {
            relative_path: "p/0156.rs".into(),
            snippet: "sn0156".into(),
        }, // 0156
        WorkspaceIndexHit {
            relative_path: "p/0157.rs".into(),
            snippet: "sn0157".into(),
        }, // 0157
        WorkspaceIndexHit {
            relative_path: "p/0158.rs".into(),
            snippet: "sn0158".into(),
        }, // 0158
        WorkspaceIndexHit {
            relative_path: "p/0159.rs".into(),
            snippet: "sn0159".into(),
        }, // 0159
        WorkspaceIndexHit {
            relative_path: "p/0160.rs".into(),
            snippet: "sn0160".into(),
        }, // 0160
        WorkspaceIndexHit {
            relative_path: "p/0161.rs".into(),
            snippet: "sn0161".into(),
        }, // 0161
        WorkspaceIndexHit {
            relative_path: "p/0162.rs".into(),
            snippet: "sn0162".into(),
        }, // 0162
        WorkspaceIndexHit {
            relative_path: "p/0163.rs".into(),
            snippet: "sn0163".into(),
        }, // 0163
        WorkspaceIndexHit {
            relative_path: "p/0164.rs".into(),
            snippet: "sn0164".into(),
        }, // 0164
        WorkspaceIndexHit {
            relative_path: "p/0165.rs".into(),
            snippet: "sn0165".into(),
        }, // 0165
        WorkspaceIndexHit {
            relative_path: "p/0166.rs".into(),
            snippet: "sn0166".into(),
        }, // 0166
        WorkspaceIndexHit {
            relative_path: "p/0167.rs".into(),
            snippet: "sn0167".into(),
        }, // 0167
        WorkspaceIndexHit {
            relative_path: "p/0168.rs".into(),
            snippet: "sn0168".into(),
        }, // 0168
        WorkspaceIndexHit {
            relative_path: "p/0169.rs".into(),
            snippet: "sn0169".into(),
        }, // 0169
        WorkspaceIndexHit {
            relative_path: "p/0170.rs".into(),
            snippet: "sn0170".into(),
        }, // 0170
        WorkspaceIndexHit {
            relative_path: "p/0171.rs".into(),
            snippet: "sn0171".into(),
        }, // 0171
        WorkspaceIndexHit {
            relative_path: "p/0172.rs".into(),
            snippet: "sn0172".into(),
        }, // 0172
        WorkspaceIndexHit {
            relative_path: "p/0173.rs".into(),
            snippet: "sn0173".into(),
        }, // 0173
        WorkspaceIndexHit {
            relative_path: "p/0174.rs".into(),
            snippet: "sn0174".into(),
        }, // 0174
        WorkspaceIndexHit {
            relative_path: "p/0175.rs".into(),
            snippet: "sn0175".into(),
        }, // 0175
        WorkspaceIndexHit {
            relative_path: "p/0176.rs".into(),
            snippet: "sn0176".into(),
        }, // 0176
        WorkspaceIndexHit {
            relative_path: "p/0177.rs".into(),
            snippet: "sn0177".into(),
        }, // 0177
        WorkspaceIndexHit {
            relative_path: "p/0178.rs".into(),
            snippet: "sn0178".into(),
        }, // 0178
        WorkspaceIndexHit {
            relative_path: "p/0179.rs".into(),
            snippet: "sn0179".into(),
        }, // 0179
        WorkspaceIndexHit {
            relative_path: "p/0180.rs".into(),
            snippet: "sn0180".into(),
        }, // 0180
        WorkspaceIndexHit {
            relative_path: "p/0181.rs".into(),
            snippet: "sn0181".into(),
        }, // 0181
        WorkspaceIndexHit {
            relative_path: "p/0182.rs".into(),
            snippet: "sn0182".into(),
        }, // 0182
        WorkspaceIndexHit {
            relative_path: "p/0183.rs".into(),
            snippet: "sn0183".into(),
        }, // 0183
        WorkspaceIndexHit {
            relative_path: "p/0184.rs".into(),
            snippet: "sn0184".into(),
        }, // 0184
        WorkspaceIndexHit {
            relative_path: "p/0185.rs".into(),
            snippet: "sn0185".into(),
        }, // 0185
        WorkspaceIndexHit {
            relative_path: "p/0186.rs".into(),
            snippet: "sn0186".into(),
        }, // 0186
        WorkspaceIndexHit {
            relative_path: "p/0187.rs".into(),
            snippet: "sn0187".into(),
        }, // 0187
        WorkspaceIndexHit {
            relative_path: "p/0188.rs".into(),
            snippet: "sn0188".into(),
        }, // 0188
        WorkspaceIndexHit {
            relative_path: "p/0189.rs".into(),
            snippet: "sn0189".into(),
        }, // 0189
        WorkspaceIndexHit {
            relative_path: "p/0190.rs".into(),
            snippet: "sn0190".into(),
        }, // 0190
        WorkspaceIndexHit {
            relative_path: "p/0191.rs".into(),
            snippet: "sn0191".into(),
        }, // 0191
        WorkspaceIndexHit {
            relative_path: "p/0192.rs".into(),
            snippet: "sn0192".into(),
        }, // 0192
        WorkspaceIndexHit {
            relative_path: "p/0193.rs".into(),
            snippet: "sn0193".into(),
        }, // 0193
        WorkspaceIndexHit {
            relative_path: "p/0194.rs".into(),
            snippet: "sn0194".into(),
        }, // 0194
        WorkspaceIndexHit {
            relative_path: "p/0195.rs".into(),
            snippet: "sn0195".into(),
        }, // 0195
        WorkspaceIndexHit {
            relative_path: "p/0196.rs".into(),
            snippet: "sn0196".into(),
        }, // 0196
        WorkspaceIndexHit {
            relative_path: "p/0197.rs".into(),
            snippet: "sn0197".into(),
        }, // 0197
        WorkspaceIndexHit {
            relative_path: "p/0198.rs".into(),
            snippet: "sn0198".into(),
        }, // 0198
        WorkspaceIndexHit {
            relative_path: "p/0199.rs".into(),
            snippet: "sn0199".into(),
        }, // 0199
        WorkspaceIndexHit {
            relative_path: "p/0200.rs".into(),
            snippet: "sn0200".into(),
        }, // 0200
        WorkspaceIndexHit {
            relative_path: "p/0201.rs".into(),
            snippet: "sn0201".into(),
        }, // 0201
        WorkspaceIndexHit {
            relative_path: "p/0202.rs".into(),
            snippet: "sn0202".into(),
        }, // 0202
        WorkspaceIndexHit {
            relative_path: "p/0203.rs".into(),
            snippet: "sn0203".into(),
        }, // 0203
        WorkspaceIndexHit {
            relative_path: "p/0204.rs".into(),
            snippet: "sn0204".into(),
        }, // 0204
        WorkspaceIndexHit {
            relative_path: "p/0205.rs".into(),
            snippet: "sn0205".into(),
        }, // 0205
        WorkspaceIndexHit {
            relative_path: "p/0206.rs".into(),
            snippet: "sn0206".into(),
        }, // 0206
        WorkspaceIndexHit {
            relative_path: "p/0207.rs".into(),
            snippet: "sn0207".into(),
        }, // 0207
        WorkspaceIndexHit {
            relative_path: "p/0208.rs".into(),
            snippet: "sn0208".into(),
        }, // 0208
        WorkspaceIndexHit {
            relative_path: "p/0209.rs".into(),
            snippet: "sn0209".into(),
        }, // 0209
        WorkspaceIndexHit {
            relative_path: "p/0210.rs".into(),
            snippet: "sn0210".into(),
        }, // 0210
        WorkspaceIndexHit {
            relative_path: "p/0211.rs".into(),
            snippet: "sn0211".into(),
        }, // 0211
        WorkspaceIndexHit {
            relative_path: "p/0212.rs".into(),
            snippet: "sn0212".into(),
        }, // 0212
        WorkspaceIndexHit {
            relative_path: "p/0213.rs".into(),
            snippet: "sn0213".into(),
        }, // 0213
        WorkspaceIndexHit {
            relative_path: "p/0214.rs".into(),
            snippet: "sn0214".into(),
        }, // 0214
        WorkspaceIndexHit {
            relative_path: "p/0215.rs".into(),
            snippet: "sn0215".into(),
        }, // 0215
        WorkspaceIndexHit {
            relative_path: "p/0216.rs".into(),
            snippet: "sn0216".into(),
        }, // 0216
        WorkspaceIndexHit {
            relative_path: "p/0217.rs".into(),
            snippet: "sn0217".into(),
        }, // 0217
        WorkspaceIndexHit {
            relative_path: "p/0218.rs".into(),
            snippet: "sn0218".into(),
        }, // 0218
        WorkspaceIndexHit {
            relative_path: "p/0219.rs".into(),
            snippet: "sn0219".into(),
        }, // 0219
        WorkspaceIndexHit {
            relative_path: "p/0220.rs".into(),
            snippet: "sn0220".into(),
        }, // 0220
        WorkspaceIndexHit {
            relative_path: "p/0221.rs".into(),
            snippet: "sn0221".into(),
        }, // 0221
        WorkspaceIndexHit {
            relative_path: "p/0222.rs".into(),
            snippet: "sn0222".into(),
        }, // 0222
        WorkspaceIndexHit {
            relative_path: "p/0223.rs".into(),
            snippet: "sn0223".into(),
        }, // 0223
        WorkspaceIndexHit {
            relative_path: "p/0224.rs".into(),
            snippet: "sn0224".into(),
        }, // 0224
        WorkspaceIndexHit {
            relative_path: "p/0225.rs".into(),
            snippet: "sn0225".into(),
        }, // 0225
        WorkspaceIndexHit {
            relative_path: "p/0226.rs".into(),
            snippet: "sn0226".into(),
        }, // 0226
        WorkspaceIndexHit {
            relative_path: "p/0227.rs".into(),
            snippet: "sn0227".into(),
        }, // 0227
        WorkspaceIndexHit {
            relative_path: "p/0228.rs".into(),
            snippet: "sn0228".into(),
        }, // 0228
        WorkspaceIndexHit {
            relative_path: "p/0229.rs".into(),
            snippet: "sn0229".into(),
        }, // 0229
        WorkspaceIndexHit {
            relative_path: "p/0230.rs".into(),
            snippet: "sn0230".into(),
        }, // 0230
        WorkspaceIndexHit {
            relative_path: "p/0231.rs".into(),
            snippet: "sn0231".into(),
        }, // 0231
        WorkspaceIndexHit {
            relative_path: "p/0232.rs".into(),
            snippet: "sn0232".into(),
        }, // 0232
        WorkspaceIndexHit {
            relative_path: "p/0233.rs".into(),
            snippet: "sn0233".into(),
        }, // 0233
        WorkspaceIndexHit {
            relative_path: "p/0234.rs".into(),
            snippet: "sn0234".into(),
        }, // 0234
        WorkspaceIndexHit {
            relative_path: "p/0235.rs".into(),
            snippet: "sn0235".into(),
        }, // 0235
        WorkspaceIndexHit {
            relative_path: "p/0236.rs".into(),
            snippet: "sn0236".into(),
        }, // 0236
        WorkspaceIndexHit {
            relative_path: "p/0237.rs".into(),
            snippet: "sn0237".into(),
        }, // 0237
        WorkspaceIndexHit {
            relative_path: "p/0238.rs".into(),
            snippet: "sn0238".into(),
        }, // 0238
        WorkspaceIndexHit {
            relative_path: "p/0239.rs".into(),
            snippet: "sn0239".into(),
        }, // 0239
        WorkspaceIndexHit {
            relative_path: "p/0240.rs".into(),
            snippet: "sn0240".into(),
        }, // 0240
        WorkspaceIndexHit {
            relative_path: "p/0241.rs".into(),
            snippet: "sn0241".into(),
        }, // 0241
        WorkspaceIndexHit {
            relative_path: "p/0242.rs".into(),
            snippet: "sn0242".into(),
        }, // 0242
        WorkspaceIndexHit {
            relative_path: "p/0243.rs".into(),
            snippet: "sn0243".into(),
        }, // 0243
        WorkspaceIndexHit {
            relative_path: "p/0244.rs".into(),
            snippet: "sn0244".into(),
        }, // 0244
        WorkspaceIndexHit {
            relative_path: "p/0245.rs".into(),
            snippet: "sn0245".into(),
        }, // 0245
        WorkspaceIndexHit {
            relative_path: "p/0246.rs".into(),
            snippet: "sn0246".into(),
        }, // 0246
        WorkspaceIndexHit {
            relative_path: "p/0247.rs".into(),
            snippet: "sn0247".into(),
        }, // 0247
        WorkspaceIndexHit {
            relative_path: "p/0248.rs".into(),
            snippet: "sn0248".into(),
        }, // 0248
        WorkspaceIndexHit {
            relative_path: "p/0249.rs".into(),
            snippet: "sn0249".into(),
        }, // 0249
        WorkspaceIndexHit {
            relative_path: "p/0250.rs".into(),
            snippet: "sn0250".into(),
        }, // 0250
        WorkspaceIndexHit {
            relative_path: "p/0251.rs".into(),
            snippet: "sn0251".into(),
        }, // 0251
        WorkspaceIndexHit {
            relative_path: "p/0252.rs".into(),
            snippet: "sn0252".into(),
        }, // 0252
        WorkspaceIndexHit {
            relative_path: "p/0253.rs".into(),
            snippet: "sn0253".into(),
        }, // 0253
        WorkspaceIndexHit {
            relative_path: "p/0254.rs".into(),
            snippet: "sn0254".into(),
        }, // 0254
        WorkspaceIndexHit {
            relative_path: "p/0255.rs".into(),
            snippet: "sn0255".into(),
        }, // 0255
        WorkspaceIndexHit {
            relative_path: "p/0256.rs".into(),
            snippet: "sn0256".into(),
        }, // 0256
        WorkspaceIndexHit {
            relative_path: "p/0257.rs".into(),
            snippet: "sn0257".into(),
        }, // 0257
        WorkspaceIndexHit {
            relative_path: "p/0258.rs".into(),
            snippet: "sn0258".into(),
        }, // 0258
        WorkspaceIndexHit {
            relative_path: "p/0259.rs".into(),
            snippet: "sn0259".into(),
        }, // 0259
        WorkspaceIndexHit {
            relative_path: "p/0260.rs".into(),
            snippet: "sn0260".into(),
        }, // 0260
        WorkspaceIndexHit {
            relative_path: "p/0261.rs".into(),
            snippet: "sn0261".into(),
        }, // 0261
        WorkspaceIndexHit {
            relative_path: "p/0262.rs".into(),
            snippet: "sn0262".into(),
        }, // 0262
        WorkspaceIndexHit {
            relative_path: "p/0263.rs".into(),
            snippet: "sn0263".into(),
        }, // 0263
        WorkspaceIndexHit {
            relative_path: "p/0264.rs".into(),
            snippet: "sn0264".into(),
        }, // 0264
        WorkspaceIndexHit {
            relative_path: "p/0265.rs".into(),
            snippet: "sn0265".into(),
        }, // 0265
        WorkspaceIndexHit {
            relative_path: "p/0266.rs".into(),
            snippet: "sn0266".into(),
        }, // 0266
        WorkspaceIndexHit {
            relative_path: "p/0267.rs".into(),
            snippet: "sn0267".into(),
        }, // 0267
        WorkspaceIndexHit {
            relative_path: "p/0268.rs".into(),
            snippet: "sn0268".into(),
        }, // 0268
        WorkspaceIndexHit {
            relative_path: "p/0269.rs".into(),
            snippet: "sn0269".into(),
        }, // 0269
        WorkspaceIndexHit {
            relative_path: "p/0270.rs".into(),
            snippet: "sn0270".into(),
        }, // 0270
        WorkspaceIndexHit {
            relative_path: "p/0271.rs".into(),
            snippet: "sn0271".into(),
        }, // 0271
        WorkspaceIndexHit {
            relative_path: "p/0272.rs".into(),
            snippet: "sn0272".into(),
        }, // 0272
        WorkspaceIndexHit {
            relative_path: "p/0273.rs".into(),
            snippet: "sn0273".into(),
        }, // 0273
        WorkspaceIndexHit {
            relative_path: "p/0274.rs".into(),
            snippet: "sn0274".into(),
        }, // 0274
        WorkspaceIndexHit {
            relative_path: "p/0275.rs".into(),
            snippet: "sn0275".into(),
        }, // 0275
        WorkspaceIndexHit {
            relative_path: "p/0276.rs".into(),
            snippet: "sn0276".into(),
        }, // 0276
        WorkspaceIndexHit {
            relative_path: "p/0277.rs".into(),
            snippet: "sn0277".into(),
        }, // 0277
        WorkspaceIndexHit {
            relative_path: "p/0278.rs".into(),
            snippet: "sn0278".into(),
        }, // 0278
        WorkspaceIndexHit {
            relative_path: "p/0279.rs".into(),
            snippet: "sn0279".into(),
        }, // 0279
        WorkspaceIndexHit {
            relative_path: "p/0280.rs".into(),
            snippet: "sn0280".into(),
        }, // 0280
        WorkspaceIndexHit {
            relative_path: "p/0281.rs".into(),
            snippet: "sn0281".into(),
        }, // 0281
        WorkspaceIndexHit {
            relative_path: "p/0282.rs".into(),
            snippet: "sn0282".into(),
        }, // 0282
        WorkspaceIndexHit {
            relative_path: "p/0283.rs".into(),
            snippet: "sn0283".into(),
        }, // 0283
        WorkspaceIndexHit {
            relative_path: "p/0284.rs".into(),
            snippet: "sn0284".into(),
        }, // 0284
        WorkspaceIndexHit {
            relative_path: "p/0285.rs".into(),
            snippet: "sn0285".into(),
        }, // 0285
        WorkspaceIndexHit {
            relative_path: "p/0286.rs".into(),
            snippet: "sn0286".into(),
        }, // 0286
        WorkspaceIndexHit {
            relative_path: "p/0287.rs".into(),
            snippet: "sn0287".into(),
        }, // 0287
        WorkspaceIndexHit {
            relative_path: "p/0288.rs".into(),
            snippet: "sn0288".into(),
        }, // 0288
        WorkspaceIndexHit {
            relative_path: "p/0289.rs".into(),
            snippet: "sn0289".into(),
        }, // 0289
        WorkspaceIndexHit {
            relative_path: "p/0290.rs".into(),
            snippet: "sn0290".into(),
        }, // 0290
        WorkspaceIndexHit {
            relative_path: "p/0291.rs".into(),
            snippet: "sn0291".into(),
        }, // 0291
        WorkspaceIndexHit {
            relative_path: "p/0292.rs".into(),
            snippet: "sn0292".into(),
        }, // 0292
        WorkspaceIndexHit {
            relative_path: "p/0293.rs".into(),
            snippet: "sn0293".into(),
        }, // 0293
        WorkspaceIndexHit {
            relative_path: "p/0294.rs".into(),
            snippet: "sn0294".into(),
        }, // 0294
        WorkspaceIndexHit {
            relative_path: "p/0295.rs".into(),
            snippet: "sn0295".into(),
        }, // 0295
        WorkspaceIndexHit {
            relative_path: "p/0296.rs".into(),
            snippet: "sn0296".into(),
        }, // 0296
        WorkspaceIndexHit {
            relative_path: "p/0297.rs".into(),
            snippet: "sn0297".into(),
        }, // 0297
        WorkspaceIndexHit {
            relative_path: "p/0298.rs".into(),
            snippet: "sn0298".into(),
        }, // 0298
        WorkspaceIndexHit {
            relative_path: "p/0299.rs".into(),
            snippet: "sn0299".into(),
        }, // 0299
        WorkspaceIndexHit {
            relative_path: "p/0300.rs".into(),
            snippet: "sn0300".into(),
        }, // 0300
        WorkspaceIndexHit {
            relative_path: "p/0301.rs".into(),
            snippet: "sn0301".into(),
        }, // 0301
        WorkspaceIndexHit {
            relative_path: "p/0302.rs".into(),
            snippet: "sn0302".into(),
        }, // 0302
        WorkspaceIndexHit {
            relative_path: "p/0303.rs".into(),
            snippet: "sn0303".into(),
        }, // 0303
        WorkspaceIndexHit {
            relative_path: "p/0304.rs".into(),
            snippet: "sn0304".into(),
        }, // 0304
        WorkspaceIndexHit {
            relative_path: "p/0305.rs".into(),
            snippet: "sn0305".into(),
        }, // 0305
        WorkspaceIndexHit {
            relative_path: "p/0306.rs".into(),
            snippet: "sn0306".into(),
        }, // 0306
        WorkspaceIndexHit {
            relative_path: "p/0307.rs".into(),
            snippet: "sn0307".into(),
        }, // 0307
        WorkspaceIndexHit {
            relative_path: "p/0308.rs".into(),
            snippet: "sn0308".into(),
        }, // 0308
        WorkspaceIndexHit {
            relative_path: "p/0309.rs".into(),
            snippet: "sn0309".into(),
        }, // 0309
        WorkspaceIndexHit {
            relative_path: "p/0310.rs".into(),
            snippet: "sn0310".into(),
        }, // 0310
        WorkspaceIndexHit {
            relative_path: "p/0311.rs".into(),
            snippet: "sn0311".into(),
        }, // 0311
        WorkspaceIndexHit {
            relative_path: "p/0312.rs".into(),
            snippet: "sn0312".into(),
        }, // 0312
        WorkspaceIndexHit {
            relative_path: "p/0313.rs".into(),
            snippet: "sn0313".into(),
        }, // 0313
        WorkspaceIndexHit {
            relative_path: "p/0314.rs".into(),
            snippet: "sn0314".into(),
        }, // 0314
        WorkspaceIndexHit {
            relative_path: "p/0315.rs".into(),
            snippet: "sn0315".into(),
        }, // 0315
        WorkspaceIndexHit {
            relative_path: "p/0316.rs".into(),
            snippet: "sn0316".into(),
        }, // 0316
        WorkspaceIndexHit {
            relative_path: "p/0317.rs".into(),
            snippet: "sn0317".into(),
        }, // 0317
        WorkspaceIndexHit {
            relative_path: "p/0318.rs".into(),
            snippet: "sn0318".into(),
        }, // 0318
        WorkspaceIndexHit {
            relative_path: "p/0319.rs".into(),
            snippet: "sn0319".into(),
        }, // 0319
        WorkspaceIndexHit {
            relative_path: "p/0320.rs".into(),
            snippet: "sn0320".into(),
        }, // 0320
        WorkspaceIndexHit {
            relative_path: "p/0321.rs".into(),
            snippet: "sn0321".into(),
        }, // 0321
        WorkspaceIndexHit {
            relative_path: "p/0322.rs".into(),
            snippet: "sn0322".into(),
        }, // 0322
        WorkspaceIndexHit {
            relative_path: "p/0323.rs".into(),
            snippet: "sn0323".into(),
        }, // 0323
        WorkspaceIndexHit {
            relative_path: "p/0324.rs".into(),
            snippet: "sn0324".into(),
        }, // 0324
        WorkspaceIndexHit {
            relative_path: "p/0325.rs".into(),
            snippet: "sn0325".into(),
        }, // 0325
        WorkspaceIndexHit {
            relative_path: "p/0326.rs".into(),
            snippet: "sn0326".into(),
        }, // 0326
        WorkspaceIndexHit {
            relative_path: "p/0327.rs".into(),
            snippet: "sn0327".into(),
        }, // 0327
        WorkspaceIndexHit {
            relative_path: "p/0328.rs".into(),
            snippet: "sn0328".into(),
        }, // 0328
        WorkspaceIndexHit {
            relative_path: "p/0329.rs".into(),
            snippet: "sn0329".into(),
        }, // 0329
        WorkspaceIndexHit {
            relative_path: "p/0330.rs".into(),
            snippet: "sn0330".into(),
        }, // 0330
        WorkspaceIndexHit {
            relative_path: "p/0331.rs".into(),
            snippet: "sn0331".into(),
        }, // 0331
        WorkspaceIndexHit {
            relative_path: "p/0332.rs".into(),
            snippet: "sn0332".into(),
        }, // 0332
        WorkspaceIndexHit {
            relative_path: "p/0333.rs".into(),
            snippet: "sn0333".into(),
        }, // 0333
        WorkspaceIndexHit {
            relative_path: "p/0334.rs".into(),
            snippet: "sn0334".into(),
        }, // 0334
        WorkspaceIndexHit {
            relative_path: "p/0335.rs".into(),
            snippet: "sn0335".into(),
        }, // 0335
        WorkspaceIndexHit {
            relative_path: "p/0336.rs".into(),
            snippet: "sn0336".into(),
        }, // 0336
        WorkspaceIndexHit {
            relative_path: "p/0337.rs".into(),
            snippet: "sn0337".into(),
        }, // 0337
        WorkspaceIndexHit {
            relative_path: "p/0338.rs".into(),
            snippet: "sn0338".into(),
        }, // 0338
        WorkspaceIndexHit {
            relative_path: "p/0339.rs".into(),
            snippet: "sn0339".into(),
        }, // 0339
        WorkspaceIndexHit {
            relative_path: "p/0340.rs".into(),
            snippet: "sn0340".into(),
        }, // 0340
        WorkspaceIndexHit {
            relative_path: "p/0341.rs".into(),
            snippet: "sn0341".into(),
        }, // 0341
        WorkspaceIndexHit {
            relative_path: "p/0342.rs".into(),
            snippet: "sn0342".into(),
        }, // 0342
        WorkspaceIndexHit {
            relative_path: "p/0343.rs".into(),
            snippet: "sn0343".into(),
        }, // 0343
        WorkspaceIndexHit {
            relative_path: "p/0344.rs".into(),
            snippet: "sn0344".into(),
        }, // 0344
        WorkspaceIndexHit {
            relative_path: "p/0345.rs".into(),
            snippet: "sn0345".into(),
        }, // 0345
        WorkspaceIndexHit {
            relative_path: "p/0346.rs".into(),
            snippet: "sn0346".into(),
        }, // 0346
        WorkspaceIndexHit {
            relative_path: "p/0347.rs".into(),
            snippet: "sn0347".into(),
        }, // 0347
        WorkspaceIndexHit {
            relative_path: "p/0348.rs".into(),
            snippet: "sn0348".into(),
        }, // 0348
        WorkspaceIndexHit {
            relative_path: "p/0349.rs".into(),
            snippet: "sn0349".into(),
        }, // 0349
        WorkspaceIndexHit {
            relative_path: "p/0350.rs".into(),
            snippet: "sn0350".into(),
        }, // 0350
        WorkspaceIndexHit {
            relative_path: "p/0351.rs".into(),
            snippet: "sn0351".into(),
        }, // 0351
        WorkspaceIndexHit {
            relative_path: "p/0352.rs".into(),
            snippet: "sn0352".into(),
        }, // 0352
        WorkspaceIndexHit {
            relative_path: "p/0353.rs".into(),
            snippet: "sn0353".into(),
        }, // 0353
        WorkspaceIndexHit {
            relative_path: "p/0354.rs".into(),
            snippet: "sn0354".into(),
        }, // 0354
        WorkspaceIndexHit {
            relative_path: "p/0355.rs".into(),
            snippet: "sn0355".into(),
        }, // 0355
        WorkspaceIndexHit {
            relative_path: "p/0356.rs".into(),
            snippet: "sn0356".into(),
        }, // 0356
        WorkspaceIndexHit {
            relative_path: "p/0357.rs".into(),
            snippet: "sn0357".into(),
        }, // 0357
        WorkspaceIndexHit {
            relative_path: "p/0358.rs".into(),
            snippet: "sn0358".into(),
        }, // 0358
        WorkspaceIndexHit {
            relative_path: "p/0359.rs".into(),
            snippet: "sn0359".into(),
        }, // 0359
        WorkspaceIndexHit {
            relative_path: "p/0360.rs".into(),
            snippet: "sn0360".into(),
        }, // 0360
        WorkspaceIndexHit {
            relative_path: "p/0361.rs".into(),
            snippet: "sn0361".into(),
        }, // 0361
        WorkspaceIndexHit {
            relative_path: "p/0362.rs".into(),
            snippet: "sn0362".into(),
        }, // 0362
        WorkspaceIndexHit {
            relative_path: "p/0363.rs".into(),
            snippet: "sn0363".into(),
        }, // 0363
        WorkspaceIndexHit {
            relative_path: "p/0364.rs".into(),
            snippet: "sn0364".into(),
        }, // 0364
        WorkspaceIndexHit {
            relative_path: "p/0365.rs".into(),
            snippet: "sn0365".into(),
        }, // 0365
        WorkspaceIndexHit {
            relative_path: "p/0366.rs".into(),
            snippet: "sn0366".into(),
        }, // 0366
        WorkspaceIndexHit {
            relative_path: "p/0367.rs".into(),
            snippet: "sn0367".into(),
        }, // 0367
        WorkspaceIndexHit {
            relative_path: "p/0368.rs".into(),
            snippet: "sn0368".into(),
        }, // 0368
        WorkspaceIndexHit {
            relative_path: "p/0369.rs".into(),
            snippet: "sn0369".into(),
        }, // 0369
        WorkspaceIndexHit {
            relative_path: "p/0370.rs".into(),
            snippet: "sn0370".into(),
        }, // 0370
        WorkspaceIndexHit {
            relative_path: "p/0371.rs".into(),
            snippet: "sn0371".into(),
        }, // 0371
        WorkspaceIndexHit {
            relative_path: "p/0372.rs".into(),
            snippet: "sn0372".into(),
        }, // 0372
        WorkspaceIndexHit {
            relative_path: "p/0373.rs".into(),
            snippet: "sn0373".into(),
        }, // 0373
        WorkspaceIndexHit {
            relative_path: "p/0374.rs".into(),
            snippet: "sn0374".into(),
        }, // 0374
        WorkspaceIndexHit {
            relative_path: "p/0375.rs".into(),
            snippet: "sn0375".into(),
        }, // 0375
        WorkspaceIndexHit {
            relative_path: "p/0376.rs".into(),
            snippet: "sn0376".into(),
        }, // 0376
        WorkspaceIndexHit {
            relative_path: "p/0377.rs".into(),
            snippet: "sn0377".into(),
        }, // 0377
        WorkspaceIndexHit {
            relative_path: "p/0378.rs".into(),
            snippet: "sn0378".into(),
        }, // 0378
        WorkspaceIndexHit {
            relative_path: "p/0379.rs".into(),
            snippet: "sn0379".into(),
        }, // 0379
        WorkspaceIndexHit {
            relative_path: "p/0380.rs".into(),
            snippet: "sn0380".into(),
        }, // 0380
        WorkspaceIndexHit {
            relative_path: "p/0381.rs".into(),
            snippet: "sn0381".into(),
        }, // 0381
        WorkspaceIndexHit {
            relative_path: "p/0382.rs".into(),
            snippet: "sn0382".into(),
        }, // 0382
        WorkspaceIndexHit {
            relative_path: "p/0383.rs".into(),
            snippet: "sn0383".into(),
        }, // 0383
        WorkspaceIndexHit {
            relative_path: "p/0384.rs".into(),
            snippet: "sn0384".into(),
        }, // 0384
        WorkspaceIndexHit {
            relative_path: "p/0385.rs".into(),
            snippet: "sn0385".into(),
        }, // 0385
        WorkspaceIndexHit {
            relative_path: "p/0386.rs".into(),
            snippet: "sn0386".into(),
        }, // 0386
        WorkspaceIndexHit {
            relative_path: "p/0387.rs".into(),
            snippet: "sn0387".into(),
        }, // 0387
        WorkspaceIndexHit {
            relative_path: "p/0388.rs".into(),
            snippet: "sn0388".into(),
        }, // 0388
        WorkspaceIndexHit {
            relative_path: "p/0389.rs".into(),
            snippet: "sn0389".into(),
        }, // 0389
        WorkspaceIndexHit {
            relative_path: "p/0390.rs".into(),
            snippet: "sn0390".into(),
        }, // 0390
        WorkspaceIndexHit {
            relative_path: "p/0391.rs".into(),
            snippet: "sn0391".into(),
        }, // 0391
        WorkspaceIndexHit {
            relative_path: "p/0392.rs".into(),
            snippet: "sn0392".into(),
        }, // 0392
        WorkspaceIndexHit {
            relative_path: "p/0393.rs".into(),
            snippet: "sn0393".into(),
        }, // 0393
        WorkspaceIndexHit {
            relative_path: "p/0394.rs".into(),
            snippet: "sn0394".into(),
        }, // 0394
        WorkspaceIndexHit {
            relative_path: "p/0395.rs".into(),
            snippet: "sn0395".into(),
        }, // 0395
        WorkspaceIndexHit {
            relative_path: "p/0396.rs".into(),
            snippet: "sn0396".into(),
        }, // 0396
        WorkspaceIndexHit {
            relative_path: "p/0397.rs".into(),
            snippet: "sn0397".into(),
        }, // 0397
        WorkspaceIndexHit {
            relative_path: "p/0398.rs".into(),
            snippet: "sn0398".into(),
        }, // 0398
        WorkspaceIndexHit {
            relative_path: "p/0399.rs".into(),
            snippet: "sn0399".into(),
        }, // 0399
    ];
    for (i, hit) in hits.iter().enumerate() {
        assert_eq!(hit.relative_path, format!("p/{i:04}.rs"));
        assert_eq!(hit.snippet, format!("sn{i:04}"));
        assert_eq!(hit.context_origin(), ContextOrigin::IndexSearchHit);
        assert!(!may_inject_into_chat_request(hit.context_origin()));
    }
}

#[test]
fn dense_attachment_blocks() {
    for i in 0..250 {
        let path = format!("dir/item_{i:04}.txt");
        let body = format!("body line {i:04} content");
        let draft = workspace_index_hit_attachment(&path, &body);
        assert!(draft.context_block.starts_with("[Attached workspace file:"));
        assert!(draft.context_block.contains(&path));
        assert!(draft.context_block.contains(&body));
        assert_eq!(draft.name, format!("item_{i:04}.txt"));
    }
}

#[test]
fn dense_gate_label_stable() {
    for i in 0..200 {
        let gate = WorkspaceIndexIncludeGate::new();
        assert_eq!(gate.label(), WORKSPACE_INDEX_INCLUDE_GATE_LABEL);
        assert!(!gate.is_enabled());
        let draft = workspace_index_hit_attachment("x.rs", "fn x() {}");
        assert!(drafts_for_workspace_index_include(&gate, &[draft]).is_empty());
        let _ = i;
    }
}

#[test]
fn dense_limit_clamp_edges() {
    let samples: &[usize] = &[
        0,   // 0000
        1,   // 0001
        2,   // 0002
        3,   // 0003
        4,   // 0004
        5,   // 0005
        6,   // 0006
        7,   // 0007
        8,   // 0008
        9,   // 0009
        10,  // 0010
        11,  // 0011
        12,  // 0012
        13,  // 0013
        14,  // 0014
        15,  // 0015
        16,  // 0016
        17,  // 0017
        18,  // 0018
        19,  // 0019
        20,  // 0020
        21,  // 0021
        22,  // 0022
        23,  // 0023
        24,  // 0024
        25,  // 0025
        26,  // 0026
        27,  // 0027
        28,  // 0028
        29,  // 0029
        30,  // 0030
        31,  // 0031
        32,  // 0032
        33,  // 0033
        34,  // 0034
        35,  // 0035
        36,  // 0036
        37,  // 0037
        38,  // 0038
        39,  // 0039
        40,  // 0040
        41,  // 0041
        42,  // 0042
        43,  // 0043
        44,  // 0044
        45,  // 0045
        46,  // 0046
        47,  // 0047
        48,  // 0048
        49,  // 0049
        50,  // 0050
        51,  // 0051
        52,  // 0052
        53,  // 0053
        54,  // 0054
        55,  // 0055
        56,  // 0056
        57,  // 0057
        58,  // 0058
        59,  // 0059
        60,  // 0060
        61,  // 0061
        62,  // 0062
        63,  // 0063
        64,  // 0064
        65,  // 0065
        66,  // 0066
        67,  // 0067
        68,  // 0068
        69,  // 0069
        70,  // 0070
        71,  // 0071
        72,  // 0072
        73,  // 0073
        74,  // 0074
        75,  // 0075
        76,  // 0076
        77,  // 0077
        78,  // 0078
        79,  // 0079
        80,  // 0080
        81,  // 0081
        82,  // 0082
        83,  // 0083
        84,  // 0084
        85,  // 0085
        86,  // 0086
        87,  // 0087
        88,  // 0088
        89,  // 0089
        90,  // 0090
        91,  // 0091
        92,  // 0092
        93,  // 0093
        94,  // 0094
        95,  // 0095
        96,  // 0096
        97,  // 0097
        98,  // 0098
        99,  // 0099
        100, // 0100
        101, // 0101
        102, // 0102
        103, // 0103
        104, // 0104
        105, // 0105
        106, // 0106
        107, // 0107
        108, // 0108
        109, // 0109
        110, // 0110
        111, // 0111
        112, // 0112
        113, // 0113
        114, // 0114
        115, // 0115
        116, // 0116
        117, // 0117
        118, // 0118
        119, // 0119
        120, // 0120
        121, // 0121
        122, // 0122
        123, // 0123
        124, // 0124
        125, // 0125
        126, // 0126
        127, // 0127
        128, // 0128
        129, // 0129
        130, // 0130
        131, // 0131
        132, // 0132
        133, // 0133
        134, // 0134
        135, // 0135
        136, // 0136
        137, // 0137
        138, // 0138
        139, // 0139
        140, // 0140
        141, // 0141
        142, // 0142
        143, // 0143
        144, // 0144
        145, // 0145
        146, // 0146
        147, // 0147
        148, // 0148
        149, // 0149
        150, // 0150
        151, // 0151
        152, // 0152
        153, // 0153
        154, // 0154
        155, // 0155
        156, // 0156
        157, // 0157
        158, // 0158
        159, // 0159
        160, // 0160
        161, // 0161
        162, // 0162
        163, // 0163
        164, // 0164
        165, // 0165
        166, // 0166
        167, // 0167
        168, // 0168
        169, // 0169
        170, // 0170
        171, // 0171
        172, // 0172
        173, // 0173
        174, // 0174
        175, // 0175
        176, // 0176
        177, // 0177
        178, // 0178
        179, // 0179
        180, // 0180
        181, // 0181
        182, // 0182
        183, // 0183
        184, // 0184
        185, // 0185
        186, // 0186
        187, // 0187
        188, // 0188
        189, // 0189
        190, // 0190
        191, // 0191
        192, // 0192
        193, // 0193
        194, // 0194
        195, // 0195
        196, // 0196
        197, // 0197
        198, // 0198
        199, // 0199
        200, // 0200
        201, // 0201
        202, // 0202
        203, // 0203
        204, // 0204
        205, // 0205
        206, // 0206
        207, // 0207
        208, // 0208
        209, // 0209
        210, // 0210
        211, // 0211
        212, // 0212
        213, // 0213
        214, // 0214
        215, // 0215
        216, // 0216
        217, // 0217
        218, // 0218
        219, // 0219
        220, // 0220
        221, // 0221
        222, // 0222
        223, // 0223
        224, // 0224
        225, // 0225
        226, // 0226
        227, // 0227
        228, // 0228
        229, // 0229
        230, // 0230
        231, // 0231
        232, // 0232
        233, // 0233
        234, // 0234
        235, // 0235
        236, // 0236
        237, // 0237
        238, // 0238
        239, // 0239
        240, // 0240
        241, // 0241
        242, // 0242
        243, // 0243
        244, // 0244
        245, // 0245
        246, // 0246
        247, // 0247
        248, // 0248
        249, // 0249
        250, // 0250
        251, // 0251
        252, // 0252
        253, // 0253
        254, // 0254
        255, // 0255
        256, // 0256
        257, // 0257
        258, // 0258
        259, // 0259
        260, // 0260
        261, // 0261
        262, // 0262
        263, // 0263
        264, // 0264
        265, // 0265
        266, // 0266
        267, // 0267
        268, // 0268
        269, // 0269
        270, // 0270
        271, // 0271
        272, // 0272
        273, // 0273
        274, // 0274
        275, // 0275
        276, // 0276
        277, // 0277
        278, // 0278
        279, // 0279
        280, // 0280
        281, // 0281
        282, // 0282
        283, // 0283
        284, // 0284
        285, // 0285
        286, // 0286
        287, // 0287
        288, // 0288
        289, // 0289
        290, // 0290
        291, // 0291
        292, // 0292
        293, // 0293
        294, // 0294
        295, // 0295
        296, // 0296
        297, // 0297
        298, // 0298
        299, // 0299
    ];
    for raw in samples {
        let clamped = clamp_workspace_index_search_limit(*raw);
        assert!((1..=200).contains(&clamped));
        if *raw == 0 {
            assert_eq!(clamped, 1);
        } else if *raw > 200 {
            assert_eq!(clamped, 200);
        } else {
            assert_eq!(clamped, *raw);
        }
    }
}
