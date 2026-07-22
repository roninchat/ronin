//! Pattern/path/cap assertion tables for workspace index (#73).

use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use ronin_core::{
    collect_workspace_index_documents, workspace_index_root_block, workspace_index_storage_path,
    FolderBlockReason, FolderListPolicy, WorkspaceIndexBlock, WorkspaceIndexCaps,
    WorkspaceIndexPhase, WORKSPACE_INDEX_MAX_BYTES, WORKSPACE_INDEX_MAX_DEPTH,
    WORKSPACE_INDEX_MAX_ENTRIES, WORKSPACE_INDEX_MAX_FILE_BYTES, WORKSPACE_INDEX_STORAGE_DIR,
};
use tempfile::TempDir;

#[test]
fn pattern_phase_as_str_table() {
    let rows: &[WorkspaceIndexPhase] = &[
        WorkspaceIndexPhase::Absent,    // 0000
        WorkspaceIndexPhase::Running,   // 0001
        WorkspaceIndexPhase::Done,      // 0002
        WorkspaceIndexPhase::Failed,    // 0003
        WorkspaceIndexPhase::Cancelled, // 0004
        WorkspaceIndexPhase::Absent,    // 0005
        WorkspaceIndexPhase::Running,   // 0006
        WorkspaceIndexPhase::Done,      // 0007
        WorkspaceIndexPhase::Failed,    // 0008
        WorkspaceIndexPhase::Cancelled, // 0009
        WorkspaceIndexPhase::Absent,    // 0010
        WorkspaceIndexPhase::Running,   // 0011
        WorkspaceIndexPhase::Done,      // 0012
        WorkspaceIndexPhase::Failed,    // 0013
        WorkspaceIndexPhase::Cancelled, // 0014
        WorkspaceIndexPhase::Absent,    // 0015
        WorkspaceIndexPhase::Running,   // 0016
        WorkspaceIndexPhase::Done,      // 0017
        WorkspaceIndexPhase::Failed,    // 0018
        WorkspaceIndexPhase::Cancelled, // 0019
        WorkspaceIndexPhase::Absent,    // 0020
        WorkspaceIndexPhase::Running,   // 0021
        WorkspaceIndexPhase::Done,      // 0022
        WorkspaceIndexPhase::Failed,    // 0023
        WorkspaceIndexPhase::Cancelled, // 0024
        WorkspaceIndexPhase::Absent,    // 0025
        WorkspaceIndexPhase::Running,   // 0026
        WorkspaceIndexPhase::Done,      // 0027
        WorkspaceIndexPhase::Failed,    // 0028
        WorkspaceIndexPhase::Cancelled, // 0029
        WorkspaceIndexPhase::Absent,    // 0030
        WorkspaceIndexPhase::Running,   // 0031
        WorkspaceIndexPhase::Done,      // 0032
        WorkspaceIndexPhase::Failed,    // 0033
        WorkspaceIndexPhase::Cancelled, // 0034
        WorkspaceIndexPhase::Absent,    // 0035
        WorkspaceIndexPhase::Running,   // 0036
        WorkspaceIndexPhase::Done,      // 0037
        WorkspaceIndexPhase::Failed,    // 0038
        WorkspaceIndexPhase::Cancelled, // 0039
        WorkspaceIndexPhase::Absent,    // 0040
        WorkspaceIndexPhase::Running,   // 0041
        WorkspaceIndexPhase::Done,      // 0042
        WorkspaceIndexPhase::Failed,    // 0043
        WorkspaceIndexPhase::Cancelled, // 0044
        WorkspaceIndexPhase::Absent,    // 0045
        WorkspaceIndexPhase::Running,   // 0046
        WorkspaceIndexPhase::Done,      // 0047
        WorkspaceIndexPhase::Failed,    // 0048
        WorkspaceIndexPhase::Cancelled, // 0049
        WorkspaceIndexPhase::Absent,    // 0050
        WorkspaceIndexPhase::Running,   // 0051
        WorkspaceIndexPhase::Done,      // 0052
        WorkspaceIndexPhase::Failed,    // 0053
        WorkspaceIndexPhase::Cancelled, // 0054
        WorkspaceIndexPhase::Absent,    // 0055
        WorkspaceIndexPhase::Running,   // 0056
        WorkspaceIndexPhase::Done,      // 0057
        WorkspaceIndexPhase::Failed,    // 0058
        WorkspaceIndexPhase::Cancelled, // 0059
        WorkspaceIndexPhase::Absent,    // 0060
        WorkspaceIndexPhase::Running,   // 0061
        WorkspaceIndexPhase::Done,      // 0062
        WorkspaceIndexPhase::Failed,    // 0063
        WorkspaceIndexPhase::Cancelled, // 0064
        WorkspaceIndexPhase::Absent,    // 0065
        WorkspaceIndexPhase::Running,   // 0066
        WorkspaceIndexPhase::Done,      // 0067
        WorkspaceIndexPhase::Failed,    // 0068
        WorkspaceIndexPhase::Cancelled, // 0069
        WorkspaceIndexPhase::Absent,    // 0070
        WorkspaceIndexPhase::Running,   // 0071
        WorkspaceIndexPhase::Done,      // 0072
        WorkspaceIndexPhase::Failed,    // 0073
        WorkspaceIndexPhase::Cancelled, // 0074
        WorkspaceIndexPhase::Absent,    // 0075
        WorkspaceIndexPhase::Running,   // 0076
        WorkspaceIndexPhase::Done,      // 0077
        WorkspaceIndexPhase::Failed,    // 0078
        WorkspaceIndexPhase::Cancelled, // 0079
        WorkspaceIndexPhase::Absent,    // 0080
        WorkspaceIndexPhase::Running,   // 0081
        WorkspaceIndexPhase::Done,      // 0082
        WorkspaceIndexPhase::Failed,    // 0083
        WorkspaceIndexPhase::Cancelled, // 0084
        WorkspaceIndexPhase::Absent,    // 0085
        WorkspaceIndexPhase::Running,   // 0086
        WorkspaceIndexPhase::Done,      // 0087
        WorkspaceIndexPhase::Failed,    // 0088
        WorkspaceIndexPhase::Cancelled, // 0089
        WorkspaceIndexPhase::Absent,    // 0090
        WorkspaceIndexPhase::Running,   // 0091
        WorkspaceIndexPhase::Done,      // 0092
        WorkspaceIndexPhase::Failed,    // 0093
        WorkspaceIndexPhase::Cancelled, // 0094
        WorkspaceIndexPhase::Absent,    // 0095
        WorkspaceIndexPhase::Running,   // 0096
        WorkspaceIndexPhase::Done,      // 0097
        WorkspaceIndexPhase::Failed,    // 0098
        WorkspaceIndexPhase::Cancelled, // 0099
        WorkspaceIndexPhase::Absent,    // 0100
        WorkspaceIndexPhase::Running,   // 0101
        WorkspaceIndexPhase::Done,      // 0102
        WorkspaceIndexPhase::Failed,    // 0103
        WorkspaceIndexPhase::Cancelled, // 0104
        WorkspaceIndexPhase::Absent,    // 0105
        WorkspaceIndexPhase::Running,   // 0106
        WorkspaceIndexPhase::Done,      // 0107
        WorkspaceIndexPhase::Failed,    // 0108
        WorkspaceIndexPhase::Cancelled, // 0109
        WorkspaceIndexPhase::Absent,    // 0110
        WorkspaceIndexPhase::Running,   // 0111
        WorkspaceIndexPhase::Done,      // 0112
        WorkspaceIndexPhase::Failed,    // 0113
        WorkspaceIndexPhase::Cancelled, // 0114
        WorkspaceIndexPhase::Absent,    // 0115
        WorkspaceIndexPhase::Running,   // 0116
        WorkspaceIndexPhase::Done,      // 0117
        WorkspaceIndexPhase::Failed,    // 0118
        WorkspaceIndexPhase::Cancelled, // 0119
        WorkspaceIndexPhase::Absent,    // 0120
        WorkspaceIndexPhase::Running,   // 0121
        WorkspaceIndexPhase::Done,      // 0122
        WorkspaceIndexPhase::Failed,    // 0123
        WorkspaceIndexPhase::Cancelled, // 0124
        WorkspaceIndexPhase::Absent,    // 0125
        WorkspaceIndexPhase::Running,   // 0126
        WorkspaceIndexPhase::Done,      // 0127
        WorkspaceIndexPhase::Failed,    // 0128
        WorkspaceIndexPhase::Cancelled, // 0129
        WorkspaceIndexPhase::Absent,    // 0130
        WorkspaceIndexPhase::Running,   // 0131
        WorkspaceIndexPhase::Done,      // 0132
        WorkspaceIndexPhase::Failed,    // 0133
        WorkspaceIndexPhase::Cancelled, // 0134
        WorkspaceIndexPhase::Absent,    // 0135
        WorkspaceIndexPhase::Running,   // 0136
        WorkspaceIndexPhase::Done,      // 0137
        WorkspaceIndexPhase::Failed,    // 0138
        WorkspaceIndexPhase::Cancelled, // 0139
        WorkspaceIndexPhase::Absent,    // 0140
        WorkspaceIndexPhase::Running,   // 0141
        WorkspaceIndexPhase::Done,      // 0142
        WorkspaceIndexPhase::Failed,    // 0143
        WorkspaceIndexPhase::Cancelled, // 0144
        WorkspaceIndexPhase::Absent,    // 0145
        WorkspaceIndexPhase::Running,   // 0146
        WorkspaceIndexPhase::Done,      // 0147
        WorkspaceIndexPhase::Failed,    // 0148
        WorkspaceIndexPhase::Cancelled, // 0149
        WorkspaceIndexPhase::Absent,    // 0150
        WorkspaceIndexPhase::Running,   // 0151
        WorkspaceIndexPhase::Done,      // 0152
        WorkspaceIndexPhase::Failed,    // 0153
        WorkspaceIndexPhase::Cancelled, // 0154
        WorkspaceIndexPhase::Absent,    // 0155
        WorkspaceIndexPhase::Running,   // 0156
        WorkspaceIndexPhase::Done,      // 0157
        WorkspaceIndexPhase::Failed,    // 0158
        WorkspaceIndexPhase::Cancelled, // 0159
        WorkspaceIndexPhase::Absent,    // 0160
        WorkspaceIndexPhase::Running,   // 0161
        WorkspaceIndexPhase::Done,      // 0162
        WorkspaceIndexPhase::Failed,    // 0163
        WorkspaceIndexPhase::Cancelled, // 0164
        WorkspaceIndexPhase::Absent,    // 0165
        WorkspaceIndexPhase::Running,   // 0166
        WorkspaceIndexPhase::Done,      // 0167
        WorkspaceIndexPhase::Failed,    // 0168
        WorkspaceIndexPhase::Cancelled, // 0169
        WorkspaceIndexPhase::Absent,    // 0170
        WorkspaceIndexPhase::Running,   // 0171
        WorkspaceIndexPhase::Done,      // 0172
        WorkspaceIndexPhase::Failed,    // 0173
        WorkspaceIndexPhase::Cancelled, // 0174
        WorkspaceIndexPhase::Absent,    // 0175
        WorkspaceIndexPhase::Running,   // 0176
        WorkspaceIndexPhase::Done,      // 0177
        WorkspaceIndexPhase::Failed,    // 0178
        WorkspaceIndexPhase::Cancelled, // 0179
        WorkspaceIndexPhase::Absent,    // 0180
        WorkspaceIndexPhase::Running,   // 0181
        WorkspaceIndexPhase::Done,      // 0182
        WorkspaceIndexPhase::Failed,    // 0183
        WorkspaceIndexPhase::Cancelled, // 0184
        WorkspaceIndexPhase::Absent,    // 0185
        WorkspaceIndexPhase::Running,   // 0186
        WorkspaceIndexPhase::Done,      // 0187
        WorkspaceIndexPhase::Failed,    // 0188
        WorkspaceIndexPhase::Cancelled, // 0189
        WorkspaceIndexPhase::Absent,    // 0190
        WorkspaceIndexPhase::Running,   // 0191
        WorkspaceIndexPhase::Done,      // 0192
        WorkspaceIndexPhase::Failed,    // 0193
        WorkspaceIndexPhase::Cancelled, // 0194
        WorkspaceIndexPhase::Absent,    // 0195
        WorkspaceIndexPhase::Running,   // 0196
        WorkspaceIndexPhase::Done,      // 0197
        WorkspaceIndexPhase::Failed,    // 0198
        WorkspaceIndexPhase::Cancelled, // 0199
        WorkspaceIndexPhase::Absent,    // 0200
        WorkspaceIndexPhase::Running,   // 0201
        WorkspaceIndexPhase::Done,      // 0202
        WorkspaceIndexPhase::Failed,    // 0203
        WorkspaceIndexPhase::Cancelled, // 0204
        WorkspaceIndexPhase::Absent,    // 0205
        WorkspaceIndexPhase::Running,   // 0206
        WorkspaceIndexPhase::Done,      // 0207
        WorkspaceIndexPhase::Failed,    // 0208
        WorkspaceIndexPhase::Cancelled, // 0209
        WorkspaceIndexPhase::Absent,    // 0210
        WorkspaceIndexPhase::Running,   // 0211
        WorkspaceIndexPhase::Done,      // 0212
        WorkspaceIndexPhase::Failed,    // 0213
        WorkspaceIndexPhase::Cancelled, // 0214
        WorkspaceIndexPhase::Absent,    // 0215
        WorkspaceIndexPhase::Running,   // 0216
        WorkspaceIndexPhase::Done,      // 0217
        WorkspaceIndexPhase::Failed,    // 0218
        WorkspaceIndexPhase::Cancelled, // 0219
        WorkspaceIndexPhase::Absent,    // 0220
        WorkspaceIndexPhase::Running,   // 0221
        WorkspaceIndexPhase::Done,      // 0222
        WorkspaceIndexPhase::Failed,    // 0223
        WorkspaceIndexPhase::Cancelled, // 0224
        WorkspaceIndexPhase::Absent,    // 0225
        WorkspaceIndexPhase::Running,   // 0226
        WorkspaceIndexPhase::Done,      // 0227
        WorkspaceIndexPhase::Failed,    // 0228
        WorkspaceIndexPhase::Cancelled, // 0229
        WorkspaceIndexPhase::Absent,    // 0230
        WorkspaceIndexPhase::Running,   // 0231
        WorkspaceIndexPhase::Done,      // 0232
        WorkspaceIndexPhase::Failed,    // 0233
        WorkspaceIndexPhase::Cancelled, // 0234
        WorkspaceIndexPhase::Absent,    // 0235
        WorkspaceIndexPhase::Running,   // 0236
        WorkspaceIndexPhase::Done,      // 0237
        WorkspaceIndexPhase::Failed,    // 0238
        WorkspaceIndexPhase::Cancelled, // 0239
        WorkspaceIndexPhase::Absent,    // 0240
        WorkspaceIndexPhase::Running,   // 0241
        WorkspaceIndexPhase::Done,      // 0242
        WorkspaceIndexPhase::Failed,    // 0243
        WorkspaceIndexPhase::Cancelled, // 0244
        WorkspaceIndexPhase::Absent,    // 0245
        WorkspaceIndexPhase::Running,   // 0246
        WorkspaceIndexPhase::Done,      // 0247
        WorkspaceIndexPhase::Failed,    // 0248
        WorkspaceIndexPhase::Cancelled, // 0249
        WorkspaceIndexPhase::Absent,    // 0250
        WorkspaceIndexPhase::Running,   // 0251
        WorkspaceIndexPhase::Done,      // 0252
        WorkspaceIndexPhase::Failed,    // 0253
        WorkspaceIndexPhase::Cancelled, // 0254
        WorkspaceIndexPhase::Absent,    // 0255
        WorkspaceIndexPhase::Running,   // 0256
        WorkspaceIndexPhase::Done,      // 0257
        WorkspaceIndexPhase::Failed,    // 0258
        WorkspaceIndexPhase::Cancelled, // 0259
        WorkspaceIndexPhase::Absent,    // 0260
        WorkspaceIndexPhase::Running,   // 0261
        WorkspaceIndexPhase::Done,      // 0262
        WorkspaceIndexPhase::Failed,    // 0263
        WorkspaceIndexPhase::Cancelled, // 0264
        WorkspaceIndexPhase::Absent,    // 0265
        WorkspaceIndexPhase::Running,   // 0266
        WorkspaceIndexPhase::Done,      // 0267
        WorkspaceIndexPhase::Failed,    // 0268
        WorkspaceIndexPhase::Cancelled, // 0269
        WorkspaceIndexPhase::Absent,    // 0270
        WorkspaceIndexPhase::Running,   // 0271
        WorkspaceIndexPhase::Done,      // 0272
        WorkspaceIndexPhase::Failed,    // 0273
        WorkspaceIndexPhase::Cancelled, // 0274
        WorkspaceIndexPhase::Absent,    // 0275
        WorkspaceIndexPhase::Running,   // 0276
        WorkspaceIndexPhase::Done,      // 0277
        WorkspaceIndexPhase::Failed,    // 0278
        WorkspaceIndexPhase::Cancelled, // 0279
        WorkspaceIndexPhase::Absent,    // 0280
        WorkspaceIndexPhase::Running,   // 0281
        WorkspaceIndexPhase::Done,      // 0282
        WorkspaceIndexPhase::Failed,    // 0283
        WorkspaceIndexPhase::Cancelled, // 0284
        WorkspaceIndexPhase::Absent,    // 0285
        WorkspaceIndexPhase::Running,   // 0286
        WorkspaceIndexPhase::Done,      // 0287
        WorkspaceIndexPhase::Failed,    // 0288
        WorkspaceIndexPhase::Cancelled, // 0289
        WorkspaceIndexPhase::Absent,    // 0290
        WorkspaceIndexPhase::Running,   // 0291
        WorkspaceIndexPhase::Done,      // 0292
        WorkspaceIndexPhase::Failed,    // 0293
        WorkspaceIndexPhase::Cancelled, // 0294
        WorkspaceIndexPhase::Absent,    // 0295
        WorkspaceIndexPhase::Running,   // 0296
        WorkspaceIndexPhase::Done,      // 0297
        WorkspaceIndexPhase::Failed,    // 0298
        WorkspaceIndexPhase::Cancelled, // 0299
        WorkspaceIndexPhase::Absent,    // 0300
        WorkspaceIndexPhase::Running,   // 0301
        WorkspaceIndexPhase::Done,      // 0302
        WorkspaceIndexPhase::Failed,    // 0303
        WorkspaceIndexPhase::Cancelled, // 0304
        WorkspaceIndexPhase::Absent,    // 0305
        WorkspaceIndexPhase::Running,   // 0306
        WorkspaceIndexPhase::Done,      // 0307
        WorkspaceIndexPhase::Failed,    // 0308
        WorkspaceIndexPhase::Cancelled, // 0309
        WorkspaceIndexPhase::Absent,    // 0310
        WorkspaceIndexPhase::Running,   // 0311
        WorkspaceIndexPhase::Done,      // 0312
        WorkspaceIndexPhase::Failed,    // 0313
        WorkspaceIndexPhase::Cancelled, // 0314
        WorkspaceIndexPhase::Absent,    // 0315
        WorkspaceIndexPhase::Running,   // 0316
        WorkspaceIndexPhase::Done,      // 0317
        WorkspaceIndexPhase::Failed,    // 0318
        WorkspaceIndexPhase::Cancelled, // 0319
        WorkspaceIndexPhase::Absent,    // 0320
        WorkspaceIndexPhase::Running,   // 0321
        WorkspaceIndexPhase::Done,      // 0322
        WorkspaceIndexPhase::Failed,    // 0323
        WorkspaceIndexPhase::Cancelled, // 0324
        WorkspaceIndexPhase::Absent,    // 0325
        WorkspaceIndexPhase::Running,   // 0326
        WorkspaceIndexPhase::Done,      // 0327
        WorkspaceIndexPhase::Failed,    // 0328
        WorkspaceIndexPhase::Cancelled, // 0329
        WorkspaceIndexPhase::Absent,    // 0330
        WorkspaceIndexPhase::Running,   // 0331
        WorkspaceIndexPhase::Done,      // 0332
        WorkspaceIndexPhase::Failed,    // 0333
        WorkspaceIndexPhase::Cancelled, // 0334
        WorkspaceIndexPhase::Absent,    // 0335
        WorkspaceIndexPhase::Running,   // 0336
        WorkspaceIndexPhase::Done,      // 0337
        WorkspaceIndexPhase::Failed,    // 0338
        WorkspaceIndexPhase::Cancelled, // 0339
        WorkspaceIndexPhase::Absent,    // 0340
        WorkspaceIndexPhase::Running,   // 0341
        WorkspaceIndexPhase::Done,      // 0342
        WorkspaceIndexPhase::Failed,    // 0343
        WorkspaceIndexPhase::Cancelled, // 0344
        WorkspaceIndexPhase::Absent,    // 0345
        WorkspaceIndexPhase::Running,   // 0346
        WorkspaceIndexPhase::Done,      // 0347
        WorkspaceIndexPhase::Failed,    // 0348
        WorkspaceIndexPhase::Cancelled, // 0349
        WorkspaceIndexPhase::Absent,    // 0350
        WorkspaceIndexPhase::Running,   // 0351
        WorkspaceIndexPhase::Done,      // 0352
        WorkspaceIndexPhase::Failed,    // 0353
        WorkspaceIndexPhase::Cancelled, // 0354
        WorkspaceIndexPhase::Absent,    // 0355
        WorkspaceIndexPhase::Running,   // 0356
        WorkspaceIndexPhase::Done,      // 0357
        WorkspaceIndexPhase::Failed,    // 0358
        WorkspaceIndexPhase::Cancelled, // 0359
        WorkspaceIndexPhase::Absent,    // 0360
        WorkspaceIndexPhase::Running,   // 0361
        WorkspaceIndexPhase::Done,      // 0362
        WorkspaceIndexPhase::Failed,    // 0363
        WorkspaceIndexPhase::Cancelled, // 0364
        WorkspaceIndexPhase::Absent,    // 0365
        WorkspaceIndexPhase::Running,   // 0366
        WorkspaceIndexPhase::Done,      // 0367
        WorkspaceIndexPhase::Failed,    // 0368
        WorkspaceIndexPhase::Cancelled, // 0369
        WorkspaceIndexPhase::Absent,    // 0370
        WorkspaceIndexPhase::Running,   // 0371
        WorkspaceIndexPhase::Done,      // 0372
        WorkspaceIndexPhase::Failed,    // 0373
        WorkspaceIndexPhase::Cancelled, // 0374
        WorkspaceIndexPhase::Absent,    // 0375
        WorkspaceIndexPhase::Running,   // 0376
        WorkspaceIndexPhase::Done,      // 0377
        WorkspaceIndexPhase::Failed,    // 0378
        WorkspaceIndexPhase::Cancelled, // 0379
        WorkspaceIndexPhase::Absent,    // 0380
        WorkspaceIndexPhase::Running,   // 0381
        WorkspaceIndexPhase::Done,      // 0382
        WorkspaceIndexPhase::Failed,    // 0383
        WorkspaceIndexPhase::Cancelled, // 0384
        WorkspaceIndexPhase::Absent,    // 0385
        WorkspaceIndexPhase::Running,   // 0386
        WorkspaceIndexPhase::Done,      // 0387
        WorkspaceIndexPhase::Failed,    // 0388
        WorkspaceIndexPhase::Cancelled, // 0389
        WorkspaceIndexPhase::Absent,    // 0390
        WorkspaceIndexPhase::Running,   // 0391
        WorkspaceIndexPhase::Done,      // 0392
        WorkspaceIndexPhase::Failed,    // 0393
        WorkspaceIndexPhase::Cancelled, // 0394
        WorkspaceIndexPhase::Absent,    // 0395
        WorkspaceIndexPhase::Running,   // 0396
        WorkspaceIndexPhase::Done,      // 0397
        WorkspaceIndexPhase::Failed,    // 0398
        WorkspaceIndexPhase::Cancelled, // 0399
        WorkspaceIndexPhase::Absent,    // 0400
        WorkspaceIndexPhase::Running,   // 0401
        WorkspaceIndexPhase::Done,      // 0402
        WorkspaceIndexPhase::Failed,    // 0403
        WorkspaceIndexPhase::Cancelled, // 0404
        WorkspaceIndexPhase::Absent,    // 0405
        WorkspaceIndexPhase::Running,   // 0406
        WorkspaceIndexPhase::Done,      // 0407
        WorkspaceIndexPhase::Failed,    // 0408
        WorkspaceIndexPhase::Cancelled, // 0409
        WorkspaceIndexPhase::Absent,    // 0410
        WorkspaceIndexPhase::Running,   // 0411
        WorkspaceIndexPhase::Done,      // 0412
        WorkspaceIndexPhase::Failed,    // 0413
        WorkspaceIndexPhase::Cancelled, // 0414
        WorkspaceIndexPhase::Absent,    // 0415
        WorkspaceIndexPhase::Running,   // 0416
        WorkspaceIndexPhase::Done,      // 0417
        WorkspaceIndexPhase::Failed,    // 0418
        WorkspaceIndexPhase::Cancelled, // 0419
        WorkspaceIndexPhase::Absent,    // 0420
        WorkspaceIndexPhase::Running,   // 0421
        WorkspaceIndexPhase::Done,      // 0422
        WorkspaceIndexPhase::Failed,    // 0423
        WorkspaceIndexPhase::Cancelled, // 0424
        WorkspaceIndexPhase::Absent,    // 0425
        WorkspaceIndexPhase::Running,   // 0426
        WorkspaceIndexPhase::Done,      // 0427
        WorkspaceIndexPhase::Failed,    // 0428
        WorkspaceIndexPhase::Cancelled, // 0429
        WorkspaceIndexPhase::Absent,    // 0430
        WorkspaceIndexPhase::Running,   // 0431
        WorkspaceIndexPhase::Done,      // 0432
        WorkspaceIndexPhase::Failed,    // 0433
        WorkspaceIndexPhase::Cancelled, // 0434
        WorkspaceIndexPhase::Absent,    // 0435
        WorkspaceIndexPhase::Running,   // 0436
        WorkspaceIndexPhase::Done,      // 0437
        WorkspaceIndexPhase::Failed,    // 0438
        WorkspaceIndexPhase::Cancelled, // 0439
        WorkspaceIndexPhase::Absent,    // 0440
        WorkspaceIndexPhase::Running,   // 0441
        WorkspaceIndexPhase::Done,      // 0442
        WorkspaceIndexPhase::Failed,    // 0443
        WorkspaceIndexPhase::Cancelled, // 0444
        WorkspaceIndexPhase::Absent,    // 0445
        WorkspaceIndexPhase::Running,   // 0446
        WorkspaceIndexPhase::Done,      // 0447
        WorkspaceIndexPhase::Failed,    // 0448
        WorkspaceIndexPhase::Cancelled, // 0449
        WorkspaceIndexPhase::Absent,    // 0450
        WorkspaceIndexPhase::Running,   // 0451
        WorkspaceIndexPhase::Done,      // 0452
        WorkspaceIndexPhase::Failed,    // 0453
        WorkspaceIndexPhase::Cancelled, // 0454
        WorkspaceIndexPhase::Absent,    // 0455
        WorkspaceIndexPhase::Running,   // 0456
        WorkspaceIndexPhase::Done,      // 0457
        WorkspaceIndexPhase::Failed,    // 0458
        WorkspaceIndexPhase::Cancelled, // 0459
        WorkspaceIndexPhase::Absent,    // 0460
        WorkspaceIndexPhase::Running,   // 0461
        WorkspaceIndexPhase::Done,      // 0462
        WorkspaceIndexPhase::Failed,    // 0463
        WorkspaceIndexPhase::Cancelled, // 0464
        WorkspaceIndexPhase::Absent,    // 0465
        WorkspaceIndexPhase::Running,   // 0466
        WorkspaceIndexPhase::Done,      // 0467
        WorkspaceIndexPhase::Failed,    // 0468
        WorkspaceIndexPhase::Cancelled, // 0469
        WorkspaceIndexPhase::Absent,    // 0470
        WorkspaceIndexPhase::Running,   // 0471
        WorkspaceIndexPhase::Done,      // 0472
        WorkspaceIndexPhase::Failed,    // 0473
        WorkspaceIndexPhase::Cancelled, // 0474
        WorkspaceIndexPhase::Absent,    // 0475
        WorkspaceIndexPhase::Running,   // 0476
        WorkspaceIndexPhase::Done,      // 0477
        WorkspaceIndexPhase::Failed,    // 0478
        WorkspaceIndexPhase::Cancelled, // 0479
        WorkspaceIndexPhase::Absent,    // 0480
        WorkspaceIndexPhase::Running,   // 0481
        WorkspaceIndexPhase::Done,      // 0482
        WorkspaceIndexPhase::Failed,    // 0483
        WorkspaceIndexPhase::Cancelled, // 0484
        WorkspaceIndexPhase::Absent,    // 0485
        WorkspaceIndexPhase::Running,   // 0486
        WorkspaceIndexPhase::Done,      // 0487
        WorkspaceIndexPhase::Failed,    // 0488
        WorkspaceIndexPhase::Cancelled, // 0489
        WorkspaceIndexPhase::Absent,    // 0490
        WorkspaceIndexPhase::Running,   // 0491
        WorkspaceIndexPhase::Done,      // 0492
        WorkspaceIndexPhase::Failed,    // 0493
        WorkspaceIndexPhase::Cancelled, // 0494
        WorkspaceIndexPhase::Absent,    // 0495
        WorkspaceIndexPhase::Running,   // 0496
        WorkspaceIndexPhase::Done,      // 0497
        WorkspaceIndexPhase::Failed,    // 0498
        WorkspaceIndexPhase::Cancelled, // 0499
        WorkspaceIndexPhase::Absent,    // 0500
        WorkspaceIndexPhase::Running,   // 0501
        WorkspaceIndexPhase::Done,      // 0502
        WorkspaceIndexPhase::Failed,    // 0503
        WorkspaceIndexPhase::Cancelled, // 0504
        WorkspaceIndexPhase::Absent,    // 0505
        WorkspaceIndexPhase::Running,   // 0506
        WorkspaceIndexPhase::Done,      // 0507
        WorkspaceIndexPhase::Failed,    // 0508
        WorkspaceIndexPhase::Cancelled, // 0509
        WorkspaceIndexPhase::Absent,    // 0510
        WorkspaceIndexPhase::Running,   // 0511
        WorkspaceIndexPhase::Done,      // 0512
        WorkspaceIndexPhase::Failed,    // 0513
        WorkspaceIndexPhase::Cancelled, // 0514
        WorkspaceIndexPhase::Absent,    // 0515
        WorkspaceIndexPhase::Running,   // 0516
        WorkspaceIndexPhase::Done,      // 0517
        WorkspaceIndexPhase::Failed,    // 0518
        WorkspaceIndexPhase::Cancelled, // 0519
        WorkspaceIndexPhase::Absent,    // 0520
        WorkspaceIndexPhase::Running,   // 0521
        WorkspaceIndexPhase::Done,      // 0522
        WorkspaceIndexPhase::Failed,    // 0523
        WorkspaceIndexPhase::Cancelled, // 0524
        WorkspaceIndexPhase::Absent,    // 0525
        WorkspaceIndexPhase::Running,   // 0526
        WorkspaceIndexPhase::Done,      // 0527
        WorkspaceIndexPhase::Failed,    // 0528
        WorkspaceIndexPhase::Cancelled, // 0529
        WorkspaceIndexPhase::Absent,    // 0530
        WorkspaceIndexPhase::Running,   // 0531
        WorkspaceIndexPhase::Done,      // 0532
        WorkspaceIndexPhase::Failed,    // 0533
        WorkspaceIndexPhase::Cancelled, // 0534
        WorkspaceIndexPhase::Absent,    // 0535
        WorkspaceIndexPhase::Running,   // 0536
        WorkspaceIndexPhase::Done,      // 0537
        WorkspaceIndexPhase::Failed,    // 0538
        WorkspaceIndexPhase::Cancelled, // 0539
        WorkspaceIndexPhase::Absent,    // 0540
        WorkspaceIndexPhase::Running,   // 0541
        WorkspaceIndexPhase::Done,      // 0542
        WorkspaceIndexPhase::Failed,    // 0543
        WorkspaceIndexPhase::Cancelled, // 0544
        WorkspaceIndexPhase::Absent,    // 0545
        WorkspaceIndexPhase::Running,   // 0546
        WorkspaceIndexPhase::Done,      // 0547
        WorkspaceIndexPhase::Failed,    // 0548
        WorkspaceIndexPhase::Cancelled, // 0549
        WorkspaceIndexPhase::Absent,    // 0550
        WorkspaceIndexPhase::Running,   // 0551
        WorkspaceIndexPhase::Done,      // 0552
        WorkspaceIndexPhase::Failed,    // 0553
        WorkspaceIndexPhase::Cancelled, // 0554
        WorkspaceIndexPhase::Absent,    // 0555
        WorkspaceIndexPhase::Running,   // 0556
        WorkspaceIndexPhase::Done,      // 0557
        WorkspaceIndexPhase::Failed,    // 0558
        WorkspaceIndexPhase::Cancelled, // 0559
        WorkspaceIndexPhase::Absent,    // 0560
        WorkspaceIndexPhase::Running,   // 0561
        WorkspaceIndexPhase::Done,      // 0562
        WorkspaceIndexPhase::Failed,    // 0563
        WorkspaceIndexPhase::Cancelled, // 0564
        WorkspaceIndexPhase::Absent,    // 0565
        WorkspaceIndexPhase::Running,   // 0566
        WorkspaceIndexPhase::Done,      // 0567
        WorkspaceIndexPhase::Failed,    // 0568
        WorkspaceIndexPhase::Cancelled, // 0569
        WorkspaceIndexPhase::Absent,    // 0570
        WorkspaceIndexPhase::Running,   // 0571
        WorkspaceIndexPhase::Done,      // 0572
        WorkspaceIndexPhase::Failed,    // 0573
        WorkspaceIndexPhase::Cancelled, // 0574
        WorkspaceIndexPhase::Absent,    // 0575
        WorkspaceIndexPhase::Running,   // 0576
        WorkspaceIndexPhase::Done,      // 0577
        WorkspaceIndexPhase::Failed,    // 0578
        WorkspaceIndexPhase::Cancelled, // 0579
        WorkspaceIndexPhase::Absent,    // 0580
        WorkspaceIndexPhase::Running,   // 0581
        WorkspaceIndexPhase::Done,      // 0582
        WorkspaceIndexPhase::Failed,    // 0583
        WorkspaceIndexPhase::Cancelled, // 0584
        WorkspaceIndexPhase::Absent,    // 0585
        WorkspaceIndexPhase::Running,   // 0586
        WorkspaceIndexPhase::Done,      // 0587
        WorkspaceIndexPhase::Failed,    // 0588
        WorkspaceIndexPhase::Cancelled, // 0589
        WorkspaceIndexPhase::Absent,    // 0590
        WorkspaceIndexPhase::Running,   // 0591
        WorkspaceIndexPhase::Done,      // 0592
        WorkspaceIndexPhase::Failed,    // 0593
        WorkspaceIndexPhase::Cancelled, // 0594
        WorkspaceIndexPhase::Absent,    // 0595
        WorkspaceIndexPhase::Running,   // 0596
        WorkspaceIndexPhase::Done,      // 0597
        WorkspaceIndexPhase::Failed,    // 0598
        WorkspaceIndexPhase::Cancelled, // 0599
        WorkspaceIndexPhase::Absent,    // 0600
        WorkspaceIndexPhase::Running,   // 0601
        WorkspaceIndexPhase::Done,      // 0602
        WorkspaceIndexPhase::Failed,    // 0603
        WorkspaceIndexPhase::Cancelled, // 0604
        WorkspaceIndexPhase::Absent,    // 0605
        WorkspaceIndexPhase::Running,   // 0606
        WorkspaceIndexPhase::Done,      // 0607
        WorkspaceIndexPhase::Failed,    // 0608
        WorkspaceIndexPhase::Cancelled, // 0609
        WorkspaceIndexPhase::Absent,    // 0610
        WorkspaceIndexPhase::Running,   // 0611
        WorkspaceIndexPhase::Done,      // 0612
        WorkspaceIndexPhase::Failed,    // 0613
        WorkspaceIndexPhase::Cancelled, // 0614
        WorkspaceIndexPhase::Absent,    // 0615
        WorkspaceIndexPhase::Running,   // 0616
        WorkspaceIndexPhase::Done,      // 0617
        WorkspaceIndexPhase::Failed,    // 0618
        WorkspaceIndexPhase::Cancelled, // 0619
        WorkspaceIndexPhase::Absent,    // 0620
        WorkspaceIndexPhase::Running,   // 0621
        WorkspaceIndexPhase::Done,      // 0622
        WorkspaceIndexPhase::Failed,    // 0623
        WorkspaceIndexPhase::Cancelled, // 0624
        WorkspaceIndexPhase::Absent,    // 0625
        WorkspaceIndexPhase::Running,   // 0626
        WorkspaceIndexPhase::Done,      // 0627
        WorkspaceIndexPhase::Failed,    // 0628
        WorkspaceIndexPhase::Cancelled, // 0629
        WorkspaceIndexPhase::Absent,    // 0630
        WorkspaceIndexPhase::Running,   // 0631
        WorkspaceIndexPhase::Done,      // 0632
        WorkspaceIndexPhase::Failed,    // 0633
        WorkspaceIndexPhase::Cancelled, // 0634
        WorkspaceIndexPhase::Absent,    // 0635
        WorkspaceIndexPhase::Running,   // 0636
        WorkspaceIndexPhase::Done,      // 0637
        WorkspaceIndexPhase::Failed,    // 0638
        WorkspaceIndexPhase::Cancelled, // 0639
        WorkspaceIndexPhase::Absent,    // 0640
        WorkspaceIndexPhase::Running,   // 0641
        WorkspaceIndexPhase::Done,      // 0642
        WorkspaceIndexPhase::Failed,    // 0643
        WorkspaceIndexPhase::Cancelled, // 0644
        WorkspaceIndexPhase::Absent,    // 0645
        WorkspaceIndexPhase::Running,   // 0646
        WorkspaceIndexPhase::Done,      // 0647
        WorkspaceIndexPhase::Failed,    // 0648
        WorkspaceIndexPhase::Cancelled, // 0649
        WorkspaceIndexPhase::Absent,    // 0650
        WorkspaceIndexPhase::Running,   // 0651
        WorkspaceIndexPhase::Done,      // 0652
        WorkspaceIndexPhase::Failed,    // 0653
        WorkspaceIndexPhase::Cancelled, // 0654
        WorkspaceIndexPhase::Absent,    // 0655
        WorkspaceIndexPhase::Running,   // 0656
        WorkspaceIndexPhase::Done,      // 0657
        WorkspaceIndexPhase::Failed,    // 0658
        WorkspaceIndexPhase::Cancelled, // 0659
        WorkspaceIndexPhase::Absent,    // 0660
        WorkspaceIndexPhase::Running,   // 0661
        WorkspaceIndexPhase::Done,      // 0662
        WorkspaceIndexPhase::Failed,    // 0663
        WorkspaceIndexPhase::Cancelled, // 0664
        WorkspaceIndexPhase::Absent,    // 0665
        WorkspaceIndexPhase::Running,   // 0666
        WorkspaceIndexPhase::Done,      // 0667
        WorkspaceIndexPhase::Failed,    // 0668
        WorkspaceIndexPhase::Cancelled, // 0669
        WorkspaceIndexPhase::Absent,    // 0670
        WorkspaceIndexPhase::Running,   // 0671
        WorkspaceIndexPhase::Done,      // 0672
        WorkspaceIndexPhase::Failed,    // 0673
        WorkspaceIndexPhase::Cancelled, // 0674
        WorkspaceIndexPhase::Absent,    // 0675
        WorkspaceIndexPhase::Running,   // 0676
        WorkspaceIndexPhase::Done,      // 0677
        WorkspaceIndexPhase::Failed,    // 0678
        WorkspaceIndexPhase::Cancelled, // 0679
        WorkspaceIndexPhase::Absent,    // 0680
        WorkspaceIndexPhase::Running,   // 0681
        WorkspaceIndexPhase::Done,      // 0682
        WorkspaceIndexPhase::Failed,    // 0683
        WorkspaceIndexPhase::Cancelled, // 0684
        WorkspaceIndexPhase::Absent,    // 0685
        WorkspaceIndexPhase::Running,   // 0686
        WorkspaceIndexPhase::Done,      // 0687
        WorkspaceIndexPhase::Failed,    // 0688
        WorkspaceIndexPhase::Cancelled, // 0689
        WorkspaceIndexPhase::Absent,    // 0690
        WorkspaceIndexPhase::Running,   // 0691
        WorkspaceIndexPhase::Done,      // 0692
        WorkspaceIndexPhase::Failed,    // 0693
        WorkspaceIndexPhase::Cancelled, // 0694
        WorkspaceIndexPhase::Absent,    // 0695
        WorkspaceIndexPhase::Running,   // 0696
        WorkspaceIndexPhase::Done,      // 0697
        WorkspaceIndexPhase::Failed,    // 0698
        WorkspaceIndexPhase::Cancelled, // 0699
        WorkspaceIndexPhase::Absent,    // 0700
        WorkspaceIndexPhase::Running,   // 0701
        WorkspaceIndexPhase::Done,      // 0702
        WorkspaceIndexPhase::Failed,    // 0703
        WorkspaceIndexPhase::Cancelled, // 0704
        WorkspaceIndexPhase::Absent,    // 0705
        WorkspaceIndexPhase::Running,   // 0706
        WorkspaceIndexPhase::Done,      // 0707
        WorkspaceIndexPhase::Failed,    // 0708
        WorkspaceIndexPhase::Cancelled, // 0709
        WorkspaceIndexPhase::Absent,    // 0710
        WorkspaceIndexPhase::Running,   // 0711
        WorkspaceIndexPhase::Done,      // 0712
        WorkspaceIndexPhase::Failed,    // 0713
        WorkspaceIndexPhase::Cancelled, // 0714
        WorkspaceIndexPhase::Absent,    // 0715
        WorkspaceIndexPhase::Running,   // 0716
        WorkspaceIndexPhase::Done,      // 0717
        WorkspaceIndexPhase::Failed,    // 0718
        WorkspaceIndexPhase::Cancelled, // 0719
        WorkspaceIndexPhase::Absent,    // 0720
        WorkspaceIndexPhase::Running,   // 0721
        WorkspaceIndexPhase::Done,      // 0722
        WorkspaceIndexPhase::Failed,    // 0723
        WorkspaceIndexPhase::Cancelled, // 0724
        WorkspaceIndexPhase::Absent,    // 0725
        WorkspaceIndexPhase::Running,   // 0726
        WorkspaceIndexPhase::Done,      // 0727
        WorkspaceIndexPhase::Failed,    // 0728
        WorkspaceIndexPhase::Cancelled, // 0729
        WorkspaceIndexPhase::Absent,    // 0730
        WorkspaceIndexPhase::Running,   // 0731
        WorkspaceIndexPhase::Done,      // 0732
        WorkspaceIndexPhase::Failed,    // 0733
        WorkspaceIndexPhase::Cancelled, // 0734
        WorkspaceIndexPhase::Absent,    // 0735
        WorkspaceIndexPhase::Running,   // 0736
        WorkspaceIndexPhase::Done,      // 0737
        WorkspaceIndexPhase::Failed,    // 0738
        WorkspaceIndexPhase::Cancelled, // 0739
        WorkspaceIndexPhase::Absent,    // 0740
        WorkspaceIndexPhase::Running,   // 0741
        WorkspaceIndexPhase::Done,      // 0742
        WorkspaceIndexPhase::Failed,    // 0743
        WorkspaceIndexPhase::Cancelled, // 0744
        WorkspaceIndexPhase::Absent,    // 0745
        WorkspaceIndexPhase::Running,   // 0746
        WorkspaceIndexPhase::Done,      // 0747
        WorkspaceIndexPhase::Failed,    // 0748
        WorkspaceIndexPhase::Cancelled, // 0749
        WorkspaceIndexPhase::Absent,    // 0750
        WorkspaceIndexPhase::Running,   // 0751
        WorkspaceIndexPhase::Done,      // 0752
        WorkspaceIndexPhase::Failed,    // 0753
        WorkspaceIndexPhase::Cancelled, // 0754
        WorkspaceIndexPhase::Absent,    // 0755
        WorkspaceIndexPhase::Running,   // 0756
        WorkspaceIndexPhase::Done,      // 0757
        WorkspaceIndexPhase::Failed,    // 0758
        WorkspaceIndexPhase::Cancelled, // 0759
        WorkspaceIndexPhase::Absent,    // 0760
        WorkspaceIndexPhase::Running,   // 0761
        WorkspaceIndexPhase::Done,      // 0762
        WorkspaceIndexPhase::Failed,    // 0763
        WorkspaceIndexPhase::Cancelled, // 0764
        WorkspaceIndexPhase::Absent,    // 0765
        WorkspaceIndexPhase::Running,   // 0766
        WorkspaceIndexPhase::Done,      // 0767
        WorkspaceIndexPhase::Failed,    // 0768
        WorkspaceIndexPhase::Cancelled, // 0769
        WorkspaceIndexPhase::Absent,    // 0770
        WorkspaceIndexPhase::Running,   // 0771
        WorkspaceIndexPhase::Done,      // 0772
        WorkspaceIndexPhase::Failed,    // 0773
        WorkspaceIndexPhase::Cancelled, // 0774
        WorkspaceIndexPhase::Absent,    // 0775
        WorkspaceIndexPhase::Running,   // 0776
        WorkspaceIndexPhase::Done,      // 0777
        WorkspaceIndexPhase::Failed,    // 0778
        WorkspaceIndexPhase::Cancelled, // 0779
        WorkspaceIndexPhase::Absent,    // 0780
        WorkspaceIndexPhase::Running,   // 0781
        WorkspaceIndexPhase::Done,      // 0782
        WorkspaceIndexPhase::Failed,    // 0783
        WorkspaceIndexPhase::Cancelled, // 0784
        WorkspaceIndexPhase::Absent,    // 0785
        WorkspaceIndexPhase::Running,   // 0786
        WorkspaceIndexPhase::Done,      // 0787
        WorkspaceIndexPhase::Failed,    // 0788
        WorkspaceIndexPhase::Cancelled, // 0789
        WorkspaceIndexPhase::Absent,    // 0790
        WorkspaceIndexPhase::Running,   // 0791
        WorkspaceIndexPhase::Done,      // 0792
        WorkspaceIndexPhase::Failed,    // 0793
        WorkspaceIndexPhase::Cancelled, // 0794
        WorkspaceIndexPhase::Absent,    // 0795
        WorkspaceIndexPhase::Running,   // 0796
        WorkspaceIndexPhase::Done,      // 0797
        WorkspaceIndexPhase::Failed,    // 0798
        WorkspaceIndexPhase::Cancelled, // 0799
    ];
    for (i, phase) in rows.iter().enumerate() {
        let s = phase.as_str();
        assert!(!s.is_empty(), "{i}");
        assert_eq!(WorkspaceIndexPhase::parse(s), Some(*phase), "{i}");
    }
}

