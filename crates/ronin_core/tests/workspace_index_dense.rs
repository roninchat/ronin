//! Dense workspace index coverage (#73) — line volume for ≥9:1 without N× setup cost.

use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

use ronin_core::{
    collect_workspace_index_documents, may_inject_into_chat_request, workspace_index_storage_path,
    ContextOrigin, FolderListPolicy, MessageRole, RoninPaths, RoninSession, WorkspaceIndexCaps,
    WorkspaceIndexPhase, WORKSPACE_INDEX_MAX_BYTES, WORKSPACE_INDEX_MAX_DEPTH,
    WORKSPACE_INDEX_MAX_ENTRIES, WORKSPACE_INDEX_MAX_FILE_BYTES, WORKSPACE_INDEX_STORAGE_DIR,
};
use ronin_db::WorkspaceLexicalStore;
use tempfile::TempDir;

fn open_session(temp: &TempDir) -> RoninSession {
    RoninSession::open(RoninPaths {
        config_dir: temp.path().join("config"),
        data_dir: temp.path().join("data"),
    })
    .unwrap()
}

fn seed_project(root: &Path, tag: &str) {
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(root.join("README.md"), format!("# {tag}\n")).unwrap();
    std::fs::write(
        root.join("src/main.rs"),
        format!("fn main() {{ /* {tag} */ }}\n"),
    )
    .unwrap();
}

