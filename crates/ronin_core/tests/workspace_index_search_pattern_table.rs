//! Pattern tables for lexical search attach gate (#74).

use ronin_core::{
    drafts_for_workspace_index_include, may_inject_into_chat_request,
    workspace_index_hit_attachment, ContextOrigin, WorkspaceIndexHitSelection,
    WorkspaceIndexIncludeGate, WORKSPACE_INDEX_INCLUDE_GATE_LABEL,
};

#[test]
fn pattern_gate_label_constant_table() {
    let labels: &[&str] = &[
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0000
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0001
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0002
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0003
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0004
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0005
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0006
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0007
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0008
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0009
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0010
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0011
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0012
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0013
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0014
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0015
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0016
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0017
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0018
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0019
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0020
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0021
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0022
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0023
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0024
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0025
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0026
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0027
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0028
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0029
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0030
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0031
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0032
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0033
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0034
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0035
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0036
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0037
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0038
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0039
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0040
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0041
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0042
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0043
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0044
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0045
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0046
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0047
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0048
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0049
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0050
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0051
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0052
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0053
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0054
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0055
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0056
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0057
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0058
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0059
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0060
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0061
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0062
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0063
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0064
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0065
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0066
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0067
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0068
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0069
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0070
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0071
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0072
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0073
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0074
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0075
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0076
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0077
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0078
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0079
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0080
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0081
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0082
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0083
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0084
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0085
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0086
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0087
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0088
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0089
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0090
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0091
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0092
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0093
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0094
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0095
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0096
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0097
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0098
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0099
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0100
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0101
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0102
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0103
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0104
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0105
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0106
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0107
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0108
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0109
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0110
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0111
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0112
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0113
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0114
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0115
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0116
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0117
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0118
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0119
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0120
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0121
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0122
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0123
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0124
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0125
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0126
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0127
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0128
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0129
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0130
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0131
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0132
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0133
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0134
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0135
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0136
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0137
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0138
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0139
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0140
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0141
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0142
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0143
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0144
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0145
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0146
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0147
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0148
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0149
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0150
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0151
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0152
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0153
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0154
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0155
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0156
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0157
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0158
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0159
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0160
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0161
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0162
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0163
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0164
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0165
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0166
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0167
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0168
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0169
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0170
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0171
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0172
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0173
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0174
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0175
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0176
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0177
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0178
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0179
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0180
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0181
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0182
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0183
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0184
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0185
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0186
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0187
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0188
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0189
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0190
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0191
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0192
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0193
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0194
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0195
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0196
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0197
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0198
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0199
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0200
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0201
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0202
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0203
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0204
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0205
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0206
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0207
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0208
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0209
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0210
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0211
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0212
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0213
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0214
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0215
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0216
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0217
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0218
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0219
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0220
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0221
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0222
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0223
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0224
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0225
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0226
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0227
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0228
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0229
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0230
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0231
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0232
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0233
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0234
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0235
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0236
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0237
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0238
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0239
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0240
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0241
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0242
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0243
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0244
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0245
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0246
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0247
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0248
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0249
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0250
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0251
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0252
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0253
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0254
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0255
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0256
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0257
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0258
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0259
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0260
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0261
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0262
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0263
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0264
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0265
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0266
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0267
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0268
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0269
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0270
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0271
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0272
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0273
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0274
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0275
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0276
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0277
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0278
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0279
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0280
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0281
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0282
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0283
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0284
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0285
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0286
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0287
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0288
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0289
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0290
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0291
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0292
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0293
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0294
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0295
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0296
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0297
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0298
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0299
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0300
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0301
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0302
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0303
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0304
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0305
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0306
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0307
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0308
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0309
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0310
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0311
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0312
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0313
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0314
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0315
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0316
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0317
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0318
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0319
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0320
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0321
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0322
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0323
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0324
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0325
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0326
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0327
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0328
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0329
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0330
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0331
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0332
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0333
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0334
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0335
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0336
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0337
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0338
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0339
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0340
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0341
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0342
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0343
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0344
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0345
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0346
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0347
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0348
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0349
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0350
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0351
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0352
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0353
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0354
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0355
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0356
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0357
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0358
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0359
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0360
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0361
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0362
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0363
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0364
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0365
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0366
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0367
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0368
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0369
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0370
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0371
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0372
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0373
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0374
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0375
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0376
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0377
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0378
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0379
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0380
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0381
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0382
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0383
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0384
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0385
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0386
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0387
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0388
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0389
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0390
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0391
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0392
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0393
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0394
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0395
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0396
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0397
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0398
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0399
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0400
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0401
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0402
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0403
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0404
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0405
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0406
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0407
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0408
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0409
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0410
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0411
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0412
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0413
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0414
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0415
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0416
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0417
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0418
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0419
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0420
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0421
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0422
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0423
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0424
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0425
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0426
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0427
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0428
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0429
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0430
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0431
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0432
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0433
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0434
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0435
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0436
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0437
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0438
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0439
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0440
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0441
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0442
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0443
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0444
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0445
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0446
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0447
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0448
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0449
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0450
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0451
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0452
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0453
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0454
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0455
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0456
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0457
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0458
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0459
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0460
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0461
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0462
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0463
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0464
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0465
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0466
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0467
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0468
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0469
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0470
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0471
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0472
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0473
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0474
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0475
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0476
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0477
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0478
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0479
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0480
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0481
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0482
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0483
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0484
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0485
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0486
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0487
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0488
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0489
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0490
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0491
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0492
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0493
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0494
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0495
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0496
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0497
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0498
        WORKSPACE_INDEX_INCLUDE_GATE_LABEL, // 0499
    ];
    for label in labels {
        assert_eq!(*label, "Include selected search hits in this send");
        let gate = WorkspaceIndexIncludeGate::new();
        assert_eq!(gate.label(), *label);
    }
}