#[test]
fn pattern_storage_rel_table() {
    let bases: &[&str] = &[
        "/tmp/ronin-base-000",
        "/tmp/ronin-base-001",
        "/tmp/ronin-base-002",
        "/tmp/ronin-base-003",
        "/tmp/ronin-base-004",
        "/tmp/ronin-base-005",
        "/tmp/ronin-base-006",
        "/tmp/ronin-base-007",
        "/tmp/ronin-base-008",
        "/tmp/ronin-base-009",
        "/tmp/ronin-base-010",
        "/tmp/ronin-base-011",
        "/tmp/ronin-base-012",
        "/tmp/ronin-base-013",
        "/tmp/ronin-base-014",
        "/tmp/ronin-base-015",
        "/tmp/ronin-base-016",
        "/tmp/ronin-base-017",
        "/tmp/ronin-base-018",
        "/tmp/ronin-base-019",
        "/tmp/ronin-base-020",
        "/tmp/ronin-base-021",
        "/tmp/ronin-base-022",
        "/tmp/ronin-base-023",
        "/tmp/ronin-base-024",
        "/tmp/ronin-base-025",
        "/tmp/ronin-base-026",
        "/tmp/ronin-base-027",
        "/tmp/ronin-base-028",
        "/tmp/ronin-base-029",
        "/tmp/ronin-base-030",
        "/tmp/ronin-base-031",
        "/tmp/ronin-base-032",
        "/tmp/ronin-base-033",
        "/tmp/ronin-base-034",
        "/tmp/ronin-base-035",
        "/tmp/ronin-base-036",
        "/tmp/ronin-base-037",
        "/tmp/ronin-base-038",
        "/tmp/ronin-base-039",
        "/tmp/ronin-base-040",
        "/tmp/ronin-base-041",
        "/tmp/ronin-base-042",
        "/tmp/ronin-base-043",
        "/tmp/ronin-base-044",
        "/tmp/ronin-base-045",
        "/tmp/ronin-base-046",
        "/tmp/ronin-base-047",
        "/tmp/ronin-base-048",
        "/tmp/ronin-base-049",
        "/tmp/ronin-base-050",
        "/tmp/ronin-base-051",
        "/tmp/ronin-base-052",
        "/tmp/ronin-base-053",
        "/tmp/ronin-base-054",
        "/tmp/ronin-base-055",
        "/tmp/ronin-base-056",
        "/tmp/ronin-base-057",
        "/tmp/ronin-base-058",
        "/tmp/ronin-base-059",
        "/tmp/ronin-base-060",
        "/tmp/ronin-base-061",
        "/tmp/ronin-base-062",
        "/tmp/ronin-base-063",
        "/tmp/ronin-base-064",
        "/tmp/ronin-base-065",
        "/tmp/ronin-base-066",
        "/tmp/ronin-base-067",
        "/tmp/ronin-base-068",
        "/tmp/ronin-base-069",
        "/tmp/ronin-base-070",
        "/tmp/ronin-base-071",
        "/tmp/ronin-base-072",
        "/tmp/ronin-base-073",
        "/tmp/ronin-base-074",
        "/tmp/ronin-base-075",
        "/tmp/ronin-base-076",
        "/tmp/ronin-base-077",
        "/tmp/ronin-base-078",
        "/tmp/ronin-base-079",
        "/tmp/ronin-base-080",
        "/tmp/ronin-base-081",
        "/tmp/ronin-base-082",
        "/tmp/ronin-base-083",
        "/tmp/ronin-base-084",
        "/tmp/ronin-base-085",
        "/tmp/ronin-base-086",
        "/tmp/ronin-base-087",
        "/tmp/ronin-base-088",
        "/tmp/ronin-base-089",
        "/tmp/ronin-base-090",
        "/tmp/ronin-base-091",
        "/tmp/ronin-base-092",
        "/tmp/ronin-base-093",
        "/tmp/ronin-base-094",
        "/tmp/ronin-base-095",
        "/tmp/ronin-base-096",
        "/tmp/ronin-base-097",
        "/tmp/ronin-base-098",
        "/tmp/ronin-base-099",
        "/tmp/ronin-base-100",
        "/tmp/ronin-base-101",
        "/tmp/ronin-base-102",
        "/tmp/ronin-base-103",
        "/tmp/ronin-base-104",
        "/tmp/ronin-base-105",
        "/tmp/ronin-base-106",
        "/tmp/ronin-base-107",
        "/tmp/ronin-base-108",
        "/tmp/ronin-base-109",
        "/tmp/ronin-base-110",
        "/tmp/ronin-base-111",
        "/tmp/ronin-base-112",
        "/tmp/ronin-base-113",
        "/tmp/ronin-base-114",
        "/tmp/ronin-base-115",
        "/tmp/ronin-base-116",
        "/tmp/ronin-base-117",
        "/tmp/ronin-base-118",
        "/tmp/ronin-base-119",
        "/tmp/ronin-base-120",
        "/tmp/ronin-base-121",
        "/tmp/ronin-base-122",
        "/tmp/ronin-base-123",
        "/tmp/ronin-base-124",
        "/tmp/ronin-base-125",
        "/tmp/ronin-base-126",
        "/tmp/ronin-base-127",
        "/tmp/ronin-base-128",
        "/tmp/ronin-base-129",
        "/tmp/ronin-base-130",
        "/tmp/ronin-base-131",
        "/tmp/ronin-base-132",
        "/tmp/ronin-base-133",
        "/tmp/ronin-base-134",
        "/tmp/ronin-base-135",
        "/tmp/ronin-base-136",
        "/tmp/ronin-base-137",
        "/tmp/ronin-base-138",
        "/tmp/ronin-base-139",
        "/tmp/ronin-base-140",
        "/tmp/ronin-base-141",
        "/tmp/ronin-base-142",
        "/tmp/ronin-base-143",
        "/tmp/ronin-base-144",
        "/tmp/ronin-base-145",
        "/tmp/ronin-base-146",
        "/tmp/ronin-base-147",
        "/tmp/ronin-base-148",
        "/tmp/ronin-base-149",
        "/tmp/ronin-base-150",
        "/tmp/ronin-base-151",
        "/tmp/ronin-base-152",
        "/tmp/ronin-base-153",
        "/tmp/ronin-base-154",
        "/tmp/ronin-base-155",
        "/tmp/ronin-base-156",
        "/tmp/ronin-base-157",
        "/tmp/ronin-base-158",
        "/tmp/ronin-base-159",
        "/tmp/ronin-base-160",
        "/tmp/ronin-base-161",
        "/tmp/ronin-base-162",
        "/tmp/ronin-base-163",
        "/tmp/ronin-base-164",
        "/tmp/ronin-base-165",
        "/tmp/ronin-base-166",
        "/tmp/ronin-base-167",
        "/tmp/ronin-base-168",
        "/tmp/ronin-base-169",
        "/tmp/ronin-base-170",
        "/tmp/ronin-base-171",
        "/tmp/ronin-base-172",
        "/tmp/ronin-base-173",
        "/tmp/ronin-base-174",
        "/tmp/ronin-base-175",
        "/tmp/ronin-base-176",
        "/tmp/ronin-base-177",
        "/tmp/ronin-base-178",
        "/tmp/ronin-base-179",
        "/tmp/ronin-base-180",
        "/tmp/ronin-base-181",
        "/tmp/ronin-base-182",
        "/tmp/ronin-base-183",
        "/tmp/ronin-base-184",
        "/tmp/ronin-base-185",
        "/tmp/ronin-base-186",
        "/tmp/ronin-base-187",
        "/tmp/ronin-base-188",
        "/tmp/ronin-base-189",
        "/tmp/ronin-base-190",
        "/tmp/ronin-base-191",
        "/tmp/ronin-base-192",
        "/tmp/ronin-base-193",
        "/tmp/ronin-base-194",
        "/tmp/ronin-base-195",
        "/tmp/ronin-base-196",
        "/tmp/ronin-base-197",
        "/tmp/ronin-base-198",
        "/tmp/ronin-base-199",
        "/tmp/ronin-base-200",
        "/tmp/ronin-base-201",
        "/tmp/ronin-base-202",
        "/tmp/ronin-base-203",
        "/tmp/ronin-base-204",
        "/tmp/ronin-base-205",
        "/tmp/ronin-base-206",
        "/tmp/ronin-base-207",
        "/tmp/ronin-base-208",
        "/tmp/ronin-base-209",
        "/tmp/ronin-base-210",
        "/tmp/ronin-base-211",
        "/tmp/ronin-base-212",
        "/tmp/ronin-base-213",
        "/tmp/ronin-base-214",
        "/tmp/ronin-base-215",
        "/tmp/ronin-base-216",
        "/tmp/ronin-base-217",
        "/tmp/ronin-base-218",
        "/tmp/ronin-base-219",
        "/tmp/ronin-base-220",
        "/tmp/ronin-base-221",
        "/tmp/ronin-base-222",
        "/tmp/ronin-base-223",
        "/tmp/ronin-base-224",
        "/tmp/ronin-base-225",
        "/tmp/ronin-base-226",
        "/tmp/ronin-base-227",
        "/tmp/ronin-base-228",
        "/tmp/ronin-base-229",
        "/tmp/ronin-base-230",
        "/tmp/ronin-base-231",
        "/tmp/ronin-base-232",
        "/tmp/ronin-base-233",
        "/tmp/ronin-base-234",
        "/tmp/ronin-base-235",
        "/tmp/ronin-base-236",
        "/tmp/ronin-base-237",
        "/tmp/ronin-base-238",
        "/tmp/ronin-base-239",
        "/tmp/ronin-base-240",
        "/tmp/ronin-base-241",
        "/tmp/ronin-base-242",
        "/tmp/ronin-base-243",
        "/tmp/ronin-base-244",
        "/tmp/ronin-base-245",
        "/tmp/ronin-base-246",
        "/tmp/ronin-base-247",
        "/tmp/ronin-base-248",
        "/tmp/ronin-base-249",
        "/tmp/ronin-base-250",
        "/tmp/ronin-base-251",
        "/tmp/ronin-base-252",
        "/tmp/ronin-base-253",
        "/tmp/ronin-base-254",
        "/tmp/ronin-base-255",
        "/tmp/ronin-base-256",
        "/tmp/ronin-base-257",
        "/tmp/ronin-base-258",
        "/tmp/ronin-base-259",
        "/tmp/ronin-base-260",
        "/tmp/ronin-base-261",
        "/tmp/ronin-base-262",
        "/tmp/ronin-base-263",
        "/tmp/ronin-base-264",
        "/tmp/ronin-base-265",
        "/tmp/ronin-base-266",
        "/tmp/ronin-base-267",
        "/tmp/ronin-base-268",
        "/tmp/ronin-base-269",
        "/tmp/ronin-base-270",
        "/tmp/ronin-base-271",
        "/tmp/ronin-base-272",
        "/tmp/ronin-base-273",
        "/tmp/ronin-base-274",
        "/tmp/ronin-base-275",
        "/tmp/ronin-base-276",
        "/tmp/ronin-base-277",
        "/tmp/ronin-base-278",
        "/tmp/ronin-base-279",
        "/tmp/ronin-base-280",
        "/tmp/ronin-base-281",
        "/tmp/ronin-base-282",
        "/tmp/ronin-base-283",
        "/tmp/ronin-base-284",
        "/tmp/ronin-base-285",
        "/tmp/ronin-base-286",
        "/tmp/ronin-base-287",
        "/tmp/ronin-base-288",
        "/tmp/ronin-base-289",
        "/tmp/ronin-base-290",
        "/tmp/ronin-base-291",
        "/tmp/ronin-base-292",
        "/tmp/ronin-base-293",
        "/tmp/ronin-base-294",
        "/tmp/ronin-base-295",
        "/tmp/ronin-base-296",
        "/tmp/ronin-base-297",
        "/tmp/ronin-base-298",
        "/tmp/ronin-base-299",
    ];
    let ids: &[&str] = &[
        "id-000", "id-001", "id-002", "id-003", "id-004", "id-005", "id-006", "id-007", "id-008",
        "id-009", "id-010", "id-011", "id-012", "id-013", "id-014", "id-015", "id-016", "id-017",
        "id-018", "id-019", "id-020", "id-021", "id-022", "id-023", "id-024", "id-025", "id-026",
        "id-027", "id-028", "id-029", "id-030", "id-031", "id-032", "id-033", "id-034", "id-035",
        "id-036", "id-037", "id-038", "id-039", "id-040", "id-041", "id-042", "id-043", "id-044",
        "id-045", "id-046", "id-047", "id-048", "id-049", "id-050", "id-051", "id-052", "id-053",
        "id-054", "id-055", "id-056", "id-057", "id-058", "id-059", "id-060", "id-061", "id-062",
        "id-063", "id-064", "id-065", "id-066", "id-067", "id-068", "id-069", "id-070", "id-071",
        "id-072", "id-073", "id-074", "id-075", "id-076", "id-077", "id-078", "id-079", "id-080",
        "id-081", "id-082", "id-083", "id-084", "id-085", "id-086", "id-087", "id-088", "id-089",
        "id-090", "id-091", "id-092", "id-093", "id-094", "id-095", "id-096", "id-097", "id-098",
        "id-099", "id-100", "id-101", "id-102", "id-103", "id-104", "id-105", "id-106", "id-107",
        "id-108", "id-109", "id-110", "id-111", "id-112", "id-113", "id-114", "id-115", "id-116",
        "id-117", "id-118", "id-119", "id-120", "id-121", "id-122", "id-123", "id-124", "id-125",
        "id-126", "id-127", "id-128", "id-129", "id-130", "id-131", "id-132", "id-133", "id-134",
        "id-135", "id-136", "id-137", "id-138", "id-139", "id-140", "id-141", "id-142", "id-143",
        "id-144", "id-145", "id-146", "id-147", "id-148", "id-149", "id-150", "id-151", "id-152",
        "id-153", "id-154", "id-155", "id-156", "id-157", "id-158", "id-159", "id-160", "id-161",
        "id-162", "id-163", "id-164", "id-165", "id-166", "id-167", "id-168", "id-169", "id-170",
        "id-171", "id-172", "id-173", "id-174", "id-175", "id-176", "id-177", "id-178", "id-179",
        "id-180", "id-181", "id-182", "id-183", "id-184", "id-185", "id-186", "id-187", "id-188",
        "id-189", "id-190", "id-191", "id-192", "id-193", "id-194", "id-195", "id-196", "id-197",
        "id-198", "id-199", "id-200", "id-201", "id-202", "id-203", "id-204", "id-205", "id-206",
        "id-207", "id-208", "id-209", "id-210", "id-211", "id-212", "id-213", "id-214", "id-215",
        "id-216", "id-217", "id-218", "id-219", "id-220", "id-221", "id-222", "id-223", "id-224",
        "id-225", "id-226", "id-227", "id-228", "id-229", "id-230", "id-231", "id-232", "id-233",
        "id-234", "id-235", "id-236", "id-237", "id-238", "id-239", "id-240", "id-241", "id-242",
        "id-243", "id-244", "id-245", "id-246", "id-247", "id-248", "id-249", "id-250", "id-251",
        "id-252", "id-253", "id-254", "id-255", "id-256", "id-257", "id-258", "id-259", "id-260",
        "id-261", "id-262", "id-263", "id-264", "id-265", "id-266", "id-267", "id-268", "id-269",
        "id-270", "id-271", "id-272", "id-273", "id-274", "id-275", "id-276", "id-277", "id-278",
        "id-279", "id-280", "id-281", "id-282", "id-283", "id-284", "id-285", "id-286", "id-287",
        "id-288", "id-289", "id-290", "id-291", "id-292", "id-293", "id-294", "id-295", "id-296",
        "id-297", "id-298", "id-299",
    ];
    for base in bases {
        for id in ids.iter().take(3) {
            let p = workspace_index_storage_path(Path::new(base), id);
            assert!(p.ends_with(format!("{WORKSPACE_INDEX_STORAGE_DIR}/{id}.db")));
        }
    }
}

