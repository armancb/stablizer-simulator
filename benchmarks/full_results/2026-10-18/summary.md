# Full Clifford benchmark against Stim

Started: 18 October 2026, 23:23:59 IST (raw timestamps are UTC).

Machine: Intel(R) Core(TM) Ultra 7 255H. Stim 1.16.0 (stim._stim_sse2); local release extension.

Completed 1090 of 1090 planned cases; 0 case errors. Native timing backends ran sequentially.

5 samples after warmup per backend (isolated stress: three gate samples); all raw samples are saved. Times include binding/result conversion. Ratios above 1 mean Stim was faster.

## Scope and interpretation

This is a finite matrix for the pure-state Clifford/Tableau APIs: gate kinds, circuit lengths, target locality, identity/prepared inputs, word/vector boundaries, product/Bell/GHZ/cluster/scrambled states, random/deterministic and single/partial/full Z measurements, native/sequential calls, X/Y measurements, joint Pauli products, interleaved circuits, repeated measurements, repeated fresh-tableau shots, allocation/copy/export/query, and isolated large-memory probes.

Noise, non-Clifford algorithms, mixed-state backends, adaptive classical control, and compiled multi-shot samplers are outside this shared API comparison. Circuit and preparation seeds are fixed and recorded; selected scrambled measurement cases are repeated with three seeds. These circuits are not uniform random Cliffords, and completed memory sizes are demonstrated bounds under the measured workload.

Normal measurement uses independent seeded RNG streams. Random/deterministic branches have the same classification despite differing signs. Small collapsed states are checked exhaustively; large cases use logical outcome replay and sampled state checks. All signed generators are compared for unitary gate cases. The local Pauli implementation uses Clifford decomposition; native_pauli compares it with Stim's native MPP. Shared decomposed cases use the same gate network in both backends. Basis/parity networks include reversal to preserve the physical post-measurement state.

Initialization does not fully touch zero-filled pages; isolated capacity workers apply a full H layer before reporting peak RSS. Copy includes RNG state. Forward-tableau export includes Stim's inverse-to-forward conversion and produces different array organization in the two APIs. Repeated-shot cases include fresh allocation, preparation, measurement, and destruction for every shot.

## Coverage

| Family | Planned | Successful | Case errors |
|---|---:|---:|---:|
| gates | 270 | 270 | 0 |
| measurement | 436 | 436 | 0 |
| protocol | 348 | 348 | 0 |
| lifecycle | 21 | 21 | 0 |
| shots | 12 | 12 | 0 |
| stress | 3 | 3 | 0 |

## Results

Per-case times are total elapsed milliseconds for that case. Measurement counts include every logical measurement, including repeats of the same target. Fresh-shot counts are listed separately. Aggregate win counts can be dominated by the chosen matrix; inspect matching workloads.