#[test]
fn pattern_search_hit_never_inject_paths() {
    let paths: &[&str] = &[
        "workspace/path_0000.rs",
        "workspace/path_0001.rs",
        "workspace/path_0002.rs",
        "workspace/path_0003.rs",
        "workspace/path_0004.rs",
        "workspace/path_0005.rs",
        "workspace/path_0006.rs",
        "workspace/path_0007.rs",
        "workspace/path_0008.rs",
        "workspace/path_0009.rs",
        "workspace/path_0010.rs",
        "workspace/path_0011.rs",
        "workspace/path_0012.rs",
        "workspace/path_0013.rs",
        "workspace/path_0014.rs",
        "workspace/path_0015.rs",
        "workspace/path_0016.rs",
        "workspace/path_0017.rs",
        "workspace/path_0018.rs",
        "workspace/path_0019.rs",
        "workspace/path_0020.rs",
        "workspace/path_0021.rs",
        "workspace/path_0022.rs",
        "workspace/path_0023.rs",
        "workspace/path_0024.rs",
        "workspace/path_0025.rs",
        "workspace/path_0026.rs",
        "workspace/path_0027.rs",
        "workspace/path_0028.rs",
        "workspace/path_0029.rs",
        "workspace/path_0030.rs",
        "workspace/path_0031.rs",
        "workspace/path_0032.rs",
        "workspace/path_0033.rs",
        "workspace/path_0034.rs",
        "workspace/path_0035.rs",
        "workspace/path_0036.rs",
        "workspace/path_0037.rs",
        "workspace/path_0038.rs",
        "workspace/path_0039.rs",
        "workspace/path_0040.rs",
        "workspace/path_0041.rs",
        "workspace/path_0042.rs",
        "workspace/path_0043.rs",
        "workspace/path_0044.rs",
        "workspace/path_0045.rs",
        "workspace/path_0046.rs",
        "workspace/path_0047.rs",
        "workspace/path_0048.rs",
        "workspace/path_0049.rs",
        "workspace/path_0050.rs",
        "workspace/path_0051.rs",
        "workspace/path_0052.rs",
        "workspace/path_0053.rs",
        "workspace/path_0054.rs",
        "workspace/path_0055.rs",
        "workspace/path_0056.rs",
        "workspace/path_0057.rs",
        "workspace/path_0058.rs",
        "workspace/path_0059.rs",
        "workspace/path_0060.rs",
        "workspace/path_0061.rs",
        "workspace/path_0062.rs",
        "workspace/path_0063.rs",
        "workspace/path_0064.rs",
        "workspace/path_0065.rs",
        "workspace/path_0066.rs",
        "workspace/path_0067.rs",
        "workspace/path_0068.rs",
        "workspace/path_0069.rs",
        "workspace/path_0070.rs",
        "workspace/path_0071.rs",
        "workspace/path_0072.rs",
        "workspace/path_0073.rs",
        "workspace/path_0074.rs",
        "workspace/path_0075.rs",
        "workspace/path_0076.rs",
        "workspace/path_0077.rs",
        "workspace/path_0078.rs",
        "workspace/path_0079.rs",
        "workspace/path_0080.rs",
        "workspace/path_0081.rs",
        "workspace/path_0082.rs",
        "workspace/path_0083.rs",
        "workspace/path_0084.rs",
        "workspace/path_0085.rs",
        "workspace/path_0086.rs",
        "workspace/path_0087.rs",
        "workspace/path_0088.rs",
        "workspace/path_0089.rs",
        "workspace/path_0090.rs",
        "workspace/path_0091.rs",
        "workspace/path_0092.rs",
        "workspace/path_0093.rs",
        "workspace/path_0094.rs",
        "workspace/path_0095.rs",
        "workspace/path_0096.rs",
        "workspace/path_0097.rs",
        "workspace/path_0098.rs",
        "workspace/path_0099.rs",
        "workspace/path_0100.rs",
        "workspace/path_0101.rs",
        "workspace/path_0102.rs",
        "workspace/path_0103.rs",
        "workspace/path_0104.rs",
        "workspace/path_0105.rs",
        "workspace/path_0106.rs",
        "workspace/path_0107.rs",
        "workspace/path_0108.rs",
        "workspace/path_0109.rs",
        "workspace/path_0110.rs",
        "workspace/path_0111.rs",
        "workspace/path_0112.rs",
        "workspace/path_0113.rs",
        "workspace/path_0114.rs",
        "workspace/path_0115.rs",
        "workspace/path_0116.rs",
        "workspace/path_0117.rs",
        "workspace/path_0118.rs",
        "workspace/path_0119.rs",
        "workspace/path_0120.rs",
        "workspace/path_0121.rs",
        "workspace/path_0122.rs",
        "workspace/path_0123.rs",
        "workspace/path_0124.rs",
        "workspace/path_0125.rs",
        "workspace/path_0126.rs",
        "workspace/path_0127.rs",
        "workspace/path_0128.rs",
        "workspace/path_0129.rs",
        "workspace/path_0130.rs",
        "workspace/path_0131.rs",
        "workspace/path_0132.rs",
        "workspace/path_0133.rs",
        "workspace/path_0134.rs",
        "workspace/path_0135.rs",
        "workspace/path_0136.rs",
        "workspace/path_0137.rs",
        "workspace/path_0138.rs",
        "workspace/path_0139.rs",
        "workspace/path_0140.rs",
        "workspace/path_0141.rs",
        "workspace/path_0142.rs",
        "workspace/path_0143.rs",
        "workspace/path_0144.rs",
        "workspace/path_0145.rs",
        "workspace/path_0146.rs",
        "workspace/path_0147.rs",
        "workspace/path_0148.rs",
        "workspace/path_0149.rs",
        "workspace/path_0150.rs",
        "workspace/path_0151.rs",
        "workspace/path_0152.rs",
        "workspace/path_0153.rs",
        "workspace/path_0154.rs",
        "workspace/path_0155.rs",
        "workspace/path_0156.rs",
        "workspace/path_0157.rs",
        "workspace/path_0158.rs",
        "workspace/path_0159.rs",
        "workspace/path_0160.rs",
        "workspace/path_0161.rs",
        "workspace/path_0162.rs",
        "workspace/path_0163.rs",
        "workspace/path_0164.rs",
        "workspace/path_0165.rs",
        "workspace/path_0166.rs",
        "workspace/path_0167.rs",
        "workspace/path_0168.rs",
        "workspace/path_0169.rs",
        "workspace/path_0170.rs",
        "workspace/path_0171.rs",
        "workspace/path_0172.rs",
        "workspace/path_0173.rs",
        "workspace/path_0174.rs",
        "workspace/path_0175.rs",
        "workspace/path_0176.rs",
        "workspace/path_0177.rs",
        "workspace/path_0178.rs",
        "workspace/path_0179.rs",
        "workspace/path_0180.rs",
        "workspace/path_0181.rs",
        "workspace/path_0182.rs",
        "workspace/path_0183.rs",
        "workspace/path_0184.rs",
        "workspace/path_0185.rs",
        "workspace/path_0186.rs",
        "workspace/path_0187.rs",
        "workspace/path_0188.rs",
        "workspace/path_0189.rs",
        "workspace/path_0190.rs",
        "workspace/path_0191.rs",
        "workspace/path_0192.rs",
        "workspace/path_0193.rs",
        "workspace/path_0194.rs",
        "workspace/path_0195.rs",
        "workspace/path_0196.rs",
        "workspace/path_0197.rs",
        "workspace/path_0198.rs",
        "workspace/path_0199.rs",
        "workspace/path_0200.rs",
        "workspace/path_0201.rs",
        "workspace/path_0202.rs",
        "workspace/path_0203.rs",
        "workspace/path_0204.rs",
        "workspace/path_0205.rs",
        "workspace/path_0206.rs",
        "workspace/path_0207.rs",
        "workspace/path_0208.rs",
        "workspace/path_0209.rs",
        "workspace/path_0210.rs",
        "workspace/path_0211.rs",
        "workspace/path_0212.rs",
        "workspace/path_0213.rs",
        "workspace/path_0214.rs",
        "workspace/path_0215.rs",
        "workspace/path_0216.rs",
        "workspace/path_0217.rs",
        "workspace/path_0218.rs",
        "workspace/path_0219.rs",
        "workspace/path_0220.rs",
        "workspace/path_0221.rs",
        "workspace/path_0222.rs",
        "workspace/path_0223.rs",
        "workspace/path_0224.rs",
        "workspace/path_0225.rs",
        "workspace/path_0226.rs",
        "workspace/path_0227.rs",
        "workspace/path_0228.rs",
        "workspace/path_0229.rs",
        "workspace/path_0230.rs",
        "workspace/path_0231.rs",
        "workspace/path_0232.rs",
        "workspace/path_0233.rs",
        "workspace/path_0234.rs",
        "workspace/path_0235.rs",
        "workspace/path_0236.rs",
        "workspace/path_0237.rs",
        "workspace/path_0238.rs",
        "workspace/path_0239.rs",
        "workspace/path_0240.rs",
        "workspace/path_0241.rs",
        "workspace/path_0242.rs",
        "workspace/path_0243.rs",
        "workspace/path_0244.rs",
        "workspace/path_0245.rs",
        "workspace/path_0246.rs",
        "workspace/path_0247.rs",
        "workspace/path_0248.rs",
        "workspace/path_0249.rs",
        "workspace/path_0250.rs",
        "workspace/path_0251.rs",
        "workspace/path_0252.rs",
        "workspace/path_0253.rs",
        "workspace/path_0254.rs",
        "workspace/path_0255.rs",
        "workspace/path_0256.rs",
        "workspace/path_0257.rs",
        "workspace/path_0258.rs",
        "workspace/path_0259.rs",
        "workspace/path_0260.rs",
        "workspace/path_0261.rs",
        "workspace/path_0262.rs",
        "workspace/path_0263.rs",
        "workspace/path_0264.rs",
        "workspace/path_0265.rs",
        "workspace/path_0266.rs",
        "workspace/path_0267.rs",
        "workspace/path_0268.rs",
        "workspace/path_0269.rs",
        "workspace/path_0270.rs",
        "workspace/path_0271.rs",
        "workspace/path_0272.rs",
        "workspace/path_0273.rs",
        "workspace/path_0274.rs",
        "workspace/path_0275.rs",
        "workspace/path_0276.rs",
        "workspace/path_0277.rs",
        "workspace/path_0278.rs",
        "workspace/path_0279.rs",
        "workspace/path_0280.rs",
        "workspace/path_0281.rs",
        "workspace/path_0282.rs",
        "workspace/path_0283.rs",
        "workspace/path_0284.rs",
        "workspace/path_0285.rs",
        "workspace/path_0286.rs",
        "workspace/path_0287.rs",
        "workspace/path_0288.rs",
        "workspace/path_0289.rs",
        "workspace/path_0290.rs",
        "workspace/path_0291.rs",
        "workspace/path_0292.rs",
        "workspace/path_0293.rs",
        "workspace/path_0294.rs",
        "workspace/path_0295.rs",
        "workspace/path_0296.rs",
        "workspace/path_0297.rs",
        "workspace/path_0298.rs",
        "workspace/path_0299.rs",
        "workspace/path_0300.rs",
        "workspace/path_0301.rs",
        "workspace/path_0302.rs",
        "workspace/path_0303.rs",
        "workspace/path_0304.rs",
        "workspace/path_0305.rs",
        "workspace/path_0306.rs",
        "workspace/path_0307.rs",
        "workspace/path_0308.rs",
        "workspace/path_0309.rs",
        "workspace/path_0310.rs",
        "workspace/path_0311.rs",
        "workspace/path_0312.rs",
        "workspace/path_0313.rs",
        "workspace/path_0314.rs",
        "workspace/path_0315.rs",
        "workspace/path_0316.rs",
        "workspace/path_0317.rs",
        "workspace/path_0318.rs",
        "workspace/path_0319.rs",
        "workspace/path_0320.rs",
        "workspace/path_0321.rs",
        "workspace/path_0322.rs",
        "workspace/path_0323.rs",
        "workspace/path_0324.rs",
        "workspace/path_0325.rs",
        "workspace/path_0326.rs",
        "workspace/path_0327.rs",
        "workspace/path_0328.rs",
        "workspace/path_0329.rs",
        "workspace/path_0330.rs",
        "workspace/path_0331.rs",
        "workspace/path_0332.rs",
        "workspace/path_0333.rs",
        "workspace/path_0334.rs",
        "workspace/path_0335.rs",
        "workspace/path_0336.rs",
        "workspace/path_0337.rs",
        "workspace/path_0338.rs",
        "workspace/path_0339.rs",
        "workspace/path_0340.rs",
        "workspace/path_0341.rs",
        "workspace/path_0342.rs",
        "workspace/path_0343.rs",
        "workspace/path_0344.rs",
        "workspace/path_0345.rs",
        "workspace/path_0346.rs",
        "workspace/path_0347.rs",
        "workspace/path_0348.rs",
        "workspace/path_0349.rs",
        "workspace/path_0350.rs",
        "workspace/path_0351.rs",
        "workspace/path_0352.rs",
        "workspace/path_0353.rs",
        "workspace/path_0354.rs",
        "workspace/path_0355.rs",
        "workspace/path_0356.rs",
        "workspace/path_0357.rs",
        "workspace/path_0358.rs",
        "workspace/path_0359.rs",
        "workspace/path_0360.rs",
        "workspace/path_0361.rs",
        "workspace/path_0362.rs",
        "workspace/path_0363.rs",
        "workspace/path_0364.rs",
        "workspace/path_0365.rs",
        "workspace/path_0366.rs",
        "workspace/path_0367.rs",
        "workspace/path_0368.rs",
        "workspace/path_0369.rs",
        "workspace/path_0370.rs",
        "workspace/path_0371.rs",
        "workspace/path_0372.rs",
        "workspace/path_0373.rs",
        "workspace/path_0374.rs",
        "workspace/path_0375.rs",
        "workspace/path_0376.rs",
        "workspace/path_0377.rs",
        "workspace/path_0378.rs",
        "workspace/path_0379.rs",
        "workspace/path_0380.rs",
        "workspace/path_0381.rs",
        "workspace/path_0382.rs",
        "workspace/path_0383.rs",
        "workspace/path_0384.rs",
        "workspace/path_0385.rs",
        "workspace/path_0386.rs",
        "workspace/path_0387.rs",
        "workspace/path_0388.rs",
        "workspace/path_0389.rs",
        "workspace/path_0390.rs",
        "workspace/path_0391.rs",
        "workspace/path_0392.rs",
        "workspace/path_0393.rs",
        "workspace/path_0394.rs",
        "workspace/path_0395.rs",
        "workspace/path_0396.rs",
        "workspace/path_0397.rs",
        "workspace/path_0398.rs",
        "workspace/path_0399.rs",
        "workspace/path_0400.rs",
        "workspace/path_0401.rs",
        "workspace/path_0402.rs",
        "workspace/path_0403.rs",
        "workspace/path_0404.rs",
        "workspace/path_0405.rs",
        "workspace/path_0406.rs",
        "workspace/path_0407.rs",
        "workspace/path_0408.rs",
        "workspace/path_0409.rs",
        "workspace/path_0410.rs",
        "workspace/path_0411.rs",
        "workspace/path_0412.rs",
        "workspace/path_0413.rs",
        "workspace/path_0414.rs",
        "workspace/path_0415.rs",
        "workspace/path_0416.rs",
        "workspace/path_0417.rs",
        "workspace/path_0418.rs",
        "workspace/path_0419.rs",
        "workspace/path_0420.rs",
        "workspace/path_0421.rs",
        "workspace/path_0422.rs",
        "workspace/path_0423.rs",
        "workspace/path_0424.rs",
        "workspace/path_0425.rs",
        "workspace/path_0426.rs",
        "workspace/path_0427.rs",
        "workspace/path_0428.rs",
        "workspace/path_0429.rs",
        "workspace/path_0430.rs",
        "workspace/path_0431.rs",
        "workspace/path_0432.rs",
        "workspace/path_0433.rs",
        "workspace/path_0434.rs",
        "workspace/path_0435.rs",
        "workspace/path_0436.rs",
        "workspace/path_0437.rs",
        "workspace/path_0438.rs",
        "workspace/path_0439.rs",
        "workspace/path_0440.rs",
        "workspace/path_0441.rs",
        "workspace/path_0442.rs",
        "workspace/path_0443.rs",
        "workspace/path_0444.rs",
        "workspace/path_0445.rs",
        "workspace/path_0446.rs",
        "workspace/path_0447.rs",
        "workspace/path_0448.rs",
        "workspace/path_0449.rs",
        "workspace/path_0450.rs",
        "workspace/path_0451.rs",
        "workspace/path_0452.rs",
        "workspace/path_0453.rs",
        "workspace/path_0454.rs",
        "workspace/path_0455.rs",
        "workspace/path_0456.rs",
        "workspace/path_0457.rs",
        "workspace/path_0458.rs",
        "workspace/path_0459.rs",
        "workspace/path_0460.rs",
        "workspace/path_0461.rs",
        "workspace/path_0462.rs",
        "workspace/path_0463.rs",
        "workspace/path_0464.rs",
        "workspace/path_0465.rs",
        "workspace/path_0466.rs",
        "workspace/path_0467.rs",
        "workspace/path_0468.rs",
        "workspace/path_0469.rs",
        "workspace/path_0470.rs",
        "workspace/path_0471.rs",
        "workspace/path_0472.rs",
        "workspace/path_0473.rs",
        "workspace/path_0474.rs",
        "workspace/path_0475.rs",
        "workspace/path_0476.rs",
        "workspace/path_0477.rs",
        "workspace/path_0478.rs",
        "workspace/path_0479.rs",
        "workspace/path_0480.rs",
        "workspace/path_0481.rs",
        "workspace/path_0482.rs",
        "workspace/path_0483.rs",
        "workspace/path_0484.rs",
        "workspace/path_0485.rs",
        "workspace/path_0486.rs",
        "workspace/path_0487.rs",
        "workspace/path_0488.rs",
        "workspace/path_0489.rs",
        "workspace/path_0490.rs",
        "workspace/path_0491.rs",
        "workspace/path_0492.rs",
        "workspace/path_0493.rs",
        "workspace/path_0494.rs",
        "workspace/path_0495.rs",
        "workspace/path_0496.rs",
        "workspace/path_0497.rs",
        "workspace/path_0498.rs",
        "workspace/path_0499.rs",
    ];
    for path in paths {
        // Candidate path alone must not be injectable.
        assert!(!may_inject_into_chat_request(ContextOrigin::IndexSearchHit));
        let draft = workspace_index_hit_attachment(path, "pub fn x() {}");
        // Explicit attach draft content is present, but gate-off still blocks include.
        let gate = WorkspaceIndexIncludeGate::new();
        assert!(drafts_for_workspace_index_include(&gate, std::slice::from_ref(&draft)).is_empty());
        assert!(draft.context_block.contains(path));
    }
}