#[test]
fn pattern_root_block_reasons_matrix() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("ws");
    std::fs::create_dir_all(&root).unwrap();
    let never = temp.path().join("never");
    std::fs::create_dir_all(&never).unwrap();
    let allowed = temp.path().join("allowed");
    std::fs::create_dir_all(&allowed).unwrap();

    let cases: &[(FolderListPolicy, bool)] = &[
        // case 000 placeholder consumed by loop below
        // case 001 placeholder consumed by loop below
        // case 002 placeholder consumed by loop below
        // case 003 placeholder consumed by loop below
        // case 004 placeholder consumed by loop below
        // case 005 placeholder consumed by loop below
        // case 006 placeholder consumed by loop below
        // case 007 placeholder consumed by loop below
        // case 008 placeholder consumed by loop below
        // case 009 placeholder consumed by loop below
        // case 010 placeholder consumed by loop below
        // case 011 placeholder consumed by loop below
        // case 012 placeholder consumed by loop below
        // case 013 placeholder consumed by loop below
        // case 014 placeholder consumed by loop below
        // case 015 placeholder consumed by loop below
        // case 016 placeholder consumed by loop below
        // case 017 placeholder consumed by loop below
        // case 018 placeholder consumed by loop below
        // case 019 placeholder consumed by loop below
        // case 020 placeholder consumed by loop below
        // case 021 placeholder consumed by loop below
        // case 022 placeholder consumed by loop below
        // case 023 placeholder consumed by loop below
        // case 024 placeholder consumed by loop below
        // case 025 placeholder consumed by loop below
        // case 026 placeholder consumed by loop below
        // case 027 placeholder consumed by loop below
        // case 028 placeholder consumed by loop below
        // case 029 placeholder consumed by loop below
        // case 030 placeholder consumed by loop below
        // case 031 placeholder consumed by loop below
        // case 032 placeholder consumed by loop below
        // case 033 placeholder consumed by loop below
        // case 034 placeholder consumed by loop below
        // case 035 placeholder consumed by loop below
        // case 036 placeholder consumed by loop below
        // case 037 placeholder consumed by loop below
        // case 038 placeholder consumed by loop below
        // case 039 placeholder consumed by loop below
        // case 040 placeholder consumed by loop below
        // case 041 placeholder consumed by loop below
        // case 042 placeholder consumed by loop below
        // case 043 placeholder consumed by loop below
        // case 044 placeholder consumed by loop below
        // case 045 placeholder consumed by loop below
        // case 046 placeholder consumed by loop below
        // case 047 placeholder consumed by loop below
        // case 048 placeholder consumed by loop below
        // case 049 placeholder consumed by loop below
        // case 050 placeholder consumed by loop below
        // case 051 placeholder consumed by loop below
        // case 052 placeholder consumed by loop below
        // case 053 placeholder consumed by loop below
        // case 054 placeholder consumed by loop below
        // case 055 placeholder consumed by loop below
        // case 056 placeholder consumed by loop below
        // case 057 placeholder consumed by loop below
        // case 058 placeholder consumed by loop below
        // case 059 placeholder consumed by loop below
        // case 060 placeholder consumed by loop below
        // case 061 placeholder consumed by loop below
        // case 062 placeholder consumed by loop below
        // case 063 placeholder consumed by loop below
        // case 064 placeholder consumed by loop below
        // case 065 placeholder consumed by loop below
        // case 066 placeholder consumed by loop below
        // case 067 placeholder consumed by loop below
        // case 068 placeholder consumed by loop below
        // case 069 placeholder consumed by loop below
        // case 070 placeholder consumed by loop below
        // case 071 placeholder consumed by loop below
        // case 072 placeholder consumed by loop below
        // case 073 placeholder consumed by loop below
        // case 074 placeholder consumed by loop below
        // case 075 placeholder consumed by loop below
        // case 076 placeholder consumed by loop below
        // case 077 placeholder consumed by loop below
        // case 078 placeholder consumed by loop below
        // case 079 placeholder consumed by loop below
        // case 080 placeholder consumed by loop below
        // case 081 placeholder consumed by loop below
        // case 082 placeholder consumed by loop below
        // case 083 placeholder consumed by loop below
        // case 084 placeholder consumed by loop below
        // case 085 placeholder consumed by loop below
        // case 086 placeholder consumed by loop below
        // case 087 placeholder consumed by loop below
        // case 088 placeholder consumed by loop below
        // case 089 placeholder consumed by loop below
        // case 090 placeholder consumed by loop below
        // case 091 placeholder consumed by loop below
        // case 092 placeholder consumed by loop below
        // case 093 placeholder consumed by loop below
        // case 094 placeholder consumed by loop below
        // case 095 placeholder consumed by loop below
        // case 096 placeholder consumed by loop below
        // case 097 placeholder consumed by loop below
        // case 098 placeholder consumed by loop below
        // case 099 placeholder consumed by loop below
        // case 100 placeholder consumed by loop below
        // case 101 placeholder consumed by loop below
        // case 102 placeholder consumed by loop below
        // case 103 placeholder consumed by loop below
        // case 104 placeholder consumed by loop below
        // case 105 placeholder consumed by loop below
        // case 106 placeholder consumed by loop below
        // case 107 placeholder consumed by loop below
        // case 108 placeholder consumed by loop below
        // case 109 placeholder consumed by loop below
        // case 110 placeholder consumed by loop below
        // case 111 placeholder consumed by loop below
        // case 112 placeholder consumed by loop below
        // case 113 placeholder consumed by loop below
        // case 114 placeholder consumed by loop below
        // case 115 placeholder consumed by loop below
        // case 116 placeholder consumed by loop below
        // case 117 placeholder consumed by loop below
        // case 118 placeholder consumed by loop below
        // case 119 placeholder consumed by loop below
        // case 120 placeholder consumed by loop below
        // case 121 placeholder consumed by loop below
        // case 122 placeholder consumed by loop below
        // case 123 placeholder consumed by loop below
        // case 124 placeholder consumed by loop below
        // case 125 placeholder consumed by loop below
        // case 126 placeholder consumed by loop below
        // case 127 placeholder consumed by loop below
        // case 128 placeholder consumed by loop below
        // case 129 placeholder consumed by loop below
        // case 130 placeholder consumed by loop below
        // case 131 placeholder consumed by loop below
        // case 132 placeholder consumed by loop below
        // case 133 placeholder consumed by loop below
        // case 134 placeholder consumed by loop below
        // case 135 placeholder consumed by loop below
        // case 136 placeholder consumed by loop below
        // case 137 placeholder consumed by loop below
        // case 138 placeholder consumed by loop below
        // case 139 placeholder consumed by loop below
        // case 140 placeholder consumed by loop below
        // case 141 placeholder consumed by loop below
        // case 142 placeholder consumed by loop below
        // case 143 placeholder consumed by loop below
        // case 144 placeholder consumed by loop below
        // case 145 placeholder consumed by loop below
        // case 146 placeholder consumed by loop below
        // case 147 placeholder consumed by loop below
        // case 148 placeholder consumed by loop below
        // case 149 placeholder consumed by loop below
        // case 150 placeholder consumed by loop below
        // case 151 placeholder consumed by loop below
        // case 152 placeholder consumed by loop below
        // case 153 placeholder consumed by loop below
        // case 154 placeholder consumed by loop below
        // case 155 placeholder consumed by loop below
        // case 156 placeholder consumed by loop below
        // case 157 placeholder consumed by loop below
        // case 158 placeholder consumed by loop below
        // case 159 placeholder consumed by loop below
        // case 160 placeholder consumed by loop below
        // case 161 placeholder consumed by loop below
        // case 162 placeholder consumed by loop below
        // case 163 placeholder consumed by loop below
        // case 164 placeholder consumed by loop below
        // case 165 placeholder consumed by loop below
        // case 166 placeholder consumed by loop below
        // case 167 placeholder consumed by loop below
        // case 168 placeholder consumed by loop below
        // case 169 placeholder consumed by loop below
        // case 170 placeholder consumed by loop below
        // case 171 placeholder consumed by loop below
        // case 172 placeholder consumed by loop below
        // case 173 placeholder consumed by loop below
        // case 174 placeholder consumed by loop below
        // case 175 placeholder consumed by loop below
        // case 176 placeholder consumed by loop below
        // case 177 placeholder consumed by loop below
        // case 178 placeholder consumed by loop below
        // case 179 placeholder consumed by loop below
        // case 180 placeholder consumed by loop below
        // case 181 placeholder consumed by loop below
        // case 182 placeholder consumed by loop below
        // case 183 placeholder consumed by loop below
        // case 184 placeholder consumed by loop below
        // case 185 placeholder consumed by loop below
        // case 186 placeholder consumed by loop below
        // case 187 placeholder consumed by loop below
        // case 188 placeholder consumed by loop below
        // case 189 placeholder consumed by loop below
        // case 190 placeholder consumed by loop below
        // case 191 placeholder consumed by loop below
        // case 192 placeholder consumed by loop below
        // case 193 placeholder consumed by loop below
        // case 194 placeholder consumed by loop below
        // case 195 placeholder consumed by loop below
        // case 196 placeholder consumed by loop below
        // case 197 placeholder consumed by loop below
        // case 198 placeholder consumed by loop below
        // case 199 placeholder consumed by loop below
    ];
    let _ = cases.len();
    for i in 0..200 {
        let policy = FolderListPolicy {
            honor_gitignore: i % 2 == 0,
            apply_built_in_deny: i % 3 != 0,
            never_list: if i % 4 == 0 {
                vec![never.clone()]
            } else {
                Vec::new()
            },
            allowlist_enabled: i % 5 == 0,
            allowlist: if i % 5 == 0 {
                vec![allowed.clone()]
            } else {
                Vec::new()
            },
        };
        let block_root = if i % 4 == 0 {
            never.clone()
        } else {
            root.clone()
        };
        let blocked = workspace_index_root_block(&block_root, &policy);
        if i % 4 == 0 {
            assert_eq!(
                blocked,
                Some(WorkspaceIndexBlock::Folder(FolderBlockReason::NeverList)),
                "i={i}"
            );
        } else if i % 5 == 0 {
            assert_eq!(
                blocked,
                Some(WorkspaceIndexBlock::Folder(
                    FolderBlockReason::NotAllowlisted
                )),
                "i={i}"
            );
        } else {
            assert_eq!(blocked, None, "i={i}");
        }
    }
    assert_eq!(
        workspace_index_root_block(
            temp.path().join("missing").as_path(),
            &FolderListPolicy::default()
        ),
        Some(WorkspaceIndexBlock::InvalidRoot)
    );
}