| Case | Family | Parameters | Ours ms | Stim ms | Ours / Stim |
|---|---|---|---:|---:|---:|
| 0e22064e1b14a9f0 | gates | n=1, workload=h, count=1, prep=0, seed=731, locality=random | 0.000156 | 0.000337 | 0.463 |
| 3da90de2b54b5181 | gates | n=1, workload=h, count=1, prep=40, seed=731, locality=random | 0.000144 | 0.000271 | 0.531 |
| f5f0b173e6e93a37 | gates | n=1, workload=h, count=1000, prep=0, seed=731, locality=random | 0.003726 | 0.005223 | 0.713 |
| 56a263d7d5bf9eff | gates | n=1, workload=h, count=1000, prep=40, seed=731, locality=random | 0.003512 | 0.005129 | 0.685 |
| 88d01db9771e530b | gates | n=1, workload=h, count=50000, prep=0, seed=731, locality=random | 0.170256 | 0.241713 | 0.704 |
| e2ab27c1e86ed53f | gates | n=1, workload=h, count=50000, prep=40, seed=731, locality=random | 0.200508 | 0.248857 | 0.806 |
| d45ff36d088ffcb8 | gates | n=1, workload=s, count=1, prep=0, seed=731, locality=random | 0.000119 | 0.000318 | 0.374 |
| 2aff586546a1d05c | gates | n=1, workload=s, count=1, prep=40, seed=731, locality=random | 0.000116 | 0.000261 | 0.444 |
| ab94845c83a887e4 | gates | n=1, workload=s, count=1000, prep=0, seed=731, locality=random | 0.004600 | 0.012138 | 0.379 |
| a819e6ecf4803c17 | gates | n=1, workload=s, count=1000, prep=40, seed=731, locality=random | 0.002844 | 0.008969 | 0.317 |
| f83ae967b70c0c9b | gates | n=1, workload=s, count=50000, prep=0, seed=731, locality=random | 0.137706 | 0.406572 | 0.339 |
| 14ff32d39bd40bf7 | gates | n=1, workload=s, count=50000, prep=40, seed=731, locality=random | 0.153878 | 0.436571 | 0.352 |
| 3494240bba72b329 | gates | n=1, workload=mixed, count=1, prep=0, seed=731, locality=random | 0.000160 | 0.000313 | 0.511 |
| c06530695cb03f24 | gates | n=1, workload=mixed, count=1, prep=40, seed=731, locality=random | 0.000128 | 0.000276 | 0.464 |
| 92f8ff9c66a66b72 | gates | n=1, workload=mixed, count=1000, prep=0, seed=731, locality=random | 0.003938 | 0.014638 | 0.269 |
| 5426a40932ac9207 | gates | n=1, workload=mixed, count=1000, prep=40, seed=731, locality=random | 0.003153 | 0.013515 | 0.233 |
| f97a8c69965c727a | gates | n=1, workload=mixed, count=50000, prep=0, seed=731, locality=random | 0.233643 | 0.918339 | 0.254 |
| 8e9831bbe2117906 | gates | n=1, workload=mixed, count=50000, prep=40, seed=731, locality=random | 0.228542 | 0.863601 | 0.265 |
| 15c0f61196ee2048 | gates | n=2, workload=h, count=1, prep=0, seed=731, locality=random | 0.000144 | 0.000279 | 0.516 |
| d6a9b31aaa20d609 | gates | n=2, workload=h, count=1, prep=80, seed=731, locality=random | 0.000109 | 0.000275 | 0.396 |
| e9b394285190e4ce | gates | n=2, workload=h, count=1000, prep=0, seed=731, locality=random | 0.002602 | 0.004878 | 0.533 |
| 07d5a6b5c7095eb8 | gates | n=2, workload=h, count=1000, prep=80, seed=731, locality=random | 0.002600 | 0.004841 | 0.537 |
| aeecb863b79ae6f1 | gates | n=2, workload=h, count=50000, prep=0, seed=731, locality=random | 0.159359 | 0.245082 | 0.650 |
| 5fd9266f468366bd | gates | n=2, workload=h, count=50000, prep=80, seed=731, locality=random | 0.200488 | 0.249829 | 0.803 |
| 4a373ffe61d75cb7 | gates | n=2, workload=s, count=1, prep=0, seed=731, locality=random | 0.000159 | 0.000293 | 0.543 |
| 9ddfec40bd885ebd | gates | n=2, workload=s, count=1, prep=80, seed=731, locality=random | 0.000117 | 0.000268 | 0.437 |
| c1b7a09d76316c65 | gates | n=2, workload=s, count=1000, prep=0, seed=731, locality=random | 0.002488 | 0.008394 | 0.296 |
| 4273a80db2fb0e1a | gates | n=2, workload=s, count=1000, prep=80, seed=731, locality=random | 0.002583 | 0.008970 | 0.288 |
| c851fca8ab0e4055 | gates | n=2, workload=s, count=50000, prep=0, seed=731, locality=random | 0.137096 | 0.439661 | 0.312 |
| c540ca757c8bcf2a | gates | n=2, workload=s, count=50000, prep=80, seed=731, locality=random | 0.131769 | 0.435362 | 0.303 |
| 387ac8687618ee56 | gates | n=2, workload=cx, count=1, prep=0, seed=731, locality=random | 0.000140 | 0.000304 | 0.461 |
| deb23dd5470e9102 | gates | n=2, workload=cx, count=1, prep=80, seed=731, locality=random | 0.000172 | 0.000384 | 0.448 |
| a7e98352b748e0a1 | gates | n=2, workload=cx, count=1000, prep=0, seed=731, locality=random | 0.005938 | 0.018872 | 0.315 |
| 8b0d56ff6666567c | gates | n=2, workload=cx, count=1000, prep=80, seed=731, locality=random | 0.005748 | 0.020050 | 0.287 |
| 9f0af54dc74869a7 | gates | n=2, workload=cx, count=50000, prep=0, seed=731, locality=random | 0.379717 | 0.909313 | 0.418 |
| fdf3860ef890f4c5 | gates | n=2, workload=cx, count=50000, prep=80, seed=731, locality=random | 0.371212 | 0.874379 | 0.425 |
| 8f2bbeeebf7760b4 | gates | n=2, workload=mixed, count=1, prep=0, seed=731, locality=random | 0.000140 | 0.000326 | 0.429 |
| a3fed9b013cf56f0 | gates | n=2, workload=mixed, count=1, prep=80, seed=731, locality=random | 0.000107 | 0.000276 | 0.388 |
| f49e4b3da00af454 | gates | n=2, workload=mixed, count=1000, prep=0, seed=731, locality=random | 0.003359 | 0.020705 | 0.162 |
| 828d32c30d278951 | gates | n=2, workload=mixed, count=1000, prep=80, seed=731, locality=random | 0.003425 | 0.022302 | 0.154 |
| c0c1a4703c316c2d | gates | n=2, workload=mixed, count=50000, prep=0, seed=731, locality=random | 0.536545 | 1.471168 | 0.365 |
| 17bedd110c664dcf | gates | n=2, workload=mixed, count=50000, prep=80, seed=731, locality=random | 0.395846 | 1.356321 | 0.292 |
| 5462f8a6f651df81 | gates | n=63, workload=h, count=1, prep=0, seed=731, locality=random | 0.000101 | 0.000258 | 0.391 |
| 825a6b695ed47bba | gates | n=63, workload=h, count=1, prep=2520, seed=731, locality=random | 0.000272 | 0.000618 | 0.440 |
| b8edb87652dbe163 | gates | n=63, workload=h, count=1000, prep=0, seed=731, locality=random | 0.010153 | 0.014527 | 0.699 |
| 569b5eaf9bc5c39a | gates | n=63, workload=h, count=1000, prep=2520, seed=731, locality=random | 0.002959 | 0.005480 | 0.540 |
| 5e4491ad889f91b3 | gates | n=63, workload=h, count=50000, prep=0, seed=731, locality=random | 0.148939 | 0.240784 | 0.619 |
| f586e72c97a289c0 | gates | n=63, workload=h, count=50000, prep=2520, seed=731, locality=random | 0.124897 | 0.273165 | 0.457 |
| 779f455268f725e5 | gates | n=63, workload=s, count=1, prep=0, seed=731, locality=random | 0.000112 | 0.000264 | 0.424 |
| 73f9fa13c3b53513 | gates | n=63, workload=s, count=1, prep=2520, seed=731, locality=random | 0.000158 | 0.000372 | 0.425 |
| 90ccc645570eed88 | gates | n=63, workload=s, count=1000, prep=0, seed=731, locality=random | 0.003716 | 0.009222 | 0.403 |
| ab3263ebb084ccc7 | gates | n=63, workload=s, count=1000, prep=2520, seed=731, locality=random | 0.002423 | 0.008600 | 0.282 |
| 7951c3c0fcc4d522 | gates | n=63, workload=s, count=50000, prep=0, seed=731, locality=random | 0.139491 | 0.437019 | 0.319 |
| 387295eae99ffe92 | gates | n=63, workload=s, count=50000, prep=2520, seed=731, locality=random | 0.151349 | 0.473565 | 0.320 |
| b2818cdb7cca03db | gates | n=63, workload=cx, count=1, prep=0, seed=731, locality=random | 0.000131 | 0.000308 | 0.425 |
| 28f1259ca1e8419b | gates | n=63, workload=cx, count=1, prep=2520, seed=731, locality=random | 0.000170 | 0.000360 | 0.472 |
| 8e6986e19e3eef43 | gates | n=63, workload=cx, count=1000, prep=0, seed=731, locality=random | 0.005297 | 0.019115 | 0.277 |
| b04bd6fe52a637e7 | gates | n=63, workload=cx, count=1000, prep=2520, seed=731, locality=random | 0.007336 | 0.019288 | 0.380 |
| 73caaa3b1b4ae57b | gates | n=63, workload=cx, count=50000, prep=0, seed=731, locality=random | 0.414602 | 0.936756 | 0.443 |
| a1db98edd44a9dfd | gates | n=63, workload=cx, count=50000, prep=2520, seed=731, locality=random | 0.401390 | 0.936263 | 0.429 |
| 22f9578f199e587d | gates | n=63, workload=mixed, count=1, prep=0, seed=731, locality=random | 0.000142 | 0.000308 | 0.461 |
| d4b9230e0543923d | gates | n=63, workload=mixed, count=1, prep=2520, seed=731, locality=random | 0.000301 | 0.000602 | 0.500 |
| 8d08e146ebfb1c67 | gates | n=63, workload=mixed, count=1000, prep=0, seed=731, locality=random | 0.003615 | 0.022334 | 0.162 |
| 44e579eadc9534fd | gates | n=63, workload=mixed, count=1000, prep=2520, seed=731, locality=random | 0.015757 | 0.062204 | 0.253 |
| 67bf7ba13f53bf29 | gates | n=63, workload=mixed, count=50000, prep=0, seed=731, locality=random | 0.399082 | 1.402884 | 0.284 |
| ee63e697f494f3a1 | gates | n=63, workload=mixed, count=50000, prep=2520, seed=731, locality=random | 0.464925 | 1.431775 | 0.325 |
| 616c0b1e1cf2de2b | gates | n=64, workload=h, count=1, prep=0, seed=731, locality=random | 0.000122 | 0.000296 | 0.412 |
| 85f55d241441b629 | gates | n=64, workload=h, count=1, prep=2560, seed=731, locality=random | 0.000222 | 0.000415 | 0.535 |
| c60a72947790cfe4 | gates | n=64, workload=h, count=1000, prep=0, seed=731, locality=random | 0.003005 | 0.005422 | 0.554 |
| 61976a37cb8d3500 | gates | n=64, workload=h, count=1000, prep=2560, seed=731, locality=random | 0.002862 | 0.005536 | 0.517 |
| e358b5bd93dc6839 | gates | n=64, workload=h, count=50000, prep=0, seed=731, locality=random | 0.129763 | 0.253553 | 0.512 |
| 0834a470143b0c05 | gates | n=64, workload=h, count=50000, prep=2560, seed=731, locality=random | 0.151166 | 0.240233 | 0.629 |
| 67f85fba71246c0c | gates | n=64, workload=s, count=1, prep=0, seed=731, locality=random | 0.000123 | 0.000261 | 0.471 |
| e74eb9914ccfb921 | gates | n=64, workload=s, count=1, prep=2560, seed=731, locality=random | 0.000166 | 0.000316 | 0.525 |
| c678b17227985e5a | gates | n=64, workload=s, count=1000, prep=0, seed=731, locality=random | 0.002481 | 0.009092 | 0.273 |
| 5db3aa53096f8e5a | gates | n=64, workload=s, count=1000, prep=2560, seed=731, locality=random | 0.002884 | 0.009136 | 0.316 |
| 28ce0fa1fd785b07 | gates | n=64, workload=s, count=50000, prep=0, seed=731, locality=random | 0.118739 | 0.427967 | 0.277 |
| d7335de6a36eb8ce | gates | n=64, workload=s, count=50000, prep=2560, seed=731, locality=random | 0.120352 | 0.441100 | 0.273 |
| 59b160d0be3a0603 | gates | n=64, workload=cx, count=1, prep=0, seed=731, locality=random | 0.000132 | 0.000301 | 0.439 |
| 64e01fc82c2a3987 | gates | n=64, workload=cx, count=1, prep=2560, seed=731, locality=random | 0.000162 | 0.000376 | 0.431 |
| 6a7074a7bcc8ecf5 | gates | n=64, workload=cx, count=1000, prep=0, seed=731, locality=random | 0.005609 | 0.018080 | 0.310 |
| 73c1f42f2ffdc109 | gates | n=64, workload=cx, count=1000, prep=2560, seed=731, locality=random | 0.006721 | 0.018006 | 0.373 |
| fc08e8af992cb08c | gates | n=64, workload=cx, count=50000, prep=0, seed=731, locality=random | 0.367040 | 0.881541 | 0.416 |
| 9272e43e6ab06f1e | gates | n=64, workload=cx, count=50000, prep=2560, seed=731, locality=random | 0.369887 | 0.882104 | 0.419 |
| e7c3832909fdad5c | gates | n=64, workload=mixed, count=1, prep=0, seed=731, locality=random | 0.000137 | 0.000285 | 0.481 |
| fcea53bd3031d651 | gates | n=64, workload=mixed, count=1, prep=2560, seed=731, locality=random | 0.000158 | 0.000368 | 0.429 |
| 00d7ccdc4ad9a0dd | gates | n=64, workload=mixed, count=1000, prep=0, seed=731, locality=random | 0.003130 | 0.021439 | 0.146 |
| 2201a608ba77aa2a | gates | n=64, workload=mixed, count=1000, prep=2560, seed=731, locality=random | 0.006246 | 0.024669 | 0.253 |
| eb2dace37c5957bd | gates | n=64, workload=mixed, count=50000, prep=0, seed=731, locality=random | 0.394469 | 1.256641 | 0.314 |
| 9444031cd086bfdc | gates | n=64, workload=mixed, count=50000, prep=2560, seed=731, locality=random | 0.391471 | 1.387068 | 0.282 |
| e3bb6af60c77740d | gates | n=65, workload=h, count=1, prep=0, seed=731, locality=random | 0.000167 | 0.000406 | 0.411 |
| 2a28bc4e32335228 | gates | n=65, workload=h, count=1, prep=2600, seed=731, locality=random | 0.000168 | 0.000381 | 0.441 |
| 084d23f3d92b8a8b | gates | n=65, workload=h, count=1000, prep=0, seed=731, locality=random | 0.002576 | 0.005202 | 0.495 |
| 114ddc6029ee40d7 | gates | n=65, workload=h, count=1000, prep=2600, seed=731, locality=random | 0.003108 | 0.005359 | 0.580 |
| d6f833f6b56acdd6 | gates | n=65, workload=h, count=50000, prep=0, seed=731, locality=random | 0.145202 | 0.235621 | 0.616 |
| 6f6181683d96f5ca | gates | n=65, workload=h, count=50000, prep=2600, seed=731, locality=random | 0.118586 | 0.231787 | 0.512 |
| 2511f8890cfe067e | gates | n=65, workload=s, count=1, prep=0, seed=731, locality=random | 0.000098 | 0.000279 | 0.351 |
| 2ec8d23beef98dc7 | gates | n=65, workload=s, count=1, prep=2600, seed=731, locality=random | 0.000162 | 0.000351 | 0.462 |
| 212c7b7161107a90 | gates | n=65, workload=s, count=1000, prep=0, seed=731, locality=random | 0.002491 | 0.008894 | 0.280 |
| b39043ee149c4941 | gates | n=65, workload=s, count=1000, prep=2600, seed=731, locality=random | 0.003214 | 0.008758 | 0.367 |
| 8225d66ef1614ead | gates | n=65, workload=s, count=50000, prep=0, seed=731, locality=random | 0.137444 | 0.422001 | 0.326 |
| aa348156760ab6f5 | gates | n=65, workload=s, count=50000, prep=2600, seed=731, locality=random | 0.120477 | 0.409740 | 0.294 |
| fe57a51aca53cadc | gates | n=65, workload=cx, count=1, prep=0, seed=731, locality=random | 0.000189 | 0.000354 | 0.534 |
| 612288d86b751625 | gates | n=65, workload=cx, count=1, prep=2600, seed=731, locality=random | 0.000161 | 0.000358 | 0.450 |
| 2e3ae36d839aa8e7 | gates | n=65, workload=cx, count=1000, prep=0, seed=731, locality=random | 0.005704 | 0.018326 | 0.311 |
| 2ab0fcc9132a3e31 | gates | n=65, workload=cx, count=1000, prep=2600, seed=731, locality=random | 0.007470 | 0.019275 | 0.388 |
| 07a221d0bca043e3 | gates | n=65, workload=cx, count=50000, prep=0, seed=731, locality=random | 0.370882 | 0.888363 | 0.417 |
| aafd22f5d598db6c | gates | n=65, workload=cx, count=50000, prep=2600, seed=731, locality=random | 0.408748 | 0.970625 | 0.421 |
| c8cabb84411ce7a4 | gates | n=65, workload=mixed, count=1, prep=0, seed=731, locality=random | 0.000124 | 0.000300 | 0.413 |
| 2bed4aadb2f5884e | gates | n=65, workload=mixed, count=1, prep=2600, seed=731, locality=random | 0.000181 | 0.000420 | 0.431 |
| 4560bf23ffc7e03a | gates | n=65, workload=mixed, count=1000, prep=0, seed=731, locality=random | 0.003422 | 0.022837 | 0.150 |
| 5f7d46918c025542 | gates | n=65, workload=mixed, count=1000, prep=2600, seed=731, locality=random | 0.007187 | 0.026349 | 0.273 |
| 8a3f725e91226b9f | gates | n=65, workload=mixed, count=50000, prep=0, seed=731, locality=random | 0.413750 | 1.428058 | 0.290 |
| 5e4ba1cac261895c | gates | n=65, workload=mixed, count=50000, prep=2600, seed=731, locality=random | 0.400157 | 1.274560 | 0.314 |
| fdbcc5cb8742e774 | gates | n=127, workload=h, count=1, prep=0, seed=731, locality=random | 0.000121 | 0.000279 | 0.434 |
| 8a751a6c34f900ad | gates | n=127, workload=h, count=1, prep=5080, seed=731, locality=random | 0.000221 | 0.000451 | 0.490 |
| 39ec395193c0b287 | gates | n=127, workload=h, count=1000, prep=0, seed=731, locality=random | 0.002507 | 0.004911 | 0.510 |
| 771d320ea2b7bbc2 | gates | n=127, workload=h, count=1000, prep=5080, seed=731, locality=random | 0.002604 | 0.005020 | 0.519 |
| 7bfaf89365b2a1b4 | gates | n=127, workload=h, count=50000, prep=0, seed=731, locality=random | 0.123219 | 0.226204 | 0.545 |
| 3aac5df49bbd7975 | gates | n=127, workload=h, count=50000, prep=5080, seed=731, locality=random | 0.166066 | 0.237779 | 0.698 |
| f57d7468c2bc92a5 | gates | n=127, workload=s, count=1, prep=0, seed=731, locality=random | 0.000378 | 0.000667 | 0.567 |
| 7b4a91984d730386 | gates | n=127, workload=s, count=1, prep=5080, seed=731, locality=random | 0.000219 | 0.000430 | 0.509 |
| 078c07aab5bd7b53 | gates | n=127, workload=s, count=1000, prep=0, seed=731, locality=random | 0.002369 | 0.008527 | 0.278 |
| bbc1f7e286b7a7ac | gates | n=127, workload=s, count=1000, prep=5080, seed=731, locality=random | 0.002474 | 0.008770 | 0.282 |
| 0ff6c1ed376a9d13 | gates | n=127, workload=s, count=50000, prep=0, seed=731, locality=random | 0.120881 | 0.424700 | 0.285 |
| 109eeeab98753a98 | gates | n=127, workload=s, count=50000, prep=5080, seed=731, locality=random | 0.119467 | 0.435459 | 0.274 |
| f9b53a2f09ac537a | gates | n=127, workload=cx, count=1, prep=0, seed=731, locality=random | 0.000136 | 0.000297 | 0.458 |
| bf15527415a863ad | gates | n=127, workload=cx, count=1, prep=5080, seed=731, locality=random | 0.000195 | 0.000495 | 0.394 |
| a025683cf05f2f37 | gates | n=127, workload=cx, count=1000, prep=0, seed=731, locality=random | 0.005910 | 0.018880 | 0.313 |
| e473d73b96ba53c8 | gates | n=127, workload=cx, count=1000, prep=5080, seed=731, locality=random | 0.007483 | 0.018626 | 0.402 |
| b767224481e565fd | gates | n=127, workload=cx, count=50000, prep=0, seed=731, locality=random | 0.380139 | 0.914203 | 0.416 |
| 6faccce48fa4c3c8 | gates | n=127, workload=cx, count=50000, prep=5080, seed=731, locality=random | 0.366178 | 0.881968 | 0.415 |
| 6ad96437971d2e49 | gates | n=127, workload=mixed, count=1, prep=0, seed=731, locality=random | 0.000132 | 0.000310 | 0.426 |
| 8c75e6d19a710a75 | gates | n=127, workload=mixed, count=1, prep=5080, seed=731, locality=random | 0.000143 | 0.000390 | 0.367 |
| 5823d8604f8d647b | gates | n=127, workload=mixed, count=1000, prep=0, seed=731, locality=random | 0.003054 | 0.021974 | 0.139 |
| 54cabb4147ef1c83 | gates | n=127, workload=mixed, count=1000, prep=5080, seed=731, locality=random | 0.007823 | 0.026810 | 0.292 |
| 0207447e86ea6b6f | gates | n=127, workload=mixed, count=50000, prep=0, seed=731, locality=random | 0.400903 | 1.318532 | 0.304 |
| 6c597b04e17ae245 | gates | n=127, workload=mixed, count=50000, prep=5080, seed=731, locality=random | 0.387649 | 1.249474 | 0.310 |
| b9648663d1dd858c | gates | n=128, workload=h, count=1, prep=0, seed=731, locality=random | 0.000115 | 0.000287 | 0.401 |
| f0a9641f175a327b | gates | n=128, workload=h, count=1, prep=5120, seed=731, locality=random | 0.000233 | 0.000387 | 0.602 |
| 3f6f1cdb627fd325 | gates | n=128, workload=h, count=1000, prep=0, seed=731, locality=random | 0.002869 | 0.004994 | 0.574 |
| b69f4e536d952cda | gates | n=128, workload=h, count=1000, prep=5120, seed=731, locality=random | 0.002970 | 0.005055 | 0.588 |
| 23a842029d8b23d1 | gates | n=128, workload=h, count=50000, prep=0, seed=731, locality=random | 0.153443 | 0.237180 | 0.647 |
| 927d66101a370e9a | gates | n=128, workload=h, count=50000, prep=5120, seed=731, locality=random | 0.167645 | 0.245312 | 0.683 |
| 5a4d0854ed19608f | gates | n=128, workload=s, count=1, prep=0, seed=731, locality=random | 0.000121 | 0.000265 | 0.457 |
| 9c033faebbefc73d | gates | n=128, workload=s, count=1, prep=5120, seed=731, locality=random | 0.000176 | 0.000394 | 0.447 |
| 9e7ce2b02a65a58e | gates | n=128, workload=s, count=1000, prep=0, seed=731, locality=random | 0.002627 | 0.008475 | 0.310 |
| 100ebf3eb9501684 | gates | n=128, workload=s, count=1000, prep=5120, seed=731, locality=random | 0.003249 | 0.009498 | 0.342 |
| d3ae4884d6351a7c | gates | n=128, workload=s, count=50000, prep=0, seed=731, locality=random | 0.135826 | 0.427756 | 0.318 |
| c2744403db0894a3 | gates | n=128, workload=s, count=50000, prep=5120, seed=731, locality=random | 0.159187 | 0.439867 | 0.362 |
| 152a244daa942733 | gates | n=128, workload=cx, count=1, prep=0, seed=731, locality=random | 0.000120 | 0.000283 | 0.424 |
| 18c065683ba6e6ce | gates | n=128, workload=cx, count=1, prep=5120, seed=731, locality=random | 0.000482 | 0.000442 | 1.090 |
| 06fb9c867905425f | gates | n=128, workload=cx, count=1000, prep=0, seed=731, locality=random | 0.007160 | 0.019827 | 0.361 |
| 6740b5edfa7713b4 | gates | n=128, workload=cx, count=1000, prep=5120, seed=731, locality=random | 0.007521 | 0.019397 | 0.388 |
| 678c97022f6e0b16 | gates | n=128, workload=cx, count=50000, prep=0, seed=731, locality=random | 0.418489 | 0.938854 | 0.446 |
| 9532b7e151d4dd6d | gates | n=128, workload=cx, count=50000, prep=5120, seed=731, locality=random | 0.408294 | 0.943330 | 0.433 |
| b0344e31ba7f0aef | gates | n=128, workload=mixed, count=1, prep=0, seed=731, locality=random | 0.000121 | 0.000295 | 0.410 |
| 63817a176a0c251d | gates | n=128, workload=mixed, count=1, prep=5120, seed=731, locality=random | 0.000200 | 0.000497 | 0.402 |
| eaf2b9e5e338a57f | gates | n=128, workload=mixed, count=1000, prep=0, seed=731, locality=random | 0.003826 | 0.023141 | 0.165 |
| a329c1da2bcfc591 | gates | n=128, workload=mixed, count=1000, prep=5120, seed=731, locality=random | 0.007329 | 0.024419 | 0.300 |
| 17f0be4ed4f576db | gates | n=128, workload=mixed, count=50000, prep=0, seed=731, locality=random | 0.474201 | 1.342635 | 0.353 |
| 405c5c92911301c5 | gates | n=128, workload=mixed, count=50000, prep=5120, seed=731, locality=random | 0.447459 | 1.402068 | 0.319 |
| e1fe69f5e383f793 | gates | n=129, workload=h, count=1, prep=0, seed=731, locality=random | 0.000130 | 0.000279 | 0.466 |
| e24b6499a1436199 | gates | n=129, workload=h, count=1, prep=5160, seed=731, locality=random | 0.000611 | 0.000994 | 0.615 |
| 8b0736632bacc708 | gates | n=129, workload=h, count=1000, prep=0, seed=731, locality=random | 0.003170 | 0.005998 | 0.529 |
| eb88541ce5e05277 | gates | n=129, workload=h, count=1000, prep=5160, seed=731, locality=random | 0.003576 | 0.006579 | 0.544 |
| be56b86ea1fac24b | gates | n=129, workload=h, count=50000, prep=0, seed=731, locality=random | 0.146183 | 0.282623 | 0.517 |
| 522fb3d354351b11 | gates | n=129, workload=h, count=50000, prep=5160, seed=731, locality=random | 0.189852 | 0.292942 | 0.648 |
| 56d560b30fc68f2c | gates | n=129, workload=s, count=1, prep=0, seed=731, locality=random | 0.000140 | 0.000297 | 0.471 |
| 0436758204f39a5b | gates | n=129, workload=s, count=1, prep=5160, seed=731, locality=random | 0.000164 | 0.000357 | 0.459 |
| b604538a0846f1a0 | gates | n=129, workload=s, count=1000, prep=0, seed=731, locality=random | 0.003290 | 0.010197 | 0.323 |
| 5b53ca190a6b4d58 | gates | n=129, workload=s, count=1000, prep=5160, seed=731, locality=random | 0.002902 | 0.009821 | 0.295 |
| 31fc824fde82a2b8 | gates | n=129, workload=s, count=50000, prep=0, seed=731, locality=random | 0.150390 | 0.524352 | 0.287 |
| 22281d73a1311149 | gates | n=129, workload=s, count=50000, prep=5160, seed=731, locality=random | 0.157809 | 0.474304 | 0.333 |
| c5cc232fcfb794f3 | gates | n=129, workload=cx, count=1, prep=0, seed=731, locality=random | 0.000120 | 0.000319 | 0.376 |
| 5412178210031a9e | gates | n=129, workload=cx, count=1, prep=5160, seed=731, locality=random | 0.000225 | 0.000620 | 0.363 |
| 544762e32f67ee67 | gates | n=129, workload=cx, count=1000, prep=0, seed=731, locality=random | 0.006954 | 0.019369 | 0.359 |
| 74068d3fd0ca20c0 | gates | n=129, workload=cx, count=1000, prep=5160, seed=731, locality=random | 0.007871 | 0.020458 | 0.385 |
| 5d9c77e6777d2a8b | gates | n=129, workload=cx, count=50000, prep=0, seed=731, locality=random | 0.397589 | 0.911070 | 0.436 |
| 6dd1ea417f151f4b | gates | n=129, workload=cx, count=50000, prep=5160, seed=731, locality=random | 0.450404 | 1.005789 | 0.448 |
| d4109d4ca95d6f60 | gates | n=129, workload=mixed, count=1, prep=0, seed=731, locality=random | 0.000111 | 0.000306 | 0.363 |
| 255db416fc3b7313 | gates | n=129, workload=mixed, count=1, prep=5160, seed=731, locality=random | 0.000503 | 0.000610 | 0.825 |
| f19ece668dddd762 | gates | n=129, workload=mixed, count=1000, prep=0, seed=731, locality=random | 0.003830 | 0.024259 | 0.158 |
| 879a2d8ea1cedb87 | gates | n=129, workload=mixed, count=1000, prep=5160, seed=731, locality=random | 0.008225 | 0.026718 | 0.308 |
| 8d11d32e2c2b3026 | gates | n=129, workload=mixed, count=50000, prep=0, seed=731, locality=random | 0.442659 | 1.400850 | 0.316 |
| 02c53516402e1c52 | gates | n=129, workload=mixed, count=50000, prep=5160, seed=731, locality=random | 0.411093 | 1.272681 | 0.323 |
| e49c4d38723ec664 | gates | n=256, workload=h, count=1, prep=0, seed=731, locality=random | 0.000109 | 0.000272 | 0.401 |
| fc56cac6328a9b41 | gates | n=256, workload=h, count=1, prep=10240, seed=731, locality=random | 0.000206 | 0.000422 | 0.488 |
| dfacba6f66515d26 | gates | n=256, workload=h, count=1000, prep=0, seed=731, locality=random | 0.004226 | 0.005957 | 0.709 |
| 4e649e8989c6ce8f | gates | n=256, workload=h, count=1000, prep=10240, seed=731, locality=random | 0.003971 | 0.006500 | 0.611 |
| 8cfa57c6beb0055d | gates | n=256, workload=h, count=50000, prep=0, seed=731, locality=random | 0.214746 | 0.272085 | 0.789 |
| 4bfac41fe3e16972 | gates | n=256, workload=h, count=50000, prep=10240, seed=731, locality=random | 0.202592 | 0.292634 | 0.692 |
| 95a1f43d862ecc20 | gates | n=256, workload=s, count=1, prep=0, seed=731, locality=random | 0.000118 | 0.000309 | 0.382 |
| 42231cad790a8402 | gates | n=256, workload=s, count=1, prep=10240, seed=731, locality=random | 0.000169 | 0.000409 | 0.413 |
| 7cbd0afaa3d48fb1 | gates | n=256, workload=s, count=1000, prep=0, seed=731, locality=random | 0.004296 | 0.010211 | 0.421 |
| 6452a6630b2dab56 | gates | n=256, workload=s, count=1000, prep=10240, seed=731, locality=random | 0.004552 | 0.010102 | 0.451 |
| 201c63cc912f74e2 | gates | n=256, workload=s, count=50000, prep=0, seed=731, locality=random | 0.174062 | 0.465173 | 0.374 |
| b2863361b0aa4724 | gates | n=256, workload=s, count=50000, prep=10240, seed=731, locality=random | 0.188932 | 0.511662 | 0.369 |
| d735081c599b64c1 | gates | n=256, workload=cx, count=1, prep=0, seed=731, locality=random | 0.000141 | 0.000357 | 0.395 |
| 75637d0a48e15878 | gates | n=256, workload=cx, count=1, prep=10240, seed=731, locality=random | 0.000887 | 0.001713 | 0.518 |
| a06637ad1a408fc3 | gates | n=256, workload=cx, count=1000, prep=0, seed=731, locality=random | 0.009622 | 0.021110 | 0.456 |
| 110895abecfced88 | gates | n=256, workload=cx, count=1000, prep=10240, seed=731, locality=random | 0.009214 | 0.019703 | 0.468 |
| 4e9462ec70d09aef | gates | n=256, workload=cx, count=50000, prep=0, seed=731, locality=random | 0.444935 | 1.004246 | 0.443 |
| 21679c70a64e4cc2 | gates | n=256, workload=cx, count=50000, prep=10240, seed=731, locality=random | 0.449756 | 1.000962 | 0.449 |
| e0c45a0e742b3759 | gates | n=256, workload=mixed, count=1, prep=0, seed=731, locality=random | 0.000142 | 0.000268 | 0.530 |
| 4377550058e652b7 | gates | n=256, workload=mixed, count=1, prep=10240, seed=731, locality=random | 0.000349 | 0.000554 | 0.630 |
| 925c9d2668dd37d0 | gates | n=256, workload=mixed, count=1000, prep=0, seed=731, locality=random | 0.005375 | 0.023666 | 0.227 |
| f12eca2e9588f35e | gates | n=256, workload=mixed, count=1000, prep=10240, seed=731, locality=random | 0.009552 | 0.027279 | 0.350 |
| 405b9604c0d25f46 | gates | n=256, workload=mixed, count=50000, prep=0, seed=731, locality=random | 0.436494 | 1.364562 | 0.320 |
| ed1caac68799525f | gates | n=256, workload=mixed, count=50000, prep=10240, seed=731, locality=random | 0.443196 | 1.370940 | 0.323 |
| 265feedb006da160 | gates | n=1024, workload=h, count=1, prep=0, seed=731, locality=random | 0.000135 | 0.000312 | 0.433 |
| 4b99170dbf8b5898 | gates | n=1024, workload=h, count=1, prep=40960, seed=731, locality=random | 0.001171 | 0.002153 | 0.544 |
| ef1e4e826602bb7e | gates | n=1024, workload=h, count=1000, prep=0, seed=731, locality=random | 0.011990 | 0.013869 | 0.865 |
| e163696f52fb2200 | gates | n=1024, workload=h, count=1000, prep=40960, seed=731, locality=random | 0.016363 | 0.015555 | 1.052 |
| 10675bfeca104a73 | gates | n=1024, workload=h, count=50000, prep=0, seed=731, locality=random | 0.655775 | 0.716715 | 0.915 |
| b473ed89e07f21e8 | gates | n=1024, workload=h, count=50000, prep=40960, seed=731, locality=random | 0.654973 | 0.723430 | 0.905 |
| d03c62d3e080dcbf | gates | n=1024, workload=s, count=1, prep=0, seed=731, locality=random | 0.000130 | 0.000323 | 0.402 |
| d9a4fb3c16cd9e83 | gates | n=1024, workload=s, count=1, prep=40960, seed=731, locality=random | 0.001369 | 0.002152 | 0.636 |
| 384cd48f2da53401 | gates | n=1024, workload=s, count=1000, prep=0, seed=731, locality=random | 0.010740 | 0.017253 | 0.623 |
| 8c6b760867804300 | gates | n=1024, workload=s, count=1000, prep=40960, seed=731, locality=random | 0.014797 | 0.018453 | 0.802 |
| a9885587cdf4b9ab | gates | n=1024, workload=s, count=50000, prep=0, seed=731, locality=random | 0.575764 | 0.889059 | 0.648 |
| a13d498d542e905a | gates | n=1024, workload=s, count=50000, prep=40960, seed=731, locality=random | 0.557033 | 0.879303 | 0.633 |
| 5903aed2bda4342a | gates | n=1024, workload=cx, count=1, prep=0, seed=731, locality=random | 0.000140 | 0.000340 | 0.412 |
| 4bf5cb6b8a780930 | gates | n=1024, workload=cx, count=1, prep=40960, seed=731, locality=random | 0.001250 | 0.001883 | 0.664 |
| 1d1f8e9fc7c9e6c7 | gates | n=1024, workload=cx, count=1000, prep=0, seed=731, locality=random | 0.018536 | 0.033636 | 0.551 |
| ce2f71b8efc3a766 | gates | n=1024, workload=cx, count=1000, prep=40960, seed=731, locality=random | 0.041081 | 0.041730 | 0.984 |
| 040dbb9da86ea9dc | gates | n=1024, workload=cx, count=50000, prep=0, seed=731, locality=random | 0.896165 | 1.777398 | 0.504 |
| 6eb04186a4e5a25f | gates | n=1024, workload=cx, count=50000, prep=40960, seed=731, locality=random | 1.153888 | 1.784228 | 0.647 |
| 03464981d8cf5776 | gates | n=1024, workload=mixed, count=1, prep=0, seed=731, locality=random | 0.000141 | 0.000360 | 0.392 |
| b0fb880ccf25e914 | gates | n=1024, workload=mixed, count=1, prep=40960, seed=731, locality=random | 0.001510 | 0.002158 | 0.700 |
| d8618ce042109cae | gates | n=1024, workload=mixed, count=1000, prep=0, seed=731, locality=random | 0.014088 | 0.035209 | 0.400 |
| 42fbf47386e27b26 | gates | n=1024, workload=mixed, count=1000, prep=40960, seed=731, locality=random | 0.021403 | 0.041264 | 0.519 |
| ee73d5685735718a | gates | n=1024, workload=mixed, count=50000, prep=0, seed=731, locality=random | 0.776158 | 1.897978 | 0.409 |
| 856715bf0d49831b | gates | n=1024, workload=mixed, count=50000, prep=40960, seed=731, locality=random | 0.833220 | 1.838037 | 0.453 |
| 825ef4314dbc99a5 | gates | n=4096, workload=h, count=1, prep=0, seed=731, locality=random | 0.001985 | 0.001736 | 1.143 |
| 59606dfbed5ecb46 | gates | n=4096, workload=h, count=1, prep=163840, seed=731, locality=random | 0.010425 | 0.012623 | 0.826 |
| dfd28a0cc47c4a0c | gates | n=4096, workload=h, count=1000, prep=0, seed=731, locality=random | 0.219529 | 0.202882 | 1.082 |
| e7e6b1a8b41b5067 | gates | n=4096, workload=h, count=1000, prep=163840, seed=731, locality=random | 0.209459 | 0.095125 | 2.202 |
| d8ce7cedc4354789 | gates | n=4096, workload=h, count=50000, prep=0, seed=731, locality=random | 3.662095 | 4.376546 | 0.837 |
| 10142998f3328eb8 | gates | n=4096, workload=h, count=50000, prep=163840, seed=731, locality=random | 3.451208 | 4.056007 | 0.851 |
| d02ccfba71891af5 | gates | n=4096, workload=s, count=1, prep=0, seed=731, locality=random | 0.001599 | 0.002514 | 0.636 |
| 99dd230f920884f7 | gates | n=4096, workload=s, count=1, prep=163840, seed=731, locality=random | 0.009962 | 0.012470 | 0.799 |
| 06e7d3d8ebf50a3e | gates | n=4096, workload=s, count=1000, prep=0, seed=731, locality=random | 0.144335 | 0.212055 | 0.681 |
| 26a86d429278eec8 | gates | n=4096, workload=s, count=1000, prep=163840, seed=731, locality=random | 0.215599 | 0.101546 | 2.123 |
| c736037694ae40a8 | gates | n=4096, workload=s, count=50000, prep=0, seed=731, locality=random | 3.271307 | 4.693803 | 0.697 |
| 5474b38635408b81 | gates | n=4096, workload=s, count=50000, prep=163840, seed=731, locality=random | 3.030467 | 4.329879 | 0.700 |
| ca20e9d03e5a2cb8 | gates | n=4096, workload=cx, count=1, prep=0, seed=731, locality=random | 0.002727 | 0.004929 | 0.553 |
| 9f15d05720ee3e66 | gates | n=4096, workload=cx, count=1, prep=163840, seed=731, locality=random | 0.010477 | 0.013769 | 0.761 |
| ffab6620f23c50a1 | gates | n=4096, workload=cx, count=1000, prep=0, seed=731, locality=random | 0.235300 | 0.441944 | 0.532 |
| 09d31a7b82b0a4e6 | gates | n=4096, workload=cx, count=1000, prep=163840, seed=731, locality=random | 0.280024 | 0.188001 | 1.489 |
| f16f0a3963cf9d26 | gates | n=4096, workload=cx, count=50000, prep=0, seed=731, locality=random | 5.412234 | 8.939866 | 0.605 |
| 8aac102da245bc34 | gates | n=4096, workload=cx, count=50000, prep=163840, seed=731, locality=random | 4.929993 | 8.263538 | 0.597 |
| f38fe67bc981b135 | gates | n=4096, workload=mixed, count=1, prep=0, seed=731, locality=random | 0.002098 | 0.004156 | 0.505 |
| 9039c7e013bc0d63 | gates | n=4096, workload=mixed, count=1, prep=163840, seed=731, locality=random | 0.009209 | 0.013859 | 0.664 |
| b633733485c6a472 | gates | n=4096, workload=mixed, count=1000, prep=0, seed=731, locality=random | 0.149108 | 0.274366 | 0.543 |
| 6d5a10939c6162d3 | gates | n=4096, workload=mixed, count=1000, prep=163840, seed=731, locality=random | 0.248176 | 0.149719 | 1.658 |
| bc8279f3552b019c | gates | n=4096, workload=mixed, count=50000, prep=0, seed=731, locality=random | 4.021039 | 7.262908 | 0.554 |
| 635c34c366ad2899 | gates | n=4096, workload=mixed, count=50000, prep=163840, seed=731, locality=random | 4.110101 | 6.567322 | 0.626 |
| d2f25dc6fcfd4695 | gates | n=65, workload=cx, count=50000, prep=2600, seed=731, locality=local | 0.227186 | 0.922664 | 0.246 |
| 19b1341e7bd0c109 | gates | n=65, workload=cx, count=50000, prep=2600, seed=731, locality=hotspot | 0.402143 | 0.943134 | 0.426 |
| 9c280b67aa513167 | gates | n=65, workload=mixed, count=50000, prep=2600, seed=731, locality=local | 0.353590 | 1.348940 | 0.262 |
| a272a76279a59f44 | gates | n=65, workload=mixed, count=50000, prep=2600, seed=731, locality=hotspot | 0.406139 | 1.351666 | 0.300 |
| e7c120a7130cabc0 | gates | n=1024, workload=cx, count=50000, prep=40960, seed=731, locality=local | 1.032235 | 1.683760 | 0.613 |
| 1d6e570dd7e19e52 | gates | n=1024, workload=cx, count=50000, prep=40960, seed=731, locality=hotspot | 0.710576 | 1.250083 | 0.568 |
| be7e917e160de39b | gates | n=1024, workload=mixed, count=50000, prep=40960, seed=731, locality=local | 0.737049 | 1.849803 | 0.398 |
| eefc988a8d568e31 | gates | n=1024, workload=mixed, count=50000, prep=40960, seed=731, locality=hotspot | 0.642307 | 1.585378 | 0.405 |
| 4ae2506e2a04906b | gates | n=4096, workload=cx, count=50000, prep=163840, seed=731, locality=local | 4.938307 | 8.187414 | 0.603 |
| e1056a31da4f61c2 | gates | n=4096, workload=cx, count=50000, prep=163840, seed=731, locality=hotspot | 1.875997 | 3.123772 | 0.601 |
| 5c5d27710181b939 | gates | n=4096, workload=mixed, count=50000, prep=163840, seed=731, locality=local | 4.187174 | 6.811641 | 0.615 |
| 54f15782a75ecd3a | gates | n=4096, workload=mixed, count=50000, prep=163840, seed=731, locality=hotspot | 1.721795 | 2.682023 | 0.642 |
| 3bc8c34e7c0661c1 | measurement | n=2, state=zero, count=1, mode=native, seed=731 | 0.000264 | 0.000411 | 0.642 |
| d14a959b196f33a3 | measurement | n=2, state=zero, count=1, mode=sequential, seed=731 | 0.000269 | 0.000275 | 0.978 |
| 6b7bd26c4c820875 | measurement | n=2, state=zero, count=2, mode=native, seed=731 | 0.000297 | 0.000562 | 0.528 |
| 516b9f89107cf39f | measurement | n=2, state=zero, count=2, mode=sequential, seed=731 | 0.000254 | 0.000391 | 0.650 |
| 11188efc9c526b3b | measurement | n=2, state=plus, count=1, mode=native, seed=731 | 0.000205 | 0.003373 | 0.061 |
| 4b41aaca67fd1106 | measurement | n=2, state=plus, count=1, mode=sequential, seed=731 | 0.000167 | 0.003325 | 0.050 |
| 42dd3ba6e7b8ec05 | measurement | n=2, state=plus, count=2, mode=native, seed=731 | 0.000215 | 0.003447 | 0.062 |
| 731999fc9940099a | measurement | n=2, state=plus, count=2, mode=sequential, seed=731 | 0.000210 | 0.006035 | 0.035 |
| ee19e3e98f87f9d8 | measurement | n=2, state=bell, count=1, mode=native, seed=731 | 0.000205 | 0.003721 | 0.055 |
| 523ca22ae4efb777 | measurement | n=2, state=bell, count=1, mode=sequential, seed=731 | 0.000224 | 0.003620 | 0.062 |
| a64419e9464868de | measurement | n=2, state=bell, count=2, mode=native, seed=731 | 0.000279 | 0.003262 | 0.086 |
| 76f99f1833d87a5a | measurement | n=2, state=bell, count=2, mode=sequential, seed=731 | 0.000272 | 0.003473 | 0.078 |
| 98a4a2ffa4883b1a | measurement | n=2, state=ghz, count=1, mode=native, seed=731 | 0.000199 | 0.003611 | 0.055 |
| 1a4d969eba2ebb58 | measurement | n=2, state=ghz, count=1, mode=sequential, seed=731 | 0.000248 | 0.003161 | 0.078 |
| fdc00e6a6c8bb887 | measurement | n=2, state=ghz, count=2, mode=native, seed=731 | 0.000356 | 0.003447 | 0.103 |
| 493fc3db2f7b7d13 | measurement | n=2, state=ghz, count=2, mode=sequential, seed=731 | 0.000311 | 0.003357 | 0.093 |
| a42c2c28d83e6c98 | measurement | n=2, state=cluster, count=1, mode=native, seed=731 | 0.000214 | 0.003256 | 0.066 |
| 48ee31d31249dada | measurement | n=2, state=cluster, count=1, mode=sequential, seed=731 | 0.000129 | 0.003023 | 0.043 |
| 2329a60a06297a25 | measurement | n=2, state=cluster, count=2, mode=native, seed=731 | 0.000208 | 0.003317 | 0.063 |
| 186d3fddf07d4a58 | measurement | n=2, state=cluster, count=2, mode=sequential, seed=731 | 0.000216 | 0.005624 | 0.038 |
| 837bd1fa4951927a | measurement | n=2, state=scrambled, count=1, mode=native, seed=731 | 0.000214 | 0.000409 | 0.523 |
| 054ec34791531b13 | measurement | n=2, state=scrambled, count=1, mode=sequential, seed=731 | 0.000180 | 0.000232 | 0.776 |
| 1e547fedfdee70cd | measurement | n=2, state=scrambled, count=2, mode=native, seed=731 | 0.000238 | 0.003454 | 0.069 |
| 6a3c80d896629a3a | measurement | n=2, state=scrambled, count=2, mode=sequential, seed=731 | 0.000606 | 0.007517 | 0.081 |
| 940817297105349f | measurement | n=2, state=deterministic, count=1, mode=native, seed=731 | 0.000273 | 0.000428 | 0.638 |
| a1e75c366ee69e89 | measurement | n=2, state=deterministic, count=1, mode=sequential, seed=731 | 0.000229 | 0.000282 | 0.812 |
| e6d0afbd4eacc88a | measurement | n=2, state=deterministic, count=2, mode=native, seed=731 | 0.000251 | 0.000447 | 0.562 |
| 0bd6eda281d8b937 | measurement | n=2, state=deterministic, count=2, mode=sequential, seed=731 | 0.000235 | 0.000388 | 0.606 |
| 69af91418ab17727 | measurement | n=63, state=zero, count=1, mode=native, seed=731 | 0.000205 | 0.000342 | 0.599 |
| 0eb4a69d5f204ba3 | measurement | n=63, state=zero, count=1, mode=sequential, seed=731 | 0.000184 | 0.000279 | 0.659 |
| 2d0f81c44417d15c | measurement | n=63, state=zero, count=8, mode=native, seed=731 | 0.000542 | 0.000945 | 0.574 |
| 23fd124afac976d5 | measurement | n=63, state=zero, count=8, mode=sequential, seed=731 | 0.000628 | 0.001208 | 0.520 |
| 6a462301fecb49c2 | measurement | n=63, state=zero, count=63, mode=native, seed=731 | 0.002737 | 0.003759 | 0.728 |
| c73c33d66935543e | measurement | n=63, state=zero, count=63, mode=sequential, seed=731 | 0.003955 | 0.006853 | 0.577 |
| 2f8c0bd73df45eb2 | measurement | n=63, state=plus, count=1, mode=native, seed=731 | 0.000335 | 0.003202 | 0.105 |
| 97003960debcf944 | measurement | n=63, state=plus, count=1, mode=sequential, seed=731 | 0.000285 | 0.003148 | 0.091 |
| 080fb9d2710634dc | measurement | n=63, state=plus, count=8, mode=native, seed=731 | 0.001389 | 0.004292 | 0.324 |
| c71a2148d9052e85 | measurement | n=63, state=plus, count=8, mode=sequential, seed=731 | 0.001504 | 0.022518 | 0.067 |
| 6a9919773549fb88 | measurement | n=63, state=plus, count=63, mode=native, seed=731 | 0.009708 | 0.012115 | 0.801 |
| 833371668e9b104b | measurement | n=63, state=plus, count=63, mode=sequential, seed=731 | 0.010029 | 0.155551 | 0.064 |
| cb1a9ed89f6c73a8 | measurement | n=63, state=bell, count=1, mode=native, seed=731 | 0.000332 | 0.003479 | 0.095 |
| 34e28aa0233044a6 | measurement | n=63, state=bell, count=1, mode=sequential, seed=731 | 0.000287 | 0.003046 | 0.094 |
| c5a1624b76e6ac05 | measurement | n=63, state=bell, count=8, mode=native, seed=731 | 0.001500 | 0.004665 | 0.322 |
| 42667c996a88a3dd | measurement | n=63, state=bell, count=8, mode=sequential, seed=731 | 0.001579 | 0.020526 | 0.077 |
| ea7a34036d2565f0 | measurement | n=63, state=bell, count=63, mode=native, seed=731 | 0.014545 | 0.010810 | 1.346 |
| c8ac4c5296488953 | measurement | n=63, state=bell, count=63, mode=sequential, seed=731 | 0.015488 | 0.081579 | 0.190 |
| 369cc2b9918b65ff | measurement | n=63, state=ghz, count=1, mode=native, seed=731 | 0.000430 | 0.003284 | 0.131 |
| 89da4855d8af0e70 | measurement | n=63, state=ghz, count=1, mode=sequential, seed=731 | 0.000392 | 0.002993 | 0.131 |
| 7dca472ca9fbb37a | measurement | n=63, state=ghz, count=8, mode=native, seed=731 | 0.004007 | 0.004319 | 0.928 |
| 448425c032f8207c | measurement | n=63, state=ghz, count=8, mode=sequential, seed=731 | 0.004174 | 0.003944 | 1.058 |
| 0f0042af3782332c | measurement | n=63, state=ghz, count=63, mode=native, seed=731 | 0.031538 | 0.009873 | 3.194 |
| 41d90a631e2645a2 | measurement | n=63, state=ghz, count=63, mode=sequential, seed=731 | 0.032598 | 0.009542 | 3.416 |
| e643c06558fbf885 | measurement | n=63, state=cluster, count=1, mode=native, seed=731 | 0.000340 | 0.003383 | 0.101 |
| ced5667a15b45268 | measurement | n=63, state=cluster, count=1, mode=sequential, seed=731 | 0.000296 | 0.003126 | 0.095 |
| 3e915412170a53be | measurement | n=63, state=cluster, count=8, mode=native, seed=731 | 0.001530 | 0.004476 | 0.342 |
| 65971ce50b9b5759 | measurement | n=63, state=cluster, count=8, mode=sequential, seed=731 | 0.001506 | 0.020869 | 0.072 |
| 994ea10527a36ff1 | measurement | n=63, state=cluster, count=63, mode=native, seed=731 | 0.009456 | 0.012165 | 0.777 |
| 8128b15e257933e2 | measurement | n=63, state=cluster, count=63, mode=sequential, seed=731 | 0.010673 | 0.158739 | 0.067 |
| b18ae739dbf359c8 | measurement | n=63, state=scrambled, count=1, mode=native, seed=731 | 0.000750 | 0.004086 | 0.184 |
| d48469bb98c7432c | measurement | n=63, state=scrambled, count=1, mode=sequential, seed=731 | 0.000624 | 0.004231 | 0.147 |
| 2afa323c1bcf324a | measurement | n=63, state=scrambled, count=8, mode=native, seed=731 | 0.003967 | 0.008803 | 0.451 |
| 61f87f10f459aa5c | measurement | n=63, state=scrambled, count=8, mode=sequential, seed=731 | 0.004063 | 0.024841 | 0.164 |
| 6555130a5a063f42 | measurement | n=63, state=scrambled, count=63, mode=native, seed=731 | 0.033821 | 0.044155 | 0.766 |
| 56b08abf1eb1fc73 | measurement | n=63, state=scrambled, count=63, mode=sequential, seed=731 | 0.035627 | 0.185960 | 0.192 |
| 36ae04df92800db4 | measurement | n=63, state=deterministic, count=1, mode=native, seed=731 | 0.000550 | 0.000453 | 1.214 |
| 18a0ba813568cc7b | measurement | n=63, state=deterministic, count=1, mode=sequential, seed=731 | 0.000554 | 0.000344 | 1.610 |
| a2862f452e431b73 | measurement | n=63, state=deterministic, count=8, mode=native, seed=731 | 0.004318 | 0.000984 | 4.388 |
| d6202ca7af07d59a | measurement | n=63, state=deterministic, count=8, mode=sequential, seed=731 | 0.004309 | 0.001254 | 3.436 |
| 89e4a017ff0ceb8d | measurement | n=63, state=deterministic, count=63, mode=native, seed=731 | 0.027991 | 0.003779 | 7.407 |
| eb219511d2b9cfa9 | measurement | n=63, state=deterministic, count=63, mode=sequential, seed=731 | 0.029165 | 0.007173 | 4.066 |
| 97daa6d83468613e | measurement | n=64, state=zero, count=1, mode=native, seed=731 | 0.000252 | 0.000347 | 0.726 |
| 13f2944bad36ac66 | measurement | n=64, state=zero, count=1, mode=sequential, seed=731 | 0.000305 | 0.000472 | 0.646 |
| 0212f894d9c9db30 | measurement | n=64, state=zero, count=8, mode=native, seed=731 | 0.000662 | 0.001111 | 0.596 |
| a0697705ba0e0c1a | measurement | n=64, state=zero, count=8, mode=sequential, seed=731 | 0.000651 | 0.001179 | 0.552 |
| d6bf45bee8d8ba6c | measurement | n=64, state=zero, count=64, mode=native, seed=731 | 0.002653 | 0.003734 | 0.710 |
| 232ebcd6e05bde25 | measurement | n=64, state=zero, count=64, mode=sequential, seed=731 | 0.004071 | 0.006963 | 0.585 |
| 681ea357282f51ab | measurement | n=64, state=plus, count=1, mode=native, seed=731 | 0.000354 | 0.003274 | 0.108 |
| c9a016c5541a0844 | measurement | n=64, state=plus, count=1, mode=sequential, seed=731 | 0.000296 | 0.003098 | 0.096 |
| 5b115246a0bffd15 | measurement | n=64, state=plus, count=8, mode=native, seed=731 | 0.001406 | 0.004399 | 0.320 |
| 3c0d2f4e6f29f226 | measurement | n=64, state=plus, count=8, mode=sequential, seed=731 | 0.001508 | 0.020588 | 0.073 |
| 04bf9c56a1a905cb | measurement | n=64, state=plus, count=64, mode=native, seed=731 | 0.009367 | 0.011641 | 0.805 |
| 000570d7c8465705 | measurement | n=64, state=plus, count=64, mode=sequential, seed=731 | 0.010640 | 0.157934 | 0.067 |
| 1eb0070dcf3a9b93 | measurement | n=64, state=bell, count=1, mode=native, seed=731 | 0.000356 | 0.003381 | 0.105 |
| 2bd7c5cfa730ee70 | measurement | n=64, state=bell, count=1, mode=sequential, seed=731 | 0.000310 | 0.003140 | 0.099 |
| 53169891b0d7f317 | measurement | n=64, state=bell, count=8, mode=native, seed=731 | 0.001471 | 0.004260 | 0.345 |
| 610ca827b5a30118 | measurement | n=64, state=bell, count=8, mode=sequential, seed=731 | 0.001664 | 0.020635 | 0.081 |
| f78dac05e57b7ec2 | measurement | n=64, state=bell, count=64, mode=native, seed=731 | 0.015975 | 0.011636 | 1.373 |
| d9785ec38506f30b | measurement | n=64, state=bell, count=64, mode=sequential, seed=731 | 0.017672 | 0.089889 | 0.197 |
| db56d90c9723356c | measurement | n=64, state=ghz, count=1, mode=native, seed=731 | 0.000463 | 0.003320 | 0.139 |
| 96c4acdd80470148 | measurement | n=64, state=ghz, count=1, mode=sequential, seed=731 | 0.000373 | 0.003042 | 0.123 |
| fd0accfb41506d08 | measurement | n=64, state=ghz, count=8, mode=native, seed=731 | 0.002586 | 0.004278 | 0.604 |
| 4bb78ff1f3dcbb40 | measurement | n=64, state=ghz, count=8, mode=sequential, seed=731 | 0.002732 | 0.004094 | 0.667 |
| 84328cec00c799bc | measurement | n=64, state=ghz, count=64, mode=native, seed=731 | 0.018997 | 0.009961 | 1.907 |
| b651b8408276f214 | measurement | n=64, state=ghz, count=64, mode=sequential, seed=731 | 0.020185 | 0.009702 | 2.080 |
| c559d960f2e9a2ff | measurement | n=64, state=cluster, count=1, mode=native, seed=731 | 0.000343 | 0.003253 | 0.105 |
| d6c9c4425f6b0ae7 | measurement | n=64, state=cluster, count=1, mode=sequential, seed=731 | 0.000331 | 0.003176 | 0.104 |
| fd86d7a34fb4ab72 | measurement | n=64, state=cluster, count=8, mode=native, seed=731 | 0.005287 | 0.018428 | 0.287 |
| e1e37934ae434723 | measurement | n=64, state=cluster, count=8, mode=sequential, seed=731 | 0.001589 | 0.022591 | 0.070 |
| 0c408a59459ea8ac | measurement | n=64, state=cluster, count=64, mode=native, seed=731 | 0.009422 | 0.011654 | 0.808 |
| f71f381ecb9cbc5f | measurement | n=64, state=cluster, count=64, mode=sequential, seed=731 | 0.010730 | 0.158298 | 0.068 |
| 262cd52424b0cf7a | measurement | n=64, state=scrambled, count=1, mode=native, seed=731 | 0.000822 | 0.004392 | 0.187 |
| 00954c2a51e9bf50 | measurement | n=64, state=scrambled, count=1, mode=sequential, seed=731 | 0.000621 | 0.003942 | 0.158 |
| a31e5754b4936890 | measurement | n=64, state=scrambled, count=8, mode=native, seed=731 | 0.004356 | 0.009131 | 0.477 |
| 424cf89ecfd5e59a | measurement | n=64, state=scrambled, count=8, mode=sequential, seed=731 | 0.003658 | 0.026177 | 0.140 |
| 2f20364ac66c90cd | measurement | n=64, state=scrambled, count=64, mode=native, seed=731 | 0.037709 | 0.046857 | 0.805 |
| ad5d757122dd88dd | measurement | n=64, state=scrambled, count=64, mode=sequential, seed=731 | 0.036282 | 0.198803 | 0.183 |
| 48ccae7ff82e1fe8 | measurement | n=64, state=deterministic, count=1, mode=native, seed=731 | 0.000650 | 0.000888 | 0.732 |
| 13f3966bf34696da | measurement | n=64, state=deterministic, count=1, mode=sequential, seed=731 | 0.000498 | 0.000281 | 1.772 |
| 2f1b09a86730bcf7 | measurement | n=64, state=deterministic, count=8, mode=native, seed=731 | 0.003314 | 0.001451 | 2.284 |
| 9a55233010f156d1 | measurement | n=64, state=deterministic, count=8, mode=sequential, seed=731 | 0.003475 | 0.001283 | 2.708 |
| 9d3c4881425f21f6 | measurement | n=64, state=deterministic, count=64, mode=native, seed=731 | 0.021732 | 0.003817 | 5.693 |
| e4bcec0b1a294ef7 | measurement | n=64, state=deterministic, count=64, mode=sequential, seed=731 | 0.023036 | 0.007361 | 3.129 |
| 5e260f5c52620485 | measurement | n=65, state=zero, count=1, mode=native, seed=731 | 0.000252 | 0.000420 | 0.600 |
| 6079bb3b0d062d6b | measurement | n=65, state=zero, count=1, mode=sequential, seed=731 | 0.000223 | 0.000250 | 0.892 |
| 0350c4803d3f1bca | measurement | n=65, state=zero, count=8, mode=native, seed=731 | 0.000606 | 0.000897 | 0.676 |
| 7f2c5690a5777770 | measurement | n=65, state=zero, count=8, mode=sequential, seed=731 | 0.000696 | 0.001141 | 0.610 |
| 689961aa83612887 | measurement | n=65, state=zero, count=64, mode=native, seed=731 | 0.003008 | 0.003944 | 0.763 |
| fb2355a57a0312ad | measurement | n=65, state=zero, count=64, mode=sequential, seed=731 | 0.004243 | 0.007329 | 0.579 |
| 278fe70af4f8d5a3 | measurement | n=65, state=zero, count=65, mode=native, seed=731 | 0.003046 | 0.004158 | 0.733 |
| eeca8ef4cfa88f13 | measurement | n=65, state=zero, count=65, mode=sequential, seed=731 | 0.004645 | 0.007556 | 0.615 |
| 476c5d06f1a48987 | measurement | n=65, state=plus, count=1, mode=native, seed=731 | 0.000397 | 0.003500 | 0.113 |
| 46b7ee7089cde061 | measurement | n=65, state=plus, count=1, mode=sequential, seed=731 | 0.000326 | 0.003245 | 0.100 |
| 59d7cc7c2db64a15 | measurement | n=65, state=plus, count=8, mode=native, seed=731 | 0.001846 | 0.005309 | 0.348 |
| 98c5cf2ac47355d4 | measurement | n=65, state=plus, count=8, mode=sequential, seed=731 | 0.001643 | 0.022416 | 0.073 |
| 1afcc1afc90626fd | measurement | n=65, state=plus, count=64, mode=native, seed=731 | 0.010170 | 0.012603 | 0.807 |
| 11c31b3b90c804d3 | measurement | n=65, state=plus, count=64, mode=sequential, seed=731 | 0.011139 | 0.169152 | 0.066 |
| cd3aac406a0d43b5 | measurement | n=65, state=plus, count=65, mode=native, seed=731 | 0.011301 | 0.012697 | 0.890 |
| 23d8d84c1567a596 | measurement | n=65, state=plus, count=65, mode=sequential, seed=731 | 0.012221 | 0.171018 | 0.071 |
| f632afea755eb034 | measurement | n=65, state=bell, count=1, mode=native, seed=731 | 0.000312 | 0.003357 | 0.093 |
| 1c35ffdd7fc10212 | measurement | n=65, state=bell, count=1, mode=sequential, seed=731 | 0.000291 | 0.003024 | 0.096 |
| f9d36acbad33df25 | measurement | n=65, state=bell, count=8, mode=native, seed=731 | 0.001810 | 0.004740 | 0.382 |
| 353e624148360451 | measurement | n=65, state=bell, count=8, mode=sequential, seed=731 | 0.002503 | 0.024803 | 0.101 |
| 87963c36bb8cc6db | measurement | n=65, state=bell, count=64, mode=native, seed=731 | 0.016890 | 0.011806 | 1.431 |
| 635e19a934a47c3a | measurement | n=65, state=bell, count=64, mode=sequential, seed=731 | 0.016900 | 0.089218 | 0.189 |
| e45b4108239a2a6e | measurement | n=65, state=bell, count=65, mode=native, seed=731 | 0.017373 | 0.012099 | 1.436 |
| e1c7e864f0035379 | measurement | n=65, state=bell, count=65, mode=sequential, seed=731 | 0.017782 | 0.090272 | 0.197 |
| 12dd6ae14d22b760 | measurement | n=65, state=ghz, count=1, mode=native, seed=731 | 0.000396 | 0.003590 | 0.110 |
| 8629c86522e88a81 | measurement | n=65, state=ghz, count=1, mode=sequential, seed=731 | 0.000403 | 0.003461 | 0.116 |
| 76a9d2ea45f80f9d | measurement | n=65, state=ghz, count=8, mode=native, seed=731 | 0.003122 | 0.004699 | 0.664 |
| d4334df75e069305 | measurement | n=65, state=ghz, count=8, mode=sequential, seed=731 | 0.002935 | 0.004396 | 0.668 |
| 639a6dddd2b6d27a | measurement | n=65, state=ghz, count=64, mode=native, seed=731 | 0.021114 | 0.011022 | 1.916 |
| 92781fcff2ba5303 | measurement | n=65, state=ghz, count=64, mode=sequential, seed=731 | 0.022257 | 0.010587 | 2.102 |
| dba1e8a277af6101 | measurement | n=65, state=ghz, count=65, mode=native, seed=731 | 0.021347 | 0.010778 | 1.981 |
| 7e91dc50ae0e4ef9 | measurement | n=65, state=ghz, count=65, mode=sequential, seed=731 | 0.022469 | 0.010768 | 2.087 |
| 75e97bba1b75da3b | measurement | n=65, state=cluster, count=1, mode=native, seed=731 | 0.000427 | 0.003556 | 0.120 |
| 8bb983f6a5b76bc6 | measurement | n=65, state=cluster, count=1, mode=sequential, seed=731 | 0.000305 | 0.003450 | 0.088 |
| 655e40ad315da0cd | measurement | n=65, state=cluster, count=8, mode=native, seed=731 | 0.001490 | 0.004740 | 0.314 |
| f70db5ba3fc56533 | measurement | n=65, state=cluster, count=8, mode=sequential, seed=731 | 0.001466 | 0.020961 | 0.070 |
| 8bc94985951ae686 | measurement | n=65, state=cluster, count=64, mode=native, seed=731 | 0.010061 | 0.012498 | 0.805 |
| 500c6bfe1be8d9fe | measurement | n=65, state=cluster, count=64, mode=sequential, seed=731 | 0.011560 | 0.168867 | 0.068 |
| acb172c4a195656e | measurement | n=65, state=cluster, count=65, mode=native, seed=731 | 0.011245 | 0.012787 | 0.879 |
| 9f067d268eed99f4 | measurement | n=65, state=cluster, count=65, mode=sequential, seed=731 | 0.012798 | 0.170598 | 0.075 |
| 58a820ae57d5e3dc | measurement | n=65, state=scrambled, count=1, mode=native, seed=731 | 0.000775 | 0.004589 | 0.169 |
| 5c1e882023daca4e | measurement | n=65, state=scrambled, count=1, mode=sequential, seed=731 | 0.000552 | 0.003733 | 0.148 |
| 5ba8c1181c5121e6 | measurement | n=65, state=scrambled, count=8, mode=native, seed=731 | 0.004798 | 0.008961 | 0.535 |
| 23e738db41cd1828 | measurement | n=65, state=scrambled, count=8, mode=sequential, seed=731 | 0.005290 | 0.027029 | 0.196 |
| 4b31e2a4facd6c57 | measurement | n=65, state=scrambled, count=64, mode=native, seed=731 | 0.038819 | 0.049362 | 0.786 |
| 1b2ac26b137dad7b | measurement | n=65, state=scrambled, count=64, mode=sequential, seed=731 | 0.037730 | 0.190479 | 0.198 |
| a830a63b4268a728 | measurement | n=65, state=scrambled, count=65, mode=native, seed=731 | 0.038740 | 0.049307 | 0.786 |
| 06ca794a76639abb | measurement | n=65, state=scrambled, count=65, mode=sequential, seed=731 | 0.039455 | 0.203574 | 0.194 |
| 72bd5805a5d4c8f1 | measurement | n=65, state=deterministic, count=1, mode=native, seed=731 | 0.000566 | 0.000412 | 1.374 |
| 06b3b6f9f7543a96 | measurement | n=65, state=deterministic, count=1, mode=sequential, seed=731 | 0.000731 | 0.000397 | 1.841 |
| 98ffda00620f7990 | measurement | n=65, state=deterministic, count=8, mode=native, seed=731 | 0.004396 | 0.001100 | 3.996 |
| b57ce7d5f652c43f | measurement | n=65, state=deterministic, count=8, mode=sequential, seed=731 | 0.004591 | 0.001393 | 3.296 |
| b4e6c25116572a1c | measurement | n=65, state=deterministic, count=64, mode=native, seed=731 | 0.036267 | 0.004121 | 8.801 |
| 1cafeb0ccc485755 | measurement | n=65, state=deterministic, count=64, mode=sequential, seed=731 | 0.036508 | 0.008396 | 4.348 |
| cf7a89f61f8cdca5 | measurement | n=65, state=deterministic, count=65, mode=native, seed=731 | 0.036249 | 0.004492 | 8.070 |
| 74f462872b8c6c27 | measurement | n=65, state=deterministic, count=65, mode=sequential, seed=731 | 0.034767 | 0.007457 | 4.662 |
| 0fb5ff1e34c8468a | measurement | n=129, state=zero, count=1, mode=native, seed=731 | 0.000217 | 0.000384 | 0.565 |
| 34091c47a5b2add2 | measurement | n=129, state=zero, count=1, mode=sequential, seed=731 | 0.000188 | 0.000256 | 0.734 |
| c7020c0eb3d3d4e8 | measurement | n=129, state=zero, count=8, mode=native, seed=731 | 0.000692 | 0.000993 | 0.697 |
| d848e76ef5e2e131 | measurement | n=129, state=zero, count=8, mode=sequential, seed=731 | 0.000713 | 0.001165 | 0.612 |
| aef5a92604640a22 | measurement | n=129, state=zero, count=64, mode=native, seed=731 | 0.003243 | 0.003862 | 0.840 |
| 038db5bcaad375b6 | measurement | n=129, state=zero, count=64, mode=sequential, seed=731 | 0.004386 | 0.007351 | 0.597 |
| a3c7221f810b6259 | measurement | n=129, state=zero, count=129, mode=native, seed=731 | 0.006787 | 0.008141 | 0.834 |
| ef9bb3bd353725fb | measurement | n=129, state=zero, count=129, mode=sequential, seed=731 | 0.008712 | 0.014680 | 0.593 |
| ab8fa1907aa604e3 | measurement | n=129, state=plus, count=1, mode=native, seed=731 | 0.000466 | 0.011914 | 0.039 |
| c5a89db414642e5d | measurement | n=129, state=plus, count=1, mode=sequential, seed=731 | 0.000541 | 0.011293 | 0.048 |
| 87a63adb7d56ef11 | measurement | n=129, state=plus, count=8, mode=native, seed=731 | 0.002738 | 0.012549 | 0.218 |
| 3e0e6c540820357a | measurement | n=129, state=plus, count=8, mode=sequential, seed=731 | 0.002303 | 0.081720 | 0.028 |
| 1b60933142b5dcd2 | measurement | n=129, state=plus, count=64, mode=native, seed=731 | 0.017827 | 0.022485 | 0.793 |
| 94f3c81ae401e885 | measurement | n=129, state=plus, count=64, mode=sequential, seed=731 | 0.020902 | 0.700046 | 0.030 |
| ecad1a49565252cb | measurement | n=129, state=plus, count=129, mode=native, seed=731 | 0.032021 | 0.034493 | 0.928 |
| 087bac53fada2714 | measurement | n=129, state=plus, count=129, mode=sequential, seed=731 | 0.036909 | 1.370365 | 0.027 |
| ec290253a66fcda2 | measurement | n=129, state=bell, count=1, mode=native, seed=731 | 0.000521 | 0.012213 | 0.043 |
| 70dee7f10a502cf8 | measurement | n=129, state=bell, count=1, mode=sequential, seed=731 | 0.000442 | 0.010958 | 0.040 |
| aa13641ad8f300cc | measurement | n=129, state=bell, count=8, mode=native, seed=731 | 0.002659 | 0.013250 | 0.201 |
| 071038eae708e0b5 | measurement | n=129, state=bell, count=8, mode=sequential, seed=731 | 0.002449 | 0.086911 | 0.028 |
| 6a12d91155229157 | measurement | n=129, state=bell, count=64, mode=native, seed=731 | 0.023308 | 0.023303 | 1.000 |
| b0dc50669e20a745 | measurement | n=129, state=bell, count=64, mode=sequential, seed=731 | 0.025275 | 0.515368 | 0.049 |
| eee96918b364352f | measurement | n=129, state=bell, count=129, mode=native, seed=731 | 0.056037 | 0.033787 | 1.659 |
| 04c0ac42196dbba7 | measurement | n=129, state=bell, count=129, mode=sequential, seed=731 | 0.056641 | 0.688483 | 0.082 |
| b9f18d4acda6582c | measurement | n=129, state=ghz, count=1, mode=native, seed=731 | 0.000609 | 0.010897 | 0.056 |
| 3daf01e55925287d | measurement | n=129, state=ghz, count=1, mode=sequential, seed=731 | 0.000596 | 0.010744 | 0.055 |
| 0ae749f414acbf5f | measurement | n=129, state=ghz, count=8, mode=native, seed=731 | 0.006416 | 0.012509 | 0.513 |
| 12bea6d81bdba2bc | measurement | n=129, state=ghz, count=8, mode=sequential, seed=731 | 0.006303 | 0.011609 | 0.543 |
| 79b7f440a89c09ec | measurement | n=129, state=ghz, count=64, mode=native, seed=731 | 0.049190 | 0.020644 | 2.383 |
| 9fc9b27d537c94d2 | measurement | n=129, state=ghz, count=64, mode=sequential, seed=731 | 0.054202 | 0.019358 | 2.800 |
| f9c132bc710e78fd | measurement | n=129, state=ghz, count=129, mode=native, seed=731 | 0.100309 | 0.030177 | 3.324 |
| 16b430653e3a6f5d | measurement | n=129, state=ghz, count=129, mode=sequential, seed=731 | 0.101293 | 0.024132 | 4.197 |
| 385dcf148ce3165b | measurement | n=129, state=cluster, count=1, mode=native, seed=731 | 0.000474 | 0.011589 | 0.041 |
| eb7517e14492bc48 | measurement | n=129, state=cluster, count=1, mode=sequential, seed=731 | 0.000436 | 0.011467 | 0.038 |
| 02f068ca1fe48415 | measurement | n=129, state=cluster, count=8, mode=native, seed=731 | 0.002344 | 0.013451 | 0.174 |
| b8cf7bd22e421fbe | measurement | n=129, state=cluster, count=8, mode=sequential, seed=731 | 0.003474 | 0.088568 | 0.039 |
| d61a9115590f765a | measurement | n=129, state=cluster, count=64, mode=native, seed=731 | 0.015927 | 0.022753 | 0.700 |
| 8f35013b6ebb473b | measurement | n=129, state=cluster, count=64, mode=sequential, seed=731 | 0.018005 | 0.658064 | 0.027 |
| 1a30f9c929131294 | measurement | n=129, state=cluster, count=129, mode=native, seed=731 | 0.034157 | 0.036638 | 0.932 |
| 729fc9dfb00ef19c | measurement | n=129, state=cluster, count=129, mode=sequential, seed=731 | 0.033973 | 1.334739 | 0.025 |
| 02250b299edc8b8d | measurement | n=129, state=scrambled, count=1, mode=native, seed=731 | 0.001609 | 0.014007 | 0.115 |
| 59635c3aaa134ea5 | measurement | n=129, state=scrambled, count=1, mode=sequential, seed=731 | 0.001480 | 0.012312 | 0.120 |
| c9566ef605ddede2 | measurement | n=129, state=scrambled, count=8, mode=native, seed=731 | 0.011236 | 0.023498 | 0.478 |
| a521dd04798f7dd6 | measurement | n=129, state=scrambled, count=8, mode=sequential, seed=731 | 0.011224 | 0.095349 | 0.118 |
| 3e921260ebef6cc4 | measurement | n=129, state=scrambled, count=64, mode=native, seed=731 | 0.085598 | 0.095937 | 0.892 |
| 6138b40353c5c0a5 | measurement | n=129, state=scrambled, count=64, mode=sequential, seed=731 | 0.089196 | 0.774909 | 0.115 |
| 02be596792ef0be2 | measurement | n=129, state=scrambled, count=129, mode=native, seed=731 | 0.170581 | 0.183392 | 0.930 |
| b35328bf2073af44 | measurement | n=129, state=scrambled, count=129, mode=sequential, seed=731 | 0.192962 | 1.720307 | 0.112 |
| 57af27ba925e7bd7 | measurement | n=129, state=deterministic, count=1, mode=native, seed=731 | 0.001390 | 0.000444 | 3.131 |
| 50cfbc894126c1ff | measurement | n=129, state=deterministic, count=1, mode=sequential, seed=731 | 0.001313 | 0.000295 | 4.451 |
| 0ae54e9fb57b8a29 | measurement | n=129, state=deterministic, count=8, mode=native, seed=731 | 0.011576 | 0.001577 | 7.341 |
| d085b4f44b19be61 | measurement | n=129, state=deterministic, count=8, mode=sequential, seed=731 | 0.011024 | 0.001187 | 9.287 |
| cb355fdfad1cd624 | measurement | n=129, state=deterministic, count=64, mode=native, seed=731 | 0.091420 | 0.003941 | 23.197 |
| e80ff1a0e34ff412 | measurement | n=129, state=deterministic, count=64, mode=sequential, seed=731 | 0.090475 | 0.007677 | 11.785 |
| 4fb64e4ddea8c5b7 | measurement | n=129, state=deterministic, count=129, mode=native, seed=731 | 0.182222 | 0.007547 | 24.145 |
| 6a13dd909c332238 | measurement | n=129, state=deterministic, count=129, mode=sequential, seed=731 | 0.177516 | 0.015612 | 11.370 |
| 1aacbd3f4ca77fc3 | measurement | n=256, state=zero, count=1, mode=native, seed=731 | 0.000193 | 0.000428 | 0.451 |
| 842a875370f30ac4 | measurement | n=256, state=zero, count=1, mode=sequential, seed=731 | 0.000176 | 0.000228 | 0.772 |
| 7b81442a891ca034 | measurement | n=256, state=zero, count=8, mode=native, seed=731 | 0.000969 | 0.000880 | 1.101 |
| 1c329d01da2671db | measurement | n=256, state=zero, count=8, mode=sequential, seed=731 | 0.000878 | 0.001042 | 0.843 |
| e1687eaf9cc276de | measurement | n=256, state=zero, count=64, mode=native, seed=731 | 0.003878 | 0.003801 | 1.020 |
| c056340ca6383482 | measurement | n=256, state=zero, count=64, mode=sequential, seed=731 | 0.005002 | 0.006916 | 0.723 |
| 95e9fc01cc023bd9 | measurement | n=256, state=zero, count=256, mode=native, seed=731 | 0.014186 | 0.012712 | 1.116 |
| 8990097acfb691c7 | measurement | n=256, state=zero, count=256, mode=sequential, seed=731 | 0.018798 | 0.025947 | 0.724 |
| 4bfe8b5821c9d249 | measurement | n=256, state=plus, count=1, mode=native, seed=731 | 0.000766 | 0.023865 | 0.032 |
| 9e5c28885d469a3a | measurement | n=256, state=plus, count=1, mode=sequential, seed=731 | 0.000763 | 0.025136 | 0.030 |
| fa13bda4933dd26c | measurement | n=256, state=plus, count=8, mode=native, seed=731 | 0.004761 | 0.026143 | 0.182 |
| 0bae51703bbe32c6 | measurement | n=256, state=plus, count=8, mode=sequential, seed=731 | 0.004805 | 0.184651 | 0.026 |
| e0f3e98400873cfa | measurement | n=256, state=plus, count=64, mode=native, seed=731 | 0.032548 | 0.040413 | 0.805 |
| feffe45d7340d32e | measurement | n=256, state=plus, count=64, mode=sequential, seed=731 | 0.038475 | 1.437905 | 0.027 |
| 6b6f9ca3a0e41116 | measurement | n=256, state=plus, count=256, mode=native, seed=731 | 0.127401 | 0.091116 | 1.398 |
| 4fc8c037ac81ad2e | measurement | n=256, state=plus, count=256, mode=sequential, seed=731 | 0.149451 | 5.861125 | 0.025 |
| 35056ab35a53fc1f | measurement | n=256, state=bell, count=1, mode=native, seed=731 | 0.000791 | 0.023856 | 0.033 |
| 0ae303bdb3f768d2 | measurement | n=256, state=bell, count=1, mode=sequential, seed=731 | 0.000806 | 0.024357 | 0.033 |
| 430bc1050f599a08 | measurement | n=256, state=bell, count=8, mode=native, seed=731 | 0.005296 | 0.027169 | 0.195 |
| c05412d170feefe6 | measurement | n=256, state=bell, count=8, mode=sequential, seed=731 | 0.005067 | 0.179937 | 0.028 |
| 5c4f7104203bf929 | measurement | n=256, state=bell, count=64, mode=native, seed=731 | 0.037183 | 0.040952 | 0.908 |
| 5bd6c774f2ad51a0 | measurement | n=256, state=bell, count=64, mode=sequential, seed=731 | 0.047567 | 1.336775 | 0.036 |
| a70093eb2af5bfcc | measurement | n=256, state=bell, count=256, mode=native, seed=731 | 0.202165 | 0.088355 | 2.288 |
| c13aa239970ae961 | measurement | n=256, state=bell, count=256, mode=sequential, seed=731 | 0.216950 | 2.905192 | 0.075 |
| 086c43321f765881 | measurement | n=256, state=ghz, count=1, mode=native, seed=731 | 0.001044 | 0.023518 | 0.044 |
| ebbefa07aa77bf99 | measurement | n=256, state=ghz, count=1, mode=sequential, seed=731 | 0.001023 | 0.023706 | 0.043 |
| a5973f463a886431 | measurement | n=256, state=ghz, count=8, mode=native, seed=731 | 0.013928 | 0.026340 | 0.529 |
| ed78aea713ee0c84 | measurement | n=256, state=ghz, count=8, mode=sequential, seed=731 | 0.017177 | 0.029484 | 0.583 |
| 9003f121fae146e5 | measurement | n=256, state=ghz, count=64, mode=native, seed=731 | 0.113426 | 0.038732 | 2.928 |
| 1af83b93291adc58 | measurement | n=256, state=ghz, count=64, mode=sequential, seed=731 | 0.117440 | 0.029690 | 3.956 |
| 50ef9b68dddfea0c | measurement | n=256, state=ghz, count=256, mode=native, seed=731 | 0.433846 | 0.083273 | 5.210 |
| 2c89dc2387a8acc0 | measurement | n=256, state=ghz, count=256, mode=sequential, seed=731 | 0.491125 | 0.051697 | 9.500 |
| 4ac605f26313b267 | measurement | n=256, state=cluster, count=1, mode=native, seed=731 | 0.000892 | 0.025367 | 0.035 |
| 4a4ac5993c67dcb4 | measurement | n=256, state=cluster, count=1, mode=sequential, seed=731 | 0.000812 | 0.024547 | 0.033 |
| 934cb10ea5876a10 | measurement | n=256, state=cluster, count=8, mode=native, seed=731 | 0.005144 | 0.026861 | 0.192 |
| 6114953eebb68b26 | measurement | n=256, state=cluster, count=8, mode=sequential, seed=731 | 0.005086 | 0.191333 | 0.027 |
| 99fefa435fb9c908 | measurement | n=256, state=cluster, count=64, mode=native, seed=731 | 0.032242 | 0.040788 | 0.790 |
| 7a50a4e730bdf92b | measurement | n=256, state=cluster, count=64, mode=sequential, seed=731 | 0.041300 | 1.540256 | 0.027 |
| ee9663f4271ad9e2 | measurement | n=256, state=cluster, count=256, mode=native, seed=731 | 0.138993 | 0.096424 | 1.441 |
| 9751c3b5b52160ef | measurement | n=256, state=cluster, count=256, mode=sequential, seed=731 | 0.163575 | 6.307229 | 0.026 |
| dea9362f089d2041 | measurement | n=256, state=scrambled, count=1, mode=native, seed=731 | 0.004031 | 0.029276 | 0.138 |
| a92073b9b1c68371 | measurement | n=256, state=scrambled, count=1, mode=sequential, seed=731 | 0.003821 | 0.027526 | 0.139 |
| 4fd0a63e0ac19e7d | measurement | n=256, state=scrambled, count=8, mode=native, seed=731 | 0.026839 | 0.047383 | 0.566 |
| a3c2a81e07e0f9a0 | measurement | n=256, state=scrambled, count=8, mode=sequential, seed=731 | 0.025888 | 0.201522 | 0.128 |
| e050bd01f1713701 | measurement | n=256, state=scrambled, count=64, mode=native, seed=731 | 0.218015 | 0.200011 | 1.090 |
| 6f2db7b0e046a94f | measurement | n=256, state=scrambled, count=64, mode=sequential, seed=731 | 0.208383 | 1.653758 | 0.126 |
| 55e7950d4c6c5918 | measurement | n=256, state=scrambled, count=256, mode=native, seed=731 | 0.743229 | 0.699160 | 1.063 |
| b4c93a4b92df66ab | measurement | n=256, state=scrambled, count=256, mode=sequential, seed=731 | 0.750347 | 6.810359 | 0.110 |
| 5dbf3ec937702563 | measurement | n=256, state=deterministic, count=1, mode=native, seed=731 | 0.004240 | 0.000508 | 8.346 |
| 5dfaf01e2e41681f | measurement | n=256, state=deterministic, count=1, mode=sequential, seed=731 | 0.004041 | 0.000384 | 10.523 |
| 9ee2e0a753583d71 | measurement | n=256, state=deterministic, count=8, mode=native, seed=731 | 0.033339 | 0.002586 | 12.892 |
| f101f6ea03278d44 | measurement | n=256, state=deterministic, count=8, mode=sequential, seed=731 | 0.032169 | 0.002030 | 15.847 |
| 4c5aa09643766029 | measurement | n=256, state=deterministic, count=64, mode=native, seed=731 | 0.249019 | 0.004516 | 55.141 |
| f6fc552270d6a4da | measurement | n=256, state=deterministic, count=64, mode=sequential, seed=731 | 0.239190 | 0.007510 | 31.850 |
| 7f2c0129b043f2e4 | measurement | n=256, state=deterministic, count=256, mode=native, seed=731 | 0.961165 | 0.016171 | 59.438 |
| 3cc94a8c8f0ecf29 | measurement | n=256, state=deterministic, count=256, mode=sequential, seed=731 | 0.962242 | 0.027283 | 35.269 |
| 256883b5fbb5eb80 | measurement | n=1024, state=zero, count=1, mode=native, seed=731 | 0.000318 | 0.000503 | 0.632 |
| ccb1233a50e5769f | measurement | n=1024, state=zero, count=1, mode=sequential, seed=731 | 0.000352 | 0.000289 | 1.218 |
| 2e17fe1ff30780aa | measurement | n=1024, state=zero, count=8, mode=native, seed=731 | 0.001080 | 0.000880 | 1.227 |
| d63dbad25d762bc0 | measurement | n=1024, state=zero, count=8, mode=sequential, seed=731 | 0.001034 | 0.001265 | 0.817 |
| 26728e024863f143 | measurement | n=1024, state=zero, count=64, mode=native, seed=731 | 0.005782 | 0.003935 | 1.469 |
| c32a71e42620f617 | measurement | n=1024, state=zero, count=64, mode=sequential, seed=731 | 0.007167 | 0.007346 | 0.976 |
| 3634a998ef95eca9 | measurement | n=1024, state=zero, count=256, mode=native, seed=731 | 0.023963 | 0.014921 | 1.606 |
| d23b5377bf647f03 | measurement | n=1024, state=zero, count=256, mode=sequential, seed=731 | 0.028721 | 0.030304 | 0.948 |
| fc0274244fd4f557 | measurement | n=1024, state=zero, count=1024, mode=native, seed=731 | 0.101426 | 0.061148 | 1.659 |
| 26af4f069cc4026b | measurement | n=1024, state=zero, count=1024, mode=sequential, seed=731 | 0.118408 | 0.106809 | 1.109 |
| 120eacda7dbb4146 | measurement | n=1024, state=plus, count=1, mode=native, seed=731 | 0.003402 | 0.212309 | 0.016 |
| 787b35855c745002 | measurement | n=1024, state=plus, count=1, mode=sequential, seed=731 | 0.003045 | 0.209863 | 0.015 |
| 2e9b1404c300322d | measurement | n=1024, state=plus, count=8, mode=native, seed=731 | 0.024004 | 0.220677 | 0.109 |
| 2c06aea29fd61f6c | measurement | n=1024, state=plus, count=8, mode=sequential, seed=731 | 0.024226 | 1.684380 | 0.014 |
| 264add11d9f34618 | measurement | n=1024, state=plus, count=64, mode=native, seed=731 | 0.187972 | 0.267121 | 0.704 |
| d609d60e82183b96 | measurement | n=1024, state=plus, count=64, mode=sequential, seed=731 | 0.209701 | 15.089917 | 0.014 |
| 8c16dad6551f993d | measurement | n=1024, state=plus, count=256, mode=native, seed=731 | 0.801214 | 0.475487 | 1.685 |
| e1e153c2ce4eea5b | measurement | n=1024, state=plus, count=256, mode=sequential, seed=731 | 0.823190 | 57.611401 | 0.014 |
| bc62ae7c69ff9204 | measurement | n=1024, state=plus, count=1024, mode=native, seed=731 | 2.982021 | 1.139532 | 2.617 |
| 9beb0a201192fa7e | measurement | n=1024, state=plus, count=1024, mode=sequential, seed=731 | 3.239852 | 243.003572 | 0.013 |
| b4acba916512e21a | measurement | n=1024, state=bell, count=1, mode=native, seed=731 | 0.003301 | 0.227188 | 0.015 |
| eab328218fa8753a | measurement | n=1024, state=bell, count=1, mode=sequential, seed=731 | 0.003568 | 0.234557 | 0.015 |
| 64758896fbe6ba3f | measurement | n=1024, state=bell, count=8, mode=native, seed=731 | 0.025744 | 0.236711 | 0.109 |
| b92dffe30c9c5c57 | measurement | n=1024, state=bell, count=8, mode=sequential, seed=731 | 0.028060 | 1.848366 | 0.015 |
| 2c2598d78563236f | measurement | n=1024, state=bell, count=64, mode=native, seed=731 | 0.229647 | 0.352981 | 0.651 |
| 1796ceefd5fa8a95 | measurement | n=1024, state=bell, count=64, mode=sequential, seed=731 | 0.205104 | 13.684750 | 0.015 |
| cc1438c742e6fe9f | measurement | n=1024, state=bell, count=256, mode=native, seed=731 | 0.833699 | 0.459624 | 1.814 |
| 9c923d768d9ca595 | measurement | n=1024, state=bell, count=256, mode=sequential, seed=731 | 0.884754 | 51.679614 | 0.017 |
| aec0de9d1b7a0f46 | measurement | n=1024, state=bell, count=1024, mode=native, seed=731 | 3.981278 | 1.095171 | 3.635 |
| be48f5b3d8cbc827 | measurement | n=1024, state=bell, count=1024, mode=sequential, seed=731 | 4.055685 | 123.458724 | 0.033 |
| 6e1d136e687d5277 | measurement | n=1024, state=ghz, count=1, mode=native, seed=731 | 0.004078 | 0.211510 | 0.019 |
| 0a14f86be29547a3 | measurement | n=1024, state=ghz, count=1, mode=sequential, seed=731 | 0.004286 | 0.220169 | 0.019 |
| ed2641740b639c93 | measurement | n=1024, state=ghz, count=8, mode=native, seed=731 | 0.111235 | 0.229776 | 0.484 |
| 94b563622d5f1510 | measurement | n=1024, state=ghz, count=8, mode=sequential, seed=731 | 0.105489 | 0.211223 | 0.499 |
| 0b861a1ad6278f8c | measurement | n=1024, state=ghz, count=64, mode=native, seed=731 | 0.921151 | 0.262509 | 3.509 |
| 1d8b342744c03147 | measurement | n=1024, state=ghz, count=64, mode=sequential, seed=731 | 0.922831 | 0.224587 | 4.109 |
| 31e29f4eea8686fc | measurement | n=1024, state=ghz, count=256, mode=native, seed=731 | 3.946876 | 0.425350 | 9.279 |
| 17df71b9f34ac6fd | measurement | n=1024, state=ghz, count=256, mode=sequential, seed=731 | 4.052538 | 0.285101 | 14.214 |
| c29897bd252daebc | measurement | n=1024, state=ghz, count=1024, mode=native, seed=731 | 17.456241 | 1.153146 | 15.138 |
| c87ed4c3fe42a80a | measurement | n=1024, state=ghz, count=1024, mode=sequential, seed=731 | 16.105997 | 0.381574 | 42.209 |
| c016f1ac10a2a838 | measurement | n=1024, state=cluster, count=1, mode=native, seed=731 | 0.003693 | 0.229257 | 0.016 |
| 026b820c4d964380 | measurement | n=1024, state=cluster, count=1, mode=sequential, seed=731 | 0.003213 | 0.213667 | 0.015 |
| e4763bdc783ba1d0 | measurement | n=1024, state=cluster, count=8, mode=native, seed=731 | 0.025642 | 0.233632 | 0.110 |
| fd0192ac10d90ac3 | measurement | n=1024, state=cluster, count=8, mode=sequential, seed=731 | 0.026145 | 1.882532 | 0.014 |
| 328ba54e7b310e08 | measurement | n=1024, state=cluster, count=64, mode=native, seed=731 | 0.197931 | 0.286127 | 0.692 |
| 50e631ccd8a87a05 | measurement | n=1024, state=cluster, count=64, mode=sequential, seed=731 | 0.205684 | 14.368802 | 0.014 |
| 307c048799bef330 | measurement | n=1024, state=cluster, count=256, mode=native, seed=731 | 0.792855 | 0.454217 | 1.746 |
| e9c17a89179f2ace | measurement | n=1024, state=cluster, count=256, mode=sequential, seed=731 | 0.777663 | 57.257647 | 0.014 |
| 461222500a41dd55 | measurement | n=1024, state=cluster, count=1024, mode=native, seed=731 | 2.995842 | 1.111965 | 2.694 |
| 85e8f0ee1d15b66f | measurement | n=1024, state=cluster, count=1024, mode=sequential, seed=731 | 3.229938 | 241.708983 | 0.013 |
| a25fc3eec46e67ff | measurement | n=1024, state=scrambled, count=1, mode=native, seed=731 | 0.031284 | 0.231028 | 0.135 |
| 8a2f57964333cb05 | measurement | n=1024, state=scrambled, count=1, mode=sequential, seed=731 | 0.035467 | 0.237720 | 0.149 |
| aba7d88b9d4770f1 | measurement | n=1024, state=scrambled, count=8, mode=native, seed=731 | 0.249465 | 0.380026 | 0.656 |
| 285d51c00b06e7d7 | measurement | n=1024, state=scrambled, count=8, mode=sequential, seed=731 | 0.248765 | 1.911717 | 0.130 |
| ac1173f10cc45946 | measurement | n=1024, state=scrambled, count=64, mode=native, seed=731 | 1.905604 | 1.284337 | 1.484 |
| 91bfa46632ce07e5 | measurement | n=1024, state=scrambled, count=64, mode=sequential, seed=731 | 2.026192 | 15.546364 | 0.130 |
| 2071afd99dc351a2 | measurement | n=1024, state=scrambled, count=256, mode=native, seed=731 | 7.640880 | 4.722791 | 1.618 |
| 02775bbcec069d2e | measurement | n=1024, state=scrambled, count=256, mode=sequential, seed=731 | 8.290793 | 63.755209 | 0.130 |
| b5403f6922b91a5f | measurement | n=1024, state=scrambled, count=1024, mode=native, seed=731 | 22.964368 | 18.684865 | 1.229 |
| 86aa7e7870c73905 | measurement | n=1024, state=scrambled, count=1024, mode=sequential, seed=731 | 24.094100 | 263.189801 | 0.092 |
| 95d7ea05a412c773 | measurement | n=1024, state=deterministic, count=1, mode=native, seed=731 | 0.059788 | 0.002787 | 21.452 |
| 482adb855f1593a5 | measurement | n=1024, state=deterministic, count=1, mode=sequential, seed=731 | 0.060039 | 0.001676 | 35.823 |
| d5c82f9cebcc002e | measurement | n=1024, state=deterministic, count=8, mode=native, seed=731 | 0.457017 | 0.003700 | 123.518 |
| 0c7ddd8c4e1bb913 | measurement | n=1024, state=deterministic, count=8, mode=sequential, seed=731 | 0.463358 | 0.003527 | 131.375 |
| 22b25b865ea6a59f | measurement | n=1024, state=deterministic, count=64, mode=native, seed=731 | 3.511274 | 0.006925 | 507.043 |
| fc2ff074e3951b41 | measurement | n=1024, state=deterministic, count=64, mode=sequential, seed=731 | 3.711560 | 0.010329 | 359.334 |
| 23e36f0affaf2dca | measurement | n=1024, state=deterministic, count=256, mode=native, seed=731 | 14.743950 | 0.020695 | 712.440 |
| c61029cde07a8f4e | measurement | n=1024, state=deterministic, count=256, mode=sequential, seed=731 | 14.503395 | 0.042012 | 345.220 |
| 7a9e2295b5ab917b | measurement | n=1024, state=deterministic, count=1024, mode=native, seed=731 | 65.061298 | 0.073330 | 887.240 |
| 395916de8f6cba18 | measurement | n=1024, state=deterministic, count=1024, mode=sequential, seed=731 | 61.687159 | 0.125477 | 491.621 |
| 12b136827f36ba0f | measurement | n=4096, state=zero, count=1, mode=native, seed=731 | 0.002435 | 0.003591 | 0.678 |
| cc1fdaa5e483b472 | measurement | n=4096, state=zero, count=1, mode=sequential, seed=731 | 0.003981 | 0.005066 | 0.786 |
| 513ea6c9ad2c899d | measurement | n=4096, state=zero, count=8, mode=native, seed=731 | 0.004057 | 0.004626 | 0.877 |
| 07a8256f0db5c440 | measurement | n=4096, state=zero, count=8, mode=sequential, seed=731 | 0.003682 | 0.003104 | 1.186 |
| d202f28f6b4f884f | measurement | n=4096, state=zero, count=64, mode=native, seed=731 | 0.019018 | 0.009350 | 2.034 |
| fab00afa0d7b2d7b | measurement | n=4096, state=zero, count=64, mode=sequential, seed=731 | 0.019655 | 0.011203 | 1.754 |
| 99d741496feafb14 | measurement | n=4096, state=zero, count=256, mode=native, seed=731 | 0.084468 | 0.045369 | 1.862 |
| a1450b181a975ec8 | measurement | n=4096, state=zero, count=256, mode=sequential, seed=731 | 0.073974 | 0.036192 | 2.044 |
| 5cc21a592b41afb2 | measurement | n=4096, state=zero, count=4096, mode=native, seed=731 | 0.985697 | 0.269607 | 3.656 |
| b6daa34bbe3bd5c8 | measurement | n=4096, state=zero, count=4096, mode=sequential, seed=731 | 1.055888 | 0.475058 | 2.223 |
| 1b3e68e7c9735eeb | measurement | n=4096, state=plus, count=1, mode=native, seed=731 | 0.044907 | 3.998223 | 0.011 |
| 926eeae28d2370d3 | measurement | n=4096, state=plus, count=1, mode=sequential, seed=731 | 0.045281 | 3.910591 | 0.012 |
| e59c4cfd74fde8c7 | measurement | n=4096, state=plus, count=8, mode=native, seed=731 | 0.312726 | 4.255869 | 0.073 |
| 886ee1a2e0263a87 | measurement | n=4096, state=plus, count=8, mode=sequential, seed=731 | 0.306338 | 33.407604 | 0.009 |
| 5d7fb4c115f77deb | measurement | n=4096, state=plus, count=64, mode=native, seed=731 | 2.194323 | 4.264297 | 0.515 |
| 3e96c759417918c6 | measurement | n=4096, state=plus, count=64, mode=sequential, seed=731 | 2.232913 | 268.433728 | 0.008 |
| 194fe69be9d190b1 | measurement | n=4096, state=plus, count=256, mode=native, seed=731 | 8.959921 | 5.500971 | 1.629 |
| 26d229a2be4c46cb | measurement | n=4096, state=plus, count=256, mode=sequential, seed=731 | 9.249780 | 1131.938582 | 0.008 |
| 47cc40a921597a36 | measurement | n=4096, state=plus, count=4096, mode=native, seed=731 | 147.936688 | 22.568707 | 6.555 |
| 45c736626bc38a89 | measurement | n=4096, state=plus, count=4096, mode=sequential, seed=731 | 141.620101 | 18599.046592 | 0.008 |
| d36882b81638b787 | measurement | n=4096, state=bell, count=1, mode=native, seed=731 | 0.044002 | 3.934081 | 0.011 |
| 77cff47dac16bb6a | measurement | n=4096, state=bell, count=1, mode=sequential, seed=731 | 0.041713 | 3.960738 | 0.011 |
| e252da530c7b7a00 | measurement | n=4096, state=bell, count=8, mode=native, seed=731 | 0.311528 | 4.306153 | 0.072 |
| e9dd895f17288c89 | measurement | n=4096, state=bell, count=8, mode=sequential, seed=731 | 0.310034 | 34.238695 | 0.009 |
| 57ee663f978fcd0b | measurement | n=4096, state=bell, count=64, mode=native, seed=731 | 2.187249 | 4.179431 | 0.523 |
| dfc26b928c00c41e | measurement | n=4096, state=bell, count=64, mode=sequential, seed=731 | 2.349668 | 263.349475 | 0.009 |
| e164a13bf4160d3c | measurement | n=4096, state=bell, count=256, mode=native, seed=731 | 9.222161 | 5.444120 | 1.694 |
| 5ddeb0abe0596596 | measurement | n=4096, state=bell, count=256, mode=sequential, seed=731 | 9.038828 | 1083.180374 | 0.008 |
| ba60c1460b2293ad | measurement | n=4096, state=bell, count=4096, mode=native, seed=731 | 121.061750 | 21.273660 | 5.691 |
| ccb6bbefd5c8d505 | measurement | n=4096, state=bell, count=4096, mode=sequential, seed=731 | 115.928930 | 9080.458353 | 0.013 |
| f13187b30fbc7a75 | measurement | n=4096, state=ghz, count=1, mode=native, seed=731 | 0.077595 | 5.588747 | 0.014 |
| acb530565a829821 | measurement | n=4096, state=ghz, count=1, mode=sequential, seed=731 | 0.076210 | 4.246599 | 0.018 |
| f59db0da1117329e | measurement | n=4096, state=ghz, count=8, mode=native, seed=731 | 0.874672 | 4.492206 | 0.195 |
| 072de372194a6623 | measurement | n=4096, state=ghz, count=8, mode=sequential, seed=731 | 0.785920 | 4.346863 | 0.181 |
| acea21728b15f7a7 | measurement | n=4096, state=ghz, count=64, mode=native, seed=731 | 6.452994 | 5.094183 | 1.267 |
| 3cc944f04dc3f111 | measurement | n=4096, state=ghz, count=64, mode=sequential, seed=731 | 7.293451 | 5.827799 | 1.251 |
| 97b8eabcc40518c0 | measurement | n=4096, state=ghz, count=256, mode=native, seed=731 | 26.012581 | 6.794226 | 3.829 |
| 8c58ff573cc83ca7 | measurement | n=4096, state=ghz, count=256, mode=sequential, seed=731 | 26.653012 | 4.591147 | 5.805 |
| 3a45f9e0b94fe4b9 | measurement | n=4096, state=ghz, count=4096, mode=native, seed=731 | 402.219323 | 22.091333 | 18.207 |
| 01c6bd39c7a39dc3 | measurement | n=4096, state=ghz, count=4096, mode=sequential, seed=731 | 434.778567 | 5.130754 | 84.740 |
| d6bf3dae44403ccb | measurement | n=4096, state=cluster, count=1, mode=native, seed=731 | 0.046301 | 4.260795 | 0.011 |
| 706886c7fb6066a1 | measurement | n=4096, state=cluster, count=1, mode=sequential, seed=731 | 0.046512 | 4.248166 | 0.011 |
| 7aded232002c73e6 | measurement | n=4096, state=cluster, count=8, mode=native, seed=731 | 0.315071 | 4.701517 | 0.067 |
| df0af5766eade610 | measurement | n=4096, state=cluster, count=8, mode=sequential, seed=731 | 0.320288 | 33.876891 | 0.009 |
| 4ad50e340d850555 | measurement | n=4096, state=cluster, count=64, mode=native, seed=731 | 2.580218 | 4.932979 | 0.523 |
| 5155a880cb07cd86 | measurement | n=4096, state=cluster, count=64, mode=sequential, seed=731 | 2.387266 | 307.289114 | 0.008 |
| 3898e99ace8d8072 | measurement | n=4096, state=cluster, count=256, mode=native, seed=731 | 9.082977 | 5.171052 | 1.757 |
| e3d849ce02a4b4dd | measurement | n=4096, state=cluster, count=256, mode=sequential, seed=731 | 9.256644 | 1225.978232 | 0.008 |
| 2ea106ba12259d38 | measurement | n=4096, state=cluster, count=4096, mode=native, seed=731 | 140.641106 | 23.021465 | 6.109 |
| 0d2d5d5e5dda6833 | measurement | n=4096, state=cluster, count=4096, mode=sequential, seed=731 | 130.121251 | 17748.473863 | 0.007 |
| 3eb932be079cc4c6 | measurement | n=4096, state=scrambled, count=1, mode=native, seed=731 | 0.450687 | 4.184274 | 0.108 |
| 6174df4ff6de05c3 | measurement | n=4096, state=scrambled, count=1, mode=sequential, seed=731 | 0.444396 | 4.334112 | 0.103 |
| f8a70ffe1ede5878 | measurement | n=4096, state=scrambled, count=8, mode=native, seed=731 | 3.586144 | 6.174331 | 0.581 |
| 8041652003bc0947 | measurement | n=4096, state=scrambled, count=8, mode=sequential, seed=731 | 3.613410 | 34.092733 | 0.106 |
| c4e71a1d7bd87d34 | measurement | n=4096, state=scrambled, count=64, mode=native, seed=731 | 30.497695 | 19.306369 | 1.580 |
| 2e135c1e9f865dd6 | measurement | n=4096, state=scrambled, count=64, mode=sequential, seed=731 | 28.335310 | 278.849776 | 0.102 |
| df55bc429fe792bb | measurement | n=4096, state=scrambled, count=256, mode=native, seed=731 | 117.597612 | 56.285519 | 2.089 |
| adc4713eceaa32fe | measurement | n=4096, state=scrambled, count=256, mode=sequential, seed=731 | 117.802614 | 1107.897059 | 0.106 |
| 45221b74d1ca4ea5 | measurement | n=4096, state=scrambled, count=4096, mode=native, seed=731 | 1355.791843 | 886.536151 | 1.529 |
| d09d95261cbfb836 | measurement | n=4096, state=scrambled, count=4096, mode=sequential, seed=731 | 1279.402335 | 18290.200627 | 0.070 |
| a3e8b62e1c34587e | measurement | n=4096, state=deterministic, count=1, mode=native, seed=731 | 1.000180 | 0.029939 | 33.407 |
| a2e564ec90fa4947 | measurement | n=4096, state=deterministic, count=1, mode=sequential, seed=731 | 0.944901 | 0.017762 | 53.198 |
| 887079132af0a6e4 | measurement | n=4096, state=deterministic, count=8, mode=native, seed=731 | 7.944742 | 0.031686 | 250.734 |
| 0b468175970d22e7 | measurement | n=4096, state=deterministic, count=8, mode=sequential, seed=731 | 7.931665 | 0.022631 | 350.478 |
| b11d4f7d043553ab | measurement | n=4096, state=deterministic, count=64, mode=native, seed=731 | 66.547592 | 0.039530 | 1683.471 |
| 381aa1c2de1d3c8d | measurement | n=4096, state=deterministic, count=64, mode=sequential, seed=731 | 66.076300 | 0.041412 | 1595.583 |
| 726bafb98b152d04 | measurement | n=4096, state=deterministic, count=256, mode=native, seed=731 | 260.405872 | 0.059198 | 4398.896 |
| 99822f1d0b0f8016 | measurement | n=4096, state=deterministic, count=256, mode=sequential, seed=731 | 264.690912 | 0.087678 | 3018.898 |
| 91c4c565dcc80d56 | measurement | n=4096, state=deterministic, count=4096, mode=native, seed=731 | 4228.929041 | 0.344622 | 12271.210 |
| 4202fe7836d67169 | measurement | n=4096, state=deterministic, count=4096, mode=sequential, seed=731 | 4277.851019 | 0.616079 | 6943.673 |
| 8529be117c6816cf | measurement | n=65, state=scrambled, count=1, mode=native, seed=0 | 0.000875 | 0.004232 | 0.207 |
| a517f79ca1cfb180 | measurement | n=65, state=scrambled, count=1, mode=sequential, seed=0 | 0.000711 | 0.003898 | 0.182 |
| 4fc6dd36ebbe4077 | measurement | n=65, state=scrambled, count=64, mode=native, seed=0 | 0.042513 | 0.048693 | 0.873 |
| d0a1cf12ce8a273b | measurement | n=65, state=scrambled, count=64, mode=sequential, seed=0 | 0.045808 | 0.203947 | 0.225 |
| 621a6ba8a34f43ed | measurement | n=65, state=scrambled, count=1, mode=native, seed=2026 | 0.001160 | 0.005940 | 0.195 |
| 3cb7ea739c8248d7 | measurement | n=65, state=scrambled, count=1, mode=sequential, seed=2026 | 0.000842 | 0.003793 | 0.222 |
| ccaf5f63f24ec274 | measurement | n=65, state=scrambled, count=64, mode=native, seed=2026 | 0.040003 | 0.047907 | 0.835 |
| cc23a63fb3ea76e1 | measurement | n=65, state=scrambled, count=64, mode=sequential, seed=2026 | 0.040031 | 0.205209 | 0.195 |
| b1d4bd22bc037c5d | measurement | n=1024, state=scrambled, count=1, mode=native, seed=0 | 0.033690 | 0.248177 | 0.136 |
| 6de2fc5bf949413a | measurement | n=1024, state=scrambled, count=1, mode=sequential, seed=0 | 0.032733 | 0.246577 | 0.133 |
| c5dc0fb10af49a90 | measurement | n=1024, state=scrambled, count=64, mode=native, seed=0 | 2.023485 | 1.361655 | 1.486 |
| 99ec4b2c8984a56c | measurement | n=1024, state=scrambled, count=64, mode=sequential, seed=0 | 2.052254 | 16.287416 | 0.126 |
| fb580c4e285368dd | measurement | n=1024, state=scrambled, count=1, mode=native, seed=2026 | 0.040187 | 0.263237 | 0.153 |
| e42c6bf24d90da3e | measurement | n=1024, state=scrambled, count=1, mode=sequential, seed=2026 | 0.034042 | 0.249911 | 0.136 |
| 8dc682edaf795c45 | measurement | n=1024, state=scrambled, count=64, mode=native, seed=2026 | 3.051611 | 1.767387 | 1.727 |
| f30c493cc5303581 | measurement | n=1024, state=scrambled, count=64, mode=sequential, seed=2026 | 2.027453 | 16.476456 | 0.123 |
| e33e233606c0e4b1 | protocol | n=65, state=zero, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.000381 | 0.003671 | 0.104 |
| 34c5a4b5357d7323 | protocol | n=65, state=zero, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.000464 | 0.003783 | 0.123 |
| e5f3ac2bed4cc8fb | protocol | n=65, state=zero, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 0.011180 | 0.165279 | 0.068 |
| ee3cf01e8019ae1b | protocol | n=65, state=zero, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 0.010538 | 0.012179 | 0.865 |
| 0af8ca42e20ddd6c | protocol | n=65, state=zero, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.000493 | 0.003913 | 0.126 |
| 7ca1668bc9a3ad30 | protocol | n=65, state=zero, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.000529 | 0.004228 | 0.125 |
| b4e3287d263fffb8 | protocol | n=65, state=zero, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 0.013363 | 0.173819 | 0.077 |
| 935c4c62f00599fc | protocol | n=65, state=zero, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 0.013249 | 0.012458 | 1.063 |
| 0ae33058b427d798 | protocol | n=65, state=plus, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.000346 | 0.000571 | 0.606 |
| a349ae06a77214e3 | protocol | n=65, state=plus, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.000360 | 0.000807 | 0.446 |
| 5cfa23bc7fd59ad8 | protocol | n=65, state=plus, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 0.003582 | 0.002892 | 1.239 |
| c4274860c7007932 | protocol | n=65, state=plus, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 0.003768 | 0.003227 | 1.168 |
| 3c301e9308a4274d | protocol | n=65, state=plus, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.000358 | 0.003565 | 0.100 |
| 38fe7ea1581cc76a | protocol | n=65, state=plus, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.000433 | 0.003831 | 0.113 |
| 97101f521249eac7 | protocol | n=65, state=plus, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 0.013528 | 0.174724 | 0.077 |
| fe1ebeee4de7ae0d | protocol | n=65, state=plus, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 0.012902 | 0.012814 | 1.007 |
| 378597ccb24ce2a2 | protocol | n=65, state=bell, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.000446 | 0.004542 | 0.098 |
| 74a0f93082532f3b | protocol | n=65, state=bell, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.000407 | 0.003795 | 0.107 |
| a8410c8f284dd437 | protocol | n=65, state=bell, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 0.016910 | 0.086910 | 0.195 |
| 5f4b416a3b15bdcf | protocol | n=65, state=bell, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 0.016482 | 0.010821 | 1.523 |
| b0154e031e4a6f03 | protocol | n=65, state=bell, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.000461 | 0.003767 | 0.122 |
| f2d83f86386d46f3 | protocol | n=65, state=bell, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.000458 | 0.003925 | 0.117 |
| 3b70d325b1b39bd1 | protocol | n=65, state=bell, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 0.020652 | 0.097766 | 0.211 |
| f285368ff82a1904 | protocol | n=65, state=bell, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 0.021991 | 0.013474 | 1.632 |
| 7d37b7982cf4df50 | protocol | n=65, state=ghz, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.000378 | 0.003701 | 0.102 |
| 070f1a5cd22aa29b | protocol | n=65, state=ghz, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.000755 | 0.004331 | 0.174 |
| 912ab0ab0b610a5d | protocol | n=65, state=ghz, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 0.010950 | 0.163334 | 0.067 |
| 792217748b67cab2 | protocol | n=65, state=ghz, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 0.011162 | 0.012074 | 0.924 |
| e45a75268913bd14 | protocol | n=65, state=ghz, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.000625 | 0.003826 | 0.163 |
| 813c2a24765a1248 | protocol | n=65, state=ghz, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.000697 | 0.003739 | 0.186 |
| 02fca5b0e9a00d50 | protocol | n=65, state=ghz, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 0.015865 | 0.177872 | 0.089 |
| 8612a0891dd9c7a1 | protocol | n=65, state=ghz, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 0.015693 | 0.015004 | 1.046 |
| 1048de48d27092b8 | protocol | n=65, state=cluster, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.000441 | 0.003750 | 0.118 |
| 35831afbca8217f3 | protocol | n=65, state=cluster, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.000402 | 0.003924 | 0.102 |
| 4c51417296c34cfe | protocol | n=65, state=cluster, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 0.012597 | 0.167422 | 0.075 |
| 970bb0393359ca93 | protocol | n=65, state=cluster, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 0.012715 | 0.014264 | 0.891 |
| a937f7d79a286abb | protocol | n=65, state=cluster, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.000477 | 0.004160 | 0.115 |
| e130e775aa3ba468 | protocol | n=65, state=cluster, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.000440 | 0.003721 | 0.118 |
| bc7792352327baba | protocol | n=65, state=cluster, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 0.015907 | 0.177481 | 0.090 |
| 46eb50a1bd115759 | protocol | n=65, state=cluster, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 0.015816 | 0.016266 | 0.972 |
| 3f400353039e7c3c | protocol | n=65, state=scrambled, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.001290 | 0.004797 | 0.269 |
| d9b9c3b426739d36 | protocol | n=65, state=scrambled, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.001001 | 0.004669 | 0.214 |
| 1dff981358e935a2 | protocol | n=65, state=scrambled, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 0.037839 | 0.190526 | 0.199 |
| 640ac3885ee9b3dd | protocol | n=65, state=scrambled, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 0.043724 | 0.043027 | 1.016 |
| ef232e5970e21ac4 | protocol | n=65, state=scrambled, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.001000 | 0.004551 | 0.220 |
| bb943972e5847d94 | protocol | n=65, state=scrambled, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.000855 | 0.004204 | 0.203 |
| 999683003cd8cd75 | protocol | n=65, state=scrambled, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 0.042678 | 0.196156 | 0.218 |
| cad6d98d4d792d93 | protocol | n=65, state=scrambled, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 0.041614 | 0.040238 | 1.034 |
| 3443d76e1c2f757e | protocol | n=65, state=deterministic, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.000598 | 0.003927 | 0.152 |
| a5c87a93ec756d64 | protocol | n=65, state=deterministic, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.000565 | 0.004326 | 0.131 |
| 10cc835df4e10301 | protocol | n=65, state=deterministic, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 0.020217 | 0.177501 | 0.114 |
| a98386bb7a5400b1 | protocol | n=65, state=deterministic, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 0.020463 | 0.024251 | 0.844 |
| 7f788cba61d8ba72 | protocol | n=65, state=deterministic, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.000596 | 0.004318 | 0.138 |
| 7d3ea27f9d754907 | protocol | n=65, state=deterministic, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.000808 | 0.004014 | 0.201 |
| bd156339b40afdbb | protocol | n=65, state=deterministic, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 0.023378 | 0.196594 | 0.119 |
| 395f85be5082b169 | protocol | n=65, state=deterministic, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 0.023309 | 0.039790 | 0.586 |
| d82b71eea38af68a | protocol | n=1024, state=zero, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.003769 | 0.227091 | 0.017 |
| 0cc3cb7c5ace53f5 | protocol | n=1024, state=zero, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.003381 | 0.223103 | 0.015 |
| 853fe73a43ae5d76 | protocol | n=1024, state=zero, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 0.209901 | 15.159658 | 0.014 |
| 789584d838308371 | protocol | n=1024, state=zero, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 0.208509 | 0.304952 | 0.684 |
| ab266ac9d279f454 | protocol | n=1024, state=zero, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.003668 | 0.227083 | 0.016 |
| 8a3b112aa1ab53a7 | protocol | n=1024, state=zero, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.003678 | 0.226176 | 0.016 |
| fba0ce3ccb8a06b4 | protocol | n=1024, state=zero, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 0.218787 | 14.937362 | 0.015 |
| b3343047d6672cbe | protocol | n=1024, state=zero, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 0.218346 | 0.295953 | 0.738 |
| 7ddd65332439afe4 | protocol | n=1024, state=plus, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.000449 | 0.000636 | 0.706 |
| 808eba4680e3f37f | protocol | n=1024, state=plus, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.000379 | 0.000823 | 0.461 |
| 4d48f87108778d18 | protocol | n=1024, state=plus, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 0.008727 | 0.003954 | 2.207 |
| c3c7283b794ffdf5 | protocol | n=1024, state=plus, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 0.008038 | 0.007096 | 1.133 |
| eb3fd7d6f0155657 | protocol | n=1024, state=plus, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.004056 | 0.236920 | 0.017 |
| 001b885d3bdf6b52 | protocol | n=1024, state=plus, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.003760 | 0.228455 | 0.016 |
| 8a424d2eadccd988 | protocol | n=1024, state=plus, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 0.214852 | 15.230340 | 0.014 |
| 09f92036a2004958 | protocol | n=1024, state=plus, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 0.211565 | 0.296301 | 0.714 |
| 9d4aca5011f75830 | protocol | n=1024, state=bell, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.003806 | 0.228252 | 0.017 |
| 1bf080aaaaa2e19c | protocol | n=1024, state=bell, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.003697 | 0.224782 | 0.016 |
| 2cb7a78a75030df6 | protocol | n=1024, state=bell, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 0.215863 | 14.954057 | 0.014 |
| 41f71a2bdfe598d7 | protocol | n=1024, state=bell, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 0.212036 | 0.294721 | 0.719 |
| 6c36d4a1d344e921 | protocol | n=1024, state=bell, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.004428 | 0.348751 | 0.013 |
| e64e435e5e086d27 | protocol | n=1024, state=bell, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.003948 | 0.244442 | 0.016 |
| b5daa76a6ce94bec | protocol | n=1024, state=bell, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 0.275930 | 15.795170 | 0.017 |
| b88ef4a93eb41177 | protocol | n=1024, state=bell, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 0.214851 | 0.299716 | 0.717 |
| 4629e91db1f9285f | protocol | n=1024, state=ghz, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.003342 | 0.224879 | 0.015 |
| 9084ee463fb4e607 | protocol | n=1024, state=ghz, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.004103 | 0.235292 | 0.017 |
| 1602633dc18d93af | protocol | n=1024, state=ghz, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 0.223716 | 15.590975 | 0.014 |
| 804e6095f9ddd6c8 | protocol | n=1024, state=ghz, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 0.217943 | 0.307650 | 0.708 |
| efeb0759d26f8977 | protocol | n=1024, state=ghz, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.007016 | 0.238538 | 0.029 |
| fe4f65f96a25fff9 | protocol | n=1024, state=ghz, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.008549 | 0.278258 | 0.031 |
| b8ac9a995e2dac4f | protocol | n=1024, state=ghz, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 0.443909 | 18.992136 | 0.023 |
| b279058dedf7eec2 | protocol | n=1024, state=ghz, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 0.223221 | 0.299928 | 0.744 |
| e3585a1963e84d8e | protocol | n=1024, state=cluster, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.003963 | 0.228082 | 0.017 |
| f4ef12b97b6900e6 | protocol | n=1024, state=cluster, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.003434 | 0.223653 | 0.015 |
| 1df33bd460708f04 | protocol | n=1024, state=cluster, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 0.217609 | 14.986642 | 0.015 |
| ff452cfd7788b9ca | protocol | n=1024, state=cluster, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 0.206391 | 0.300006 | 0.688 |
| 46be832240edf40d | protocol | n=1024, state=cluster, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.003769 | 0.225212 | 0.017 |
| db149351b017da47 | protocol | n=1024, state=cluster, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.003923 | 0.252060 | 0.016 |
| 16a1461d714a573c | protocol | n=1024, state=cluster, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 0.263601 | 16.000297 | 0.016 |
| ad5596ea8758e74f | protocol | n=1024, state=cluster, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 0.236553 | 0.321602 | 0.736 |
| 3c2c320f7321528a | protocol | n=1024, state=scrambled, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.040058 | 0.278768 | 0.144 |
| b4b139c37a0ccb41 | protocol | n=1024, state=scrambled, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.035645 | 0.260942 | 0.137 |
| b9d80568d7c39fd5 | protocol | n=1024, state=scrambled, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 2.027312 | 15.976980 | 0.127 |
| bbbb94f25964ab29 | protocol | n=1024, state=scrambled, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 2.115817 | 1.211032 | 1.747 |
| b85c9cd3c041d569 | protocol | n=1024, state=scrambled, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.033986 | 0.246281 | 0.138 |
| 08ffee9680029a05 | protocol | n=1024, state=scrambled, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.033582 | 0.248575 | 0.135 |
| 42090786cb23d31c | protocol | n=1024, state=scrambled, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 1.986439 | 17.870472 | 0.111 |
| 02e239bf9a24d704 | protocol | n=1024, state=scrambled, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 2.018039 | 1.211611 | 1.666 |
| e4fb19c8a6d03737 | protocol | n=1024, state=deterministic, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.013496 | 0.241985 | 0.056 |
| 86077ac10ec83c63 | protocol | n=1024, state=deterministic, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.015129 | 0.251308 | 0.060 |
| 8e825fb0f4dccf4d | protocol | n=1024, state=deterministic, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 0.849749 | 16.992167 | 0.050 |
| 36a1c7c7f8e8bcd8 | protocol | n=1024, state=deterministic, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 0.833737 | 1.133038 | 0.736 |
| 767fe37a918a87bc | protocol | n=1024, state=deterministic, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.022983 | 0.244808 | 0.094 |
| 85c05f37f35b94c9 | protocol | n=1024, state=deterministic, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.024461 | 0.255107 | 0.096 |
| 461a3de1b34e64fc | protocol | n=1024, state=deterministic, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 1.176923 | 16.512253 | 0.071 |
| 3bfb4d914b871e12 | protocol | n=1024, state=deterministic, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 1.121296 | 1.239426 | 0.905 |
| e8aea92a5eae9c4b | protocol | n=4096, state=zero, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.042281 | 4.429906 | 0.010 |
| d7e15f7a24144251 | protocol | n=4096, state=zero, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.039854 | 4.915221 | 0.008 |
| 59ab86879286f4c0 | protocol | n=4096, state=zero, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 2.327682 | 298.698118 | 0.008 |
| b47e841bb001e1de | protocol | n=4096, state=zero, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 2.313777 | 4.902620 | 0.472 |
| a92b6cb03ce986dc | protocol | n=4096, state=zero, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.040710 | 4.350964 | 0.009 |
| e103faba3a17d1a0 | protocol | n=4096, state=zero, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.049816 | 5.195994 | 0.010 |
| 6bbd80a15f38f649 | protocol | n=4096, state=zero, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 2.428002 | 294.329863 | 0.008 |
| 253f3a49f76c6fed | protocol | n=4096, state=zero, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 2.329044 | 4.881821 | 0.477 |
| 9c206c64f7919401 | protocol | n=4096, state=plus, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.005390 | 0.006827 | 0.790 |
| 31ececfa61a2aa9e | protocol | n=4096, state=plus, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.004432 | 0.006450 | 0.687 |
| 424262e5ef1b0401 | protocol | n=4096, state=plus, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 0.028430 | 0.014213 | 2.000 |
| a2da255e1fef86f5 | protocol | n=4096, state=plus, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 0.026750 | 0.023731 | 1.127 |
| e5c523af1acfe4a3 | protocol | n=4096, state=plus, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.043562 | 3.995805 | 0.011 |
| cf46e860c626ba95 | protocol | n=4096, state=plus, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.048322 | 4.142155 | 0.012 |
| 5210391edd28356a | protocol | n=4096, state=plus, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 2.385702 | 297.538568 | 0.008 |
| af19272ebe07e193 | protocol | n=4096, state=plus, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 2.327748 | 5.016247 | 0.464 |
| 18c52bd2176b837d | protocol | n=4096, state=bell, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.043920 | 5.403486 | 0.008 |
| 2199d68a40d78f7d | protocol | n=4096, state=bell, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.046012 | 4.279190 | 0.011 |
| d5df572946e46ea9 | protocol | n=4096, state=bell, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 2.454004 | 302.451054 | 0.008 |
| 13e9bab5854cea3b | protocol | n=4096, state=bell, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 2.355331 | 4.494955 | 0.524 |
| 2cbc5640edd17efd | protocol | n=4096, state=bell, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.046302 | 4.387414 | 0.011 |
| 59d0c2576f8d1c26 | protocol | n=4096, state=bell, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.044994 | 4.354932 | 0.010 |
| 880e0d93e5e1ba15 | protocol | n=4096, state=bell, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 2.358571 | 301.501217 | 0.008 |
| 393b36599641e34e | protocol | n=4096, state=bell, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 2.217230 | 4.424735 | 0.501 |
| 412390a427ace399 | protocol | n=4096, state=ghz, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.043406 | 4.135946 | 0.010 |
| ed82edbe393d7db2 | protocol | n=4096, state=ghz, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.042793 | 4.013970 | 0.011 |
| 525f9957b2c2e853 | protocol | n=4096, state=ghz, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 2.199799 | 287.998438 | 0.008 |
| df60cf42efba33ed | protocol | n=4096, state=ghz, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 2.282897 | 4.946824 | 0.461 |
| 640f41e84ebc9a51 | protocol | n=4096, state=ghz, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.081217 | 4.170142 | 0.019 |
| b2c23d9c3f1c6c24 | protocol | n=4096, state=ghz, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.084842 | 4.926951 | 0.017 |
| 2b8ff9a9ae5eab12 | protocol | n=4096, state=ghz, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 2.447107 | 290.868466 | 0.008 |
| 43e7f47e71c5500d | protocol | n=4096, state=ghz, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 2.417998 | 4.972600 | 0.486 |
| f016a50d61d2b201 | protocol | n=4096, state=cluster, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.044010 | 4.451199 | 0.010 |
| 265f0469fcbab01b | protocol | n=4096, state=cluster, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.048329 | 4.096383 | 0.012 |
| 3cec6d23d1f427fe | protocol | n=4096, state=cluster, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 2.272735 | 300.684755 | 0.008 |
| 5f7cc47fe6f2dbe5 | protocol | n=4096, state=cluster, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 2.274879 | 4.839189 | 0.470 |
| 90053768248b894b | protocol | n=4096, state=cluster, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.046014 | 4.122379 | 0.011 |
| dc132a21ece7abdd | protocol | n=4096, state=cluster, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.046044 | 4.280133 | 0.011 |
| 8dfd200989cdf6f6 | protocol | n=4096, state=cluster, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 2.327709 | 290.562227 | 0.008 |
| 7f91eb68aacddc01 | protocol | n=4096, state=cluster, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 5.783972 | 12.018709 | 0.481 |
| 8182985e2efcb19b | protocol | n=4096, state=scrambled, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 1.722415 | 15.091310 | 0.114 |
| 486fef8188435567 | protocol | n=4096, state=scrambled, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 1.224128 | 11.895775 | 0.103 |
| 5f5b971b3ac32123 | protocol | n=4096, state=scrambled, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 73.982015 | 813.076667 | 0.091 |
| 4a54723ba306af68 | protocol | n=4096, state=scrambled, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 83.660551 | 41.563458 | 2.013 |
| c0d4aaedae95badf | protocol | n=4096, state=scrambled, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 1.253380 | 13.303929 | 0.094 |
| a668a8ede3a260ee | protocol | n=4096, state=scrambled, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 1.288977 | 12.083007 | 0.107 |
| cc97703df32197c0 | protocol | n=4096, state=scrambled, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 72.479080 | 752.653942 | 0.096 |
| 672b81184d7ec508 | protocol | n=4096, state=scrambled, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 76.258857 | 78.458024 | 0.972 |
| 03d27c280c4b2f0f | protocol | n=4096, state=deterministic, kind=basis, count=1, seed=731, basis=X, stim_mode=decomposed | 0.433875 | 13.216460 | 0.033 |
| 8e1fd0766a598c54 | protocol | n=4096, state=deterministic, kind=basis, count=1, seed=731, basis=X, stim_mode=native_pauli | 0.404514 | 11.770543 | 0.034 |
| b1c77deeba073221 | protocol | n=4096, state=deterministic, kind=basis, count=64, seed=731, basis=X, stim_mode=decomposed | 28.779075 | 707.730043 | 0.041 |
| 197c7c6e89457e1d | protocol | n=4096, state=deterministic, kind=basis, count=64, seed=731, basis=X, stim_mode=native_pauli | 27.632082 | 36.846155 | 0.750 |
| a115b993d4621add | protocol | n=4096, state=deterministic, kind=basis, count=1, seed=731, basis=Y, stim_mode=decomposed | 0.476005 | 8.966854 | 0.053 |
| 5e2e417e4cd85510 | protocol | n=4096, state=deterministic, kind=basis, count=1, seed=731, basis=Y, stim_mode=native_pauli | 0.515255 | 10.895947 | 0.047 |
| b4350438289b330d | protocol | n=4096, state=deterministic, kind=basis, count=64, seed=731, basis=Y, stim_mode=decomposed | 42.598273 | 850.332813 | 0.050 |
| 6dab1bd8f82ab4d8 | protocol | n=4096, state=deterministic, kind=basis, count=64, seed=731, basis=Y, stim_mode=native_pauli | 19.508398 | 20.033604 | 0.974 |
| da1d7c3386df2bb9 | protocol | n=65, state=ghz, kind=parity, count=1, seed=731, weight=2, stim_mode=decomposed | 0.001003 | 0.008197 | 0.122 |
| 80fc4ed5c4b4a2b7 | protocol | n=65, state=ghz, kind=parity, count=1, seed=731, weight=2, stim_mode=native_pauli | 0.001045 | 0.007465 | 0.140 |
| e2cbed244d2b01e3 | protocol | n=65, state=ghz, kind=parity, count=64, seed=731, weight=2, stim_mode=decomposed | 0.022677 | 0.250078 | 0.091 |
| c1f656f239206352 | protocol | n=65, state=ghz, kind=parity, count=64, seed=731, weight=2, stim_mode=native_pauli | 0.017333 | 0.054229 | 0.320 |
| 3871046740ba1211 | protocol | n=65, state=ghz, kind=parity, count=1, seed=731, weight=16, stim_mode=decomposed | 0.001254 | 0.005904 | 0.212 |
| f95c508ca1b8e466 | protocol | n=65, state=ghz, kind=parity, count=1, seed=731, weight=16, stim_mode=native_pauli | 0.001212 | 0.004901 | 0.247 |
| 214a661541e60959 | protocol | n=65, state=ghz, kind=parity, count=64, seed=731, weight=16, stim_mode=decomposed | 0.061560 | 0.298628 | 0.206 |
| 9c07f5669e626c33 | protocol | n=65, state=ghz, kind=parity, count=64, seed=731, weight=16, stim_mode=native_pauli | 0.060454 | 0.255513 | 0.237 |
| 5dbbbfd42c18ae32 | protocol | n=65, state=ghz, kind=parity, count=1, seed=731, weight=64, stim_mode=decomposed | 0.001937 | 0.010406 | 0.186 |
| 7d4a4dcc1d776966 | protocol | n=65, state=ghz, kind=parity, count=1, seed=731, weight=64, stim_mode=native_pauli | 0.002087 | 0.008137 | 0.256 |
| f05245f0e3ac6ef9 | protocol | n=65, state=ghz, kind=parity, count=64, seed=731, weight=64, stim_mode=decomposed | 0.347352 | 1.367933 | 0.254 |
| 1772038140b63d43 | protocol | n=65, state=ghz, kind=parity, count=64, seed=731, weight=64, stim_mode=native_pauli | 0.355744 | 0.944019 | 0.377 |
| a11b7ee231421735 | protocol | n=65, state=ghz, kind=parity, count=1, seed=731, weight=65, stim_mode=decomposed | 0.005823 | 0.023284 | 0.250 |
| 52cc66b6bd4587dc | protocol | n=65, state=ghz, kind=parity, count=1, seed=731, weight=65, stim_mode=native_pauli | 0.004682 | 0.017706 | 0.264 |
| 5ee2f68d27afc3bd | protocol | n=65, state=ghz, kind=parity, count=64, seed=731, weight=65, stim_mode=decomposed | 0.387229 | 1.358194 | 0.285 |
| 6d48cbd6fc2c85e1 | protocol | n=65, state=ghz, kind=parity, count=64, seed=731, weight=65, stim_mode=native_pauli | 0.316126 | 1.078969 | 0.293 |
| f921bb96cdf01521 | protocol | n=65, state=cluster, kind=parity, count=1, seed=731, weight=2, stim_mode=decomposed | 0.001091 | 0.008430 | 0.129 |
| 6f46570946d2cf60 | protocol | n=65, state=cluster, kind=parity, count=1, seed=731, weight=2, stim_mode=native_pauli | 0.001012 | 0.008437 | 0.120 |
| fee7fdfbeadb16ef | protocol | n=65, state=cluster, kind=parity, count=64, seed=731, weight=2, stim_mode=decomposed | 0.045096 | 0.503221 | 0.090 |
| cb3e3a430c03c7f3 | protocol | n=65, state=cluster, kind=parity, count=64, seed=731, weight=2, stim_mode=native_pauli | 0.037210 | 0.131446 | 0.283 |
| 58e86ed895ee1d5f | protocol | n=65, state=cluster, kind=parity, count=1, seed=731, weight=16, stim_mode=decomposed | 0.001938 | 0.012156 | 0.159 |
| 55038fe8fc10123b | protocol | n=65, state=cluster, kind=parity, count=1, seed=731, weight=16, stim_mode=native_pauli | 0.002027 | 0.010534 | 0.192 |
| 006265aa7ece3a85 | protocol | n=65, state=cluster, kind=parity, count=64, seed=731, weight=16, stim_mode=decomposed | 0.125887 | 0.642983 | 0.196 |
| efe63db280bb9fe5 | protocol | n=65, state=cluster, kind=parity, count=64, seed=731, weight=16, stim_mode=native_pauli | 0.115241 | 0.544404 | 0.212 |
| ae6472f4b408e1aa | protocol | n=65, state=cluster, kind=parity, count=1, seed=731, weight=64, stim_mode=decomposed | 0.004358 | 0.021533 | 0.202 |
| 3bdd0bf2b1873ee0 | protocol | n=65, state=cluster, kind=parity, count=1, seed=731, weight=64, stim_mode=native_pauli | 0.006993 | 0.028106 | 0.249 |
| 5d5ce7107513b3da | protocol | n=65, state=cluster, kind=parity, count=64, seed=731, weight=64, stim_mode=decomposed | 0.444687 | 1.772794 | 0.251 |
| 8eef569627dd446f | protocol | n=65, state=cluster, kind=parity, count=64, seed=731, weight=64, stim_mode=native_pauli | 0.334190 | 0.952994 | 0.351 |
| 4319ba9bfd0d6d39 | protocol | n=65, state=cluster, kind=parity, count=1, seed=731, weight=65, stim_mode=decomposed | 0.006294 | 0.029529 | 0.213 |
| a592ca61e122696b | protocol | n=65, state=cluster, kind=parity, count=1, seed=731, weight=65, stim_mode=native_pauli | 0.006969 | 0.027205 | 0.256 |
| 0daac769c3da39a2 | protocol | n=65, state=cluster, kind=parity, count=64, seed=731, weight=65, stim_mode=decomposed | 0.378144 | 1.709783 | 0.221 |
| 36a5c5b1d3abd220 | protocol | n=65, state=cluster, kind=parity, count=64, seed=731, weight=65, stim_mode=native_pauli | 0.408199 | 1.370556 | 0.298 |
| 38e299493c11003c | protocol | n=65, state=scrambled, kind=parity, count=1, seed=731, weight=2, stim_mode=decomposed | 0.002080 | 0.010067 | 0.207 |
| dffcb4396915b194 | protocol | n=65, state=scrambled, kind=parity, count=1, seed=731, weight=2, stim_mode=native_pauli | 0.002662 | 0.009688 | 0.275 |
| f019a207153054f2 | protocol | n=65, state=scrambled, kind=parity, count=64, seed=731, weight=2, stim_mode=decomposed | 0.073454 | 0.490075 | 0.150 |
| 189a6123a99137e8 | protocol | n=65, state=scrambled, kind=parity, count=64, seed=731, weight=2, stim_mode=native_pauli | 0.092803 | 0.252691 | 0.367 |
| 0525ff957d0d023d | protocol | n=65, state=scrambled, kind=parity, count=1, seed=731, weight=16, stim_mode=decomposed | 0.003963 | 0.014039 | 0.282 |
| 089b0d21b0b48bbd | protocol | n=65, state=scrambled, kind=parity, count=1, seed=731, weight=16, stim_mode=native_pauli | 0.005790 | 0.016426 | 0.352 |
| 539bd7daa6ed38eb | protocol | n=65, state=scrambled, kind=parity, count=64, seed=731, weight=16, stim_mode=decomposed | 0.130836 | 0.642267 | 0.204 |
| e684aafae51bd7a4 | protocol | n=65, state=scrambled, kind=parity, count=64, seed=731, weight=16, stim_mode=native_pauli | 0.127868 | 0.537843 | 0.238 |
| ec08ff31f5ecdf67 | protocol | n=65, state=scrambled, kind=parity, count=1, seed=731, weight=64, stim_mode=decomposed | 0.005166 | 0.023440 | 0.220 |
| 4eec68e12c689857 | protocol | n=65, state=scrambled, kind=parity, count=1, seed=731, weight=64, stim_mode=native_pauli | 0.010602 | 0.035866 | 0.296 |
| 99e5164395dea557 | protocol | n=65, state=scrambled, kind=parity, count=64, seed=731, weight=64, stim_mode=decomposed | 0.320420 | 1.313632 | 0.244 |
| 66f350798f671c53 | protocol | n=65, state=scrambled, kind=parity, count=64, seed=731, weight=64, stim_mode=native_pauli | 0.304182 | 0.950942 | 0.320 |
| 12e9294782830522 | protocol | n=65, state=scrambled, kind=parity, count=1, seed=731, weight=65, stim_mode=decomposed | 0.006293 | 0.026777 | 0.235 |
| f4f7d3e8d60dadcd | protocol | n=65, state=scrambled, kind=parity, count=1, seed=731, weight=65, stim_mode=native_pauli | 0.005662 | 0.018438 | 0.307 |
| bffd4619aa45452c | protocol | n=65, state=scrambled, kind=parity, count=64, seed=731, weight=65, stim_mode=decomposed | 0.390754 | 2.347663 | 0.166 |
| 663d431bc3d238a3 | protocol | n=65, state=scrambled, kind=parity, count=64, seed=731, weight=65, stim_mode=native_pauli | 0.392520 | 1.206964 | 0.325 |
| caa9070c31c9e615 | protocol | n=1024, state=ghz, kind=parity, count=1, seed=731, weight=2, stim_mode=decomposed | 0.011276 | 0.679598 | 0.017 |
| 34069c63d553bdbc | protocol | n=1024, state=ghz, kind=parity, count=1, seed=731, weight=2, stim_mode=native_pauli | 0.010384 | 0.570235 | 0.018 |
| 9c1f46aed789aeb5 | protocol | n=1024, state=ghz, kind=parity, count=64, seed=731, weight=2, stim_mode=decomposed | 0.836340 | 32.218928 | 0.026 |
| 97dbe40ba34a2653 | protocol | n=1024, state=ghz, kind=parity, count=64, seed=731, weight=2, stim_mode=native_pauli | 0.679578 | 2.070411 | 0.328 |
| da97569d29df824b | protocol | n=1024, state=ghz, kind=parity, count=1, seed=731, weight=16, stim_mode=decomposed | 0.012161 | 0.580719 | 0.021 |
| 75f7b9e57d2d9b16 | protocol | n=1024, state=ghz, kind=parity, count=1, seed=731, weight=16, stim_mode=native_pauli | 0.011638 | 0.507365 | 0.023 |
| b4304c3209aa5969 | protocol | n=1024, state=ghz, kind=parity, count=64, seed=731, weight=16, stim_mode=decomposed | 1.379469 | 39.323828 | 0.035 |
| 214a72d5cf33a188 | protocol | n=1024, state=ghz, kind=parity, count=64, seed=731, weight=16, stim_mode=native_pauli | 1.413710 | 15.273430 | 0.093 |
| e4241885bf614e19 | protocol | n=1024, state=ghz, kind=parity, count=1, seed=731, weight=64, stim_mode=decomposed | 0.031150 | 0.724164 | 0.043 |
| 0fbbfc19dc54daa1 | protocol | n=1024, state=ghz, kind=parity, count=1, seed=731, weight=64, stim_mode=native_pauli | 0.026928 | 0.651840 | 0.041 |
| 906fca08ee01b5f2 | protocol | n=1024, state=ghz, kind=parity, count=64, seed=731, weight=64, stim_mode=decomposed | 1.990464 | 38.413965 | 0.052 |
| 0b29c7749ab83cc7 | protocol | n=1024, state=ghz, kind=parity, count=64, seed=731, weight=64, stim_mode=native_pauli | 2.469881 | 41.468418 | 0.060 |
| fb351cd86bfef474 | protocol | n=1024, state=ghz, kind=parity, count=1, seed=731, weight=1024, stim_mode=decomposed | 0.172592 | 0.816343 | 0.211 |
| dbf53e4e10f78d73 | protocol | n=1024, state=ghz, kind=parity, count=1, seed=731, weight=1024, stim_mode=native_pauli | 0.191672 | 0.797037 | 0.240 |
| 4005a87a8d3172b9 | protocol | n=1024, state=ghz, kind=parity, count=64, seed=731, weight=1024, stim_mode=decomposed | 17.430496 | 59.110389 | 0.295 |
| 947355e8356a76f7 | protocol | n=1024, state=ghz, kind=parity, count=64, seed=731, weight=1024, stim_mode=native_pauli | 20.510550 | 53.750261 | 0.382 |
| 9c4bbaa8dbc4778a | protocol | n=1024, state=cluster, kind=parity, count=1, seed=731, weight=2, stim_mode=decomposed | 0.007986 | 0.484001 | 0.016 |
| 5ffc4918432aa878 | protocol | n=1024, state=cluster, kind=parity, count=1, seed=731, weight=2, stim_mode=native_pauli | 0.010543 | 0.597849 | 0.018 |
| 9d6bf21fc0897388 | protocol | n=1024, state=cluster, kind=parity, count=64, seed=731, weight=2, stim_mode=decomposed | 0.593323 | 37.330568 | 0.016 |
| 36d9931429475366 | protocol | n=1024, state=cluster, kind=parity, count=64, seed=731, weight=2, stim_mode=native_pauli | 0.586999 | 2.617737 | 0.224 |
| d2c541dcaf5e10fb | protocol | n=1024, state=cluster, kind=parity, count=1, seed=731, weight=16, stim_mode=decomposed | 0.015651 | 0.639752 | 0.024 |
| 65816eaffee83e47 | protocol | n=1024, state=cluster, kind=parity, count=1, seed=731, weight=16, stim_mode=native_pauli | 0.014763 | 0.633299 | 0.023 |
| ffcaf4eb881d7a4b | protocol | n=1024, state=cluster, kind=parity, count=64, seed=731, weight=16, stim_mode=decomposed | 0.787179 | 36.810720 | 0.021 |
| 0888a89fcb0a8451 | protocol | n=1024, state=cluster, kind=parity, count=64, seed=731, weight=16, stim_mode=native_pauli | 0.810853 | 16.038140 | 0.051 |
| d217f153976c9c4e | protocol | n=1024, state=cluster, kind=parity, count=1, seed=731, weight=64, stim_mode=decomposed | 0.022342 | 0.503517 | 0.044 |
| abbb42de7d93cf9e | protocol | n=1024, state=cluster, kind=parity, count=1, seed=731, weight=64, stim_mode=native_pauli | 0.021877 | 0.520968 | 0.042 |
| 64ef9fa79e1834b9 | protocol | n=1024, state=cluster, kind=parity, count=64, seed=731, weight=64, stim_mode=decomposed | 2.352131 | 41.260060 | 0.057 |
| 2adca8e6486eb42a | protocol | n=1024, state=cluster, kind=parity, count=64, seed=731, weight=64, stim_mode=native_pauli | 2.250708 | 37.674657 | 0.060 |
| b2115b8093bf74a0 | protocol | n=1024, state=cluster, kind=parity, count=1, seed=731, weight=1024, stim_mode=decomposed | 0.222791 | 1.046398 | 0.213 |
| 13e81f0bb9f0b3ca | protocol | n=1024, state=cluster, kind=parity, count=1, seed=731, weight=1024, stim_mode=native_pauli | 0.178570 | 0.710266 | 0.251 |
| c2c50df8f0fa4daa | protocol | n=1024, state=cluster, kind=parity, count=64, seed=731, weight=1024, stim_mode=decomposed | 23.992314 | 67.902491 | 0.353 |
| c83062ae11b4a8e8 | protocol | n=1024, state=cluster, kind=parity, count=64, seed=731, weight=1024, stim_mode=native_pauli | 17.672804 | 56.413417 | 0.313 |
| 06f55a23f1772715 | protocol | n=1024, state=scrambled, kind=parity, count=1, seed=731, weight=2, stim_mode=decomposed | 0.077919 | 0.515454 | 0.151 |
| 6a71f3a60475b788 | protocol | n=1024, state=scrambled, kind=parity, count=1, seed=731, weight=2, stim_mode=native_pauli | 0.090895 | 0.679364 | 0.134 |
| d4cb00512fb55dd2 | protocol | n=1024, state=scrambled, kind=parity, count=64, seed=731, weight=2, stim_mode=decomposed | 4.627008 | 38.019988 | 0.122 |
| 9929b145da08e0ac | protocol | n=1024, state=scrambled, kind=parity, count=64, seed=731, weight=2, stim_mode=native_pauli | 5.167751 | 3.986023 | 1.296 |
| 95ec56cadc1eb2f8 | protocol | n=1024, state=scrambled, kind=parity, count=1, seed=731, weight=16, stim_mode=decomposed | 0.082126 | 0.557469 | 0.147 |
| baef4689233d26de | protocol | n=1024, state=scrambled, kind=parity, count=1, seed=731, weight=16, stim_mode=native_pauli | 0.074780 | 0.779179 | 0.096 |
| 0a7b4270d5718ddb | protocol | n=1024, state=scrambled, kind=parity, count=64, seed=731, weight=16, stim_mode=decomposed | 1.720232 | 40.382912 | 0.043 |
| 338953208a3669ac | protocol | n=1024, state=scrambled, kind=parity, count=64, seed=731, weight=16, stim_mode=native_pauli | 1.754645 | 17.666747 | 0.099 |
| 3c5335691f2866a0 | protocol | n=1024, state=scrambled, kind=parity, count=1, seed=731, weight=64, stim_mode=decomposed | 0.097861 | 0.667975 | 0.147 |
| 8d0740409c3c8914 | protocol | n=1024, state=scrambled, kind=parity, count=1, seed=731, weight=64, stim_mode=native_pauli | 0.070948 | 0.487029 | 0.146 |
| aadc4c0325102358 | protocol | n=1024, state=scrambled, kind=parity, count=64, seed=731, weight=64, stim_mode=decomposed | 2.430932 | 41.421283 | 0.059 |
| aa200a2fe1030b91 | protocol | n=1024, state=scrambled, kind=parity, count=64, seed=731, weight=64, stim_mode=native_pauli | 2.773070 | 40.438659 | 0.069 |
| e9a565efcc808820 | protocol | n=1024, state=scrambled, kind=parity, count=1, seed=731, weight=1024, stim_mode=decomposed | 0.265348 | 1.033600 | 0.257 |
| 441212543fc25b39 | protocol | n=1024, state=scrambled, kind=parity, count=1, seed=731, weight=1024, stim_mode=native_pauli | 0.255827 | 0.724780 | 0.353 |
| 0c53ba1ed912f6fb | protocol | n=1024, state=scrambled, kind=parity, count=64, seed=731, weight=1024, stim_mode=decomposed | 20.192906 | 62.541753 | 0.323 |
| b7640468e271d8d7 | protocol | n=1024, state=scrambled, kind=parity, count=64, seed=731, weight=1024, stim_mode=native_pauli | 17.258217 | 50.724117 | 0.340 |
| 3986ccdc2724a2d8 | protocol | n=65, state=zero, kind=interleaved, count=128, seed=731, layer=1, locality=random | 0.032808 | 0.141156 | 0.232 |
| 1eeec9b6d4a0b62a | protocol | n=65, state=zero, kind=interleaved, count=128, seed=731, layer=1, locality=local | 0.039782 | 0.143590 | 0.277 |
| 3cae8d8414606c87 | protocol | n=65, state=zero, kind=interleaved, count=128, seed=731, layer=8, locality=random | 0.103301 | 0.609976 | 0.169 |
| 34be646018a935fc | protocol | n=65, state=zero, kind=interleaved, count=128, seed=731, layer=8, locality=local | 0.071781 | 0.523720 | 0.137 |
| cc07d160ecf10ce1 | protocol | n=65, state=zero, kind=interleaved, count=128, seed=731, layer=64, locality=random | 0.292692 | 1.232703 | 0.237 |
| 7575f7b142d991d6 | protocol | n=65, state=zero, kind=interleaved, count=128, seed=731, layer=64, locality=local | 0.177466 | 1.009970 | 0.176 |
| 8c72b5590245f33b | protocol | n=65, state=cluster, kind=interleaved, count=128, seed=731, layer=1, locality=random | 0.052725 | 0.432949 | 0.122 |
| 5e631fbf34e8bf63 | protocol | n=65, state=cluster, kind=interleaved, count=128, seed=731, layer=1, locality=local | 0.083997 | 0.688580 | 0.122 |
| 1cc5c9f08b28e510 | protocol | n=65, state=cluster, kind=interleaved, count=128, seed=731, layer=8, locality=random | 0.184071 | 1.009427 | 0.182 |
| cee931ce16766a9e | protocol | n=65, state=cluster, kind=interleaved, count=128, seed=731, layer=8, locality=local | 0.093107 | 0.756621 | 0.123 |
| e34f23fec263ff17 | protocol | n=65, state=cluster, kind=interleaved, count=128, seed=731, layer=64, locality=random | 0.421496 | 1.867583 | 0.226 |
| 0df93c56caa055b3 | protocol | n=65, state=cluster, kind=interleaved, count=128, seed=731, layer=64, locality=local | 0.278024 | 1.604950 | 0.173 |
| 55829f4ba5c76dbc | protocol | n=65, state=scrambled, kind=interleaved, count=128, seed=731, layer=1, locality=random | 0.108743 | 0.520635 | 0.209 |
| 014816a9aaba79c5 | protocol | n=65, state=scrambled, kind=interleaved, count=128, seed=731, layer=1, locality=local | 0.122365 | 0.546253 | 0.224 |
| fc06f555561675ce | protocol | n=65, state=scrambled, kind=interleaved, count=128, seed=731, layer=8, locality=random | 0.160656 | 0.848347 | 0.189 |
| 3a9d133ae3804682 | protocol | n=65, state=scrambled, kind=interleaved, count=128, seed=731, layer=8, locality=local | 0.232168 | 1.531257 | 0.152 |
| b5e1237f1ab3421f | protocol | n=65, state=scrambled, kind=interleaved, count=128, seed=731, layer=64, locality=random | 0.351248 | 1.498717 | 0.234 |
| 8191f6706eeb076d | protocol | n=65, state=scrambled, kind=interleaved, count=128, seed=731, layer=64, locality=local | 0.446756 | 2.266460 | 0.197 |
| 77521f3a977b06e5 | protocol | n=256, state=zero, kind=interleaved, count=128, seed=731, layer=1, locality=random | 0.093930 | 0.547011 | 0.172 |
| 454b1419ea6d3d42 | protocol | n=256, state=zero, kind=interleaved, count=128, seed=731, layer=1, locality=local | 0.094761 | 0.873016 | 0.109 |
| fade1780e83bb7ba | protocol | n=256, state=zero, kind=interleaved, count=128, seed=731, layer=8, locality=random | 0.231638 | 3.313263 | 0.070 |
| 473f94899f9f6c24 | protocol | n=256, state=zero, kind=interleaved, count=128, seed=731, layer=8, locality=local | 0.161838 | 3.027553 | 0.053 |
| 5cbfe5c76dab1dd4 | protocol | n=256, state=zero, kind=interleaved, count=128, seed=731, layer=64, locality=random | 0.736501 | 7.556078 | 0.097 |
| f8c40ea2cb485644 | protocol | n=256, state=zero, kind=interleaved, count=128, seed=731, layer=64, locality=local | 0.436429 | 8.050333 | 0.054 |
| 8b3bfcd0345c7714 | protocol | n=256, state=cluster, kind=interleaved, count=128, seed=731, layer=1, locality=random | 0.172121 | 6.834323 | 0.025 |
| 26acbfec75de470a | protocol | n=256, state=cluster, kind=interleaved, count=128, seed=731, layer=1, locality=local | 0.163740 | 6.372892 | 0.026 |
| 5cc525d53d6b71c7 | protocol | n=256, state=cluster, kind=interleaved, count=128, seed=731, layer=8, locality=random | 0.263956 | 8.102088 | 0.033 |
| 39d07b6a16229a25 | protocol | n=256, state=cluster, kind=interleaved, count=128, seed=731, layer=8, locality=local | 0.208661 | 7.468898 | 0.028 |
| 0aa50c370153c5f0 | protocol | n=256, state=cluster, kind=interleaved, count=128, seed=731, layer=64, locality=random | 0.963584 | 9.312786 | 0.103 |
| b37801fa3d06e801 | protocol | n=256, state=cluster, kind=interleaved, count=128, seed=731, layer=64, locality=local | 0.343596 | 7.995336 | 0.043 |
| 5a2b967f671be84b | protocol | n=256, state=scrambled, kind=interleaved, count=128, seed=731, layer=1, locality=random | 0.926873 | 6.992889 | 0.133 |
| a36fba1f0c7cabdc | protocol | n=256, state=scrambled, kind=interleaved, count=128, seed=731, layer=1, locality=local | 0.919349 | 7.075710 | 0.130 |
| 19e43ad47db861e8 | protocol | n=256, state=scrambled, kind=interleaved, count=128, seed=731, layer=8, locality=random | 1.015324 | 9.085175 | 0.112 |
| 7df3d02f43c248d2 | protocol | n=256, state=scrambled, kind=interleaved, count=128, seed=731, layer=8, locality=local | 0.988393 | 7.836585 | 0.126 |
| cad35beb1d5f8be4 | protocol | n=256, state=scrambled, kind=interleaved, count=128, seed=731, layer=64, locality=random | 1.191407 | 9.018878 | 0.132 |
| f6cc20166e75c2e1 | protocol | n=256, state=scrambled, kind=interleaved, count=128, seed=731, layer=64, locality=local | 1.057917 | 10.839255 | 0.098 |
| 13499ee780c83c65 | protocol | n=1024, state=zero, kind=interleaved, count=128, seed=731, layer=1, locality=random | 0.135068 | 2.167224 | 0.062 |
| 43dea46fe1611dfa | protocol | n=1024, state=zero, kind=interleaved, count=128, seed=731, layer=1, locality=local | 0.113237 | 2.442656 | 0.046 |
| c16efe6569578f47 | protocol | n=1024, state=zero, kind=interleaved, count=128, seed=731, layer=8, locality=random | 0.732622 | 11.471144 | 0.064 |
| 8dc8e96c76788ef4 | protocol | n=1024, state=zero, kind=interleaved, count=128, seed=731, layer=8, locality=local | 0.389793 | 9.190288 | 0.042 |
| 168b896f1df13aa1 | protocol | n=1024, state=zero, kind=interleaved, count=128, seed=731, layer=64, locality=random | 1.258512 | 49.711171 | 0.025 |
| 7801637fd4463247 | protocol | n=1024, state=zero, kind=interleaved, count=128, seed=731, layer=64, locality=local | 0.871152 | 32.242942 | 0.027 |
| b9fea14f5ba81f26 | protocol | n=1024, state=cluster, kind=interleaved, count=128, seed=731, layer=1, locality=random | 0.436674 | 34.383872 | 0.013 |
| cdad1bb3bae13adb | protocol | n=1024, state=cluster, kind=interleaved, count=128, seed=731, layer=1, locality=local | 0.408441 | 29.338093 | 0.014 |
| 2fbf1de39e018d67 | protocol | n=1024, state=cluster, kind=interleaved, count=128, seed=731, layer=8, locality=random | 0.430868 | 29.158754 | 0.015 |
| 0d532a89f3d8938f | protocol | n=1024, state=cluster, kind=interleaved, count=128, seed=731, layer=8, locality=local | 0.431616 | 29.500708 | 0.015 |
| 28a729366fc8bddf | protocol | n=1024, state=cluster, kind=interleaved, count=128, seed=731, layer=64, locality=random | 0.930354 | 37.994154 | 0.024 |
| c52c4a056c2c198f | protocol | n=1024, state=cluster, kind=interleaved, count=128, seed=731, layer=64, locality=local | 0.661411 | 31.872070 | 0.021 |
| 8c8f471c44f01c73 | protocol | n=1024, state=scrambled, kind=interleaved, count=128, seed=731, layer=1, locality=random | 3.969845 | 31.476238 | 0.126 |
| 0a30d84bde4f0f00 | protocol | n=1024, state=scrambled, kind=interleaved, count=128, seed=731, layer=1, locality=local | 4.090161 | 32.705684 | 0.125 |
| d2170fc7023b75fe | protocol | n=1024, state=scrambled, kind=interleaved, count=128, seed=731, layer=8, locality=random | 4.822295 | 38.821511 | 0.124 |
| 2eb31eb67183c1e1 | protocol | n=1024, state=scrambled, kind=interleaved, count=128, seed=731, layer=8, locality=local | 3.828721 | 31.642012 | 0.121 |
| 81feff2834195b66 | protocol | n=1024, state=scrambled, kind=interleaved, count=128, seed=731, layer=64, locality=random | 3.871851 | 30.308483 | 0.128 |
| c6fc56007d74d2bd | protocol | n=1024, state=scrambled, kind=interleaved, count=128, seed=731, layer=64, locality=local | 5.020532 | 43.192106 | 0.116 |
| 029d894a128f51b6 | protocol | n=4096, state=zero, kind=interleaved, count=128, seed=731, layer=1, locality=random | 0.301030 | 0.064293 | 4.682 |
| 99f2a279700621e6 | protocol | n=4096, state=zero, kind=interleaved, count=128, seed=731, layer=1, locality=local | 0.052435 | 0.025541 | 2.053 |
| f4da71e82a91c40f | protocol | n=4096, state=zero, kind=interleaved, count=128, seed=731, layer=8, locality=random | 1.161123 | 27.987335 | 0.041 |
| 067b9445de6b2137 | protocol | n=4096, state=zero, kind=interleaved, count=128, seed=731, layer=8, locality=local | 1.300861 | 45.354007 | 0.029 |
| 0b5f8f2a815b117a | protocol | n=4096, state=zero, kind=interleaved, count=128, seed=731, layer=64, locality=random | 8.082959 | 451.739117 | 0.018 |
| 9260f6ee80761a7e | protocol | n=4096, state=zero, kind=interleaved, count=128, seed=731, layer=64, locality=local | 5.037656 | 390.894198 | 0.013 |
| 881825f626e8a0b6 | protocol | n=4096, state=cluster, kind=interleaved, count=128, seed=731, layer=1, locality=random | 10.985067 | 1627.325118 | 0.007 |
| 5bdaa400c1af1d3b | protocol | n=4096, state=cluster, kind=interleaved, count=128, seed=731, layer=1, locality=local | 4.600501 | 573.878939 | 0.008 |
| 42e1e0501f8ab51a | protocol | n=4096, state=cluster, kind=interleaved, count=128, seed=731, layer=8, locality=random | 4.834578 | 606.753892 | 0.008 |
| c69b0bc6f8cfbe68 | protocol | n=4096, state=cluster, kind=interleaved, count=128, seed=731, layer=8, locality=local | 5.030656 | 629.513140 | 0.008 |
| a0e8783c50827e9a | protocol | n=4096, state=cluster, kind=interleaved, count=128, seed=731, layer=64, locality=random | 5.871172 | 646.677629 | 0.009 |
| 4bf512d37d4550b6 | protocol | n=4096, state=cluster, kind=interleaved, count=128, seed=731, layer=64, locality=local | 5.357604 | 628.230580 | 0.009 |
| 922b28c9c6ab1008 | protocol | n=4096, state=scrambled, kind=interleaved, count=128, seed=731, layer=1, locality=random | 61.990907 | 580.278699 | 0.107 |
| 01f8cf3eab777d2c | protocol | n=4096, state=scrambled, kind=interleaved, count=128, seed=731, layer=1, locality=local | 64.975851 | 600.428123 | 0.108 |
| 66e627b28965bab6 | protocol | n=4096, state=scrambled, kind=interleaved, count=128, seed=731, layer=8, locality=random | 62.316211 | 650.842047 | 0.096 |
| 2d7f2ded38735ed2 | protocol | n=4096, state=scrambled, kind=interleaved, count=128, seed=731, layer=8, locality=local | 60.063708 | 650.004371 | 0.092 |
| c319429b87fa9c40 | protocol | n=4096, state=scrambled, kind=interleaved, count=128, seed=731, layer=64, locality=random | 64.658841 | 691.193785 | 0.094 |
| 53e124ecf091303d | protocol | n=4096, state=scrambled, kind=interleaved, count=128, seed=731, layer=64, locality=local | 62.378521 | 646.144769 | 0.097 |
| 87f318aab885a64b | protocol | n=65, state=plus, kind=repeated, count=64, seed=731 | 0.003368 | 0.007786 | 0.433 |
| 2115bfcdbfa6c242 | protocol | n=65, state=plus, kind=repeated, count=1024, seed=731 | 0.040728 | 0.065342 | 0.623 |
| 278e4480d7e2fa99 | protocol | n=65, state=scrambled, kind=repeated, count=64, seed=731 | 0.004041 | 0.008289 | 0.488 |
| 5e73eaeb438f570d | protocol | n=65, state=scrambled, kind=repeated, count=1024, seed=731 | 0.045166 | 0.068186 | 0.662 |
| 0575f4e308c2ca1c | protocol | n=1024, state=plus, kind=repeated, count=64, seed=731 | 0.010360 | 0.282974 | 0.037 |
| 04b61a2e0fb4e0b7 | protocol | n=1024, state=plus, kind=repeated, count=1024, seed=731 | 0.097638 | 1.071071 | 0.091 |
| 04f51296112640a8 | protocol | n=1024, state=scrambled, kind=repeated, count=64, seed=731 | 0.043565 | 0.309975 | 0.141 |
| 5b81219a2bf28f10 | protocol | n=1024, state=scrambled, kind=repeated, count=1024, seed=731 | 0.136271 | 1.094526 | 0.125 |
| 4d17dd05b42affd3 | protocol | n=4096, state=plus, kind=repeated, count=64, seed=731 | 0.071037 | 4.395658 | 0.016 |
| 39bfb9b633922b2f | protocol | n=4096, state=plus, kind=repeated, count=1024, seed=731 | 0.300154 | 8.912968 | 0.034 |
| f3d3fbe8e4b26d46 | protocol | n=4096, state=scrambled, kind=repeated, count=64, seed=731 | 0.518213 | 4.690728 | 0.110 |
| 5549553cad36fa02 | protocol | n=4096, state=scrambled, kind=repeated, count=1024, seed=731 | 0.734066 | 9.016318 | 0.081 |
| 4414d69409683d08 | lifecycle | n=1, kind=initialize, seed=731 | 0.000324 | 0.001270 | 0.255 |
| da1119c9b3e143db | lifecycle | n=1, kind=copy, seed=731 | 0.000425 | 0.001627 | 0.261 |
| 0a987ea75b6e2d59 | lifecycle | n=1, kind=peek, seed=731 | 0.000431 | 0.000594 | 0.726 |
| 6a38cd5944b59f42 | lifecycle | n=1, kind=export_cold, seed=731 | 0.003328 | 0.006799 | 0.489 |
| 3bd4a49ea2a757ae | lifecycle | n=1, kind=export_warm, seed=731 | 0.001975 | 0.005644 | 0.350 |
| 8d63622f4be11a22 | lifecycle | n=65, kind=initialize, seed=731 | 0.000469 | 0.001351 | 0.347 |
| 4786a349fec6d92c | lifecycle | n=65, kind=copy, seed=731 | 0.000230 | 0.000918 | 0.251 |
| e9afead39f0613ff | lifecycle | n=65, kind=peek, seed=731 | 0.003195 | 0.006961 | 0.459 |
| c5f44c755b98da99 | lifecycle | n=65, kind=export_cold, seed=731 | 0.263138 | 0.077181 | 3.409 |
| 3cff368d4a40a548 | lifecycle | n=65, kind=export_warm, seed=731 | 0.251106 | 0.076693 | 3.274 |
| 70658bcf9d9b1586 | lifecycle | n=1024, kind=initialize, seed=731 | 0.008906 | 0.011583 | 0.769 |
| 44ff20a828b1461f | lifecycle | n=1024, kind=copy, seed=731 | 0.022601 | 0.040991 | 0.551 |
| 6253b61784464ede | lifecycle | n=1024, kind=peek, seed=731 | 0.003212 | 0.008697 | 0.369 |
| eea19552b5c52c96 | lifecycle | n=1024, kind=export_cold, seed=731 | 67.299081 | 13.932311 | 4.830 |
| 9fe73b66a6508ea0 | lifecycle | n=1024, kind=export_warm, seed=731 | 67.143457 | 13.043870 | 5.148 |
| 0368a942ec282cd3 | lifecycle | n=4096, kind=initialize, seed=731 | 0.339206 | 0.427223 | 0.794 |
| acf81a23aa1f0948 | lifecycle | n=4096, kind=copy, seed=731 | 3.565479 | 5.608893 | 0.636 |
| 2b1e67383693cdbb | lifecycle | n=4096, kind=peek, seed=731 | 0.009917 | 0.017084 | 0.580 |
| d59794667a566e80 | lifecycle | n=16384, kind=initialize, seed=731 | 35.604616 | 61.089074 | 0.583 |
| aebc8dd4bc721d70 | lifecycle | n=16384, kind=copy, seed=731 | 49.435841 | 68.991472 | 0.717 |
| 179baa775ff62c07 | lifecycle | n=16384, kind=peek, seed=731 | 0.028302 | 0.044877 | 0.631 |
| 585ad884fa6a4492 | shots | n=65, state=cluster, shots=1, seed=731 | 0.012290 | 0.014947 | 0.822 |
| ff11ebed715f91b5 | shots | n=65, state=cluster, shots=32, seed=731 | 0.350666 | 0.450849 | 0.778 |
| 74ad9f29ad6e4db4 | shots | n=65, state=scrambled, shots=1, seed=731 | 0.055286 | 0.108322 | 0.510 |
| dda6eaaa00463990 | shots | n=65, state=scrambled, shots=32, seed=731 | 1.310053 | 3.263291 | 0.401 |
| d0db088f494ef212 | shots | n=256, state=cluster, shots=1, seed=731 | 0.134473 | 0.124798 | 1.078 |
| 77ac77359c5742f6 | shots | n=256, state=cluster, shots=32, seed=731 | 4.363565 | 3.359700 | 1.299 |
| d572d0f02cf919fb | shots | n=256, state=scrambled, shots=1, seed=731 | 0.838545 | 0.894783 | 0.937 |
| 509adb24d2766729 | shots | n=256, state=scrambled, shots=32, seed=731 | 26.392170 | 28.867063 | 0.914 |
| f909089b7ac45eda | shots | n=1024, state=cluster, shots=1, seed=731 | 3.221469 | 1.261414 | 2.554 |
| 928d0618a9e33fdc | shots | n=1024, state=cluster, shots=32, seed=731 | 108.072903 | 42.680233 | 2.532 |
| e6760a34c20c4bb3 | shots | n=1024, state=scrambled, shots=1, seed=731 | 24.146784 | 17.655007 | 1.368 |
| 67cf8d9a01013cbd | shots | n=1024, state=scrambled, shots=32, seed=731 | 760.335528 | 575.723044 | 1.321 |