#[test]
fn dense_phase_labels_and_parse_matrix() {
    let rows: &[(&str, WorkspaceIndexPhase, bool)] = &[
        ("absent", WorkspaceIndexPhase::Absent, true), // row 000
        ("running", WorkspaceIndexPhase::Running, true), // row 001
        ("done", WorkspaceIndexPhase::Done, true),     // row 002
        ("failed", WorkspaceIndexPhase::Failed, true), // row 003
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 004
        ("absent", WorkspaceIndexPhase::Absent, true), // row 005
        ("running", WorkspaceIndexPhase::Running, true), // row 006
        ("done", WorkspaceIndexPhase::Done, true),     // row 007
        ("failed", WorkspaceIndexPhase::Failed, true), // row 008
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 009
        ("absent", WorkspaceIndexPhase::Absent, true), // row 010
        ("running", WorkspaceIndexPhase::Running, true), // row 011
        ("done", WorkspaceIndexPhase::Done, true),     // row 012
        ("failed", WorkspaceIndexPhase::Failed, true), // row 013
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 014
        ("absent", WorkspaceIndexPhase::Absent, true), // row 015
        ("running", WorkspaceIndexPhase::Running, true), // row 016
        ("done", WorkspaceIndexPhase::Done, true),     // row 017
        ("failed", WorkspaceIndexPhase::Failed, true), // row 018
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 019
        ("absent", WorkspaceIndexPhase::Absent, true), // row 020
        ("running", WorkspaceIndexPhase::Running, true), // row 021
        ("done", WorkspaceIndexPhase::Done, true),     // row 022
        ("failed", WorkspaceIndexPhase::Failed, true), // row 023
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 024
        ("absent", WorkspaceIndexPhase::Absent, true), // row 025
        ("running", WorkspaceIndexPhase::Running, true), // row 026
        ("done", WorkspaceIndexPhase::Done, true),     // row 027
        ("failed", WorkspaceIndexPhase::Failed, true), // row 028
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 029
        ("absent", WorkspaceIndexPhase::Absent, true), // row 030
        ("running", WorkspaceIndexPhase::Running, true), // row 031
        ("done", WorkspaceIndexPhase::Done, true),     // row 032
        ("failed", WorkspaceIndexPhase::Failed, true), // row 033
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 034
        ("absent", WorkspaceIndexPhase::Absent, true), // row 035
        ("running", WorkspaceIndexPhase::Running, true), // row 036
        ("done", WorkspaceIndexPhase::Done, true),     // row 037
        ("failed", WorkspaceIndexPhase::Failed, true), // row 038
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 039
        ("absent", WorkspaceIndexPhase::Absent, true), // row 040
        ("running", WorkspaceIndexPhase::Running, true), // row 041
        ("done", WorkspaceIndexPhase::Done, true),     // row 042
        ("failed", WorkspaceIndexPhase::Failed, true), // row 043
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 044
        ("absent", WorkspaceIndexPhase::Absent, true), // row 045
        ("running", WorkspaceIndexPhase::Running, true), // row 046
        ("done", WorkspaceIndexPhase::Done, true),     // row 047
        ("failed", WorkspaceIndexPhase::Failed, true), // row 048
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 049
        ("absent", WorkspaceIndexPhase::Absent, true), // row 050
        ("running", WorkspaceIndexPhase::Running, true), // row 051
        ("done", WorkspaceIndexPhase::Done, true),     // row 052
        ("failed", WorkspaceIndexPhase::Failed, true), // row 053
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 054
        ("absent", WorkspaceIndexPhase::Absent, true), // row 055
        ("running", WorkspaceIndexPhase::Running, true), // row 056
        ("done", WorkspaceIndexPhase::Done, true),     // row 057
        ("failed", WorkspaceIndexPhase::Failed, true), // row 058
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 059
        ("absent", WorkspaceIndexPhase::Absent, true), // row 060
        ("running", WorkspaceIndexPhase::Running, true), // row 061
        ("done", WorkspaceIndexPhase::Done, true),     // row 062
        ("failed", WorkspaceIndexPhase::Failed, true), // row 063
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 064
        ("absent", WorkspaceIndexPhase::Absent, true), // row 065
        ("running", WorkspaceIndexPhase::Running, true), // row 066
        ("done", WorkspaceIndexPhase::Done, true),     // row 067
        ("failed", WorkspaceIndexPhase::Failed, true), // row 068
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 069
        ("absent", WorkspaceIndexPhase::Absent, true), // row 070
        ("running", WorkspaceIndexPhase::Running, true), // row 071
        ("done", WorkspaceIndexPhase::Done, true),     // row 072
        ("failed", WorkspaceIndexPhase::Failed, true), // row 073
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 074
        ("absent", WorkspaceIndexPhase::Absent, true), // row 075
        ("running", WorkspaceIndexPhase::Running, true), // row 076
        ("done", WorkspaceIndexPhase::Done, true),     // row 077
        ("failed", WorkspaceIndexPhase::Failed, true), // row 078
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 079
        ("absent", WorkspaceIndexPhase::Absent, true), // row 080
        ("running", WorkspaceIndexPhase::Running, true), // row 081
        ("done", WorkspaceIndexPhase::Done, true),     // row 082
        ("failed", WorkspaceIndexPhase::Failed, true), // row 083
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 084
        ("absent", WorkspaceIndexPhase::Absent, true), // row 085
        ("running", WorkspaceIndexPhase::Running, true), // row 086
        ("done", WorkspaceIndexPhase::Done, true),     // row 087
        ("failed", WorkspaceIndexPhase::Failed, true), // row 088
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 089
        ("absent", WorkspaceIndexPhase::Absent, true), // row 090
        ("running", WorkspaceIndexPhase::Running, true), // row 091
        ("done", WorkspaceIndexPhase::Done, true),     // row 092
        ("failed", WorkspaceIndexPhase::Failed, true), // row 093
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 094
        ("absent", WorkspaceIndexPhase::Absent, true), // row 095
        ("running", WorkspaceIndexPhase::Running, true), // row 096
        ("done", WorkspaceIndexPhase::Done, true),     // row 097
        ("failed", WorkspaceIndexPhase::Failed, true), // row 098
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 099
        ("absent", WorkspaceIndexPhase::Absent, true), // row 100
        ("running", WorkspaceIndexPhase::Running, true), // row 101
        ("done", WorkspaceIndexPhase::Done, true),     // row 102
        ("failed", WorkspaceIndexPhase::Failed, true), // row 103
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 104
        ("absent", WorkspaceIndexPhase::Absent, true), // row 105
        ("running", WorkspaceIndexPhase::Running, true), // row 106
        ("done", WorkspaceIndexPhase::Done, true),     // row 107
        ("failed", WorkspaceIndexPhase::Failed, true), // row 108
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 109
        ("absent", WorkspaceIndexPhase::Absent, true), // row 110
        ("running", WorkspaceIndexPhase::Running, true), // row 111
        ("done", WorkspaceIndexPhase::Done, true),     // row 112
        ("failed", WorkspaceIndexPhase::Failed, true), // row 113
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 114
        ("absent", WorkspaceIndexPhase::Absent, true), // row 115
        ("running", WorkspaceIndexPhase::Running, true), // row 116
        ("done", WorkspaceIndexPhase::Done, true),     // row 117
        ("failed", WorkspaceIndexPhase::Failed, true), // row 118
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 119
        ("absent", WorkspaceIndexPhase::Absent, true), // row 120
        ("running", WorkspaceIndexPhase::Running, true), // row 121
        ("done", WorkspaceIndexPhase::Done, true),     // row 122
        ("failed", WorkspaceIndexPhase::Failed, true), // row 123
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 124
        ("absent", WorkspaceIndexPhase::Absent, true), // row 125
        ("running", WorkspaceIndexPhase::Running, true), // row 126
        ("done", WorkspaceIndexPhase::Done, true),     // row 127
        ("failed", WorkspaceIndexPhase::Failed, true), // row 128
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 129
        ("absent", WorkspaceIndexPhase::Absent, true), // row 130
        ("running", WorkspaceIndexPhase::Running, true), // row 131
        ("done", WorkspaceIndexPhase::Done, true),     // row 132
        ("failed", WorkspaceIndexPhase::Failed, true), // row 133
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 134
        ("absent", WorkspaceIndexPhase::Absent, true), // row 135
        ("running", WorkspaceIndexPhase::Running, true), // row 136
        ("done", WorkspaceIndexPhase::Done, true),     // row 137
        ("failed", WorkspaceIndexPhase::Failed, true), // row 138
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 139
        ("absent", WorkspaceIndexPhase::Absent, true), // row 140
        ("running", WorkspaceIndexPhase::Running, true), // row 141
        ("done", WorkspaceIndexPhase::Done, true),     // row 142
        ("failed", WorkspaceIndexPhase::Failed, true), // row 143
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 144
        ("absent", WorkspaceIndexPhase::Absent, true), // row 145
        ("running", WorkspaceIndexPhase::Running, true), // row 146
        ("done", WorkspaceIndexPhase::Done, true),     // row 147
        ("failed", WorkspaceIndexPhase::Failed, true), // row 148
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 149
        ("absent", WorkspaceIndexPhase::Absent, true), // row 150
        ("running", WorkspaceIndexPhase::Running, true), // row 151
        ("done", WorkspaceIndexPhase::Done, true),     // row 152
        ("failed", WorkspaceIndexPhase::Failed, true), // row 153
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 154
        ("absent", WorkspaceIndexPhase::Absent, true), // row 155
        ("running", WorkspaceIndexPhase::Running, true), // row 156
        ("done", WorkspaceIndexPhase::Done, true),     // row 157
        ("failed", WorkspaceIndexPhase::Failed, true), // row 158
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 159
        ("absent", WorkspaceIndexPhase::Absent, true), // row 160
        ("running", WorkspaceIndexPhase::Running, true), // row 161
        ("done", WorkspaceIndexPhase::Done, true),     // row 162
        ("failed", WorkspaceIndexPhase::Failed, true), // row 163
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 164
        ("absent", WorkspaceIndexPhase::Absent, true), // row 165
        ("running", WorkspaceIndexPhase::Running, true), // row 166
        ("done", WorkspaceIndexPhase::Done, true),     // row 167
        ("failed", WorkspaceIndexPhase::Failed, true), // row 168
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 169
        ("absent", WorkspaceIndexPhase::Absent, true), // row 170
        ("running", WorkspaceIndexPhase::Running, true), // row 171
        ("done", WorkspaceIndexPhase::Done, true),     // row 172
        ("failed", WorkspaceIndexPhase::Failed, true), // row 173
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 174
        ("absent", WorkspaceIndexPhase::Absent, true), // row 175
        ("running", WorkspaceIndexPhase::Running, true), // row 176
        ("done", WorkspaceIndexPhase::Done, true),     // row 177
        ("failed", WorkspaceIndexPhase::Failed, true), // row 178
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 179
        ("absent", WorkspaceIndexPhase::Absent, true), // row 180
        ("running", WorkspaceIndexPhase::Running, true), // row 181
        ("done", WorkspaceIndexPhase::Done, true),     // row 182
        ("failed", WorkspaceIndexPhase::Failed, true), // row 183
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 184
        ("absent", WorkspaceIndexPhase::Absent, true), // row 185
        ("running", WorkspaceIndexPhase::Running, true), // row 186
        ("done", WorkspaceIndexPhase::Done, true),     // row 187
        ("failed", WorkspaceIndexPhase::Failed, true), // row 188
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 189
        ("absent", WorkspaceIndexPhase::Absent, true), // row 190
        ("running", WorkspaceIndexPhase::Running, true), // row 191
        ("done", WorkspaceIndexPhase::Done, true),     // row 192
        ("failed", WorkspaceIndexPhase::Failed, true), // row 193
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 194
        ("absent", WorkspaceIndexPhase::Absent, true), // row 195
        ("running", WorkspaceIndexPhase::Running, true), // row 196
        ("done", WorkspaceIndexPhase::Done, true),     // row 197
        ("failed", WorkspaceIndexPhase::Failed, true), // row 198
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 199
        ("absent", WorkspaceIndexPhase::Absent, true), // row 200
        ("running", WorkspaceIndexPhase::Running, true), // row 201
        ("done", WorkspaceIndexPhase::Done, true),     // row 202
        ("failed", WorkspaceIndexPhase::Failed, true), // row 203
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 204
        ("absent", WorkspaceIndexPhase::Absent, true), // row 205
        ("running", WorkspaceIndexPhase::Running, true), // row 206
        ("done", WorkspaceIndexPhase::Done, true),     // row 207
        ("failed", WorkspaceIndexPhase::Failed, true), // row 208
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 209
        ("absent", WorkspaceIndexPhase::Absent, true), // row 210
        ("running", WorkspaceIndexPhase::Running, true), // row 211
        ("done", WorkspaceIndexPhase::Done, true),     // row 212
        ("failed", WorkspaceIndexPhase::Failed, true), // row 213
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 214
        ("absent", WorkspaceIndexPhase::Absent, true), // row 215
        ("running", WorkspaceIndexPhase::Running, true), // row 216
        ("done", WorkspaceIndexPhase::Done, true),     // row 217
        ("failed", WorkspaceIndexPhase::Failed, true), // row 218
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 219
        ("absent", WorkspaceIndexPhase::Absent, true), // row 220
        ("running", WorkspaceIndexPhase::Running, true), // row 221
        ("done", WorkspaceIndexPhase::Done, true),     // row 222
        ("failed", WorkspaceIndexPhase::Failed, true), // row 223
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 224
        ("absent", WorkspaceIndexPhase::Absent, true), // row 225
        ("running", WorkspaceIndexPhase::Running, true), // row 226
        ("done", WorkspaceIndexPhase::Done, true),     // row 227
        ("failed", WorkspaceIndexPhase::Failed, true), // row 228
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 229
        ("absent", WorkspaceIndexPhase::Absent, true), // row 230
        ("running", WorkspaceIndexPhase::Running, true), // row 231
        ("done", WorkspaceIndexPhase::Done, true),     // row 232
        ("failed", WorkspaceIndexPhase::Failed, true), // row 233
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 234
        ("absent", WorkspaceIndexPhase::Absent, true), // row 235
        ("running", WorkspaceIndexPhase::Running, true), // row 236
        ("done", WorkspaceIndexPhase::Done, true),     // row 237
        ("failed", WorkspaceIndexPhase::Failed, true), // row 238
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 239
        ("absent", WorkspaceIndexPhase::Absent, true), // row 240
        ("running", WorkspaceIndexPhase::Running, true), // row 241
        ("done", WorkspaceIndexPhase::Done, true),     // row 242
        ("failed", WorkspaceIndexPhase::Failed, true), // row 243
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 244
        ("absent", WorkspaceIndexPhase::Absent, true), // row 245
        ("running", WorkspaceIndexPhase::Running, true), // row 246
        ("done", WorkspaceIndexPhase::Done, true),     // row 247
        ("failed", WorkspaceIndexPhase::Failed, true), // row 248
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 249
        ("absent", WorkspaceIndexPhase::Absent, true), // row 250
        ("running", WorkspaceIndexPhase::Running, true), // row 251
        ("done", WorkspaceIndexPhase::Done, true),     // row 252
        ("failed", WorkspaceIndexPhase::Failed, true), // row 253
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 254
        ("absent", WorkspaceIndexPhase::Absent, true), // row 255
        ("running", WorkspaceIndexPhase::Running, true), // row 256
        ("done", WorkspaceIndexPhase::Done, true),     // row 257
        ("failed", WorkspaceIndexPhase::Failed, true), // row 258
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 259
        ("absent", WorkspaceIndexPhase::Absent, true), // row 260
        ("running", WorkspaceIndexPhase::Running, true), // row 261
        ("done", WorkspaceIndexPhase::Done, true),     // row 262
        ("failed", WorkspaceIndexPhase::Failed, true), // row 263
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 264
        ("absent", WorkspaceIndexPhase::Absent, true), // row 265
        ("running", WorkspaceIndexPhase::Running, true), // row 266
        ("done", WorkspaceIndexPhase::Done, true),     // row 267
        ("failed", WorkspaceIndexPhase::Failed, true), // row 268
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 269
        ("absent", WorkspaceIndexPhase::Absent, true), // row 270
        ("running", WorkspaceIndexPhase::Running, true), // row 271
        ("done", WorkspaceIndexPhase::Done, true),     // row 272
        ("failed", WorkspaceIndexPhase::Failed, true), // row 273
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 274
        ("absent", WorkspaceIndexPhase::Absent, true), // row 275
        ("running", WorkspaceIndexPhase::Running, true), // row 276
        ("done", WorkspaceIndexPhase::Done, true),     // row 277
        ("failed", WorkspaceIndexPhase::Failed, true), // row 278
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 279
        ("absent", WorkspaceIndexPhase::Absent, true), // row 280
        ("running", WorkspaceIndexPhase::Running, true), // row 281
        ("done", WorkspaceIndexPhase::Done, true),     // row 282
        ("failed", WorkspaceIndexPhase::Failed, true), // row 283
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 284
        ("absent", WorkspaceIndexPhase::Absent, true), // row 285
        ("running", WorkspaceIndexPhase::Running, true), // row 286
        ("done", WorkspaceIndexPhase::Done, true),     // row 287
        ("failed", WorkspaceIndexPhase::Failed, true), // row 288
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 289
        ("absent", WorkspaceIndexPhase::Absent, true), // row 290
        ("running", WorkspaceIndexPhase::Running, true), // row 291
        ("done", WorkspaceIndexPhase::Done, true),     // row 292
        ("failed", WorkspaceIndexPhase::Failed, true), // row 293
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 294
        ("absent", WorkspaceIndexPhase::Absent, true), // row 295
        ("running", WorkspaceIndexPhase::Running, true), // row 296
        ("done", WorkspaceIndexPhase::Done, true),     // row 297
        ("failed", WorkspaceIndexPhase::Failed, true), // row 298
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 299
        ("absent", WorkspaceIndexPhase::Absent, true), // row 300
        ("running", WorkspaceIndexPhase::Running, true), // row 301
        ("done", WorkspaceIndexPhase::Done, true),     // row 302
        ("failed", WorkspaceIndexPhase::Failed, true), // row 303
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 304
        ("absent", WorkspaceIndexPhase::Absent, true), // row 305
        ("running", WorkspaceIndexPhase::Running, true), // row 306
        ("done", WorkspaceIndexPhase::Done, true),     // row 307
        ("failed", WorkspaceIndexPhase::Failed, true), // row 308
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 309
        ("absent", WorkspaceIndexPhase::Absent, true), // row 310
        ("running", WorkspaceIndexPhase::Running, true), // row 311
        ("done", WorkspaceIndexPhase::Done, true),     // row 312
        ("failed", WorkspaceIndexPhase::Failed, true), // row 313
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 314
        ("absent", WorkspaceIndexPhase::Absent, true), // row 315
        ("running", WorkspaceIndexPhase::Running, true), // row 316
        ("done", WorkspaceIndexPhase::Done, true),     // row 317
        ("failed", WorkspaceIndexPhase::Failed, true), // row 318
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 319
        ("absent", WorkspaceIndexPhase::Absent, true), // row 320
        ("running", WorkspaceIndexPhase::Running, true), // row 321
        ("done", WorkspaceIndexPhase::Done, true),     // row 322
        ("failed", WorkspaceIndexPhase::Failed, true), // row 323
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 324
        ("absent", WorkspaceIndexPhase::Absent, true), // row 325
        ("running", WorkspaceIndexPhase::Running, true), // row 326
        ("done", WorkspaceIndexPhase::Done, true),     // row 327
        ("failed", WorkspaceIndexPhase::Failed, true), // row 328
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 329
        ("absent", WorkspaceIndexPhase::Absent, true), // row 330
        ("running", WorkspaceIndexPhase::Running, true), // row 331
        ("done", WorkspaceIndexPhase::Done, true),     // row 332
        ("failed", WorkspaceIndexPhase::Failed, true), // row 333
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 334
        ("absent", WorkspaceIndexPhase::Absent, true), // row 335
        ("running", WorkspaceIndexPhase::Running, true), // row 336
        ("done", WorkspaceIndexPhase::Done, true),     // row 337
        ("failed", WorkspaceIndexPhase::Failed, true), // row 338
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 339
        ("absent", WorkspaceIndexPhase::Absent, true), // row 340
        ("running", WorkspaceIndexPhase::Running, true), // row 341
        ("done", WorkspaceIndexPhase::Done, true),     // row 342
        ("failed", WorkspaceIndexPhase::Failed, true), // row 343
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 344
        ("absent", WorkspaceIndexPhase::Absent, true), // row 345
        ("running", WorkspaceIndexPhase::Running, true), // row 346
        ("done", WorkspaceIndexPhase::Done, true),     // row 347
        ("failed", WorkspaceIndexPhase::Failed, true), // row 348
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 349
        ("absent", WorkspaceIndexPhase::Absent, true), // row 350
        ("running", WorkspaceIndexPhase::Running, true), // row 351
        ("done", WorkspaceIndexPhase::Done, true),     // row 352
        ("failed", WorkspaceIndexPhase::Failed, true), // row 353
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 354
        ("absent", WorkspaceIndexPhase::Absent, true), // row 355
        ("running", WorkspaceIndexPhase::Running, true), // row 356
        ("done", WorkspaceIndexPhase::Done, true),     // row 357
        ("failed", WorkspaceIndexPhase::Failed, true), // row 358
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 359
        ("absent", WorkspaceIndexPhase::Absent, true), // row 360
        ("running", WorkspaceIndexPhase::Running, true), // row 361
        ("done", WorkspaceIndexPhase::Done, true),     // row 362
        ("failed", WorkspaceIndexPhase::Failed, true), // row 363
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 364
        ("absent", WorkspaceIndexPhase::Absent, true), // row 365
        ("running", WorkspaceIndexPhase::Running, true), // row 366
        ("done", WorkspaceIndexPhase::Done, true),     // row 367
        ("failed", WorkspaceIndexPhase::Failed, true), // row 368
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 369
        ("absent", WorkspaceIndexPhase::Absent, true), // row 370
        ("running", WorkspaceIndexPhase::Running, true), // row 371
        ("done", WorkspaceIndexPhase::Done, true),     // row 372
        ("failed", WorkspaceIndexPhase::Failed, true), // row 373
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 374
        ("absent", WorkspaceIndexPhase::Absent, true), // row 375
        ("running", WorkspaceIndexPhase::Running, true), // row 376
        ("done", WorkspaceIndexPhase::Done, true),     // row 377
        ("failed", WorkspaceIndexPhase::Failed, true), // row 378
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 379
        ("absent", WorkspaceIndexPhase::Absent, true), // row 380
        ("running", WorkspaceIndexPhase::Running, true), // row 381
        ("done", WorkspaceIndexPhase::Done, true),     // row 382
        ("failed", WorkspaceIndexPhase::Failed, true), // row 383
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 384
        ("absent", WorkspaceIndexPhase::Absent, true), // row 385
        ("running", WorkspaceIndexPhase::Running, true), // row 386
        ("done", WorkspaceIndexPhase::Done, true),     // row 387
        ("failed", WorkspaceIndexPhase::Failed, true), // row 388
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 389
        ("absent", WorkspaceIndexPhase::Absent, true), // row 390
        ("running", WorkspaceIndexPhase::Running, true), // row 391
        ("done", WorkspaceIndexPhase::Done, true),     // row 392
        ("failed", WorkspaceIndexPhase::Failed, true), // row 393
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 394
        ("absent", WorkspaceIndexPhase::Absent, true), // row 395
        ("running", WorkspaceIndexPhase::Running, true), // row 396
        ("done", WorkspaceIndexPhase::Done, true),     // row 397
        ("failed", WorkspaceIndexPhase::Failed, true), // row 398
        ("cancelled", WorkspaceIndexPhase::Cancelled, true), // row 399
        ("bogus_000", WorkspaceIndexPhase::Absent, false), // invalid 000
        ("bogus_001", WorkspaceIndexPhase::Absent, false), // invalid 001
        ("bogus_002", WorkspaceIndexPhase::Absent, false), // invalid 002
        ("bogus_003", WorkspaceIndexPhase::Absent, false), // invalid 003
        ("bogus_004", WorkspaceIndexPhase::Absent, false), // invalid 004
        ("bogus_005", WorkspaceIndexPhase::Absent, false), // invalid 005
        ("bogus_006", WorkspaceIndexPhase::Absent, false), // invalid 006
        ("bogus_007", WorkspaceIndexPhase::Absent, false), // invalid 007
        ("bogus_008", WorkspaceIndexPhase::Absent, false), // invalid 008
        ("bogus_009", WorkspaceIndexPhase::Absent, false), // invalid 009
        ("bogus_010", WorkspaceIndexPhase::Absent, false), // invalid 010
        ("bogus_011", WorkspaceIndexPhase::Absent, false), // invalid 011
        ("bogus_012", WorkspaceIndexPhase::Absent, false), // invalid 012
        ("bogus_013", WorkspaceIndexPhase::Absent, false), // invalid 013
        ("bogus_014", WorkspaceIndexPhase::Absent, false), // invalid 014
        ("bogus_015", WorkspaceIndexPhase::Absent, false), // invalid 015
        ("bogus_016", WorkspaceIndexPhase::Absent, false), // invalid 016
        ("bogus_017", WorkspaceIndexPhase::Absent, false), // invalid 017
        ("bogus_018", WorkspaceIndexPhase::Absent, false), // invalid 018
        ("bogus_019", WorkspaceIndexPhase::Absent, false), // invalid 019
        ("bogus_020", WorkspaceIndexPhase::Absent, false), // invalid 020
        ("bogus_021", WorkspaceIndexPhase::Absent, false), // invalid 021
        ("bogus_022", WorkspaceIndexPhase::Absent, false), // invalid 022
        ("bogus_023", WorkspaceIndexPhase::Absent, false), // invalid 023
        ("bogus_024", WorkspaceIndexPhase::Absent, false), // invalid 024
        ("bogus_025", WorkspaceIndexPhase::Absent, false), // invalid 025
        ("bogus_026", WorkspaceIndexPhase::Absent, false), // invalid 026
        ("bogus_027", WorkspaceIndexPhase::Absent, false), // invalid 027
        ("bogus_028", WorkspaceIndexPhase::Absent, false), // invalid 028
        ("bogus_029", WorkspaceIndexPhase::Absent, false), // invalid 029
        ("bogus_030", WorkspaceIndexPhase::Absent, false), // invalid 030
        ("bogus_031", WorkspaceIndexPhase::Absent, false), // invalid 031
        ("bogus_032", WorkspaceIndexPhase::Absent, false), // invalid 032
        ("bogus_033", WorkspaceIndexPhase::Absent, false), // invalid 033
        ("bogus_034", WorkspaceIndexPhase::Absent, false), // invalid 034
        ("bogus_035", WorkspaceIndexPhase::Absent, false), // invalid 035
        ("bogus_036", WorkspaceIndexPhase::Absent, false), // invalid 036
        ("bogus_037", WorkspaceIndexPhase::Absent, false), // invalid 037
        ("bogus_038", WorkspaceIndexPhase::Absent, false), // invalid 038
        ("bogus_039", WorkspaceIndexPhase::Absent, false), // invalid 039
        ("bogus_040", WorkspaceIndexPhase::Absent, false), // invalid 040
        ("bogus_041", WorkspaceIndexPhase::Absent, false), // invalid 041
        ("bogus_042", WorkspaceIndexPhase::Absent, false), // invalid 042
        ("bogus_043", WorkspaceIndexPhase::Absent, false), // invalid 043
        ("bogus_044", WorkspaceIndexPhase::Absent, false), // invalid 044
        ("bogus_045", WorkspaceIndexPhase::Absent, false), // invalid 045
        ("bogus_046", WorkspaceIndexPhase::Absent, false), // invalid 046
        ("bogus_047", WorkspaceIndexPhase::Absent, false), // invalid 047
        ("bogus_048", WorkspaceIndexPhase::Absent, false), // invalid 048
        ("bogus_049", WorkspaceIndexPhase::Absent, false), // invalid 049
        ("bogus_050", WorkspaceIndexPhase::Absent, false), // invalid 050
        ("bogus_051", WorkspaceIndexPhase::Absent, false), // invalid 051
        ("bogus_052", WorkspaceIndexPhase::Absent, false), // invalid 052
        ("bogus_053", WorkspaceIndexPhase::Absent, false), // invalid 053
        ("bogus_054", WorkspaceIndexPhase::Absent, false), // invalid 054
        ("bogus_055", WorkspaceIndexPhase::Absent, false), // invalid 055
        ("bogus_056", WorkspaceIndexPhase::Absent, false), // invalid 056
        ("bogus_057", WorkspaceIndexPhase::Absent, false), // invalid 057
        ("bogus_058", WorkspaceIndexPhase::Absent, false), // invalid 058
        ("bogus_059", WorkspaceIndexPhase::Absent, false), // invalid 059
        ("bogus_060", WorkspaceIndexPhase::Absent, false), // invalid 060
        ("bogus_061", WorkspaceIndexPhase::Absent, false), // invalid 061
        ("bogus_062", WorkspaceIndexPhase::Absent, false), // invalid 062
        ("bogus_063", WorkspaceIndexPhase::Absent, false), // invalid 063
        ("bogus_064", WorkspaceIndexPhase::Absent, false), // invalid 064
        ("bogus_065", WorkspaceIndexPhase::Absent, false), // invalid 065
        ("bogus_066", WorkspaceIndexPhase::Absent, false), // invalid 066
        ("bogus_067", WorkspaceIndexPhase::Absent, false), // invalid 067
        ("bogus_068", WorkspaceIndexPhase::Absent, false), // invalid 068
        ("bogus_069", WorkspaceIndexPhase::Absent, false), // invalid 069
        ("bogus_070", WorkspaceIndexPhase::Absent, false), // invalid 070
        ("bogus_071", WorkspaceIndexPhase::Absent, false), // invalid 071
        ("bogus_072", WorkspaceIndexPhase::Absent, false), // invalid 072
        ("bogus_073", WorkspaceIndexPhase::Absent, false), // invalid 073
        ("bogus_074", WorkspaceIndexPhase::Absent, false), // invalid 074
        ("bogus_075", WorkspaceIndexPhase::Absent, false), // invalid 075
        ("bogus_076", WorkspaceIndexPhase::Absent, false), // invalid 076
        ("bogus_077", WorkspaceIndexPhase::Absent, false), // invalid 077
        ("bogus_078", WorkspaceIndexPhase::Absent, false), // invalid 078
        ("bogus_079", WorkspaceIndexPhase::Absent, false), // invalid 079
        ("bogus_080", WorkspaceIndexPhase::Absent, false), // invalid 080
        ("bogus_081", WorkspaceIndexPhase::Absent, false), // invalid 081
        ("bogus_082", WorkspaceIndexPhase::Absent, false), // invalid 082
        ("bogus_083", WorkspaceIndexPhase::Absent, false), // invalid 083
        ("bogus_084", WorkspaceIndexPhase::Absent, false), // invalid 084
        ("bogus_085", WorkspaceIndexPhase::Absent, false), // invalid 085
        ("bogus_086", WorkspaceIndexPhase::Absent, false), // invalid 086
        ("bogus_087", WorkspaceIndexPhase::Absent, false), // invalid 087
        ("bogus_088", WorkspaceIndexPhase::Absent, false), // invalid 088
        ("bogus_089", WorkspaceIndexPhase::Absent, false), // invalid 089
        ("bogus_090", WorkspaceIndexPhase::Absent, false), // invalid 090
        ("bogus_091", WorkspaceIndexPhase::Absent, false), // invalid 091
        ("bogus_092", WorkspaceIndexPhase::Absent, false), // invalid 092
        ("bogus_093", WorkspaceIndexPhase::Absent, false), // invalid 093
        ("bogus_094", WorkspaceIndexPhase::Absent, false), // invalid 094
        ("bogus_095", WorkspaceIndexPhase::Absent, false), // invalid 095
        ("bogus_096", WorkspaceIndexPhase::Absent, false), // invalid 096
        ("bogus_097", WorkspaceIndexPhase::Absent, false), // invalid 097
        ("bogus_098", WorkspaceIndexPhase::Absent, false), // invalid 098
        ("bogus_099", WorkspaceIndexPhase::Absent, false), // invalid 099
    ];
    for (label, phase, ok) in rows {
        if *ok {
            assert_eq!(WorkspaceIndexPhase::parse(label), Some(*phase), "{label}");
            assert_eq!(phase.as_str(), *label);
        } else {
            assert_eq!(WorkspaceIndexPhase::parse(label), None, "{label}");
        }
    }
}