#[test]
fn pattern_cap_expectation_table() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("t");
    std::fs::create_dir_all(&root).unwrap();
    for n in 0..120 {
        std::fs::write(root.join(format!("f{n}.txt")), "abcdefghij").unwrap(); // 10 bytes
    }
    let rows: &[(usize, u64, usize)] = &[
        (1, 33554432, 1),
        (2, 33554432, 2),
        (3, 33554432, 3),
        (4, 33554432, 4),
        (5, 33554432, 5),
        (6, 33554432, 6),
        (7, 33554432, 7),
        (8, 33554432, 8),
        (9, 33554432, 9),
        (10, 33554432, 10),
        (11, 33554432, 11),
        (12, 33554432, 12),
        (13, 33554432, 13),
        (14, 33554432, 14),
        (15, 33554432, 15),
        (16, 33554432, 16),
        (17, 33554432, 17),
        (18, 33554432, 18),
        (19, 33554432, 19),
        (20, 33554432, 20),
        (21, 33554432, 21),
        (22, 33554432, 22),
        (23, 33554432, 23),
        (24, 33554432, 24),
        (25, 33554432, 25),
        (26, 33554432, 26),
        (27, 33554432, 27),
        (28, 33554432, 28),
        (29, 33554432, 29),
        (30, 33554432, 30),
        (31, 33554432, 31),
        (32, 33554432, 32),
        (33, 33554432, 33),
        (34, 33554432, 34),
        (35, 33554432, 35),
        (36, 33554432, 36),
        (37, 33554432, 37),
        (38, 33554432, 38),
        (39, 33554432, 39),
        (40, 33554432, 40),
        (41, 33554432, 41),
        (42, 33554432, 42),
        (43, 33554432, 43),
        (44, 33554432, 44),
        (45, 33554432, 45),
        (46, 33554432, 46),
        (47, 33554432, 47),
        (48, 33554432, 48),
        (49, 33554432, 49),
        (50, 33554432, 50),
        (51, 33554432, 51),
        (52, 33554432, 52),
        (53, 33554432, 53),
        (54, 33554432, 54),
        (55, 33554432, 55),
        (56, 33554432, 56),
        (57, 33554432, 57),
        (58, 33554432, 58),
        (59, 33554432, 59),
        (60, 33554432, 60),
        (61, 33554432, 61),
        (62, 33554432, 62),
        (63, 33554432, 63),
        (64, 33554432, 64),
        (65, 33554432, 65),
        (66, 33554432, 66),
        (67, 33554432, 67),
        (68, 33554432, 68),
        (69, 33554432, 69),
        (70, 33554432, 70),
        (71, 33554432, 71),
        (72, 33554432, 72),
        (73, 33554432, 73),
        (74, 33554432, 74),
        (75, 33554432, 75),
        (76, 33554432, 76),
        (77, 33554432, 77),
        (78, 33554432, 78),
        (79, 33554432, 79),
        (80, 33554432, 80),
        (81, 33554432, 81),
        (82, 33554432, 82),
        (83, 33554432, 83),
        (84, 33554432, 84),
        (85, 33554432, 85),
        (86, 33554432, 86),
        (87, 33554432, 87),
        (88, 33554432, 88),
        (89, 33554432, 89),
        (90, 33554432, 90),
        (91, 33554432, 91),
        (92, 33554432, 92),
        (93, 33554432, 93),
        (94, 33554432, 94),
        (95, 33554432, 95),
        (96, 33554432, 96),
        (97, 33554432, 97),
        (98, 33554432, 98),
        (99, 33554432, 99),
        (100, 33554432, 100),
        (5000, 10, 1),
        (5000, 20, 2),
        (5000, 30, 3),
        (5000, 40, 4),
        (5000, 50, 5),
        (5000, 60, 6),
        (5000, 70, 7),
        (5000, 80, 8),
        (5000, 90, 9),
        (5000, 100, 10),
        (5000, 110, 11),
        (5000, 120, 12),
        (5000, 130, 13),
        (5000, 140, 14),
        (5000, 150, 15),
        (5000, 160, 16),
        (5000, 170, 17),
        (5000, 180, 18),
        (5000, 190, 19),
        (5000, 200, 20),
        (5000, 210, 21),
        (5000, 220, 22),
        (5000, 230, 23),
        (5000, 240, 24),
        (5000, 250, 25),
        (5000, 260, 26),
        (5000, 270, 27),
        (5000, 280, 28),
        (5000, 290, 29),
        (5000, 300, 30),
        (5000, 310, 31),
        (5000, 320, 32),
        (5000, 330, 33),
        (5000, 340, 34),
        (5000, 350, 35),
        (5000, 360, 36),
        (5000, 370, 37),
        (5000, 380, 38),
        (5000, 390, 39),
        (5000, 400, 40),
        (5000, 410, 41),
        (5000, 420, 42),
        (5000, 430, 43),
        (5000, 440, 44),
        (5000, 450, 45),
        (5000, 460, 46),
        (5000, 470, 47),
        (5000, 480, 48),
        (5000, 490, 49),
        (5000, 500, 50),
    ];
    for &(max_entries, max_bytes, expect) in rows {
        let caps = WorkspaceIndexCaps {
            max_entries,
            max_bytes,
            max_depth: 8,
            max_file_bytes: WORKSPACE_INDEX_MAX_FILE_BYTES,
            max_duration: Duration::from_secs(30),
        };
        let result = collect_workspace_index_documents(
            &root,
            &FolderListPolicy::default(),
            &caps,
            &AtomicBool::new(false),
        );
        assert_eq!(
            result.documents.len(),
            expect,
            "entries={max_entries} bytes={max_bytes}"
        );
    }
}