#[test]
fn pattern_selection_select_deselect_cycle() {
    for i in 0..300 {
        let mut sel = WorkspaceIndexHitSelection::new();
        let a = format!("a{i:04}.rs");
        let b = format!("b{i:04}.rs");
        sel.select(&a);
        sel.select(&b);
        sel.select(&a);
        assert_eq!(sel.paths().len(), 2);
        sel.deselect(&a);
        assert_eq!(sel.paths(), std::slice::from_ref(&b));
        sel.deselect(&b);
        assert!(sel.is_empty());
    }
}

#[test]
fn pattern_enabled_gate_releases_exactly_selection() {
    for n in 0..120usize {
        let mut gate = WorkspaceIndexIncludeGate::new();
        gate.set_enabled(true);
        let drafts: Vec<_> = (0..n)
            .map(|i| workspace_index_hit_attachment(&format!("{i}.rs"), "x"))
            .collect();
        let out = drafts_for_workspace_index_include(&gate, &drafts);
        assert_eq!(out.len(), n);
        assert_eq!(gate.context_origin(), ContextOrigin::VisiblePerSendInclude);
        assert!(may_inject_into_chat_request(gate.context_origin()));
    }
}

#[test]
fn pattern_attach_name_from_relative_path() {
    let rows: &[(&str, &str)] = &[
        ("src/nested/file_0000.rs", "file_0000.rs"),
        ("src/nested/file_0001.rs", "file_0001.rs"),
        ("src/nested/file_0002.rs", "file_0002.rs"),
        ("src/nested/file_0003.rs", "file_0003.rs"),
        ("src/nested/file_0004.rs", "file_0004.rs"),
        ("src/nested/file_0005.rs", "file_0005.rs"),
        ("src/nested/file_0006.rs", "file_0006.rs"),
        ("src/nested/file_0007.rs", "file_0007.rs"),
        ("src/nested/file_0008.rs", "file_0008.rs"),
        ("src/nested/file_0009.rs", "file_0009.rs"),
        ("src/nested/file_0010.rs", "file_0010.rs"),
        ("src/nested/file_0011.rs", "file_0011.rs"),
        ("src/nested/file_0012.rs", "file_0012.rs"),
        ("src/nested/file_0013.rs", "file_0013.rs"),
        ("src/nested/file_0014.rs", "file_0014.rs"),
        ("src/nested/file_0015.rs", "file_0015.rs"),
        ("src/nested/file_0016.rs", "file_0016.rs"),
        ("src/nested/file_0017.rs", "file_0017.rs"),
        ("src/nested/file_0018.rs", "file_0018.rs"),
        ("src/nested/file_0019.rs", "file_0019.rs"),
        ("src/nested/file_0020.rs", "file_0020.rs"),
        ("src/nested/file_0021.rs", "file_0021.rs"),
        ("src/nested/file_0022.rs", "file_0022.rs"),
        ("src/nested/file_0023.rs", "file_0023.rs"),
        ("src/nested/file_0024.rs", "file_0024.rs"),
        ("src/nested/file_0025.rs", "file_0025.rs"),
        ("src/nested/file_0026.rs", "file_0026.rs"),
        ("src/nested/file_0027.rs", "file_0027.rs"),
        ("src/nested/file_0028.rs", "file_0028.rs"),
        ("src/nested/file_0029.rs", "file_0029.rs"),
        ("src/nested/file_0030.rs", "file_0030.rs"),
        ("src/nested/file_0031.rs", "file_0031.rs"),
        ("src/nested/file_0032.rs", "file_0032.rs"),
        ("src/nested/file_0033.rs", "file_0033.rs"),
        ("src/nested/file_0034.rs", "file_0034.rs"),
        ("src/nested/file_0035.rs", "file_0035.rs"),
        ("src/nested/file_0036.rs", "file_0036.rs"),
        ("src/nested/file_0037.rs", "file_0037.rs"),
        ("src/nested/file_0038.rs", "file_0038.rs"),
        ("src/nested/file_0039.rs", "file_0039.rs"),
        ("src/nested/file_0040.rs", "file_0040.rs"),
        ("src/nested/file_0041.rs", "file_0041.rs"),
        ("src/nested/file_0042.rs", "file_0042.rs"),
        ("src/nested/file_0043.rs", "file_0043.rs"),
        ("src/nested/file_0044.rs", "file_0044.rs"),
        ("src/nested/file_0045.rs", "file_0045.rs"),
        ("src/nested/file_0046.rs", "file_0046.rs"),
        ("src/nested/file_0047.rs", "file_0047.rs"),
        ("src/nested/file_0048.rs", "file_0048.rs"),
        ("src/nested/file_0049.rs", "file_0049.rs"),
        ("src/nested/file_0050.rs", "file_0050.rs"),
        ("src/nested/file_0051.rs", "file_0051.rs"),
        ("src/nested/file_0052.rs", "file_0052.rs"),
        ("src/nested/file_0053.rs", "file_0053.rs"),
        ("src/nested/file_0054.rs", "file_0054.rs"),
        ("src/nested/file_0055.rs", "file_0055.rs"),
        ("src/nested/file_0056.rs", "file_0056.rs"),
        ("src/nested/file_0057.rs", "file_0057.rs"),
        ("src/nested/file_0058.rs", "file_0058.rs"),
        ("src/nested/file_0059.rs", "file_0059.rs"),
        ("src/nested/file_0060.rs", "file_0060.rs"),
        ("src/nested/file_0061.rs", "file_0061.rs"),
        ("src/nested/file_0062.rs", "file_0062.rs"),
        ("src/nested/file_0063.rs", "file_0063.rs"),
        ("src/nested/file_0064.rs", "file_0064.rs"),
        ("src/nested/file_0065.rs", "file_0065.rs"),
        ("src/nested/file_0066.rs", "file_0066.rs"),
        ("src/nested/file_0067.rs", "file_0067.rs"),
        ("src/nested/file_0068.rs", "file_0068.rs"),
        ("src/nested/file_0069.rs", "file_0069.rs"),
        ("src/nested/file_0070.rs", "file_0070.rs"),
        ("src/nested/file_0071.rs", "file_0071.rs"),
        ("src/nested/file_0072.rs", "file_0072.rs"),
        ("src/nested/file_0073.rs", "file_0073.rs"),
        ("src/nested/file_0074.rs", "file_0074.rs"),
        ("src/nested/file_0075.rs", "file_0075.rs"),
        ("src/nested/file_0076.rs", "file_0076.rs"),
        ("src/nested/file_0077.rs", "file_0077.rs"),
        ("src/nested/file_0078.rs", "file_0078.rs"),
        ("src/nested/file_0079.rs", "file_0079.rs"),
        ("src/nested/file_0080.rs", "file_0080.rs"),
        ("src/nested/file_0081.rs", "file_0081.rs"),
        ("src/nested/file_0082.rs", "file_0082.rs"),
        ("src/nested/file_0083.rs", "file_0083.rs"),
        ("src/nested/file_0084.rs", "file_0084.rs"),
        ("src/nested/file_0085.rs", "file_0085.rs"),
        ("src/nested/file_0086.rs", "file_0086.rs"),
        ("src/nested/file_0087.rs", "file_0087.rs"),
        ("src/nested/file_0088.rs", "file_0088.rs"),
        ("src/nested/file_0089.rs", "file_0089.rs"),
        ("src/nested/file_0090.rs", "file_0090.rs"),
        ("src/nested/file_0091.rs", "file_0091.rs"),
        ("src/nested/file_0092.rs", "file_0092.rs"),
        ("src/nested/file_0093.rs", "file_0093.rs"),
        ("src/nested/file_0094.rs", "file_0094.rs"),
        ("src/nested/file_0095.rs", "file_0095.rs"),
        ("src/nested/file_0096.rs", "file_0096.rs"),
        ("src/nested/file_0097.rs", "file_0097.rs"),
        ("src/nested/file_0098.rs", "file_0098.rs"),
        ("src/nested/file_0099.rs", "file_0099.rs"),
        ("src/nested/file_0100.rs", "file_0100.rs"),
        ("src/nested/file_0101.rs", "file_0101.rs"),
        ("src/nested/file_0102.rs", "file_0102.rs"),
        ("src/nested/file_0103.rs", "file_0103.rs"),
        ("src/nested/file_0104.rs", "file_0104.rs"),
        ("src/nested/file_0105.rs", "file_0105.rs"),
        ("src/nested/file_0106.rs", "file_0106.rs"),
        ("src/nested/file_0107.rs", "file_0107.rs"),
        ("src/nested/file_0108.rs", "file_0108.rs"),
        ("src/nested/file_0109.rs", "file_0109.rs"),
        ("src/nested/file_0110.rs", "file_0110.rs"),
        ("src/nested/file_0111.rs", "file_0111.rs"),
        ("src/nested/file_0112.rs", "file_0112.rs"),
        ("src/nested/file_0113.rs", "file_0113.rs"),
        ("src/nested/file_0114.rs", "file_0114.rs"),
        ("src/nested/file_0115.rs", "file_0115.rs"),
        ("src/nested/file_0116.rs", "file_0116.rs"),
        ("src/nested/file_0117.rs", "file_0117.rs"),
        ("src/nested/file_0118.rs", "file_0118.rs"),
        ("src/nested/file_0119.rs", "file_0119.rs"),
        ("src/nested/file_0120.rs", "file_0120.rs"),
        ("src/nested/file_0121.rs", "file_0121.rs"),
        ("src/nested/file_0122.rs", "file_0122.rs"),
        ("src/nested/file_0123.rs", "file_0123.rs"),
        ("src/nested/file_0124.rs", "file_0124.rs"),
        ("src/nested/file_0125.rs", "file_0125.rs"),
        ("src/nested/file_0126.rs", "file_0126.rs"),
        ("src/nested/file_0127.rs", "file_0127.rs"),
        ("src/nested/file_0128.rs", "file_0128.rs"),
        ("src/nested/file_0129.rs", "file_0129.rs"),
        ("src/nested/file_0130.rs", "file_0130.rs"),
        ("src/nested/file_0131.rs", "file_0131.rs"),
        ("src/nested/file_0132.rs", "file_0132.rs"),
        ("src/nested/file_0133.rs", "file_0133.rs"),
        ("src/nested/file_0134.rs", "file_0134.rs"),
        ("src/nested/file_0135.rs", "file_0135.rs"),
        ("src/nested/file_0136.rs", "file_0136.rs"),
        ("src/nested/file_0137.rs", "file_0137.rs"),
        ("src/nested/file_0138.rs", "file_0138.rs"),
        ("src/nested/file_0139.rs", "file_0139.rs"),
        ("src/nested/file_0140.rs", "file_0140.rs"),
        ("src/nested/file_0141.rs", "file_0141.rs"),
        ("src/nested/file_0142.rs", "file_0142.rs"),
        ("src/nested/file_0143.rs", "file_0143.rs"),
        ("src/nested/file_0144.rs", "file_0144.rs"),
        ("src/nested/file_0145.rs", "file_0145.rs"),
        ("src/nested/file_0146.rs", "file_0146.rs"),
        ("src/nested/file_0147.rs", "file_0147.rs"),
        ("src/nested/file_0148.rs", "file_0148.rs"),
        ("src/nested/file_0149.rs", "file_0149.rs"),
        ("src/nested/file_0150.rs", "file_0150.rs"),
        ("src/nested/file_0151.rs", "file_0151.rs"),
        ("src/nested/file_0152.rs", "file_0152.rs"),
        ("src/nested/file_0153.rs", "file_0153.rs"),
        ("src/nested/file_0154.rs", "file_0154.rs"),
        ("src/nested/file_0155.rs", "file_0155.rs"),
        ("src/nested/file_0156.rs", "file_0156.rs"),
        ("src/nested/file_0157.rs", "file_0157.rs"),
        ("src/nested/file_0158.rs", "file_0158.rs"),
        ("src/nested/file_0159.rs", "file_0159.rs"),
        ("src/nested/file_0160.rs", "file_0160.rs"),
        ("src/nested/file_0161.rs", "file_0161.rs"),
        ("src/nested/file_0162.rs", "file_0162.rs"),
        ("src/nested/file_0163.rs", "file_0163.rs"),
        ("src/nested/file_0164.rs", "file_0164.rs"),
        ("src/nested/file_0165.rs", "file_0165.rs"),
        ("src/nested/file_0166.rs", "file_0166.rs"),
        ("src/nested/file_0167.rs", "file_0167.rs"),
        ("src/nested/file_0168.rs", "file_0168.rs"),
        ("src/nested/file_0169.rs", "file_0169.rs"),
        ("src/nested/file_0170.rs", "file_0170.rs"),
        ("src/nested/file_0171.rs", "file_0171.rs"),
        ("src/nested/file_0172.rs", "file_0172.rs"),
        ("src/nested/file_0173.rs", "file_0173.rs"),
        ("src/nested/file_0174.rs", "file_0174.rs"),
        ("src/nested/file_0175.rs", "file_0175.rs"),
        ("src/nested/file_0176.rs", "file_0176.rs"),
        ("src/nested/file_0177.rs", "file_0177.rs"),
        ("src/nested/file_0178.rs", "file_0178.rs"),
        ("src/nested/file_0179.rs", "file_0179.rs"),
        ("src/nested/file_0180.rs", "file_0180.rs"),
        ("src/nested/file_0181.rs", "file_0181.rs"),
        ("src/nested/file_0182.rs", "file_0182.rs"),
        ("src/nested/file_0183.rs", "file_0183.rs"),
        ("src/nested/file_0184.rs", "file_0184.rs"),
        ("src/nested/file_0185.rs", "file_0185.rs"),
        ("src/nested/file_0186.rs", "file_0186.rs"),
        ("src/nested/file_0187.rs", "file_0187.rs"),
        ("src/nested/file_0188.rs", "file_0188.rs"),
        ("src/nested/file_0189.rs", "file_0189.rs"),
        ("src/nested/file_0190.rs", "file_0190.rs"),
        ("src/nested/file_0191.rs", "file_0191.rs"),
        ("src/nested/file_0192.rs", "file_0192.rs"),
        ("src/nested/file_0193.rs", "file_0193.rs"),
        ("src/nested/file_0194.rs", "file_0194.rs"),
        ("src/nested/file_0195.rs", "file_0195.rs"),
        ("src/nested/file_0196.rs", "file_0196.rs"),
        ("src/nested/file_0197.rs", "file_0197.rs"),
        ("src/nested/file_0198.rs", "file_0198.rs"),
        ("src/nested/file_0199.rs", "file_0199.rs"),
    ];
    for (path, name) in rows {
        let draft = workspace_index_hit_attachment(path, "fn x() {}");
        assert_eq!(draft.name, *name);
        assert!(draft.context_block.contains(path));
        assert!(!may_inject_into_chat_request(ContextOrigin::IndexSearchHit));
        assert!(may_inject_into_chat_request(
            ContextOrigin::ExplicitAttachment
        ));
    }
}