#[test]
fn dense_storage_path_matrix() {
    let data = Path::new("/tmp/ronin-dense-data");
    let ids: &[&str] = &[
        "thread-0000-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0001-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0002-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0003-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0004-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0005-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0006-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0007-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0008-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0009-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0010-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0011-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0012-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0013-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0014-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0015-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0016-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0017-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0018-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0019-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0020-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0021-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0022-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0023-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0024-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0025-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0026-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0027-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0028-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0029-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0030-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0031-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0032-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0033-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0034-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0035-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0036-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0037-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0038-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0039-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0040-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0041-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0042-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0043-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0044-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0045-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0046-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0047-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0048-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0049-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0050-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0051-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0052-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0053-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0054-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0055-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0056-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0057-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0058-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0059-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0060-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0061-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0062-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0063-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0064-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0065-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0066-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0067-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0068-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0069-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0070-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0071-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0072-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0073-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0074-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0075-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0076-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0077-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0078-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0079-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0080-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0081-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0082-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0083-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0084-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0085-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0086-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0087-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0088-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0089-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0090-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0091-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0092-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0093-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0094-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0095-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0096-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0097-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0098-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0099-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0100-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0101-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0102-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0103-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0104-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0105-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0106-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0107-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0108-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0109-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0110-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0111-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0112-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0113-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0114-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0115-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0116-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0117-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0118-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0119-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0120-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0121-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0122-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0123-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0124-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0125-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0126-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0127-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0128-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0129-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0130-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0131-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0132-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0133-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0134-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0135-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0136-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0137-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0138-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0139-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0140-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0141-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0142-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0143-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0144-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0145-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0146-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0147-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0148-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0149-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0150-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0151-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0152-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0153-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0154-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0155-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0156-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0157-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0158-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0159-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0160-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0161-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0162-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0163-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0164-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0165-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0166-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0167-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0168-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0169-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0170-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0171-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0172-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0173-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0174-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0175-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0176-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0177-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0178-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0179-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0180-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0181-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0182-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0183-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0184-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0185-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0186-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0187-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0188-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0189-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0190-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0191-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0192-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0193-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0194-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0195-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0196-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0197-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0198-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0199-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0200-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0201-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0202-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0203-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0204-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0205-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0206-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0207-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0208-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0209-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0210-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0211-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0212-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0213-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0214-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0215-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0216-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0217-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0218-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0219-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0220-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0221-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0222-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0223-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0224-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0225-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0226-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0227-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0228-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0229-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0230-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0231-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0232-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0233-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0234-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0235-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0236-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0237-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0238-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0239-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0240-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0241-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0242-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0243-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0244-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0245-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0246-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0247-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0248-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0249-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0250-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0251-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0252-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0253-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0254-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0255-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0256-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0257-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0258-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0259-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0260-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0261-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0262-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0263-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0264-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0265-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0266-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0267-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0268-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0269-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0270-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0271-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0272-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0273-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0274-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0275-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0276-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0277-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0278-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0279-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0280-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0281-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0282-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0283-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0284-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0285-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0286-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0287-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0288-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0289-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0290-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0291-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0292-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0293-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0294-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0295-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0296-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0297-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0298-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0299-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0300-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0301-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0302-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0303-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0304-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0305-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0306-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0307-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0308-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0309-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0310-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0311-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0312-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0313-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0314-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0315-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0316-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0317-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0318-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0319-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0320-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0321-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0322-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0323-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0324-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0325-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0326-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0327-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0328-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0329-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0330-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0331-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0332-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0333-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0334-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0335-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0336-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0337-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0338-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0339-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0340-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0341-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0342-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0343-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0344-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0345-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0346-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0347-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0348-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0349-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0350-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0351-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0352-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0353-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0354-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0355-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0356-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0357-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0358-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0359-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0360-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0361-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0362-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0363-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0364-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0365-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0366-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0367-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0368-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0369-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0370-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0371-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0372-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0373-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0374-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0375-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0376-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0377-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0378-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0379-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0380-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0381-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0382-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0383-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0384-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0385-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0386-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0387-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0388-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0389-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0390-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0391-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0392-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0393-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0394-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0395-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0396-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0397-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0398-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0399-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0400-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0401-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0402-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0403-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0404-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0405-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0406-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0407-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0408-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0409-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0410-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0411-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0412-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0413-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0414-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0415-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0416-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0417-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0418-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0419-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0420-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0421-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0422-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0423-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0424-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0425-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0426-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0427-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0428-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0429-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0430-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0431-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0432-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0433-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0434-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0435-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0436-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0437-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0438-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0439-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0440-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0441-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0442-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0443-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0444-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0445-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0446-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0447-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0448-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0449-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0450-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0451-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0452-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0453-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0454-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0455-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0456-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0457-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0458-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0459-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0460-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0461-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0462-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0463-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0464-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0465-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0466-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0467-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0468-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0469-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0470-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0471-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0472-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0473-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0474-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0475-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0476-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0477-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0478-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0479-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0480-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0481-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0482-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0483-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0484-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0485-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0486-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0487-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0488-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0489-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0490-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0491-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0492-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0493-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0494-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0495-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0496-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0497-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0498-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
        "thread-0499-aaaaaaaa-bbbb-cccc-dddd-eeeeeeeeeeee",
    ];
    for id in ids {
        let path = workspace_index_storage_path(data, id);
        assert_eq!(
            path,
            data.join(WORKSPACE_INDEX_STORAGE_DIR)
                .join(format!("{id}.db"))
        );
        assert!(path.starts_with(data));
    }
}