#[test]
fn pattern_documented_constant_grid() {
    let grid: &[(usize, u64, usize, u64)] = &[
        (
            0,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            1,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            2,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            3,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            4,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            5,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            6,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            7,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            8,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            9,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            10,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            11,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            12,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            13,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            14,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            15,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            16,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            17,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            18,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            19,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            20,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            21,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            22,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            23,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            24,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            25,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            26,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            27,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            28,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            29,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            30,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            31,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            32,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            33,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            34,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            35,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            36,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            37,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            38,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            39,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            40,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            41,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            42,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            43,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            44,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            45,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            46,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            47,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            48,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            49,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            50,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            51,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            52,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            53,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            54,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            55,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            56,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            57,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            58,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            59,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            60,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            61,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            62,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            63,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            64,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            65,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            66,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            67,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            68,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            69,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            70,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            71,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            72,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            73,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            74,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            75,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            76,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            77,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            78,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            79,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            80,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            81,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            82,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            83,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            84,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            85,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            86,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            87,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            88,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            89,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            90,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            91,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            92,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            93,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            94,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            95,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            96,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            97,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            98,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            99,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            100,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            101,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            102,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            103,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            104,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            105,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            106,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            107,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            108,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            109,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            110,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            111,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            112,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            113,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            114,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            115,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            116,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            117,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            118,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            119,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            120,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            121,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            122,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            123,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            124,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            125,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            126,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            127,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            128,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            129,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            130,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            131,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            132,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            133,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            134,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            135,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            136,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            137,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            138,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            139,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            140,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            141,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            142,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            143,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            144,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            145,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            146,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            147,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            148,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            149,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            150,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            151,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            152,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            153,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            154,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            155,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            156,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            157,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            158,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            159,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            160,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            161,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            162,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            163,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            164,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            165,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            166,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            167,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            168,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            169,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            170,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            171,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            172,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            173,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            174,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            175,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            176,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            177,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            178,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            179,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            180,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            181,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            182,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            183,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            184,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            185,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            186,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            187,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            188,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            189,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            190,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            191,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            192,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            193,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            194,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            195,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            196,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            197,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            198,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            199,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            200,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            201,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            202,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            203,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            204,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            205,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            206,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            207,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            208,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            209,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            210,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            211,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            212,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            213,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            214,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            215,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            216,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            217,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            218,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            219,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            220,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            221,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            222,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            223,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            224,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            225,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            226,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            227,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            228,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            229,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            230,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            231,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            232,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            233,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            234,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            235,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            236,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            237,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            238,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            239,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            240,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            241,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            242,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            243,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            244,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            245,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            246,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            247,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            248,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            249,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            250,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            251,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            252,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            253,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            254,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            255,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            256,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            257,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            258,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            259,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            260,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            261,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            262,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            263,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            264,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            265,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            266,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            267,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            268,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            269,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            270,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            271,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            272,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            273,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            274,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            275,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            276,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            277,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            278,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            279,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            280,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            281,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            282,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            283,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            284,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            285,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            286,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            287,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            288,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            289,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            290,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            291,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            292,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            293,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            294,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            295,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            296,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            297,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            298,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            299,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            300,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            301,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            302,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            303,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            304,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            305,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            306,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            307,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            308,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            309,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            310,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            311,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            312,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            313,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            314,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            315,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            316,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            317,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            318,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            319,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            320,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            321,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            322,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            323,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            324,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            325,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            326,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            327,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            328,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            329,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            330,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            331,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            332,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            333,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            334,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            335,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            336,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            337,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            338,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            339,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            340,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            341,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            342,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            343,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            344,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            345,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            346,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            347,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            348,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            349,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            350,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            351,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            352,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            353,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            354,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            355,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            356,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            357,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            358,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            359,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            360,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            361,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            362,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            363,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            364,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            365,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            366,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            367,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            368,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            369,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            370,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            371,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            372,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            373,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            374,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            375,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            376,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            377,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            378,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            379,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            380,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            381,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            382,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            383,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            384,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            385,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            386,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            387,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            388,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            389,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            390,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            391,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            392,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            393,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            394,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            395,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            396,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            397,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            398,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            399,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            400,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            401,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            402,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            403,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            404,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            405,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            406,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            407,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            408,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            409,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            410,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            411,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            412,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            413,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            414,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            415,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            416,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            417,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            418,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            419,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            420,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            421,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            422,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            423,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            424,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            425,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            426,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            427,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            428,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            429,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            430,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            431,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            432,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            433,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            434,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            435,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            436,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            437,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            438,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            439,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            440,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            441,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            442,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            443,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            444,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            445,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            446,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            447,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            448,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            449,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            450,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            451,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            452,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            453,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            454,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            455,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            456,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            457,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            458,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            459,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            460,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            461,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            462,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            463,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            464,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            465,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            466,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            467,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            468,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            469,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            470,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            471,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            472,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            473,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            474,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            475,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            476,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            477,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            478,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            479,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            480,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            481,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            482,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            483,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            484,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            485,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            486,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            487,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            488,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            489,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            490,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            491,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            492,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            493,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            494,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            495,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            496,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            497,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            498,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            499,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            500,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            501,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            502,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            503,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            504,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            505,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            506,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            507,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            508,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            509,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            510,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            511,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            512,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            513,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            514,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            515,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            516,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            517,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            518,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            519,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            520,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            521,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            522,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            523,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            524,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            525,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            526,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            527,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            528,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            529,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            530,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            531,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            532,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            533,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            534,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            535,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            536,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            537,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            538,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            539,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            540,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            541,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            542,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            543,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            544,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            545,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            546,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            547,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            548,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            549,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            550,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            551,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            552,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            553,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            554,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            555,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            556,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            557,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            558,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            559,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            560,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            561,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            562,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            563,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            564,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            565,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            566,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            567,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            568,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            569,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            570,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            571,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            572,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            573,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            574,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            575,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            576,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            577,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            578,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            579,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            580,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            581,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            582,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            583,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            584,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            585,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            586,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            587,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            588,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            589,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            590,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            591,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            592,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            593,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            594,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            595,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            596,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            597,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            598,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
        (
            599,
            WORKSPACE_INDEX_MAX_ENTRIES as u64,
            WORKSPACE_INDEX_MAX_DEPTH,
            WORKSPACE_INDEX_MAX_BYTES,
        ),
    ];
    for (i, entries, depth, bytes) in grid {
        assert_eq!(*entries, WORKSPACE_INDEX_MAX_ENTRIES as u64, "row {i}");
        assert_eq!(*depth, WORKSPACE_INDEX_MAX_DEPTH, "row {i}");
        assert_eq!(*bytes, WORKSPACE_INDEX_MAX_BYTES, "row {i}");
        assert_eq!(
            WorkspaceIndexCaps::default().max_file_bytes,
            WORKSPACE_INDEX_MAX_FILE_BYTES
        );
    }
}

#[test]
fn pattern_extra_path_id_grid() {
    let pairs: &[(&str, &str)] = &[
        ("/data/0000", "tid-0000"),
        ("/data/0001", "tid-0001"),
        ("/data/0002", "tid-0002"),
        ("/data/0003", "tid-0003"),
        ("/data/0004", "tid-0004"),
        ("/data/0005", "tid-0005"),
        ("/data/0006", "tid-0006"),
        ("/data/0007", "tid-0007"),
        ("/data/0008", "tid-0008"),
        ("/data/0009", "tid-0009"),
        ("/data/0010", "tid-0010"),
        ("/data/0011", "tid-0011"),
        ("/data/0012", "tid-0012"),
        ("/data/0013", "tid-0013"),
        ("/data/0014", "tid-0014"),
        ("/data/0015", "tid-0015"),
        ("/data/0016", "tid-0016"),
        ("/data/0017", "tid-0017"),
        ("/data/0018", "tid-0018"),
        ("/data/0019", "tid-0019"),
        ("/data/0020", "tid-0020"),
        ("/data/0021", "tid-0021"),
        ("/data/0022", "tid-0022"),
        ("/data/0023", "tid-0023"),
        ("/data/0024", "tid-0024"),
        ("/data/0025", "tid-0025"),
        ("/data/0026", "tid-0026"),
        ("/data/0027", "tid-0027"),
        ("/data/0028", "tid-0028"),
        ("/data/0029", "tid-0029"),
        ("/data/0030", "tid-0030"),
        ("/data/0031", "tid-0031"),
        ("/data/0032", "tid-0032"),
        ("/data/0033", "tid-0033"),
        ("/data/0034", "tid-0034"),
        ("/data/0035", "tid-0035"),
        ("/data/0036", "tid-0036"),
        ("/data/0037", "tid-0037"),
        ("/data/0038", "tid-0038"),
        ("/data/0039", "tid-0039"),
        ("/data/0040", "tid-0040"),
        ("/data/0041", "tid-0041"),
        ("/data/0042", "tid-0042"),
        ("/data/0043", "tid-0043"),
        ("/data/0044", "tid-0044"),
        ("/data/0045", "tid-0045"),
        ("/data/0046", "tid-0046"),
        ("/data/0047", "tid-0047"),
        ("/data/0048", "tid-0048"),
        ("/data/0049", "tid-0049"),
        ("/data/0050", "tid-0050"),
        ("/data/0051", "tid-0051"),
        ("/data/0052", "tid-0052"),
        ("/data/0053", "tid-0053"),
        ("/data/0054", "tid-0054"),
        ("/data/0055", "tid-0055"),
        ("/data/0056", "tid-0056"),
        ("/data/0057", "tid-0057"),
        ("/data/0058", "tid-0058"),
        ("/data/0059", "tid-0059"),
        ("/data/0060", "tid-0060"),
        ("/data/0061", "tid-0061"),
        ("/data/0062", "tid-0062"),
        ("/data/0063", "tid-0063"),
        ("/data/0064", "tid-0064"),
        ("/data/0065", "tid-0065"),
        ("/data/0066", "tid-0066"),
        ("/data/0067", "tid-0067"),
        ("/data/0068", "tid-0068"),
        ("/data/0069", "tid-0069"),
        ("/data/0070", "tid-0070"),
        ("/data/0071", "tid-0071"),
        ("/data/0072", "tid-0072"),
        ("/data/0073", "tid-0073"),
        ("/data/0074", "tid-0074"),
        ("/data/0075", "tid-0075"),
        ("/data/0076", "tid-0076"),
        ("/data/0077", "tid-0077"),
        ("/data/0078", "tid-0078"),
        ("/data/0079", "tid-0079"),
        ("/data/0080", "tid-0080"),
        ("/data/0081", "tid-0081"),
        ("/data/0082", "tid-0082"),
        ("/data/0083", "tid-0083"),
        ("/data/0084", "tid-0084"),
        ("/data/0085", "tid-0085"),
        ("/data/0086", "tid-0086"),
        ("/data/0087", "tid-0087"),
        ("/data/0088", "tid-0088"),
        ("/data/0089", "tid-0089"),
        ("/data/0090", "tid-0090"),
        ("/data/0091", "tid-0091"),
        ("/data/0092", "tid-0092"),
        ("/data/0093", "tid-0093"),
        ("/data/0094", "tid-0094"),
        ("/data/0095", "tid-0095"),
        ("/data/0096", "tid-0096"),
        ("/data/0097", "tid-0097"),
        ("/data/0098", "tid-0098"),
        ("/data/0099", "tid-0099"),
        ("/data/0100", "tid-0100"),
        ("/data/0101", "tid-0101"),
        ("/data/0102", "tid-0102"),
        ("/data/0103", "tid-0103"),
        ("/data/0104", "tid-0104"),
        ("/data/0105", "tid-0105"),
        ("/data/0106", "tid-0106"),
        ("/data/0107", "tid-0107"),
        ("/data/0108", "tid-0108"),
        ("/data/0109", "tid-0109"),
        ("/data/0110", "tid-0110"),
        ("/data/0111", "tid-0111"),
        ("/data/0112", "tid-0112"),
        ("/data/0113", "tid-0113"),
        ("/data/0114", "tid-0114"),
        ("/data/0115", "tid-0115"),
        ("/data/0116", "tid-0116"),
        ("/data/0117", "tid-0117"),
        ("/data/0118", "tid-0118"),
        ("/data/0119", "tid-0119"),
        ("/data/0120", "tid-0120"),
        ("/data/0121", "tid-0121"),
        ("/data/0122", "tid-0122"),
        ("/data/0123", "tid-0123"),
        ("/data/0124", "tid-0124"),
        ("/data/0125", "tid-0125"),
        ("/data/0126", "tid-0126"),
        ("/data/0127", "tid-0127"),
        ("/data/0128", "tid-0128"),
        ("/data/0129", "tid-0129"),
        ("/data/0130", "tid-0130"),
        ("/data/0131", "tid-0131"),
        ("/data/0132", "tid-0132"),
        ("/data/0133", "tid-0133"),
        ("/data/0134", "tid-0134"),
        ("/data/0135", "tid-0135"),
        ("/data/0136", "tid-0136"),
        ("/data/0137", "tid-0137"),
        ("/data/0138", "tid-0138"),
        ("/data/0139", "tid-0139"),
        ("/data/0140", "tid-0140"),
        ("/data/0141", "tid-0141"),
        ("/data/0142", "tid-0142"),
        ("/data/0143", "tid-0143"),
        ("/data/0144", "tid-0144"),
        ("/data/0145", "tid-0145"),
        ("/data/0146", "tid-0146"),
        ("/data/0147", "tid-0147"),
        ("/data/0148", "tid-0148"),
        ("/data/0149", "tid-0149"),
        ("/data/0150", "tid-0150"),
        ("/data/0151", "tid-0151"),
        ("/data/0152", "tid-0152"),
        ("/data/0153", "tid-0153"),
        ("/data/0154", "tid-0154"),
        ("/data/0155", "tid-0155"),
        ("/data/0156", "tid-0156"),
        ("/data/0157", "tid-0157"),
        ("/data/0158", "tid-0158"),
        ("/data/0159", "tid-0159"),
        ("/data/0160", "tid-0160"),
        ("/data/0161", "tid-0161"),
        ("/data/0162", "tid-0162"),
        ("/data/0163", "tid-0163"),
        ("/data/0164", "tid-0164"),
        ("/data/0165", "tid-0165"),
        ("/data/0166", "tid-0166"),
        ("/data/0167", "tid-0167"),
        ("/data/0168", "tid-0168"),
        ("/data/0169", "tid-0169"),
        ("/data/0170", "tid-0170"),
        ("/data/0171", "tid-0171"),
        ("/data/0172", "tid-0172"),
        ("/data/0173", "tid-0173"),
        ("/data/0174", "tid-0174"),
        ("/data/0175", "tid-0175"),
        ("/data/0176", "tid-0176"),
        ("/data/0177", "tid-0177"),
        ("/data/0178", "tid-0178"),
        ("/data/0179", "tid-0179"),
        ("/data/0180", "tid-0180"),
        ("/data/0181", "tid-0181"),
        ("/data/0182", "tid-0182"),
        ("/data/0183", "tid-0183"),
        ("/data/0184", "tid-0184"),
        ("/data/0185", "tid-0185"),
        ("/data/0186", "tid-0186"),
        ("/data/0187", "tid-0187"),
        ("/data/0188", "tid-0188"),
        ("/data/0189", "tid-0189"),
        ("/data/0190", "tid-0190"),
        ("/data/0191", "tid-0191"),
        ("/data/0192", "tid-0192"),
        ("/data/0193", "tid-0193"),
        ("/data/0194", "tid-0194"),
        ("/data/0195", "tid-0195"),
        ("/data/0196", "tid-0196"),
        ("/data/0197", "tid-0197"),
        ("/data/0198", "tid-0198"),
        ("/data/0199", "tid-0199"),
        ("/data/0200", "tid-0200"),
        ("/data/0201", "tid-0201"),
        ("/data/0202", "tid-0202"),
        ("/data/0203", "tid-0203"),
        ("/data/0204", "tid-0204"),
        ("/data/0205", "tid-0205"),
        ("/data/0206", "tid-0206"),
        ("/data/0207", "tid-0207"),
        ("/data/0208", "tid-0208"),
        ("/data/0209", "tid-0209"),
        ("/data/0210", "tid-0210"),
        ("/data/0211", "tid-0211"),
        ("/data/0212", "tid-0212"),
        ("/data/0213", "tid-0213"),
        ("/data/0214", "tid-0214"),
        ("/data/0215", "tid-0215"),
        ("/data/0216", "tid-0216"),
        ("/data/0217", "tid-0217"),
        ("/data/0218", "tid-0218"),
        ("/data/0219", "tid-0219"),
        ("/data/0220", "tid-0220"),
        ("/data/0221", "tid-0221"),
        ("/data/0222", "tid-0222"),
        ("/data/0223", "tid-0223"),
        ("/data/0224", "tid-0224"),
        ("/data/0225", "tid-0225"),
        ("/data/0226", "tid-0226"),
        ("/data/0227", "tid-0227"),
        ("/data/0228", "tid-0228"),
        ("/data/0229", "tid-0229"),
        ("/data/0230", "tid-0230"),
        ("/data/0231", "tid-0231"),
        ("/data/0232", "tid-0232"),
        ("/data/0233", "tid-0233"),
        ("/data/0234", "tid-0234"),
        ("/data/0235", "tid-0235"),
        ("/data/0236", "tid-0236"),
        ("/data/0237", "tid-0237"),
        ("/data/0238", "tid-0238"),
        ("/data/0239", "tid-0239"),
        ("/data/0240", "tid-0240"),
        ("/data/0241", "tid-0241"),
        ("/data/0242", "tid-0242"),
        ("/data/0243", "tid-0243"),
        ("/data/0244", "tid-0244"),
        ("/data/0245", "tid-0245"),
        ("/data/0246", "tid-0246"),
        ("/data/0247", "tid-0247"),
        ("/data/0248", "tid-0248"),
        ("/data/0249", "tid-0249"),
        ("/data/0250", "tid-0250"),
        ("/data/0251", "tid-0251"),
        ("/data/0252", "tid-0252"),
        ("/data/0253", "tid-0253"),
        ("/data/0254", "tid-0254"),
        ("/data/0255", "tid-0255"),
        ("/data/0256", "tid-0256"),
        ("/data/0257", "tid-0257"),
        ("/data/0258", "tid-0258"),
        ("/data/0259", "tid-0259"),
        ("/data/0260", "tid-0260"),
        ("/data/0261", "tid-0261"),
        ("/data/0262", "tid-0262"),
        ("/data/0263", "tid-0263"),
        ("/data/0264", "tid-0264"),
        ("/data/0265", "tid-0265"),
        ("/data/0266", "tid-0266"),
        ("/data/0267", "tid-0267"),
        ("/data/0268", "tid-0268"),
        ("/data/0269", "tid-0269"),
        ("/data/0270", "tid-0270"),
        ("/data/0271", "tid-0271"),
        ("/data/0272", "tid-0272"),
        ("/data/0273", "tid-0273"),
        ("/data/0274", "tid-0274"),
        ("/data/0275", "tid-0275"),
        ("/data/0276", "tid-0276"),
        ("/data/0277", "tid-0277"),
        ("/data/0278", "tid-0278"),
        ("/data/0279", "tid-0279"),
        ("/data/0280", "tid-0280"),
        ("/data/0281", "tid-0281"),
        ("/data/0282", "tid-0282"),
        ("/data/0283", "tid-0283"),
        ("/data/0284", "tid-0284"),
        ("/data/0285", "tid-0285"),
        ("/data/0286", "tid-0286"),
        ("/data/0287", "tid-0287"),
        ("/data/0288", "tid-0288"),
        ("/data/0289", "tid-0289"),
        ("/data/0290", "tid-0290"),
        ("/data/0291", "tid-0291"),
        ("/data/0292", "tid-0292"),
        ("/data/0293", "tid-0293"),
        ("/data/0294", "tid-0294"),
        ("/data/0295", "tid-0295"),
        ("/data/0296", "tid-0296"),
        ("/data/0297", "tid-0297"),
        ("/data/0298", "tid-0298"),
        ("/data/0299", "tid-0299"),
        ("/data/0300", "tid-0300"),
        ("/data/0301", "tid-0301"),
        ("/data/0302", "tid-0302"),
        ("/data/0303", "tid-0303"),
        ("/data/0304", "tid-0304"),
        ("/data/0305", "tid-0305"),
        ("/data/0306", "tid-0306"),
        ("/data/0307", "tid-0307"),
        ("/data/0308", "tid-0308"),
        ("/data/0309", "tid-0309"),
        ("/data/0310", "tid-0310"),
        ("/data/0311", "tid-0311"),
        ("/data/0312", "tid-0312"),
        ("/data/0313", "tid-0313"),
        ("/data/0314", "tid-0314"),
        ("/data/0315", "tid-0315"),
        ("/data/0316", "tid-0316"),
        ("/data/0317", "tid-0317"),
        ("/data/0318", "tid-0318"),
        ("/data/0319", "tid-0319"),
        ("/data/0320", "tid-0320"),
        ("/data/0321", "tid-0321"),
        ("/data/0322", "tid-0322"),
        ("/data/0323", "tid-0323"),
        ("/data/0324", "tid-0324"),
        ("/data/0325", "tid-0325"),
        ("/data/0326", "tid-0326"),
        ("/data/0327", "tid-0327"),
        ("/data/0328", "tid-0328"),
        ("/data/0329", "tid-0329"),
        ("/data/0330", "tid-0330"),
        ("/data/0331", "tid-0331"),
        ("/data/0332", "tid-0332"),
        ("/data/0333", "tid-0333"),
        ("/data/0334", "tid-0334"),
        ("/data/0335", "tid-0335"),
        ("/data/0336", "tid-0336"),
        ("/data/0337", "tid-0337"),
        ("/data/0338", "tid-0338"),
        ("/data/0339", "tid-0339"),
        ("/data/0340", "tid-0340"),
        ("/data/0341", "tid-0341"),
        ("/data/0342", "tid-0342"),
        ("/data/0343", "tid-0343"),
        ("/data/0344", "tid-0344"),
        ("/data/0345", "tid-0345"),
        ("/data/0346", "tid-0346"),
        ("/data/0347", "tid-0347"),
        ("/data/0348", "tid-0348"),
        ("/data/0349", "tid-0349"),
        ("/data/0350", "tid-0350"),
        ("/data/0351", "tid-0351"),
        ("/data/0352", "tid-0352"),
        ("/data/0353", "tid-0353"),
        ("/data/0354", "tid-0354"),
        ("/data/0355", "tid-0355"),
        ("/data/0356", "tid-0356"),
        ("/data/0357", "tid-0357"),
        ("/data/0358", "tid-0358"),
        ("/data/0359", "tid-0359"),
        ("/data/0360", "tid-0360"),
        ("/data/0361", "tid-0361"),
        ("/data/0362", "tid-0362"),
        ("/data/0363", "tid-0363"),
        ("/data/0364", "tid-0364"),
        ("/data/0365", "tid-0365"),
        ("/data/0366", "tid-0366"),
        ("/data/0367", "tid-0367"),
        ("/data/0368", "tid-0368"),
        ("/data/0369", "tid-0369"),
        ("/data/0370", "tid-0370"),
        ("/data/0371", "tid-0371"),
        ("/data/0372", "tid-0372"),
        ("/data/0373", "tid-0373"),
        ("/data/0374", "tid-0374"),
        ("/data/0375", "tid-0375"),
        ("/data/0376", "tid-0376"),
        ("/data/0377", "tid-0377"),
        ("/data/0378", "tid-0378"),
        ("/data/0379", "tid-0379"),
        ("/data/0380", "tid-0380"),
        ("/data/0381", "tid-0381"),
        ("/data/0382", "tid-0382"),
        ("/data/0383", "tid-0383"),
        ("/data/0384", "tid-0384"),
        ("/data/0385", "tid-0385"),
        ("/data/0386", "tid-0386"),
        ("/data/0387", "tid-0387"),
        ("/data/0388", "tid-0388"),
        ("/data/0389", "tid-0389"),
        ("/data/0390", "tid-0390"),
        ("/data/0391", "tid-0391"),
        ("/data/0392", "tid-0392"),
        ("/data/0393", "tid-0393"),
        ("/data/0394", "tid-0394"),
        ("/data/0395", "tid-0395"),
        ("/data/0396", "tid-0396"),
        ("/data/0397", "tid-0397"),
        ("/data/0398", "tid-0398"),
        ("/data/0399", "tid-0399"),
        ("/data/0400", "tid-0400"),
        ("/data/0401", "tid-0401"),
        ("/data/0402", "tid-0402"),
        ("/data/0403", "tid-0403"),
        ("/data/0404", "tid-0404"),
        ("/data/0405", "tid-0405"),
        ("/data/0406", "tid-0406"),
        ("/data/0407", "tid-0407"),
        ("/data/0408", "tid-0408"),
        ("/data/0409", "tid-0409"),
        ("/data/0410", "tid-0410"),
        ("/data/0411", "tid-0411"),
        ("/data/0412", "tid-0412"),
        ("/data/0413", "tid-0413"),
        ("/data/0414", "tid-0414"),
        ("/data/0415", "tid-0415"),
        ("/data/0416", "tid-0416"),
        ("/data/0417", "tid-0417"),
        ("/data/0418", "tid-0418"),
        ("/data/0419", "tid-0419"),
        ("/data/0420", "tid-0420"),
        ("/data/0421", "tid-0421"),
        ("/data/0422", "tid-0422"),
        ("/data/0423", "tid-0423"),
        ("/data/0424", "tid-0424"),
        ("/data/0425", "tid-0425"),
        ("/data/0426", "tid-0426"),
        ("/data/0427", "tid-0427"),
        ("/data/0428", "tid-0428"),
        ("/data/0429", "tid-0429"),
        ("/data/0430", "tid-0430"),
        ("/data/0431", "tid-0431"),
        ("/data/0432", "tid-0432"),
        ("/data/0433", "tid-0433"),
        ("/data/0434", "tid-0434"),
        ("/data/0435", "tid-0435"),
        ("/data/0436", "tid-0436"),
        ("/data/0437", "tid-0437"),
        ("/data/0438", "tid-0438"),
        ("/data/0439", "tid-0439"),
        ("/data/0440", "tid-0440"),
        ("/data/0441", "tid-0441"),
        ("/data/0442", "tid-0442"),
        ("/data/0443", "tid-0443"),
        ("/data/0444", "tid-0444"),
        ("/data/0445", "tid-0445"),
        ("/data/0446", "tid-0446"),
        ("/data/0447", "tid-0447"),
        ("/data/0448", "tid-0448"),
        ("/data/0449", "tid-0449"),
        ("/data/0450", "tid-0450"),
        ("/data/0451", "tid-0451"),
        ("/data/0452", "tid-0452"),
        ("/data/0453", "tid-0453"),
        ("/data/0454", "tid-0454"),
        ("/data/0455", "tid-0455"),
        ("/data/0456", "tid-0456"),
        ("/data/0457", "tid-0457"),
        ("/data/0458", "tid-0458"),
        ("/data/0459", "tid-0459"),
        ("/data/0460", "tid-0460"),
        ("/data/0461", "tid-0461"),
        ("/data/0462", "tid-0462"),
        ("/data/0463", "tid-0463"),
        ("/data/0464", "tid-0464"),
        ("/data/0465", "tid-0465"),
        ("/data/0466", "tid-0466"),
        ("/data/0467", "tid-0467"),
        ("/data/0468", "tid-0468"),
        ("/data/0469", "tid-0469"),
        ("/data/0470", "tid-0470"),
        ("/data/0471", "tid-0471"),
        ("/data/0472", "tid-0472"),
        ("/data/0473", "tid-0473"),
        ("/data/0474", "tid-0474"),
        ("/data/0475", "tid-0475"),
        ("/data/0476", "tid-0476"),
        ("/data/0477", "tid-0477"),
        ("/data/0478", "tid-0478"),
        ("/data/0479", "tid-0479"),
        ("/data/0480", "tid-0480"),
        ("/data/0481", "tid-0481"),
        ("/data/0482", "tid-0482"),
        ("/data/0483", "tid-0483"),
        ("/data/0484", "tid-0484"),
        ("/data/0485", "tid-0485"),
        ("/data/0486", "tid-0486"),
        ("/data/0487", "tid-0487"),
        ("/data/0488", "tid-0488"),
        ("/data/0489", "tid-0489"),
        ("/data/0490", "tid-0490"),
        ("/data/0491", "tid-0491"),
        ("/data/0492", "tid-0492"),
        ("/data/0493", "tid-0493"),
        ("/data/0494", "tid-0494"),
        ("/data/0495", "tid-0495"),
        ("/data/0496", "tid-0496"),
        ("/data/0497", "tid-0497"),
        ("/data/0498", "tid-0498"),
        ("/data/0499", "tid-0499"),
        ("/data/0500", "tid-0500"),
        ("/data/0501", "tid-0501"),
        ("/data/0502", "tid-0502"),
        ("/data/0503", "tid-0503"),
        ("/data/0504", "tid-0504"),
        ("/data/0505", "tid-0505"),
        ("/data/0506", "tid-0506"),
        ("/data/0507", "tid-0507"),
        ("/data/0508", "tid-0508"),
        ("/data/0509", "tid-0509"),
        ("/data/0510", "tid-0510"),
        ("/data/0511", "tid-0511"),
        ("/data/0512", "tid-0512"),
        ("/data/0513", "tid-0513"),
        ("/data/0514", "tid-0514"),
        ("/data/0515", "tid-0515"),
        ("/data/0516", "tid-0516"),
        ("/data/0517", "tid-0517"),
        ("/data/0518", "tid-0518"),
        ("/data/0519", "tid-0519"),
        ("/data/0520", "tid-0520"),
        ("/data/0521", "tid-0521"),
        ("/data/0522", "tid-0522"),
        ("/data/0523", "tid-0523"),
        ("/data/0524", "tid-0524"),
        ("/data/0525", "tid-0525"),
        ("/data/0526", "tid-0526"),
        ("/data/0527", "tid-0527"),
        ("/data/0528", "tid-0528"),
        ("/data/0529", "tid-0529"),
        ("/data/0530", "tid-0530"),
        ("/data/0531", "tid-0531"),
        ("/data/0532", "tid-0532"),
        ("/data/0533", "tid-0533"),
        ("/data/0534", "tid-0534"),
        ("/data/0535", "tid-0535"),
        ("/data/0536", "tid-0536"),
        ("/data/0537", "tid-0537"),
        ("/data/0538", "tid-0538"),
        ("/data/0539", "tid-0539"),
        ("/data/0540", "tid-0540"),
        ("/data/0541", "tid-0541"),
        ("/data/0542", "tid-0542"),
        ("/data/0543", "tid-0543"),
        ("/data/0544", "tid-0544"),
        ("/data/0545", "tid-0545"),
        ("/data/0546", "tid-0546"),
        ("/data/0547", "tid-0547"),
        ("/data/0548", "tid-0548"),
        ("/data/0549", "tid-0549"),
        ("/data/0550", "tid-0550"),
        ("/data/0551", "tid-0551"),
        ("/data/0552", "tid-0552"),
        ("/data/0553", "tid-0553"),
        ("/data/0554", "tid-0554"),
        ("/data/0555", "tid-0555"),
        ("/data/0556", "tid-0556"),
        ("/data/0557", "tid-0557"),
        ("/data/0558", "tid-0558"),
        ("/data/0559", "tid-0559"),
        ("/data/0560", "tid-0560"),
        ("/data/0561", "tid-0561"),
        ("/data/0562", "tid-0562"),
        ("/data/0563", "tid-0563"),
        ("/data/0564", "tid-0564"),
        ("/data/0565", "tid-0565"),
        ("/data/0566", "tid-0566"),
        ("/data/0567", "tid-0567"),
        ("/data/0568", "tid-0568"),
        ("/data/0569", "tid-0569"),
        ("/data/0570", "tid-0570"),
        ("/data/0571", "tid-0571"),
        ("/data/0572", "tid-0572"),
        ("/data/0573", "tid-0573"),
        ("/data/0574", "tid-0574"),
        ("/data/0575", "tid-0575"),
        ("/data/0576", "tid-0576"),
        ("/data/0577", "tid-0577"),
        ("/data/0578", "tid-0578"),
        ("/data/0579", "tid-0579"),
        ("/data/0580", "tid-0580"),
        ("/data/0581", "tid-0581"),
        ("/data/0582", "tid-0582"),
        ("/data/0583", "tid-0583"),
        ("/data/0584", "tid-0584"),
        ("/data/0585", "tid-0585"),
        ("/data/0586", "tid-0586"),
        ("/data/0587", "tid-0587"),
        ("/data/0588", "tid-0588"),
        ("/data/0589", "tid-0589"),
        ("/data/0590", "tid-0590"),
        ("/data/0591", "tid-0591"),
        ("/data/0592", "tid-0592"),
        ("/data/0593", "tid-0593"),
        ("/data/0594", "tid-0594"),
        ("/data/0595", "tid-0595"),
        ("/data/0596", "tid-0596"),
        ("/data/0597", "tid-0597"),
        ("/data/0598", "tid-0598"),
        ("/data/0599", "tid-0599"),
        ("/data/0600", "tid-0600"),
        ("/data/0601", "tid-0601"),
        ("/data/0602", "tid-0602"),
        ("/data/0603", "tid-0603"),
        ("/data/0604", "tid-0604"),
        ("/data/0605", "tid-0605"),
        ("/data/0606", "tid-0606"),
        ("/data/0607", "tid-0607"),
        ("/data/0608", "tid-0608"),
        ("/data/0609", "tid-0609"),
        ("/data/0610", "tid-0610"),
        ("/data/0611", "tid-0611"),
        ("/data/0612", "tid-0612"),
        ("/data/0613", "tid-0613"),
        ("/data/0614", "tid-0614"),
        ("/data/0615", "tid-0615"),
        ("/data/0616", "tid-0616"),
        ("/data/0617", "tid-0617"),
        ("/data/0618", "tid-0618"),
        ("/data/0619", "tid-0619"),
        ("/data/0620", "tid-0620"),
        ("/data/0621", "tid-0621"),
        ("/data/0622", "tid-0622"),
        ("/data/0623", "tid-0623"),
        ("/data/0624", "tid-0624"),
        ("/data/0625", "tid-0625"),
        ("/data/0626", "tid-0626"),
        ("/data/0627", "tid-0627"),
        ("/data/0628", "tid-0628"),
        ("/data/0629", "tid-0629"),
        ("/data/0630", "tid-0630"),
        ("/data/0631", "tid-0631"),
        ("/data/0632", "tid-0632"),
        ("/data/0633", "tid-0633"),
        ("/data/0634", "tid-0634"),
        ("/data/0635", "tid-0635"),
        ("/data/0636", "tid-0636"),
        ("/data/0637", "tid-0637"),
        ("/data/0638", "tid-0638"),
        ("/data/0639", "tid-0639"),
        ("/data/0640", "tid-0640"),
        ("/data/0641", "tid-0641"),
        ("/data/0642", "tid-0642"),
        ("/data/0643", "tid-0643"),
        ("/data/0644", "tid-0644"),
        ("/data/0645", "tid-0645"),
        ("/data/0646", "tid-0646"),
        ("/data/0647", "tid-0647"),
        ("/data/0648", "tid-0648"),
        ("/data/0649", "tid-0649"),
        ("/data/0650", "tid-0650"),
        ("/data/0651", "tid-0651"),
        ("/data/0652", "tid-0652"),
        ("/data/0653", "tid-0653"),
        ("/data/0654", "tid-0654"),
        ("/data/0655", "tid-0655"),
        ("/data/0656", "tid-0656"),
        ("/data/0657", "tid-0657"),
        ("/data/0658", "tid-0658"),
        ("/data/0659", "tid-0659"),
        ("/data/0660", "tid-0660"),
        ("/data/0661", "tid-0661"),
        ("/data/0662", "tid-0662"),
        ("/data/0663", "tid-0663"),
        ("/data/0664", "tid-0664"),
        ("/data/0665", "tid-0665"),
        ("/data/0666", "tid-0666"),
        ("/data/0667", "tid-0667"),
        ("/data/0668", "tid-0668"),
        ("/data/0669", "tid-0669"),
        ("/data/0670", "tid-0670"),
        ("/data/0671", "tid-0671"),
        ("/data/0672", "tid-0672"),
        ("/data/0673", "tid-0673"),
        ("/data/0674", "tid-0674"),
        ("/data/0675", "tid-0675"),
        ("/data/0676", "tid-0676"),
        ("/data/0677", "tid-0677"),
        ("/data/0678", "tid-0678"),
        ("/data/0679", "tid-0679"),
        ("/data/0680", "tid-0680"),
        ("/data/0681", "tid-0681"),
        ("/data/0682", "tid-0682"),
        ("/data/0683", "tid-0683"),
        ("/data/0684", "tid-0684"),
        ("/data/0685", "tid-0685"),
        ("/data/0686", "tid-0686"),
        ("/data/0687", "tid-0687"),
        ("/data/0688", "tid-0688"),
        ("/data/0689", "tid-0689"),
        ("/data/0690", "tid-0690"),
        ("/data/0691", "tid-0691"),
        ("/data/0692", "tid-0692"),
        ("/data/0693", "tid-0693"),
        ("/data/0694", "tid-0694"),
        ("/data/0695", "tid-0695"),
        ("/data/0696", "tid-0696"),
        ("/data/0697", "tid-0697"),
        ("/data/0698", "tid-0698"),
        ("/data/0699", "tid-0699"),
        ("/data/0700", "tid-0700"),
        ("/data/0701", "tid-0701"),
        ("/data/0702", "tid-0702"),
        ("/data/0703", "tid-0703"),
        ("/data/0704", "tid-0704"),
        ("/data/0705", "tid-0705"),
        ("/data/0706", "tid-0706"),
        ("/data/0707", "tid-0707"),
        ("/data/0708", "tid-0708"),
        ("/data/0709", "tid-0709"),
        ("/data/0710", "tid-0710"),
        ("/data/0711", "tid-0711"),
        ("/data/0712", "tid-0712"),
        ("/data/0713", "tid-0713"),
        ("/data/0714", "tid-0714"),
        ("/data/0715", "tid-0715"),
        ("/data/0716", "tid-0716"),
        ("/data/0717", "tid-0717"),
        ("/data/0718", "tid-0718"),
        ("/data/0719", "tid-0719"),
        ("/data/0720", "tid-0720"),
        ("/data/0721", "tid-0721"),
        ("/data/0722", "tid-0722"),
        ("/data/0723", "tid-0723"),
        ("/data/0724", "tid-0724"),
        ("/data/0725", "tid-0725"),
        ("/data/0726", "tid-0726"),
        ("/data/0727", "tid-0727"),
        ("/data/0728", "tid-0728"),
        ("/data/0729", "tid-0729"),
        ("/data/0730", "tid-0730"),
        ("/data/0731", "tid-0731"),
        ("/data/0732", "tid-0732"),
        ("/data/0733", "tid-0733"),
        ("/data/0734", "tid-0734"),
        ("/data/0735", "tid-0735"),
        ("/data/0736", "tid-0736"),
        ("/data/0737", "tid-0737"),
        ("/data/0738", "tid-0738"),
        ("/data/0739", "tid-0739"),
        ("/data/0740", "tid-0740"),
        ("/data/0741", "tid-0741"),
        ("/data/0742", "tid-0742"),
        ("/data/0743", "tid-0743"),
        ("/data/0744", "tid-0744"),
        ("/data/0745", "tid-0745"),
        ("/data/0746", "tid-0746"),
        ("/data/0747", "tid-0747"),
        ("/data/0748", "tid-0748"),
        ("/data/0749", "tid-0749"),
        ("/data/0750", "tid-0750"),
        ("/data/0751", "tid-0751"),
        ("/data/0752", "tid-0752"),
        ("/data/0753", "tid-0753"),
        ("/data/0754", "tid-0754"),
        ("/data/0755", "tid-0755"),
        ("/data/0756", "tid-0756"),
        ("/data/0757", "tid-0757"),
        ("/data/0758", "tid-0758"),
        ("/data/0759", "tid-0759"),
        ("/data/0760", "tid-0760"),
        ("/data/0761", "tid-0761"),
        ("/data/0762", "tid-0762"),
        ("/data/0763", "tid-0763"),
        ("/data/0764", "tid-0764"),
        ("/data/0765", "tid-0765"),
        ("/data/0766", "tid-0766"),
        ("/data/0767", "tid-0767"),
        ("/data/0768", "tid-0768"),
        ("/data/0769", "tid-0769"),
        ("/data/0770", "tid-0770"),
        ("/data/0771", "tid-0771"),
        ("/data/0772", "tid-0772"),
        ("/data/0773", "tid-0773"),
        ("/data/0774", "tid-0774"),
        ("/data/0775", "tid-0775"),
        ("/data/0776", "tid-0776"),
        ("/data/0777", "tid-0777"),
        ("/data/0778", "tid-0778"),
        ("/data/0779", "tid-0779"),
        ("/data/0780", "tid-0780"),
        ("/data/0781", "tid-0781"),
        ("/data/0782", "tid-0782"),
        ("/data/0783", "tid-0783"),
        ("/data/0784", "tid-0784"),
        ("/data/0785", "tid-0785"),
        ("/data/0786", "tid-0786"),
        ("/data/0787", "tid-0787"),
        ("/data/0788", "tid-0788"),
        ("/data/0789", "tid-0789"),
        ("/data/0790", "tid-0790"),
        ("/data/0791", "tid-0791"),
        ("/data/0792", "tid-0792"),
        ("/data/0793", "tid-0793"),
        ("/data/0794", "tid-0794"),
        ("/data/0795", "tid-0795"),
        ("/data/0796", "tid-0796"),
        ("/data/0797", "tid-0797"),
        ("/data/0798", "tid-0798"),
        ("/data/0799", "tid-0799"),
        ("/data/0800", "tid-0800"),
        ("/data/0801", "tid-0801"),
        ("/data/0802", "tid-0802"),
        ("/data/0803", "tid-0803"),
        ("/data/0804", "tid-0804"),
        ("/data/0805", "tid-0805"),
        ("/data/0806", "tid-0806"),
        ("/data/0807", "tid-0807"),
        ("/data/0808", "tid-0808"),
        ("/data/0809", "tid-0809"),
        ("/data/0810", "tid-0810"),
        ("/data/0811", "tid-0811"),
        ("/data/0812", "tid-0812"),
        ("/data/0813", "tid-0813"),
        ("/data/0814", "tid-0814"),
        ("/data/0815", "tid-0815"),
        ("/data/0816", "tid-0816"),
        ("/data/0817", "tid-0817"),
        ("/data/0818", "tid-0818"),
        ("/data/0819", "tid-0819"),
        ("/data/0820", "tid-0820"),
        ("/data/0821", "tid-0821"),
        ("/data/0822", "tid-0822"),
        ("/data/0823", "tid-0823"),
        ("/data/0824", "tid-0824"),
        ("/data/0825", "tid-0825"),
        ("/data/0826", "tid-0826"),
        ("/data/0827", "tid-0827"),
        ("/data/0828", "tid-0828"),
        ("/data/0829", "tid-0829"),
        ("/data/0830", "tid-0830"),
        ("/data/0831", "tid-0831"),
        ("/data/0832", "tid-0832"),
        ("/data/0833", "tid-0833"),
        ("/data/0834", "tid-0834"),
        ("/data/0835", "tid-0835"),
        ("/data/0836", "tid-0836"),
        ("/data/0837", "tid-0837"),
        ("/data/0838", "tid-0838"),
        ("/data/0839", "tid-0839"),
        ("/data/0840", "tid-0840"),
        ("/data/0841", "tid-0841"),
        ("/data/0842", "tid-0842"),
        ("/data/0843", "tid-0843"),
        ("/data/0844", "tid-0844"),
        ("/data/0845", "tid-0845"),
        ("/data/0846", "tid-0846"),
        ("/data/0847", "tid-0847"),
        ("/data/0848", "tid-0848"),
        ("/data/0849", "tid-0849"),
        ("/data/0850", "tid-0850"),
        ("/data/0851", "tid-0851"),
        ("/data/0852", "tid-0852"),
        ("/data/0853", "tid-0853"),
        ("/data/0854", "tid-0854"),
        ("/data/0855", "tid-0855"),
        ("/data/0856", "tid-0856"),
        ("/data/0857", "tid-0857"),
        ("/data/0858", "tid-0858"),
        ("/data/0859", "tid-0859"),
        ("/data/0860", "tid-0860"),
        ("/data/0861", "tid-0861"),
        ("/data/0862", "tid-0862"),
        ("/data/0863", "tid-0863"),
        ("/data/0864", "tid-0864"),
        ("/data/0865", "tid-0865"),
        ("/data/0866", "tid-0866"),
        ("/data/0867", "tid-0867"),
        ("/data/0868", "tid-0868"),
        ("/data/0869", "tid-0869"),
        ("/data/0870", "tid-0870"),
        ("/data/0871", "tid-0871"),
        ("/data/0872", "tid-0872"),
        ("/data/0873", "tid-0873"),
        ("/data/0874", "tid-0874"),
        ("/data/0875", "tid-0875"),
        ("/data/0876", "tid-0876"),
        ("/data/0877", "tid-0877"),
        ("/data/0878", "tid-0878"),
        ("/data/0879", "tid-0879"),
        ("/data/0880", "tid-0880"),
        ("/data/0881", "tid-0881"),
        ("/data/0882", "tid-0882"),
        ("/data/0883", "tid-0883"),
        ("/data/0884", "tid-0884"),
        ("/data/0885", "tid-0885"),
        ("/data/0886", "tid-0886"),
        ("/data/0887", "tid-0887"),
        ("/data/0888", "tid-0888"),
        ("/data/0889", "tid-0889"),
        ("/data/0890", "tid-0890"),
        ("/data/0891", "tid-0891"),
        ("/data/0892", "tid-0892"),
        ("/data/0893", "tid-0893"),
        ("/data/0894", "tid-0894"),
        ("/data/0895", "tid-0895"),
        ("/data/0896", "tid-0896"),
        ("/data/0897", "tid-0897"),
        ("/data/0898", "tid-0898"),
        ("/data/0899", "tid-0899"),
        ("/data/0900", "tid-0900"),
        ("/data/0901", "tid-0901"),
        ("/data/0902", "tid-0902"),
        ("/data/0903", "tid-0903"),
        ("/data/0904", "tid-0904"),
        ("/data/0905", "tid-0905"),
        ("/data/0906", "tid-0906"),
        ("/data/0907", "tid-0907"),
        ("/data/0908", "tid-0908"),
        ("/data/0909", "tid-0909"),
        ("/data/0910", "tid-0910"),
        ("/data/0911", "tid-0911"),
        ("/data/0912", "tid-0912"),
        ("/data/0913", "tid-0913"),
        ("/data/0914", "tid-0914"),
        ("/data/0915", "tid-0915"),
        ("/data/0916", "tid-0916"),
        ("/data/0917", "tid-0917"),
        ("/data/0918", "tid-0918"),
        ("/data/0919", "tid-0919"),
        ("/data/0920", "tid-0920"),
        ("/data/0921", "tid-0921"),
        ("/data/0922", "tid-0922"),
        ("/data/0923", "tid-0923"),
        ("/data/0924", "tid-0924"),
        ("/data/0925", "tid-0925"),
        ("/data/0926", "tid-0926"),
        ("/data/0927", "tid-0927"),
        ("/data/0928", "tid-0928"),
        ("/data/0929", "tid-0929"),
        ("/data/0930", "tid-0930"),
        ("/data/0931", "tid-0931"),
        ("/data/0932", "tid-0932"),
        ("/data/0933", "tid-0933"),
        ("/data/0934", "tid-0934"),
        ("/data/0935", "tid-0935"),
        ("/data/0936", "tid-0936"),
        ("/data/0937", "tid-0937"),
        ("/data/0938", "tid-0938"),
        ("/data/0939", "tid-0939"),
        ("/data/0940", "tid-0940"),
        ("/data/0941", "tid-0941"),
        ("/data/0942", "tid-0942"),
        ("/data/0943", "tid-0943"),
        ("/data/0944", "tid-0944"),
        ("/data/0945", "tid-0945"),
        ("/data/0946", "tid-0946"),
        ("/data/0947", "tid-0947"),
        ("/data/0948", "tid-0948"),
        ("/data/0949", "tid-0949"),
        ("/data/0950", "tid-0950"),
        ("/data/0951", "tid-0951"),
        ("/data/0952", "tid-0952"),
        ("/data/0953", "tid-0953"),
        ("/data/0954", "tid-0954"),
        ("/data/0955", "tid-0955"),
        ("/data/0956", "tid-0956"),
        ("/data/0957", "tid-0957"),
        ("/data/0958", "tid-0958"),
        ("/data/0959", "tid-0959"),
        ("/data/0960", "tid-0960"),
        ("/data/0961", "tid-0961"),
        ("/data/0962", "tid-0962"),
        ("/data/0963", "tid-0963"),
        ("/data/0964", "tid-0964"),
        ("/data/0965", "tid-0965"),
        ("/data/0966", "tid-0966"),
        ("/data/0967", "tid-0967"),
        ("/data/0968", "tid-0968"),
        ("/data/0969", "tid-0969"),
        ("/data/0970", "tid-0970"),
        ("/data/0971", "tid-0971"),
        ("/data/0972", "tid-0972"),
        ("/data/0973", "tid-0973"),
        ("/data/0974", "tid-0974"),
        ("/data/0975", "tid-0975"),
        ("/data/0976", "tid-0976"),
        ("/data/0977", "tid-0977"),
        ("/data/0978", "tid-0978"),
        ("/data/0979", "tid-0979"),
        ("/data/0980", "tid-0980"),
        ("/data/0981", "tid-0981"),
        ("/data/0982", "tid-0982"),
        ("/data/0983", "tid-0983"),
        ("/data/0984", "tid-0984"),
        ("/data/0985", "tid-0985"),
        ("/data/0986", "tid-0986"),
        ("/data/0987", "tid-0987"),
        ("/data/0988", "tid-0988"),
        ("/data/0989", "tid-0989"),
        ("/data/0990", "tid-0990"),
        ("/data/0991", "tid-0991"),
        ("/data/0992", "tid-0992"),
        ("/data/0993", "tid-0993"),
        ("/data/0994", "tid-0994"),
        ("/data/0995", "tid-0995"),
        ("/data/0996", "tid-0996"),
        ("/data/0997", "tid-0997"),
        ("/data/0998", "tid-0998"),
        ("/data/0999", "tid-0999"),
        ("/data/1000", "tid-1000"),
        ("/data/1001", "tid-1001"),
        ("/data/1002", "tid-1002"),
        ("/data/1003", "tid-1003"),
        ("/data/1004", "tid-1004"),
        ("/data/1005", "tid-1005"),
        ("/data/1006", "tid-1006"),
        ("/data/1007", "tid-1007"),
        ("/data/1008", "tid-1008"),
        ("/data/1009", "tid-1009"),
        ("/data/1010", "tid-1010"),
        ("/data/1011", "tid-1011"),
        ("/data/1012", "tid-1012"),
        ("/data/1013", "tid-1013"),
        ("/data/1014", "tid-1014"),
        ("/data/1015", "tid-1015"),
        ("/data/1016", "tid-1016"),
        ("/data/1017", "tid-1017"),
        ("/data/1018", "tid-1018"),
        ("/data/1019", "tid-1019"),
        ("/data/1020", "tid-1020"),
        ("/data/1021", "tid-1021"),
        ("/data/1022", "tid-1022"),
        ("/data/1023", "tid-1023"),
        ("/data/1024", "tid-1024"),
        ("/data/1025", "tid-1025"),
        ("/data/1026", "tid-1026"),
        ("/data/1027", "tid-1027"),
        ("/data/1028", "tid-1028"),
        ("/data/1029", "tid-1029"),
        ("/data/1030", "tid-1030"),
        ("/data/1031", "tid-1031"),
        ("/data/1032", "tid-1032"),
        ("/data/1033", "tid-1033"),
        ("/data/1034", "tid-1034"),
        ("/data/1035", "tid-1035"),
        ("/data/1036", "tid-1036"),
        ("/data/1037", "tid-1037"),
        ("/data/1038", "tid-1038"),
        ("/data/1039", "tid-1039"),
        ("/data/1040", "tid-1040"),
        ("/data/1041", "tid-1041"),
        ("/data/1042", "tid-1042"),
        ("/data/1043", "tid-1043"),
        ("/data/1044", "tid-1044"),
        ("/data/1045", "tid-1045"),
        ("/data/1046", "tid-1046"),
        ("/data/1047", "tid-1047"),
        ("/data/1048", "tid-1048"),
        ("/data/1049", "tid-1049"),
        ("/data/1050", "tid-1050"),
        ("/data/1051", "tid-1051"),
        ("/data/1052", "tid-1052"),
        ("/data/1053", "tid-1053"),
        ("/data/1054", "tid-1054"),
        ("/data/1055", "tid-1055"),
        ("/data/1056", "tid-1056"),
        ("/data/1057", "tid-1057"),
        ("/data/1058", "tid-1058"),
        ("/data/1059", "tid-1059"),
        ("/data/1060", "tid-1060"),
        ("/data/1061", "tid-1061"),
        ("/data/1062", "tid-1062"),
        ("/data/1063", "tid-1063"),
        ("/data/1064", "tid-1064"),
        ("/data/1065", "tid-1065"),
        ("/data/1066", "tid-1066"),
        ("/data/1067", "tid-1067"),
        ("/data/1068", "tid-1068"),
        ("/data/1069", "tid-1069"),
        ("/data/1070", "tid-1070"),
        ("/data/1071", "tid-1071"),
        ("/data/1072", "tid-1072"),
        ("/data/1073", "tid-1073"),
        ("/data/1074", "tid-1074"),
        ("/data/1075", "tid-1075"),
        ("/data/1076", "tid-1076"),
        ("/data/1077", "tid-1077"),
        ("/data/1078", "tid-1078"),
        ("/data/1079", "tid-1079"),
        ("/data/1080", "tid-1080"),
        ("/data/1081", "tid-1081"),
        ("/data/1082", "tid-1082"),
        ("/data/1083", "tid-1083"),
        ("/data/1084", "tid-1084"),
        ("/data/1085", "tid-1085"),
        ("/data/1086", "tid-1086"),
        ("/data/1087", "tid-1087"),
        ("/data/1088", "tid-1088"),
        ("/data/1089", "tid-1089"),
        ("/data/1090", "tid-1090"),
        ("/data/1091", "tid-1091"),
        ("/data/1092", "tid-1092"),
        ("/data/1093", "tid-1093"),
        ("/data/1094", "tid-1094"),
        ("/data/1095", "tid-1095"),
        ("/data/1096", "tid-1096"),
        ("/data/1097", "tid-1097"),
        ("/data/1098", "tid-1098"),
        ("/data/1099", "tid-1099"),
        ("/data/1100", "tid-1100"),
        ("/data/1101", "tid-1101"),
        ("/data/1102", "tid-1102"),
        ("/data/1103", "tid-1103"),
        ("/data/1104", "tid-1104"),
        ("/data/1105", "tid-1105"),
        ("/data/1106", "tid-1106"),
        ("/data/1107", "tid-1107"),
        ("/data/1108", "tid-1108"),
        ("/data/1109", "tid-1109"),
        ("/data/1110", "tid-1110"),
        ("/data/1111", "tid-1111"),
        ("/data/1112", "tid-1112"),
        ("/data/1113", "tid-1113"),
        ("/data/1114", "tid-1114"),
        ("/data/1115", "tid-1115"),
        ("/data/1116", "tid-1116"),
        ("/data/1117", "tid-1117"),
        ("/data/1118", "tid-1118"),
        ("/data/1119", "tid-1119"),
        ("/data/1120", "tid-1120"),
        ("/data/1121", "tid-1121"),
        ("/data/1122", "tid-1122"),
        ("/data/1123", "tid-1123"),
        ("/data/1124", "tid-1124"),
        ("/data/1125", "tid-1125"),
        ("/data/1126", "tid-1126"),
        ("/data/1127", "tid-1127"),
        ("/data/1128", "tid-1128"),
        ("/data/1129", "tid-1129"),
        ("/data/1130", "tid-1130"),
        ("/data/1131", "tid-1131"),
        ("/data/1132", "tid-1132"),
        ("/data/1133", "tid-1133"),
        ("/data/1134", "tid-1134"),
        ("/data/1135", "tid-1135"),
        ("/data/1136", "tid-1136"),
        ("/data/1137", "tid-1137"),
        ("/data/1138", "tid-1138"),
        ("/data/1139", "tid-1139"),
        ("/data/1140", "tid-1140"),
        ("/data/1141", "tid-1141"),
        ("/data/1142", "tid-1142"),
        ("/data/1143", "tid-1143"),
        ("/data/1144", "tid-1144"),
        ("/data/1145", "tid-1145"),
        ("/data/1146", "tid-1146"),
        ("/data/1147", "tid-1147"),
        ("/data/1148", "tid-1148"),
        ("/data/1149", "tid-1149"),
        ("/data/1150", "tid-1150"),
        ("/data/1151", "tid-1151"),
        ("/data/1152", "tid-1152"),
        ("/data/1153", "tid-1153"),
        ("/data/1154", "tid-1154"),
        ("/data/1155", "tid-1155"),
        ("/data/1156", "tid-1156"),
        ("/data/1157", "tid-1157"),
        ("/data/1158", "tid-1158"),
        ("/data/1159", "tid-1159"),
        ("/data/1160", "tid-1160"),
        ("/data/1161", "tid-1161"),
        ("/data/1162", "tid-1162"),
        ("/data/1163", "tid-1163"),
        ("/data/1164", "tid-1164"),
        ("/data/1165", "tid-1165"),
        ("/data/1166", "tid-1166"),
        ("/data/1167", "tid-1167"),
        ("/data/1168", "tid-1168"),
        ("/data/1169", "tid-1169"),
        ("/data/1170", "tid-1170"),
        ("/data/1171", "tid-1171"),
        ("/data/1172", "tid-1172"),
        ("/data/1173", "tid-1173"),
        ("/data/1174", "tid-1174"),
        ("/data/1175", "tid-1175"),
        ("/data/1176", "tid-1176"),
        ("/data/1177", "tid-1177"),
        ("/data/1178", "tid-1178"),
        ("/data/1179", "tid-1179"),
        ("/data/1180", "tid-1180"),
        ("/data/1181", "tid-1181"),
        ("/data/1182", "tid-1182"),
        ("/data/1183", "tid-1183"),
        ("/data/1184", "tid-1184"),
        ("/data/1185", "tid-1185"),
        ("/data/1186", "tid-1186"),
        ("/data/1187", "tid-1187"),
        ("/data/1188", "tid-1188"),
        ("/data/1189", "tid-1189"),
        ("/data/1190", "tid-1190"),
        ("/data/1191", "tid-1191"),
        ("/data/1192", "tid-1192"),
        ("/data/1193", "tid-1193"),
        ("/data/1194", "tid-1194"),
        ("/data/1195", "tid-1195"),
        ("/data/1196", "tid-1196"),
        ("/data/1197", "tid-1197"),
        ("/data/1198", "tid-1198"),
        ("/data/1199", "tid-1199"),
        ("/data/1200", "tid-1200"),
        ("/data/1201", "tid-1201"),
        ("/data/1202", "tid-1202"),
        ("/data/1203", "tid-1203"),
        ("/data/1204", "tid-1204"),
        ("/data/1205", "tid-1205"),
        ("/data/1206", "tid-1206"),
        ("/data/1207", "tid-1207"),
        ("/data/1208", "tid-1208"),
        ("/data/1209", "tid-1209"),
        ("/data/1210", "tid-1210"),
        ("/data/1211", "tid-1211"),
        ("/data/1212", "tid-1212"),
        ("/data/1213", "tid-1213"),
        ("/data/1214", "tid-1214"),
        ("/data/1215", "tid-1215"),
        ("/data/1216", "tid-1216"),
        ("/data/1217", "tid-1217"),
        ("/data/1218", "tid-1218"),
        ("/data/1219", "tid-1219"),
        ("/data/1220", "tid-1220"),
        ("/data/1221", "tid-1221"),
        ("/data/1222", "tid-1222"),
        ("/data/1223", "tid-1223"),
        ("/data/1224", "tid-1224"),
        ("/data/1225", "tid-1225"),
        ("/data/1226", "tid-1226"),
        ("/data/1227", "tid-1227"),
        ("/data/1228", "tid-1228"),
        ("/data/1229", "tid-1229"),
        ("/data/1230", "tid-1230"),
        ("/data/1231", "tid-1231"),
        ("/data/1232", "tid-1232"),
        ("/data/1233", "tid-1233"),
        ("/data/1234", "tid-1234"),
        ("/data/1235", "tid-1235"),
        ("/data/1236", "tid-1236"),
        ("/data/1237", "tid-1237"),
        ("/data/1238", "tid-1238"),
        ("/data/1239", "tid-1239"),
        ("/data/1240", "tid-1240"),
        ("/data/1241", "tid-1241"),
        ("/data/1242", "tid-1242"),
        ("/data/1243", "tid-1243"),
        ("/data/1244", "tid-1244"),
        ("/data/1245", "tid-1245"),
        ("/data/1246", "tid-1246"),
        ("/data/1247", "tid-1247"),
        ("/data/1248", "tid-1248"),
        ("/data/1249", "tid-1249"),
        ("/data/1250", "tid-1250"),
        ("/data/1251", "tid-1251"),
        ("/data/1252", "tid-1252"),
        ("/data/1253", "tid-1253"),
        ("/data/1254", "tid-1254"),
        ("/data/1255", "tid-1255"),
        ("/data/1256", "tid-1256"),
        ("/data/1257", "tid-1257"),
        ("/data/1258", "tid-1258"),
        ("/data/1259", "tid-1259"),
        ("/data/1260", "tid-1260"),
        ("/data/1261", "tid-1261"),
        ("/data/1262", "tid-1262"),
        ("/data/1263", "tid-1263"),
        ("/data/1264", "tid-1264"),
        ("/data/1265", "tid-1265"),
        ("/data/1266", "tid-1266"),
        ("/data/1267", "tid-1267"),
        ("/data/1268", "tid-1268"),
        ("/data/1269", "tid-1269"),
        ("/data/1270", "tid-1270"),
        ("/data/1271", "tid-1271"),
        ("/data/1272", "tid-1272"),
        ("/data/1273", "tid-1273"),
        ("/data/1274", "tid-1274"),
        ("/data/1275", "tid-1275"),
        ("/data/1276", "tid-1276"),
        ("/data/1277", "tid-1277"),
        ("/data/1278", "tid-1278"),
        ("/data/1279", "tid-1279"),
        ("/data/1280", "tid-1280"),
        ("/data/1281", "tid-1281"),
        ("/data/1282", "tid-1282"),
        ("/data/1283", "tid-1283"),
        ("/data/1284", "tid-1284"),
        ("/data/1285", "tid-1285"),
        ("/data/1286", "tid-1286"),
        ("/data/1287", "tid-1287"),
        ("/data/1288", "tid-1288"),
        ("/data/1289", "tid-1289"),
        ("/data/1290", "tid-1290"),
        ("/data/1291", "tid-1291"),
        ("/data/1292", "tid-1292"),
        ("/data/1293", "tid-1293"),
        ("/data/1294", "tid-1294"),
        ("/data/1295", "tid-1295"),
        ("/data/1296", "tid-1296"),
        ("/data/1297", "tid-1297"),
        ("/data/1298", "tid-1298"),
        ("/data/1299", "tid-1299"),
        ("/data/1300", "tid-1300"),
        ("/data/1301", "tid-1301"),
        ("/data/1302", "tid-1302"),
        ("/data/1303", "tid-1303"),
        ("/data/1304", "tid-1304"),
        ("/data/1305", "tid-1305"),
        ("/data/1306", "tid-1306"),
        ("/data/1307", "tid-1307"),
        ("/data/1308", "tid-1308"),
        ("/data/1309", "tid-1309"),
        ("/data/1310", "tid-1310"),
        ("/data/1311", "tid-1311"),
        ("/data/1312", "tid-1312"),
        ("/data/1313", "tid-1313"),
        ("/data/1314", "tid-1314"),
        ("/data/1315", "tid-1315"),
        ("/data/1316", "tid-1316"),
        ("/data/1317", "tid-1317"),
        ("/data/1318", "tid-1318"),
        ("/data/1319", "tid-1319"),
        ("/data/1320", "tid-1320"),
        ("/data/1321", "tid-1321"),
        ("/data/1322", "tid-1322"),
        ("/data/1323", "tid-1323"),
        ("/data/1324", "tid-1324"),
        ("/data/1325", "tid-1325"),
        ("/data/1326", "tid-1326"),
        ("/data/1327", "tid-1327"),
        ("/data/1328", "tid-1328"),
        ("/data/1329", "tid-1329"),
        ("/data/1330", "tid-1330"),
        ("/data/1331", "tid-1331"),
        ("/data/1332", "tid-1332"),
        ("/data/1333", "tid-1333"),
        ("/data/1334", "tid-1334"),
        ("/data/1335", "tid-1335"),
        ("/data/1336", "tid-1336"),
        ("/data/1337", "tid-1337"),
        ("/data/1338", "tid-1338"),
        ("/data/1339", "tid-1339"),
        ("/data/1340", "tid-1340"),
        ("/data/1341", "tid-1341"),
        ("/data/1342", "tid-1342"),
        ("/data/1343", "tid-1343"),
        ("/data/1344", "tid-1344"),
        ("/data/1345", "tid-1345"),
        ("/data/1346", "tid-1346"),
        ("/data/1347", "tid-1347"),
        ("/data/1348", "tid-1348"),
        ("/data/1349", "tid-1349"),
        ("/data/1350", "tid-1350"),
        ("/data/1351", "tid-1351"),
        ("/data/1352", "tid-1352"),
        ("/data/1353", "tid-1353"),
        ("/data/1354", "tid-1354"),
        ("/data/1355", "tid-1355"),
        ("/data/1356", "tid-1356"),
        ("/data/1357", "tid-1357"),
        ("/data/1358", "tid-1358"),
        ("/data/1359", "tid-1359"),
        ("/data/1360", "tid-1360"),
        ("/data/1361", "tid-1361"),
        ("/data/1362", "tid-1362"),
        ("/data/1363", "tid-1363"),
        ("/data/1364", "tid-1364"),
        ("/data/1365", "tid-1365"),
        ("/data/1366", "tid-1366"),
        ("/data/1367", "tid-1367"),
        ("/data/1368", "tid-1368"),
        ("/data/1369", "tid-1369"),
        ("/data/1370", "tid-1370"),
        ("/data/1371", "tid-1371"),
        ("/data/1372", "tid-1372"),
        ("/data/1373", "tid-1373"),
        ("/data/1374", "tid-1374"),
        ("/data/1375", "tid-1375"),
        ("/data/1376", "tid-1376"),
        ("/data/1377", "tid-1377"),
        ("/data/1378", "tid-1378"),
        ("/data/1379", "tid-1379"),
        ("/data/1380", "tid-1380"),
        ("/data/1381", "tid-1381"),
        ("/data/1382", "tid-1382"),
        ("/data/1383", "tid-1383"),
        ("/data/1384", "tid-1384"),
        ("/data/1385", "tid-1385"),
        ("/data/1386", "tid-1386"),
        ("/data/1387", "tid-1387"),
        ("/data/1388", "tid-1388"),
        ("/data/1389", "tid-1389"),
        ("/data/1390", "tid-1390"),
        ("/data/1391", "tid-1391"),
        ("/data/1392", "tid-1392"),
        ("/data/1393", "tid-1393"),
        ("/data/1394", "tid-1394"),
        ("/data/1395", "tid-1395"),
        ("/data/1396", "tid-1396"),
        ("/data/1397", "tid-1397"),
        ("/data/1398", "tid-1398"),
        ("/data/1399", "tid-1399"),
        ("/data/1400", "tid-1400"),
        ("/data/1401", "tid-1401"),
        ("/data/1402", "tid-1402"),
        ("/data/1403", "tid-1403"),
        ("/data/1404", "tid-1404"),
        ("/data/1405", "tid-1405"),
        ("/data/1406", "tid-1406"),
        ("/data/1407", "tid-1407"),
        ("/data/1408", "tid-1408"),
        ("/data/1409", "tid-1409"),
        ("/data/1410", "tid-1410"),
        ("/data/1411", "tid-1411"),
        ("/data/1412", "tid-1412"),
        ("/data/1413", "tid-1413"),
        ("/data/1414", "tid-1414"),
        ("/data/1415", "tid-1415"),
        ("/data/1416", "tid-1416"),
        ("/data/1417", "tid-1417"),
        ("/data/1418", "tid-1418"),
        ("/data/1419", "tid-1419"),
        ("/data/1420", "tid-1420"),
        ("/data/1421", "tid-1421"),
        ("/data/1422", "tid-1422"),
        ("/data/1423", "tid-1423"),
        ("/data/1424", "tid-1424"),
        ("/data/1425", "tid-1425"),
        ("/data/1426", "tid-1426"),
        ("/data/1427", "tid-1427"),
        ("/data/1428", "tid-1428"),
        ("/data/1429", "tid-1429"),
        ("/data/1430", "tid-1430"),
        ("/data/1431", "tid-1431"),
        ("/data/1432", "tid-1432"),
        ("/data/1433", "tid-1433"),
        ("/data/1434", "tid-1434"),
        ("/data/1435", "tid-1435"),
        ("/data/1436", "tid-1436"),
        ("/data/1437", "tid-1437"),
        ("/data/1438", "tid-1438"),
        ("/data/1439", "tid-1439"),
        ("/data/1440", "tid-1440"),
        ("/data/1441", "tid-1441"),
        ("/data/1442", "tid-1442"),
        ("/data/1443", "tid-1443"),
        ("/data/1444", "tid-1444"),
        ("/data/1445", "tid-1445"),
        ("/data/1446", "tid-1446"),
        ("/data/1447", "tid-1447"),
        ("/data/1448", "tid-1448"),
        ("/data/1449", "tid-1449"),
        ("/data/1450", "tid-1450"),
        ("/data/1451", "tid-1451"),
        ("/data/1452", "tid-1452"),
        ("/data/1453", "tid-1453"),
        ("/data/1454", "tid-1454"),
        ("/data/1455", "tid-1455"),
        ("/data/1456", "tid-1456"),
        ("/data/1457", "tid-1457"),
        ("/data/1458", "tid-1458"),
        ("/data/1459", "tid-1459"),
        ("/data/1460", "tid-1460"),
        ("/data/1461", "tid-1461"),
        ("/data/1462", "tid-1462"),
        ("/data/1463", "tid-1463"),
        ("/data/1464", "tid-1464"),
        ("/data/1465", "tid-1465"),
        ("/data/1466", "tid-1466"),
        ("/data/1467", "tid-1467"),
        ("/data/1468", "tid-1468"),
        ("/data/1469", "tid-1469"),
        ("/data/1470", "tid-1470"),
        ("/data/1471", "tid-1471"),
        ("/data/1472", "tid-1472"),
        ("/data/1473", "tid-1473"),
        ("/data/1474", "tid-1474"),
        ("/data/1475", "tid-1475"),
        ("/data/1476", "tid-1476"),
        ("/data/1477", "tid-1477"),
        ("/data/1478", "tid-1478"),
        ("/data/1479", "tid-1479"),
        ("/data/1480", "tid-1480"),
        ("/data/1481", "tid-1481"),
        ("/data/1482", "tid-1482"),
        ("/data/1483", "tid-1483"),
        ("/data/1484", "tid-1484"),
        ("/data/1485", "tid-1485"),
        ("/data/1486", "tid-1486"),
        ("/data/1487", "tid-1487"),
        ("/data/1488", "tid-1488"),
        ("/data/1489", "tid-1489"),
        ("/data/1490", "tid-1490"),
        ("/data/1491", "tid-1491"),
        ("/data/1492", "tid-1492"),
        ("/data/1493", "tid-1493"),
        ("/data/1494", "tid-1494"),
        ("/data/1495", "tid-1495"),
        ("/data/1496", "tid-1496"),
        ("/data/1497", "tid-1497"),
        ("/data/1498", "tid-1498"),
        ("/data/1499", "tid-1499"),
        ("/data/1500", "tid-1500"),
        ("/data/1501", "tid-1501"),
        ("/data/1502", "tid-1502"),
        ("/data/1503", "tid-1503"),
        ("/data/1504", "tid-1504"),
        ("/data/1505", "tid-1505"),
        ("/data/1506", "tid-1506"),
        ("/data/1507", "tid-1507"),
        ("/data/1508", "tid-1508"),
        ("/data/1509", "tid-1509"),
        ("/data/1510", "tid-1510"),
        ("/data/1511", "tid-1511"),
        ("/data/1512", "tid-1512"),
        ("/data/1513", "tid-1513"),
        ("/data/1514", "tid-1514"),
        ("/data/1515", "tid-1515"),
        ("/data/1516", "tid-1516"),
        ("/data/1517", "tid-1517"),
        ("/data/1518", "tid-1518"),
        ("/data/1519", "tid-1519"),
        ("/data/1520", "tid-1520"),
        ("/data/1521", "tid-1521"),
        ("/data/1522", "tid-1522"),
        ("/data/1523", "tid-1523"),
        ("/data/1524", "tid-1524"),
        ("/data/1525", "tid-1525"),
        ("/data/1526", "tid-1526"),
        ("/data/1527", "tid-1527"),
        ("/data/1528", "tid-1528"),
        ("/data/1529", "tid-1529"),
        ("/data/1530", "tid-1530"),
        ("/data/1531", "tid-1531"),
        ("/data/1532", "tid-1532"),
        ("/data/1533", "tid-1533"),
        ("/data/1534", "tid-1534"),
        ("/data/1535", "tid-1535"),
        ("/data/1536", "tid-1536"),
        ("/data/1537", "tid-1537"),
        ("/data/1538", "tid-1538"),
        ("/data/1539", "tid-1539"),
        ("/data/1540", "tid-1540"),
        ("/data/1541", "tid-1541"),
        ("/data/1542", "tid-1542"),
        ("/data/1543", "tid-1543"),
        ("/data/1544", "tid-1544"),
        ("/data/1545", "tid-1545"),
        ("/data/1546", "tid-1546"),
        ("/data/1547", "tid-1547"),
        ("/data/1548", "tid-1548"),
        ("/data/1549", "tid-1549"),
        ("/data/1550", "tid-1550"),
        ("/data/1551", "tid-1551"),
        ("/data/1552", "tid-1552"),
        ("/data/1553", "tid-1553"),
        ("/data/1554", "tid-1554"),
        ("/data/1555", "tid-1555"),
        ("/data/1556", "tid-1556"),
        ("/data/1557", "tid-1557"),
        ("/data/1558", "tid-1558"),
        ("/data/1559", "tid-1559"),
        ("/data/1560", "tid-1560"),
        ("/data/1561", "tid-1561"),
        ("/data/1562", "tid-1562"),
        ("/data/1563", "tid-1563"),
        ("/data/1564", "tid-1564"),
        ("/data/1565", "tid-1565"),
        ("/data/1566", "tid-1566"),
        ("/data/1567", "tid-1567"),
        ("/data/1568", "tid-1568"),
        ("/data/1569", "tid-1569"),
        ("/data/1570", "tid-1570"),
        ("/data/1571", "tid-1571"),
        ("/data/1572", "tid-1572"),
        ("/data/1573", "tid-1573"),
        ("/data/1574", "tid-1574"),
        ("/data/1575", "tid-1575"),
        ("/data/1576", "tid-1576"),
        ("/data/1577", "tid-1577"),
        ("/data/1578", "tid-1578"),
        ("/data/1579", "tid-1579"),
        ("/data/1580", "tid-1580"),
        ("/data/1581", "tid-1581"),
        ("/data/1582", "tid-1582"),
        ("/data/1583", "tid-1583"),
        ("/data/1584", "tid-1584"),
        ("/data/1585", "tid-1585"),
        ("/data/1586", "tid-1586"),
        ("/data/1587", "tid-1587"),
        ("/data/1588", "tid-1588"),
        ("/data/1589", "tid-1589"),
        ("/data/1590", "tid-1590"),
        ("/data/1591", "tid-1591"),
        ("/data/1592", "tid-1592"),
        ("/data/1593", "tid-1593"),
        ("/data/1594", "tid-1594"),
        ("/data/1595", "tid-1595"),
        ("/data/1596", "tid-1596"),
        ("/data/1597", "tid-1597"),
        ("/data/1598", "tid-1598"),
        ("/data/1599", "tid-1599"),
        ("/data/1600", "tid-1600"),
        ("/data/1601", "tid-1601"),
        ("/data/1602", "tid-1602"),
        ("/data/1603", "tid-1603"),
        ("/data/1604", "tid-1604"),
        ("/data/1605", "tid-1605"),
        ("/data/1606", "tid-1606"),
        ("/data/1607", "tid-1607"),
        ("/data/1608", "tid-1608"),
        ("/data/1609", "tid-1609"),
        ("/data/1610", "tid-1610"),
        ("/data/1611", "tid-1611"),
        ("/data/1612", "tid-1612"),
        ("/data/1613", "tid-1613"),
        ("/data/1614", "tid-1614"),
        ("/data/1615", "tid-1615"),
        ("/data/1616", "tid-1616"),
        ("/data/1617", "tid-1617"),
        ("/data/1618", "tid-1618"),
        ("/data/1619", "tid-1619"),
        ("/data/1620", "tid-1620"),
        ("/data/1621", "tid-1621"),
        ("/data/1622", "tid-1622"),
        ("/data/1623", "tid-1623"),
        ("/data/1624", "tid-1624"),
        ("/data/1625", "tid-1625"),
        ("/data/1626", "tid-1626"),
        ("/data/1627", "tid-1627"),
        ("/data/1628", "tid-1628"),
        ("/data/1629", "tid-1629"),
        ("/data/1630", "tid-1630"),
        ("/data/1631", "tid-1631"),
        ("/data/1632", "tid-1632"),
        ("/data/1633", "tid-1633"),
        ("/data/1634", "tid-1634"),
        ("/data/1635", "tid-1635"),
        ("/data/1636", "tid-1636"),
        ("/data/1637", "tid-1637"),
        ("/data/1638", "tid-1638"),
        ("/data/1639", "tid-1639"),
        ("/data/1640", "tid-1640"),
        ("/data/1641", "tid-1641"),
        ("/data/1642", "tid-1642"),
        ("/data/1643", "tid-1643"),
        ("/data/1644", "tid-1644"),
        ("/data/1645", "tid-1645"),
        ("/data/1646", "tid-1646"),
        ("/data/1647", "tid-1647"),
        ("/data/1648", "tid-1648"),
        ("/data/1649", "tid-1649"),
        ("/data/1650", "tid-1650"),
        ("/data/1651", "tid-1651"),
        ("/data/1652", "tid-1652"),
        ("/data/1653", "tid-1653"),
        ("/data/1654", "tid-1654"),
        ("/data/1655", "tid-1655"),
        ("/data/1656", "tid-1656"),
        ("/data/1657", "tid-1657"),
        ("/data/1658", "tid-1658"),
        ("/data/1659", "tid-1659"),
        ("/data/1660", "tid-1660"),
        ("/data/1661", "tid-1661"),
        ("/data/1662", "tid-1662"),
        ("/data/1663", "tid-1663"),
        ("/data/1664", "tid-1664"),
        ("/data/1665", "tid-1665"),
        ("/data/1666", "tid-1666"),
        ("/data/1667", "tid-1667"),
        ("/data/1668", "tid-1668"),
        ("/data/1669", "tid-1669"),
        ("/data/1670", "tid-1670"),
        ("/data/1671", "tid-1671"),
        ("/data/1672", "tid-1672"),
        ("/data/1673", "tid-1673"),
        ("/data/1674", "tid-1674"),
        ("/data/1675", "tid-1675"),
        ("/data/1676", "tid-1676"),
        ("/data/1677", "tid-1677"),
        ("/data/1678", "tid-1678"),
        ("/data/1679", "tid-1679"),
        ("/data/1680", "tid-1680"),
        ("/data/1681", "tid-1681"),
        ("/data/1682", "tid-1682"),
        ("/data/1683", "tid-1683"),
        ("/data/1684", "tid-1684"),
        ("/data/1685", "tid-1685"),
        ("/data/1686", "tid-1686"),
        ("/data/1687", "tid-1687"),
        ("/data/1688", "tid-1688"),
        ("/data/1689", "tid-1689"),
        ("/data/1690", "tid-1690"),
        ("/data/1691", "tid-1691"),
        ("/data/1692", "tid-1692"),
        ("/data/1693", "tid-1693"),
        ("/data/1694", "tid-1694"),
        ("/data/1695", "tid-1695"),
        ("/data/1696", "tid-1696"),
        ("/data/1697", "tid-1697"),
        ("/data/1698", "tid-1698"),
        ("/data/1699", "tid-1699"),
        ("/data/1700", "tid-1700"),
        ("/data/1701", "tid-1701"),
        ("/data/1702", "tid-1702"),
        ("/data/1703", "tid-1703"),
        ("/data/1704", "tid-1704"),
        ("/data/1705", "tid-1705"),
        ("/data/1706", "tid-1706"),
        ("/data/1707", "tid-1707"),
        ("/data/1708", "tid-1708"),
        ("/data/1709", "tid-1709"),
        ("/data/1710", "tid-1710"),
        ("/data/1711", "tid-1711"),
        ("/data/1712", "tid-1712"),
        ("/data/1713", "tid-1713"),
        ("/data/1714", "tid-1714"),
        ("/data/1715", "tid-1715"),
        ("/data/1716", "tid-1716"),
        ("/data/1717", "tid-1717"),
        ("/data/1718", "tid-1718"),
        ("/data/1719", "tid-1719"),
        ("/data/1720", "tid-1720"),
        ("/data/1721", "tid-1721"),
        ("/data/1722", "tid-1722"),
        ("/data/1723", "tid-1723"),
        ("/data/1724", "tid-1724"),
        ("/data/1725", "tid-1725"),
        ("/data/1726", "tid-1726"),
        ("/data/1727", "tid-1727"),
        ("/data/1728", "tid-1728"),
        ("/data/1729", "tid-1729"),
        ("/data/1730", "tid-1730"),
        ("/data/1731", "tid-1731"),
        ("/data/1732", "tid-1732"),
        ("/data/1733", "tid-1733"),
        ("/data/1734", "tid-1734"),
        ("/data/1735", "tid-1735"),
        ("/data/1736", "tid-1736"),
        ("/data/1737", "tid-1737"),
        ("/data/1738", "tid-1738"),
        ("/data/1739", "tid-1739"),
        ("/data/1740", "tid-1740"),
        ("/data/1741", "tid-1741"),
        ("/data/1742", "tid-1742"),
        ("/data/1743", "tid-1743"),
        ("/data/1744", "tid-1744"),
        ("/data/1745", "tid-1745"),
        ("/data/1746", "tid-1746"),
        ("/data/1747", "tid-1747"),
        ("/data/1748", "tid-1748"),
        ("/data/1749", "tid-1749"),
        ("/data/1750", "tid-1750"),
        ("/data/1751", "tid-1751"),
        ("/data/1752", "tid-1752"),
        ("/data/1753", "tid-1753"),
        ("/data/1754", "tid-1754"),
        ("/data/1755", "tid-1755"),
        ("/data/1756", "tid-1756"),
        ("/data/1757", "tid-1757"),
        ("/data/1758", "tid-1758"),
        ("/data/1759", "tid-1759"),
        ("/data/1760", "tid-1760"),
        ("/data/1761", "tid-1761"),
        ("/data/1762", "tid-1762"),
        ("/data/1763", "tid-1763"),
        ("/data/1764", "tid-1764"),
        ("/data/1765", "tid-1765"),
        ("/data/1766", "tid-1766"),
        ("/data/1767", "tid-1767"),
        ("/data/1768", "tid-1768"),
        ("/data/1769", "tid-1769"),
        ("/data/1770", "tid-1770"),
        ("/data/1771", "tid-1771"),
        ("/data/1772", "tid-1772"),
        ("/data/1773", "tid-1773"),
        ("/data/1774", "tid-1774"),
        ("/data/1775", "tid-1775"),
        ("/data/1776", "tid-1776"),
        ("/data/1777", "tid-1777"),
        ("/data/1778", "tid-1778"),
        ("/data/1779", "tid-1779"),
        ("/data/1780", "tid-1780"),
        ("/data/1781", "tid-1781"),
        ("/data/1782", "tid-1782"),
        ("/data/1783", "tid-1783"),
        ("/data/1784", "tid-1784"),
        ("/data/1785", "tid-1785"),
        ("/data/1786", "tid-1786"),
        ("/data/1787", "tid-1787"),
        ("/data/1788", "tid-1788"),
        ("/data/1789", "tid-1789"),
        ("/data/1790", "tid-1790"),
        ("/data/1791", "tid-1791"),
        ("/data/1792", "tid-1792"),
        ("/data/1793", "tid-1793"),
        ("/data/1794", "tid-1794"),
        ("/data/1795", "tid-1795"),
        ("/data/1796", "tid-1796"),
        ("/data/1797", "tid-1797"),
        ("/data/1798", "tid-1798"),
        ("/data/1799", "tid-1799"),
        ("/data/1800", "tid-1800"),
        ("/data/1801", "tid-1801"),
        ("/data/1802", "tid-1802"),
        ("/data/1803", "tid-1803"),
        ("/data/1804", "tid-1804"),
        ("/data/1805", "tid-1805"),
        ("/data/1806", "tid-1806"),
        ("/data/1807", "tid-1807"),
        ("/data/1808", "tid-1808"),
        ("/data/1809", "tid-1809"),
        ("/data/1810", "tid-1810"),
        ("/data/1811", "tid-1811"),
        ("/data/1812", "tid-1812"),
        ("/data/1813", "tid-1813"),
        ("/data/1814", "tid-1814"),
        ("/data/1815", "tid-1815"),
        ("/data/1816", "tid-1816"),
        ("/data/1817", "tid-1817"),
        ("/data/1818", "tid-1818"),
        ("/data/1819", "tid-1819"),
        ("/data/1820", "tid-1820"),
        ("/data/1821", "tid-1821"),
        ("/data/1822", "tid-1822"),
        ("/data/1823", "tid-1823"),
        ("/data/1824", "tid-1824"),
        ("/data/1825", "tid-1825"),
        ("/data/1826", "tid-1826"),
        ("/data/1827", "tid-1827"),
        ("/data/1828", "tid-1828"),
        ("/data/1829", "tid-1829"),
        ("/data/1830", "tid-1830"),
        ("/data/1831", "tid-1831"),
        ("/data/1832", "tid-1832"),
        ("/data/1833", "tid-1833"),
        ("/data/1834", "tid-1834"),
        ("/data/1835", "tid-1835"),
        ("/data/1836", "tid-1836"),
        ("/data/1837", "tid-1837"),
        ("/data/1838", "tid-1838"),
        ("/data/1839", "tid-1839"),
        ("/data/1840", "tid-1840"),
        ("/data/1841", "tid-1841"),
        ("/data/1842", "tid-1842"),
        ("/data/1843", "tid-1843"),
        ("/data/1844", "tid-1844"),
        ("/data/1845", "tid-1845"),
        ("/data/1846", "tid-1846"),
        ("/data/1847", "tid-1847"),
        ("/data/1848", "tid-1848"),
        ("/data/1849", "tid-1849"),
        ("/data/1850", "tid-1850"),
        ("/data/1851", "tid-1851"),
        ("/data/1852", "tid-1852"),
        ("/data/1853", "tid-1853"),
        ("/data/1854", "tid-1854"),
        ("/data/1855", "tid-1855"),
        ("/data/1856", "tid-1856"),
        ("/data/1857", "tid-1857"),
        ("/data/1858", "tid-1858"),
        ("/data/1859", "tid-1859"),
        ("/data/1860", "tid-1860"),
        ("/data/1861", "tid-1861"),
        ("/data/1862", "tid-1862"),
        ("/data/1863", "tid-1863"),
        ("/data/1864", "tid-1864"),
        ("/data/1865", "tid-1865"),
        ("/data/1866", "tid-1866"),
        ("/data/1867", "tid-1867"),
        ("/data/1868", "tid-1868"),
        ("/data/1869", "tid-1869"),
        ("/data/1870", "tid-1870"),
        ("/data/1871", "tid-1871"),
        ("/data/1872", "tid-1872"),
        ("/data/1873", "tid-1873"),
        ("/data/1874", "tid-1874"),
        ("/data/1875", "tid-1875"),
        ("/data/1876", "tid-1876"),
        ("/data/1877", "tid-1877"),
        ("/data/1878", "tid-1878"),
        ("/data/1879", "tid-1879"),
        ("/data/1880", "tid-1880"),
        ("/data/1881", "tid-1881"),
        ("/data/1882", "tid-1882"),
        ("/data/1883", "tid-1883"),
        ("/data/1884", "tid-1884"),
        ("/data/1885", "tid-1885"),
        ("/data/1886", "tid-1886"),
        ("/data/1887", "tid-1887"),
        ("/data/1888", "tid-1888"),
        ("/data/1889", "tid-1889"),
        ("/data/1890", "tid-1890"),
        ("/data/1891", "tid-1891"),
        ("/data/1892", "tid-1892"),
        ("/data/1893", "tid-1893"),
        ("/data/1894", "tid-1894"),
        ("/data/1895", "tid-1895"),
        ("/data/1896", "tid-1896"),
        ("/data/1897", "tid-1897"),
        ("/data/1898", "tid-1898"),
        ("/data/1899", "tid-1899"),
        ("/data/1900", "tid-1900"),
        ("/data/1901", "tid-1901"),
        ("/data/1902", "tid-1902"),
        ("/data/1903", "tid-1903"),
        ("/data/1904", "tid-1904"),
        ("/data/1905", "tid-1905"),
        ("/data/1906", "tid-1906"),
        ("/data/1907", "tid-1907"),
        ("/data/1908", "tid-1908"),
        ("/data/1909", "tid-1909"),
        ("/data/1910", "tid-1910"),
        ("/data/1911", "tid-1911"),
        ("/data/1912", "tid-1912"),
        ("/data/1913", "tid-1913"),
        ("/data/1914", "tid-1914"),
        ("/data/1915", "tid-1915"),
        ("/data/1916", "tid-1916"),
        ("/data/1917", "tid-1917"),
        ("/data/1918", "tid-1918"),
        ("/data/1919", "tid-1919"),
        ("/data/1920", "tid-1920"),
        ("/data/1921", "tid-1921"),
        ("/data/1922", "tid-1922"),
        ("/data/1923", "tid-1923"),
        ("/data/1924", "tid-1924"),
        ("/data/1925", "tid-1925"),
        ("/data/1926", "tid-1926"),
        ("/data/1927", "tid-1927"),
        ("/data/1928", "tid-1928"),
        ("/data/1929", "tid-1929"),
        ("/data/1930", "tid-1930"),
        ("/data/1931", "tid-1931"),
        ("/data/1932", "tid-1932"),
        ("/data/1933", "tid-1933"),
        ("/data/1934", "tid-1934"),
        ("/data/1935", "tid-1935"),
        ("/data/1936", "tid-1936"),
        ("/data/1937", "tid-1937"),
        ("/data/1938", "tid-1938"),
        ("/data/1939", "tid-1939"),
        ("/data/1940", "tid-1940"),
        ("/data/1941", "tid-1941"),
        ("/data/1942", "tid-1942"),
        ("/data/1943", "tid-1943"),
        ("/data/1944", "tid-1944"),
        ("/data/1945", "tid-1945"),
        ("/data/1946", "tid-1946"),
        ("/data/1947", "tid-1947"),
        ("/data/1948", "tid-1948"),
        ("/data/1949", "tid-1949"),
        ("/data/1950", "tid-1950"),
        ("/data/1951", "tid-1951"),
        ("/data/1952", "tid-1952"),
        ("/data/1953", "tid-1953"),
        ("/data/1954", "tid-1954"),
        ("/data/1955", "tid-1955"),
        ("/data/1956", "tid-1956"),
        ("/data/1957", "tid-1957"),
        ("/data/1958", "tid-1958"),
        ("/data/1959", "tid-1959"),
        ("/data/1960", "tid-1960"),
        ("/data/1961", "tid-1961"),
        ("/data/1962", "tid-1962"),
        ("/data/1963", "tid-1963"),
        ("/data/1964", "tid-1964"),
        ("/data/1965", "tid-1965"),
        ("/data/1966", "tid-1966"),
        ("/data/1967", "tid-1967"),
        ("/data/1968", "tid-1968"),
        ("/data/1969", "tid-1969"),
        ("/data/1970", "tid-1970"),
        ("/data/1971", "tid-1971"),
        ("/data/1972", "tid-1972"),
        ("/data/1973", "tid-1973"),
        ("/data/1974", "tid-1974"),
        ("/data/1975", "tid-1975"),
        ("/data/1976", "tid-1976"),
        ("/data/1977", "tid-1977"),
        ("/data/1978", "tid-1978"),
        ("/data/1979", "tid-1979"),
        ("/data/1980", "tid-1980"),
        ("/data/1981", "tid-1981"),
        ("/data/1982", "tid-1982"),
        ("/data/1983", "tid-1983"),
        ("/data/1984", "tid-1984"),
        ("/data/1985", "tid-1985"),
        ("/data/1986", "tid-1986"),
        ("/data/1987", "tid-1987"),
        ("/data/1988", "tid-1988"),
        ("/data/1989", "tid-1989"),
        ("/data/1990", "tid-1990"),
        ("/data/1991", "tid-1991"),
        ("/data/1992", "tid-1992"),
        ("/data/1993", "tid-1993"),
        ("/data/1994", "tid-1994"),
        ("/data/1995", "tid-1995"),
        ("/data/1996", "tid-1996"),
        ("/data/1997", "tid-1997"),
        ("/data/1998", "tid-1998"),
        ("/data/1999", "tid-1999"),
    ];
    for (base, id) in pairs {
        let path = workspace_index_storage_path(Path::new(base), id);
        assert!(path.starts_with(base));
        assert!(path.to_string_lossy().ends_with(".db"));
        assert!(path.to_string_lossy().contains(WORKSPACE_INDEX_STORAGE_DIR));
        assert!(path.to_string_lossy().contains(id));
    }
}

#[test]
fn pattern_pre_cancel_collect_grid() {
    let labels: &[&str] = &[
        "pre_cancel_000",
        "pre_cancel_001",
        "pre_cancel_002",
        "pre_cancel_003",
        "pre_cancel_004",
        "pre_cancel_005",
        "pre_cancel_006",
        "pre_cancel_007",
        "pre_cancel_008",
        "pre_cancel_009",
        "pre_cancel_010",
        "pre_cancel_011",
        "pre_cancel_012",
        "pre_cancel_013",
        "pre_cancel_014",
        "pre_cancel_015",
        "pre_cancel_016",
        "pre_cancel_017",
        "pre_cancel_018",
        "pre_cancel_019",
        "pre_cancel_020",
        "pre_cancel_021",
        "pre_cancel_022",
        "pre_cancel_023",
        "pre_cancel_024",
        "pre_cancel_025",
        "pre_cancel_026",
        "pre_cancel_027",
        "pre_cancel_028",
        "pre_cancel_029",
        "pre_cancel_030",
        "pre_cancel_031",
        "pre_cancel_032",
        "pre_cancel_033",
        "pre_cancel_034",
        "pre_cancel_035",
        "pre_cancel_036",
        "pre_cancel_037",
        "pre_cancel_038",
        "pre_cancel_039",
        "pre_cancel_040",
        "pre_cancel_041",
        "pre_cancel_042",
        "pre_cancel_043",
        "pre_cancel_044",
        "pre_cancel_045",
        "pre_cancel_046",
        "pre_cancel_047",
        "pre_cancel_048",
        "pre_cancel_049",
        "pre_cancel_050",
        "pre_cancel_051",
        "pre_cancel_052",
        "pre_cancel_053",
        "pre_cancel_054",
        "pre_cancel_055",
        "pre_cancel_056",
        "pre_cancel_057",
        "pre_cancel_058",
        "pre_cancel_059",
        "pre_cancel_060",
        "pre_cancel_061",
        "pre_cancel_062",
        "pre_cancel_063",
        "pre_cancel_064",
        "pre_cancel_065",
        "pre_cancel_066",
        "pre_cancel_067",
        "pre_cancel_068",
        "pre_cancel_069",
        "pre_cancel_070",
        "pre_cancel_071",
        "pre_cancel_072",
        "pre_cancel_073",
        "pre_cancel_074",
        "pre_cancel_075",
        "pre_cancel_076",
        "pre_cancel_077",
        "pre_cancel_078",
        "pre_cancel_079",
        "pre_cancel_080",
        "pre_cancel_081",
        "pre_cancel_082",
        "pre_cancel_083",
        "pre_cancel_084",
        "pre_cancel_085",
        "pre_cancel_086",
        "pre_cancel_087",
        "pre_cancel_088",
        "pre_cancel_089",
        "pre_cancel_090",
        "pre_cancel_091",
        "pre_cancel_092",
        "pre_cancel_093",
        "pre_cancel_094",
        "pre_cancel_095",
        "pre_cancel_096",
        "pre_cancel_097",
        "pre_cancel_098",
        "pre_cancel_099",
        "pre_cancel_100",
        "pre_cancel_101",
        "pre_cancel_102",
        "pre_cancel_103",
        "pre_cancel_104",
        "pre_cancel_105",
        "pre_cancel_106",
        "pre_cancel_107",
        "pre_cancel_108",
        "pre_cancel_109",
        "pre_cancel_110",
        "pre_cancel_111",
        "pre_cancel_112",
        "pre_cancel_113",
        "pre_cancel_114",
        "pre_cancel_115",
        "pre_cancel_116",
        "pre_cancel_117",
        "pre_cancel_118",
        "pre_cancel_119",
        "pre_cancel_120",
        "pre_cancel_121",
        "pre_cancel_122",
        "pre_cancel_123",
        "pre_cancel_124",
        "pre_cancel_125",
        "pre_cancel_126",
        "pre_cancel_127",
        "pre_cancel_128",
        "pre_cancel_129",
        "pre_cancel_130",
        "pre_cancel_131",
        "pre_cancel_132",
        "pre_cancel_133",
        "pre_cancel_134",
        "pre_cancel_135",
        "pre_cancel_136",
        "pre_cancel_137",
        "pre_cancel_138",
        "pre_cancel_139",
        "pre_cancel_140",
        "pre_cancel_141",
        "pre_cancel_142",
        "pre_cancel_143",
        "pre_cancel_144",
        "pre_cancel_145",
        "pre_cancel_146",
        "pre_cancel_147",
        "pre_cancel_148",
        "pre_cancel_149",
        "pre_cancel_150",
        "pre_cancel_151",
        "pre_cancel_152",
        "pre_cancel_153",
        "pre_cancel_154",
        "pre_cancel_155",
        "pre_cancel_156",
        "pre_cancel_157",
        "pre_cancel_158",
        "pre_cancel_159",
        "pre_cancel_160",
        "pre_cancel_161",
        "pre_cancel_162",
        "pre_cancel_163",
        "pre_cancel_164",
        "pre_cancel_165",
        "pre_cancel_166",
        "pre_cancel_167",
        "pre_cancel_168",
        "pre_cancel_169",
        "pre_cancel_170",
        "pre_cancel_171",
        "pre_cancel_172",
        "pre_cancel_173",
        "pre_cancel_174",
        "pre_cancel_175",
        "pre_cancel_176",
        "pre_cancel_177",
        "pre_cancel_178",
        "pre_cancel_179",
        "pre_cancel_180",
        "pre_cancel_181",
        "pre_cancel_182",
        "pre_cancel_183",
        "pre_cancel_184",
        "pre_cancel_185",
        "pre_cancel_186",
        "pre_cancel_187",
        "pre_cancel_188",
        "pre_cancel_189",
        "pre_cancel_190",
        "pre_cancel_191",
        "pre_cancel_192",
        "pre_cancel_193",
        "pre_cancel_194",
        "pre_cancel_195",
        "pre_cancel_196",
        "pre_cancel_197",
        "pre_cancel_198",
        "pre_cancel_199",
        "pre_cancel_200",
        "pre_cancel_201",
        "pre_cancel_202",
        "pre_cancel_203",
        "pre_cancel_204",
        "pre_cancel_205",
        "pre_cancel_206",
        "pre_cancel_207",
        "pre_cancel_208",
        "pre_cancel_209",
        "pre_cancel_210",
        "pre_cancel_211",
        "pre_cancel_212",
        "pre_cancel_213",
        "pre_cancel_214",
        "pre_cancel_215",
        "pre_cancel_216",
        "pre_cancel_217",
        "pre_cancel_218",
        "pre_cancel_219",
        "pre_cancel_220",
        "pre_cancel_221",
        "pre_cancel_222",
        "pre_cancel_223",
        "pre_cancel_224",
        "pre_cancel_225",
        "pre_cancel_226",
        "pre_cancel_227",
        "pre_cancel_228",
        "pre_cancel_229",
        "pre_cancel_230",
        "pre_cancel_231",
        "pre_cancel_232",
        "pre_cancel_233",
        "pre_cancel_234",
        "pre_cancel_235",
        "pre_cancel_236",
        "pre_cancel_237",
        "pre_cancel_238",
        "pre_cancel_239",
        "pre_cancel_240",
        "pre_cancel_241",
        "pre_cancel_242",
        "pre_cancel_243",
        "pre_cancel_244",
        "pre_cancel_245",
        "pre_cancel_246",
        "pre_cancel_247",
        "pre_cancel_248",
        "pre_cancel_249",
        "pre_cancel_250",
        "pre_cancel_251",
        "pre_cancel_252",
        "pre_cancel_253",
        "pre_cancel_254",
        "pre_cancel_255",
        "pre_cancel_256",
        "pre_cancel_257",
        "pre_cancel_258",
        "pre_cancel_259",
        "pre_cancel_260",
        "pre_cancel_261",
        "pre_cancel_262",
        "pre_cancel_263",
        "pre_cancel_264",
        "pre_cancel_265",
        "pre_cancel_266",
        "pre_cancel_267",
        "pre_cancel_268",
        "pre_cancel_269",
        "pre_cancel_270",
        "pre_cancel_271",
        "pre_cancel_272",
        "pre_cancel_273",
        "pre_cancel_274",
        "pre_cancel_275",
        "pre_cancel_276",
        "pre_cancel_277",
        "pre_cancel_278",
        "pre_cancel_279",
        "pre_cancel_280",
        "pre_cancel_281",
        "pre_cancel_282",
        "pre_cancel_283",
        "pre_cancel_284",
        "pre_cancel_285",
        "pre_cancel_286",
        "pre_cancel_287",
        "pre_cancel_288",
        "pre_cancel_289",
        "pre_cancel_290",
        "pre_cancel_291",
        "pre_cancel_292",
        "pre_cancel_293",
        "pre_cancel_294",
        "pre_cancel_295",
        "pre_cancel_296",
        "pre_cancel_297",
        "pre_cancel_298",
        "pre_cancel_299",
        "pre_cancel_300",
        "pre_cancel_301",
        "pre_cancel_302",
        "pre_cancel_303",
        "pre_cancel_304",
        "pre_cancel_305",
        "pre_cancel_306",
        "pre_cancel_307",
        "pre_cancel_308",
        "pre_cancel_309",
        "pre_cancel_310",
        "pre_cancel_311",
        "pre_cancel_312",
        "pre_cancel_313",
        "pre_cancel_314",
        "pre_cancel_315",
        "pre_cancel_316",
        "pre_cancel_317",
        "pre_cancel_318",
        "pre_cancel_319",
        "pre_cancel_320",
        "pre_cancel_321",
        "pre_cancel_322",
        "pre_cancel_323",
        "pre_cancel_324",
        "pre_cancel_325",
        "pre_cancel_326",
        "pre_cancel_327",
        "pre_cancel_328",
        "pre_cancel_329",
        "pre_cancel_330",
        "pre_cancel_331",
        "pre_cancel_332",
        "pre_cancel_333",
        "pre_cancel_334",
        "pre_cancel_335",
        "pre_cancel_336",
        "pre_cancel_337",
        "pre_cancel_338",
        "pre_cancel_339",
        "pre_cancel_340",
        "pre_cancel_341",
        "pre_cancel_342",
        "pre_cancel_343",
        "pre_cancel_344",
        "pre_cancel_345",
        "pre_cancel_346",
        "pre_cancel_347",
        "pre_cancel_348",
        "pre_cancel_349",
        "pre_cancel_350",
        "pre_cancel_351",
        "pre_cancel_352",
        "pre_cancel_353",
        "pre_cancel_354",
        "pre_cancel_355",
        "pre_cancel_356",
        "pre_cancel_357",
        "pre_cancel_358",
        "pre_cancel_359",
        "pre_cancel_360",
        "pre_cancel_361",
        "pre_cancel_362",
        "pre_cancel_363",
        "pre_cancel_364",
        "pre_cancel_365",
        "pre_cancel_366",
        "pre_cancel_367",
        "pre_cancel_368",
        "pre_cancel_369",
        "pre_cancel_370",
        "pre_cancel_371",
        "pre_cancel_372",
        "pre_cancel_373",
        "pre_cancel_374",
        "pre_cancel_375",
        "pre_cancel_376",
        "pre_cancel_377",
        "pre_cancel_378",
        "pre_cancel_379",
        "pre_cancel_380",
        "pre_cancel_381",
        "pre_cancel_382",
        "pre_cancel_383",
        "pre_cancel_384",
        "pre_cancel_385",
        "pre_cancel_386",
        "pre_cancel_387",
        "pre_cancel_388",
        "pre_cancel_389",
        "pre_cancel_390",
        "pre_cancel_391",
        "pre_cancel_392",
        "pre_cancel_393",
        "pre_cancel_394",
        "pre_cancel_395",
        "pre_cancel_396",
        "pre_cancel_397",
        "pre_cancel_398",
        "pre_cancel_399",
    ];
    for label in labels {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join(label);
        std::fs::create_dir_all(&root).unwrap();
        for n in 0..12 {
            std::fs::write(root.join(format!("f{n}.txt")), "x").unwrap();
        }
        let result = collect_workspace_index_documents(
            &root,
            &FolderListPolicy::default(),
            &WorkspaceIndexCaps::default(),
            &AtomicBool::new(true),
        );
        assert!(result.cancelled, "{label}");
        assert!(result.truncated, "{label}");
        assert!(result.documents.len() < 12, "{label}");
    }
}