## Isolated memory/depth probes

| Mode | Backend | Qubits | Status | Peak RSS GiB | Gate time s | Projection s |
|---|---|---:|---|---:|---:|---:|
| gates | stabilizer | 8192 | success | 0.056 | 0.002389 | — |
| gates | stim | 8192 | success | 0.067 | 0.004007 | — |
| gates | stim | 16384 | success | 0.180 | 0.009564 | — |
| gates | stabilizer | 16384 | success | 0.150 | 0.007119 | — |
| gates | stabilizer | 32768 | success | 0.526 | 0.014070 | — |
| gates | stim | 32768 | success | 0.635 | 0.018112 | — |
| gates | stim | 65536 | success | 2.456 | 0.032080 | — |
| gates | stabilizer | 65536 | success | 2.029 | 0.026995 | — |
| gates | stabilizer | 98304 | success | 4.532 | 0.037637 | — |
| gates | stim | 98304 | success | 5.477 | 0.044352 | — |
| gates | stim | 126976 | success | 9.127 | 0.055237 | — |
| gates | stabilizer | 126976 | success | 7.542 | 0.047138 | — |
| gates | stabilizer | 131072 | success | 8.034 | 0.056116 | — |
| gates | stim | 131072 | success | 9.720 | 0.057009 | — |
| gates | stim | 139264 | success | 10.962 | 0.059706 | — |
| gates | stabilizer | 139264 | success | 9.066 | 0.050334 | — |
| gates | stabilizer | 141312 | success | 9.334 | 0.061211 | — |
| gates | stim | 141312 | success | 11.293 | 0.059840 | — |
| gates | stabilizer | 155648 | success | 11.317 | 0.070043 | — |
| measurement | stabilizer | 8192 | success | 0.056 | 0.002434 | 0.000244 |
| measurement | stim | 8192 | success | 0.067 | 0.004834 | 0.030167 |
| measurement | stim | 16384 | success | 0.180 | 0.010617 | 0.156124 |
| measurement | stabilizer | 16384 | success | 0.150 | 0.009200 | 0.000701 |
| measurement | stabilizer | 32768 | success | 0.526 | 0.017010 | 0.001914 |
| measurement | stim | 32768 | success | 0.636 | 0.019175 | 0.664821 |
| measurement | stim | 65536 | success | 2.456 | 0.032493 | 2.641392 |
| measurement | stabilizer | 65536 | success | 2.029 | 0.025804 | 0.002925 |
| measurement | stabilizer | 98304 | success | 4.532 | 0.036370 | 0.004560 |
| measurement | stim | 98304 | success | 5.477 | 0.044844 | 6.294430 |
| measurement | stim | 128000 | success | 9.261 | 0.062070 | 11.475461 |
| measurement | stabilizer | 128000 | success | 7.663 | 0.048871 | 0.006833 |
| measurement | stabilizer | 131072 | success | 8.034 | 0.048963 | 0.006633 |
| measurement | stim | 131072 | success | 9.720 | 0.057943 | 12.342854 |
| measurement | stim | 140288 | success | 11.127 | 0.064478 | 15.102623 |
| measurement | stabilizer | 140288 | success | 9.200 | 0.049272 | 0.008351 |
| measurement | stabilizer | 142336 | success | 9.469 | 0.049351 | 0.009310 |
| measurement | stim | 141312 | success | 11.293 | 0.062860 | 15.545531 |
| measurement | stim | 142336 | success | 11.460 | 0.062688 | 16.178030 |
| measurement | stabilizer | 156672 | success | 11.467 | 0.071487 | 0.011133 |
| gates | stabilizer | 100000 | success | 4.746 | 3.727354 | — |
| gates | stim | 100000 | success | 5.790 | 4.508815 | — |

Raw records: [results.json](results.json), [results.jsonl](results.jsonl). Exact matrix and machine/build metadata: [manifest.json](manifest.json). Per-case JSON and tracebacks are in `cases/`; stress worker reports and logs are adjacent to this summary.

Resume completed: 19 October 2026, 01:55:35 IST. All 59 remaining cases finished; the first 1,031 result records were verified unchanged using canonical JSON checksums. The full suite has 1,090 successful case records. Capacity probes retain their individual success/budget status in the raw reports.