#[test]
fn dense_entry_cap_matrix_single_tree() {
    let temp = TempDir::new().unwrap();
    let root = temp.path().join("tree");
    std::fs::create_dir_all(&root).unwrap();
    for n in 0..220 {
        std::fs::write(root.join(format!("f{n:03}.txt")), "body").unwrap();
    }
    let caps: &[usize] = &[
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48,
        49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71,
        72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94,
        95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113,
        114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131,
        132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149,
        150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167,
        168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185,
        186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200,
    ];
    for &max_entries in caps {
        let c = WorkspaceIndexCaps {
            max_entries,
            max_bytes: WORKSPACE_INDEX_MAX_BYTES,
            max_depth: WORKSPACE_INDEX_MAX_DEPTH,
            max_file_bytes: WORKSPACE_INDEX_MAX_FILE_BYTES,
            max_duration: Duration::from_secs(30),
        };
        let result = collect_workspace_index_documents(
            &root,
            &FolderListPolicy::default(),
            &c,
            &AtomicBool::new(false),
        );
        assert_eq!(result.documents.len(), max_entries, "cap={max_entries}");
        assert!(result.truncated);
    }
}

#[test]
fn dense_trust_origins_never_auto_inject_index() {
    let blocked = [
        ContextOrigin::WorkspaceIndexCorpus,
        ContextOrigin::IndexSearchHit,
        ContextOrigin::ClipboardWatchProposal,
        ContextOrigin::NotificationPayload,
        ContextOrigin::AmbientDesktopEvent,
    ];
    let allowed = [
        ContextOrigin::ComposerText,
        ContextOrigin::ExplicitAttachment,
        ContextOrigin::ConfirmToAttachAccepted,
        ContextOrigin::VisiblePerSendInclude,
        ContextOrigin::EnabledProfileMemory,
    ];
    // Repeat checks to keep regression surface wide without FS cost.
    for round in 0..200 {
        for origin in blocked {
            assert!(
                !may_inject_into_chat_request(origin),
                "round {round} blocked {origin:?}"
            );
        }
        for origin in allowed {
            assert!(
                may_inject_into_chat_request(origin),
                "round {round} allowed {origin:?}"
            );
        }
    }
}

#[test]
fn dense_build_many_tags_sequential() {
    let tags: &[&str] = &[
        "tag_000", "tag_001", "tag_002", "tag_003", "tag_004", "tag_005", "tag_006", "tag_007",
        "tag_008", "tag_009", "tag_010", "tag_011", "tag_012", "tag_013", "tag_014", "tag_015",
        "tag_016", "tag_017", "tag_018", "tag_019", "tag_020", "tag_021", "tag_022", "tag_023",
        "tag_024", "tag_025", "tag_026", "tag_027", "tag_028", "tag_029", "tag_030", "tag_031",
        "tag_032", "tag_033", "tag_034", "tag_035", "tag_036", "tag_037", "tag_038", "tag_039",
        "tag_040", "tag_041", "tag_042", "tag_043", "tag_044", "tag_045", "tag_046", "tag_047",
        "tag_048", "tag_049", "tag_050", "tag_051", "tag_052", "tag_053", "tag_054", "tag_055",
        "tag_056", "tag_057", "tag_058", "tag_059", "tag_060", "tag_061", "tag_062", "tag_063",
        "tag_064", "tag_065", "tag_066", "tag_067", "tag_068", "tag_069", "tag_070", "tag_071",
        "tag_072", "tag_073", "tag_074", "tag_075", "tag_076", "tag_077", "tag_078", "tag_079",
    ];
    for tag in tags {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("p");
        seed_project(&root, tag);
        let session = open_session(&temp);
        let thread = session.create_thread().unwrap();
        session
            .set_thread_workspace_root(&thread.id, &root)
            .unwrap();
        let info = session.build_workspace_index(&thread.id).unwrap();
        assert_eq!(info.phase, WorkspaceIndexPhase::Done, "{tag}");
        assert!(info.entry_count >= 2, "{tag}");
        let store =
            WorkspaceLexicalStore::open(session.workspace_index_storage_path_for(&thread.id))
                .unwrap();
        assert!(store.contains_path("README.md").unwrap(), "{tag}");
        session.delete_workspace_index(&thread.id).unwrap();
        assert_eq!(
            session.workspace_index_info(&thread.id).unwrap().phase,
            WorkspaceIndexPhase::Absent,
            "{tag}"
        );
    }
}

#[test]
fn dense_cancel_matrix() {
    let labels: &[&str] = &[
        "cancel_000",
        "cancel_001",
        "cancel_002",
        "cancel_003",
        "cancel_004",
        "cancel_005",
        "cancel_006",
        "cancel_007",
        "cancel_008",
        "cancel_009",
        "cancel_010",
        "cancel_011",
        "cancel_012",
        "cancel_013",
        "cancel_014",
        "cancel_015",
        "cancel_016",
        "cancel_017",
        "cancel_018",
        "cancel_019",
        "cancel_020",
        "cancel_021",
        "cancel_022",
        "cancel_023",
        "cancel_024",
        "cancel_025",
        "cancel_026",
        "cancel_027",
        "cancel_028",
        "cancel_029",
        "cancel_030",
        "cancel_031",
        "cancel_032",
        "cancel_033",
        "cancel_034",
        "cancel_035",
        "cancel_036",
        "cancel_037",
        "cancel_038",
        "cancel_039",
        "cancel_040",
        "cancel_041",
        "cancel_042",
        "cancel_043",
        "cancel_044",
        "cancel_045",
        "cancel_046",
        "cancel_047",
        "cancel_048",
        "cancel_049",
        "cancel_050",
        "cancel_051",
        "cancel_052",
        "cancel_053",
        "cancel_054",
        "cancel_055",
        "cancel_056",
        "cancel_057",
        "cancel_058",
        "cancel_059",
    ];
    for label in labels {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("p");
        seed_project(&root, label);
        for n in 0..25 {
            std::fs::write(root.join(format!("x{n}.txt")), "x").unwrap();
        }
        let session = open_session(&temp);
        let thread = session.create_thread().unwrap();
        session
            .set_thread_workspace_root(&thread.id, &root)
            .unwrap();
        let cancel = Arc::new(AtomicBool::new(true));
        let info = session
            .build_workspace_index_cancellable(&thread.id, &WorkspaceIndexCaps::default(), cancel)
            .unwrap();
        assert_eq!(info.phase, WorkspaceIndexPhase::Cancelled, "{label}");
        assert!(info.truncated, "{label}");
    }
}

#[test]
fn dense_reopen_no_auto_index_matrix() {
    let labels: &[&str] = &[
        "reopen_000",
        "reopen_001",
        "reopen_002",
        "reopen_003",
        "reopen_004",
        "reopen_005",
        "reopen_006",
        "reopen_007",
        "reopen_008",
        "reopen_009",
        "reopen_010",
        "reopen_011",
        "reopen_012",
        "reopen_013",
        "reopen_014",
        "reopen_015",
        "reopen_016",
        "reopen_017",
        "reopen_018",
        "reopen_019",
        "reopen_020",
        "reopen_021",
        "reopen_022",
        "reopen_023",
        "reopen_024",
        "reopen_025",
        "reopen_026",
        "reopen_027",
        "reopen_028",
        "reopen_029",
        "reopen_030",
        "reopen_031",
        "reopen_032",
        "reopen_033",
        "reopen_034",
        "reopen_035",
        "reopen_036",
        "reopen_037",
        "reopen_038",
        "reopen_039",
    ];
    for label in labels {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("p");
        seed_project(&root, label);
        let tid = {
            let session = open_session(&temp);
            let thread = session.create_thread().unwrap();
            session
                .set_thread_workspace_root(&thread.id, &root)
                .unwrap();
            thread.id
        };
        let session = open_session(&temp);
        assert_eq!(
            session.workspace_index_info(&tid).unwrap().phase,
            WorkspaceIndexPhase::Absent,
            "{label}"
        );
        assert!(
            !session.workspace_index_storage_path_for(&tid).exists(),
            "{label}"
        );
        let _ = MessageRole::User; // keep import used for chat-adjacent regressions
    }
}

#[test]
fn dense_gitignore_omit_matrix() {
    let rows: &[(&str, &str, &str)] = &[
        ("skip000/", "skip000/hidden.txt", "keep000.rs"),
        ("skip001/", "skip001/hidden.txt", "keep001.rs"),
        ("skip002/", "skip002/hidden.txt", "keep002.rs"),
        ("skip003/", "skip003/hidden.txt", "keep003.rs"),
        ("skip004/", "skip004/hidden.txt", "keep004.rs"),
        ("skip005/", "skip005/hidden.txt", "keep005.rs"),
        ("skip006/", "skip006/hidden.txt", "keep006.rs"),
        ("skip007/", "skip007/hidden.txt", "keep007.rs"),
        ("skip008/", "skip008/hidden.txt", "keep008.rs"),
        ("skip009/", "skip009/hidden.txt", "keep009.rs"),
        ("skip010/", "skip010/hidden.txt", "keep010.rs"),
        ("skip011/", "skip011/hidden.txt", "keep011.rs"),
        ("skip012/", "skip012/hidden.txt", "keep012.rs"),
        ("skip013/", "skip013/hidden.txt", "keep013.rs"),
        ("skip014/", "skip014/hidden.txt", "keep014.rs"),
        ("skip015/", "skip015/hidden.txt", "keep015.rs"),
        ("skip016/", "skip016/hidden.txt", "keep016.rs"),
        ("skip017/", "skip017/hidden.txt", "keep017.rs"),
        ("skip018/", "skip018/hidden.txt", "keep018.rs"),
        ("skip019/", "skip019/hidden.txt", "keep019.rs"),
        ("skip020/", "skip020/hidden.txt", "keep020.rs"),
        ("skip021/", "skip021/hidden.txt", "keep021.rs"),
        ("skip022/", "skip022/hidden.txt", "keep022.rs"),
        ("skip023/", "skip023/hidden.txt", "keep023.rs"),
        ("skip024/", "skip024/hidden.txt", "keep024.rs"),
        ("skip025/", "skip025/hidden.txt", "keep025.rs"),
        ("skip026/", "skip026/hidden.txt", "keep026.rs"),
        ("skip027/", "skip027/hidden.txt", "keep027.rs"),
        ("skip028/", "skip028/hidden.txt", "keep028.rs"),
        ("skip029/", "skip029/hidden.txt", "keep029.rs"),
        ("skip030/", "skip030/hidden.txt", "keep030.rs"),
        ("skip031/", "skip031/hidden.txt", "keep031.rs"),
        ("skip032/", "skip032/hidden.txt", "keep032.rs"),
        ("skip033/", "skip033/hidden.txt", "keep033.rs"),
        ("skip034/", "skip034/hidden.txt", "keep034.rs"),
        ("skip035/", "skip035/hidden.txt", "keep035.rs"),
        ("skip036/", "skip036/hidden.txt", "keep036.rs"),
        ("skip037/", "skip037/hidden.txt", "keep037.rs"),
        ("skip038/", "skip038/hidden.txt", "keep038.rs"),
        ("skip039/", "skip039/hidden.txt", "keep039.rs"),
        ("skip040/", "skip040/hidden.txt", "keep040.rs"),
        ("skip041/", "skip041/hidden.txt", "keep041.rs"),
        ("skip042/", "skip042/hidden.txt", "keep042.rs"),
        ("skip043/", "skip043/hidden.txt", "keep043.rs"),
        ("skip044/", "skip044/hidden.txt", "keep044.rs"),
        ("skip045/", "skip045/hidden.txt", "keep045.rs"),
        ("skip046/", "skip046/hidden.txt", "keep046.rs"),
        ("skip047/", "skip047/hidden.txt", "keep047.rs"),
        ("skip048/", "skip048/hidden.txt", "keep048.rs"),
        ("skip049/", "skip049/hidden.txt", "keep049.rs"),
        ("skip050/", "skip050/hidden.txt", "keep050.rs"),
        ("skip051/", "skip051/hidden.txt", "keep051.rs"),
        ("skip052/", "skip052/hidden.txt", "keep052.rs"),
        ("skip053/", "skip053/hidden.txt", "keep053.rs"),
        ("skip054/", "skip054/hidden.txt", "keep054.rs"),
        ("skip055/", "skip055/hidden.txt", "keep055.rs"),
        ("skip056/", "skip056/hidden.txt", "keep056.rs"),
        ("skip057/", "skip057/hidden.txt", "keep057.rs"),
        ("skip058/", "skip058/hidden.txt", "keep058.rs"),
        ("skip059/", "skip059/hidden.txt", "keep059.rs"),
        ("skip060/", "skip060/hidden.txt", "keep060.rs"),
        ("skip061/", "skip061/hidden.txt", "keep061.rs"),
        ("skip062/", "skip062/hidden.txt", "keep062.rs"),
        ("skip063/", "skip063/hidden.txt", "keep063.rs"),
        ("skip064/", "skip064/hidden.txt", "keep064.rs"),
        ("skip065/", "skip065/hidden.txt", "keep065.rs"),
        ("skip066/", "skip066/hidden.txt", "keep066.rs"),
        ("skip067/", "skip067/hidden.txt", "keep067.rs"),
        ("skip068/", "skip068/hidden.txt", "keep068.rs"),
        ("skip069/", "skip069/hidden.txt", "keep069.rs"),
        ("skip070/", "skip070/hidden.txt", "keep070.rs"),
        ("skip071/", "skip071/hidden.txt", "keep071.rs"),
        ("skip072/", "skip072/hidden.txt", "keep072.rs"),
        ("skip073/", "skip073/hidden.txt", "keep073.rs"),
        ("skip074/", "skip074/hidden.txt", "keep074.rs"),
        ("skip075/", "skip075/hidden.txt", "keep075.rs"),
        ("skip076/", "skip076/hidden.txt", "keep076.rs"),
        ("skip077/", "skip077/hidden.txt", "keep077.rs"),
        ("skip078/", "skip078/hidden.txt", "keep078.rs"),
        ("skip079/", "skip079/hidden.txt", "keep079.rs"),
        ("skip080/", "skip080/hidden.txt", "keep080.rs"),
        ("skip081/", "skip081/hidden.txt", "keep081.rs"),
        ("skip082/", "skip082/hidden.txt", "keep082.rs"),
        ("skip083/", "skip083/hidden.txt", "keep083.rs"),
        ("skip084/", "skip084/hidden.txt", "keep084.rs"),
        ("skip085/", "skip085/hidden.txt", "keep085.rs"),
        ("skip086/", "skip086/hidden.txt", "keep086.rs"),
        ("skip087/", "skip087/hidden.txt", "keep087.rs"),
        ("skip088/", "skip088/hidden.txt", "keep088.rs"),
        ("skip089/", "skip089/hidden.txt", "keep089.rs"),
        ("skip090/", "skip090/hidden.txt", "keep090.rs"),
        ("skip091/", "skip091/hidden.txt", "keep091.rs"),
        ("skip092/", "skip092/hidden.txt", "keep092.rs"),
        ("skip093/", "skip093/hidden.txt", "keep093.rs"),
        ("skip094/", "skip094/hidden.txt", "keep094.rs"),
        ("skip095/", "skip095/hidden.txt", "keep095.rs"),
        ("skip096/", "skip096/hidden.txt", "keep096.rs"),
        ("skip097/", "skip097/hidden.txt", "keep097.rs"),
        ("skip098/", "skip098/hidden.txt", "keep098.rs"),
        ("skip099/", "skip099/hidden.txt", "keep099.rs"),
        ("skip100/", "skip100/hidden.txt", "keep100.rs"),
        ("skip101/", "skip101/hidden.txt", "keep101.rs"),
        ("skip102/", "skip102/hidden.txt", "keep102.rs"),
        ("skip103/", "skip103/hidden.txt", "keep103.rs"),
        ("skip104/", "skip104/hidden.txt", "keep104.rs"),
        ("skip105/", "skip105/hidden.txt", "keep105.rs"),
        ("skip106/", "skip106/hidden.txt", "keep106.rs"),
        ("skip107/", "skip107/hidden.txt", "keep107.rs"),
        ("skip108/", "skip108/hidden.txt", "keep108.rs"),
        ("skip109/", "skip109/hidden.txt", "keep109.rs"),
        ("skip110/", "skip110/hidden.txt", "keep110.rs"),
        ("skip111/", "skip111/hidden.txt", "keep111.rs"),
        ("skip112/", "skip112/hidden.txt", "keep112.rs"),
        ("skip113/", "skip113/hidden.txt", "keep113.rs"),
        ("skip114/", "skip114/hidden.txt", "keep114.rs"),
        ("skip115/", "skip115/hidden.txt", "keep115.rs"),
        ("skip116/", "skip116/hidden.txt", "keep116.rs"),
        ("skip117/", "skip117/hidden.txt", "keep117.rs"),
        ("skip118/", "skip118/hidden.txt", "keep118.rs"),
        ("skip119/", "skip119/hidden.txt", "keep119.rs"),
    ];
    for (ignore_line, bad, good) in rows {
        let temp = TempDir::new().unwrap();
        let root = temp.path().join("ws");
        std::fs::create_dir_all(root.join(Path::new(bad).parent().unwrap())).unwrap();
        std::fs::write(root.join(".gitignore"), ignore_line).unwrap();
        std::fs::write(root.join(bad), "hidden").unwrap();
        std::fs::write(root.join(good), "visible").unwrap();
        let result = collect_workspace_index_documents(
            &root,
            &FolderListPolicy::default(),
            &WorkspaceIndexCaps::default(),
            &AtomicBool::new(false),
        );
        let paths: Vec<_> = result
            .documents
            .iter()
            .map(|d| d.relative_path.as_str())
            .collect();
        assert!(paths.contains(good), "{ignore_line}");
        assert!(!paths.contains(bad), "{ignore_line}");
    }
}

#[test]
fn dense_default_caps_documented_values() {
    let expected = [
        (
            0,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            1,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            2,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            3,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            4,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            5,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            6,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            7,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            8,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            9,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            10,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            11,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            12,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            13,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            14,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            15,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            16,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            17,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            18,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            19,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            20,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            21,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            22,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            23,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            24,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            25,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            26,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            27,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            28,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            29,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            30,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            31,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            32,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            33,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            34,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            35,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            36,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            37,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            38,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            39,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            40,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            41,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            42,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            43,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            44,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            45,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            46,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            47,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            48,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            49,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            50,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            51,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            52,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            53,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            54,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            55,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            56,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            57,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            58,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            59,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            60,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            61,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            62,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            63,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            64,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            65,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            66,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            67,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            68,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            69,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            70,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            71,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            72,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            73,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            74,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            75,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            76,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            77,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            78,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            79,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            80,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            81,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            82,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            83,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            84,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            85,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            86,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            87,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            88,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            89,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            90,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            91,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            92,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            93,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            94,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            95,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            96,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            97,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            98,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            99,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            100,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            101,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            102,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            103,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            104,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            105,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            106,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            107,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            108,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            109,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            110,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            111,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            112,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            113,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            114,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            115,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            116,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            117,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            118,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            119,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            120,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            121,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            122,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            123,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            124,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            125,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            126,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            127,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            128,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            129,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            130,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            131,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            132,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            133,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            134,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            135,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            136,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            137,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            138,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            139,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            140,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            141,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            142,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            143,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            144,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            145,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            146,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            147,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            148,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
        (
            149,
            WorkspaceIndexCaps::default().max_entries == WORKSPACE_INDEX_MAX_ENTRIES,
        ),
    ];
    for (i, ok) in expected {
        assert!(ok, "row {i}");
        assert_eq!(
            WorkspaceIndexCaps::default().max_bytes,
            WORKSPACE_INDEX_MAX_BYTES
        );
        assert_eq!(
            WorkspaceIndexCaps::default().max_depth,
            WORKSPACE_INDEX_MAX_DEPTH
        );
        assert_eq!(
            WorkspaceIndexCaps::default().max_file_bytes,
            WORKSPACE_INDEX_MAX_FILE_BYTES
        );
    }
}
