# Benchmark overview

1090 successful cases of 1090 planned; 0 case errors.

Intel(R) Core(TM) Ultra 7 255H; Stim 1.16.0 (stim._stim_sse2); local release extension.

Medians of 5 timed samples after warmup. Preparation and validation are excluded except where an end-to-end lifecycle/shot scope is specified. Backends run sequentially. Full case parameters and all raw samples are retained.

## 50,000 gates at 4,096 qubits

Times are total milliseconds; ratio >1 means Stim is faster.

| Parameters | Ours ms | Stim ms | Ours / Stim |
|---|---:|---:|---:|
| workload=h, count=50000, prep=0, locality=random | 3.662 | 4.377 | 0.84 |
| workload=h, count=50000, prep=163840, locality=random | 3.451 | 4.056 | 0.85 |
| workload=s, count=50000, prep=0, locality=random | 3.271 | 4.694 | 0.70 |
| workload=s, count=50000, prep=163840, locality=random | 3.030 | 4.330 | 0.70 |
| workload=cx, count=50000, prep=0, locality=random | 5.412 | 8.940 | 0.61 |
| workload=cx, count=50000, prep=163840, locality=random | 4.930 | 8.264 | 0.60 |
| workload=mixed, count=50000, prep=0, locality=random | 4.021 | 7.263 | 0.55 |
| workload=mixed, count=50000, prep=163840, locality=random | 4.110 | 6.567 | 0.63 |
| workload=cx, count=50000, prep=163840, locality=local | 4.938 | 8.187 | 0.60 |
| workload=cx, count=50000, prep=163840, locality=hotspot | 1.876 | 3.124 | 0.60 |
| workload=mixed, count=50000, prep=163840, locality=local | 4.187 | 6.812 | 0.61 |
| workload=mixed, count=50000, prep=163840, locality=hotspot | 1.722 | 2.682 | 0.64 |

## Z measurement at 4,096 qubits

Times are total milliseconds; ratio >1 means Stim is faster.

| Parameters | Ours ms | Stim ms | Ours / Stim |
|---|---:|---:|---:|
| state=zero, count=1, mode=native | 0.002 | 0.004 | 0.68 |
| state=zero, count=1, mode=sequential | 0.004 | 0.005 | 0.79 |
| state=zero, count=64, mode=native | 0.019 | 0.009 | 2.03 |
| state=zero, count=64, mode=sequential | 0.020 | 0.011 | 1.75 |
| state=zero, count=256, mode=native | 0.084 | 0.045 | 1.86 |
| state=zero, count=256, mode=sequential | 0.074 | 0.036 | 2.04 |
| state=zero, count=4096, mode=native | 0.986 | 0.270 | 3.66 |
| state=zero, count=4096, mode=sequential | 1.056 | 0.475 | 2.22 |
| state=plus, count=1, mode=native | 0.045 | 3.998 | 0.01 |
| state=plus, count=1, mode=sequential | 0.045 | 3.911 | 0.01 |
| state=plus, count=64, mode=native | 2.194 | 4.264 | 0.51 |
| state=plus, count=64, mode=sequential | 2.233 | 268.434 | 0.01 |
| state=plus, count=256, mode=native | 8.960 | 5.501 | 1.63 |
| state=plus, count=256, mode=sequential | 9.250 | 1131.939 | 0.01 |
| state=plus, count=4096, mode=native | 147.937 | 22.569 | 6.55 |
| state=plus, count=4096, mode=sequential | 141.620 | 18599.047 | 0.01 |
| state=bell, count=1, mode=native | 0.044 | 3.934 | 0.01 |
| state=bell, count=1, mode=sequential | 0.042 | 3.961 | 0.01 |
| state=bell, count=64, mode=native | 2.187 | 4.179 | 0.52 |
| state=bell, count=64, mode=sequential | 2.350 | 263.349 | 0.01 |
| state=bell, count=256, mode=native | 9.222 | 5.444 | 1.69 |
| state=bell, count=256, mode=sequential | 9.039 | 1083.180 | 0.01 |
| state=bell, count=4096, mode=native | 121.062 | 21.274 | 5.69 |
| state=bell, count=4096, mode=sequential | 115.929 | 9080.458 | 0.01 |
| state=ghz, count=1, mode=native | 0.078 | 5.589 | 0.01 |
| state=ghz, count=1, mode=sequential | 0.076 | 4.247 | 0.02 |
| state=ghz, count=64, mode=native | 6.453 | 5.094 | 1.27 |
| state=ghz, count=64, mode=sequential | 7.293 | 5.828 | 1.25 |
| state=ghz, count=256, mode=native | 26.013 | 6.794 | 3.83 |
| state=ghz, count=256, mode=sequential | 26.653 | 4.591 | 5.81 |
| state=ghz, count=4096, mode=native | 402.219 | 22.091 | 18.21 |
| state=ghz, count=4096, mode=sequential | 434.779 | 5.131 | 84.74 |
| state=cluster, count=1, mode=native | 0.046 | 4.261 | 0.01 |
| state=cluster, count=1, mode=sequential | 0.047 | 4.248 | 0.01 |
| state=cluster, count=64, mode=native | 2.580 | 4.933 | 0.52 |
| state=cluster, count=64, mode=sequential | 2.387 | 307.289 | 0.01 |
| state=cluster, count=256, mode=native | 9.083 | 5.171 | 1.76 |
| state=cluster, count=256, mode=sequential | 9.257 | 1225.978 | 0.01 |
| state=cluster, count=4096, mode=native | 140.641 | 23.021 | 6.11 |
| state=cluster, count=4096, mode=sequential | 130.121 | 17748.474 | 0.01 |
| state=scrambled, count=1, mode=native | 0.451 | 4.184 | 0.11 |
| state=scrambled, count=1, mode=sequential | 0.444 | 4.334 | 0.10 |
| state=scrambled, count=64, mode=native | 30.498 | 19.306 | 1.58 |
| state=scrambled, count=64, mode=sequential | 28.335 | 278.850 | 0.10 |
| state=scrambled, count=256, mode=native | 117.598 | 56.286 | 2.09 |
| state=scrambled, count=256, mode=sequential | 117.803 | 1107.897 | 0.11 |
| state=scrambled, count=4096, mode=native | 1355.792 | 886.536 | 1.53 |
| state=scrambled, count=4096, mode=sequential | 1279.402 | 18290.201 | 0.07 |
| state=deterministic, count=1, mode=native | 1.000 | 0.030 | 33.41 |
| state=deterministic, count=1, mode=sequential | 0.945 | 0.018 | 53.20 |
| state=deterministic, count=64, mode=native | 66.548 | 0.040 | 1683.47 |
| state=deterministic, count=64, mode=sequential | 66.076 | 0.041 | 1595.58 |
| state=deterministic, count=256, mode=native | 260.406 | 0.059 | 4398.90 |
| state=deterministic, count=256, mode=sequential | 264.691 | 0.088 | 3018.90 |
| state=deterministic, count=4096, mode=native | 4228.929 | 0.345 | 12271.21 |
| state=deterministic, count=4096, mode=sequential | 4277.851 | 0.616 | 6943.67 |

## X/Y measurements and parity at 1,024 qubits

Times are total milliseconds; ratio >1 means Stim is faster.

| Parameters | Ours ms | Stim ms | Ours / Stim |
|---|---:|---:|---:|
| state=zero, kind=basis, count=64, basis=X, stim_mode=decomposed | 0.210 | 15.160 | 0.01 |
| state=zero, kind=basis, count=64, basis=X, stim_mode=native_pauli | 0.209 | 0.305 | 0.68 |
| state=zero, kind=basis, count=64, basis=Y, stim_mode=decomposed | 0.219 | 14.937 | 0.01 |
| state=zero, kind=basis, count=64, basis=Y, stim_mode=native_pauli | 0.218 | 0.296 | 0.74 |
| state=plus, kind=basis, count=64, basis=X, stim_mode=decomposed | 0.009 | 0.004 | 2.21 |
| state=plus, kind=basis, count=64, basis=X, stim_mode=native_pauli | 0.008 | 0.007 | 1.13 |
| state=plus, kind=basis, count=64, basis=Y, stim_mode=decomposed | 0.215 | 15.230 | 0.01 |
| state=plus, kind=basis, count=64, basis=Y, stim_mode=native_pauli | 0.212 | 0.296 | 0.71 |
| state=bell, kind=basis, count=64, basis=X, stim_mode=decomposed | 0.216 | 14.954 | 0.01 |
| state=bell, kind=basis, count=64, basis=X, stim_mode=native_pauli | 0.212 | 0.295 | 0.72 |
| state=bell, kind=basis, count=64, basis=Y, stim_mode=decomposed | 0.276 | 15.795 | 0.02 |
| state=bell, kind=basis, count=64, basis=Y, stim_mode=native_pauli | 0.215 | 0.300 | 0.72 |
| state=ghz, kind=basis, count=64, basis=X, stim_mode=decomposed | 0.224 | 15.591 | 0.01 |
| state=ghz, kind=basis, count=64, basis=X, stim_mode=native_pauli | 0.218 | 0.308 | 0.71 |
| state=ghz, kind=basis, count=64, basis=Y, stim_mode=decomposed | 0.444 | 18.992 | 0.02 |
| state=ghz, kind=basis, count=64, basis=Y, stim_mode=native_pauli | 0.223 | 0.300 | 0.74 |
| state=cluster, kind=basis, count=64, basis=X, stim_mode=decomposed | 0.218 | 14.987 | 0.01 |
| state=cluster, kind=basis, count=64, basis=X, stim_mode=native_pauli | 0.206 | 0.300 | 0.69 |
| state=cluster, kind=basis, count=64, basis=Y, stim_mode=decomposed | 0.264 | 16.000 | 0.02 |
| state=cluster, kind=basis, count=64, basis=Y, stim_mode=native_pauli | 0.237 | 0.322 | 0.74 |
| state=scrambled, kind=basis, count=64, basis=X, stim_mode=decomposed | 2.027 | 15.977 | 0.13 |
| state=scrambled, kind=basis, count=64, basis=X, stim_mode=native_pauli | 2.116 | 1.211 | 1.75 |
| state=scrambled, kind=basis, count=64, basis=Y, stim_mode=decomposed | 1.986 | 17.870 | 0.11 |
| state=scrambled, kind=basis, count=64, basis=Y, stim_mode=native_pauli | 2.018 | 1.212 | 1.67 |
| state=deterministic, kind=basis, count=64, basis=X, stim_mode=decomposed | 0.850 | 16.992 | 0.05 |
| state=deterministic, kind=basis, count=64, basis=X, stim_mode=native_pauli | 0.834 | 1.133 | 0.74 |
| state=deterministic, kind=basis, count=64, basis=Y, stim_mode=decomposed | 1.177 | 16.512 | 0.07 |
| state=deterministic, kind=basis, count=64, basis=Y, stim_mode=native_pauli | 1.121 | 1.239 | 0.90 |
| state=ghz, kind=parity, count=64, weight=2, stim_mode=decomposed | 0.836 | 32.219 | 0.03 |
| state=ghz, kind=parity, count=64, weight=2, stim_mode=native_pauli | 0.680 | 2.070 | 0.33 |
| state=ghz, kind=parity, count=64, weight=16, stim_mode=decomposed | 1.379 | 39.324 | 0.04 |
| state=ghz, kind=parity, count=64, weight=16, stim_mode=native_pauli | 1.414 | 15.273 | 0.09 |
| state=ghz, kind=parity, count=64, weight=64, stim_mode=decomposed | 1.990 | 38.414 | 0.05 |
| state=ghz, kind=parity, count=64, weight=64, stim_mode=native_pauli | 2.470 | 41.468 | 0.06 |
| state=ghz, kind=parity, count=64, weight=1024, stim_mode=decomposed | 17.430 | 59.110 | 0.29 |
| state=ghz, kind=parity, count=64, weight=1024, stim_mode=native_pauli | 20.511 | 53.750 | 0.38 |
| state=cluster, kind=parity, count=64, weight=2, stim_mode=decomposed | 0.593 | 37.331 | 0.02 |
| state=cluster, kind=parity, count=64, weight=2, stim_mode=native_pauli | 0.587 | 2.618 | 0.22 |
| state=cluster, kind=parity, count=64, weight=16, stim_mode=decomposed | 0.787 | 36.811 | 0.02 |
| state=cluster, kind=parity, count=64, weight=16, stim_mode=native_pauli | 0.811 | 16.038 | 0.05 |
| state=cluster, kind=parity, count=64, weight=64, stim_mode=decomposed | 2.352 | 41.260 | 0.06 |
| state=cluster, kind=parity, count=64, weight=64, stim_mode=native_pauli | 2.251 | 37.675 | 0.06 |
| state=cluster, kind=parity, count=64, weight=1024, stim_mode=decomposed | 23.992 | 67.902 | 0.35 |
| state=cluster, kind=parity, count=64, weight=1024, stim_mode=native_pauli | 17.673 | 56.413 | 0.31 |
| state=scrambled, kind=parity, count=64, weight=2, stim_mode=decomposed | 4.627 | 38.020 | 0.12 |
| state=scrambled, kind=parity, count=64, weight=2, stim_mode=native_pauli | 5.168 | 3.986 | 1.30 |
| state=scrambled, kind=parity, count=64, weight=16, stim_mode=decomposed | 1.720 | 40.383 | 0.04 |
| state=scrambled, kind=parity, count=64, weight=16, stim_mode=native_pauli | 1.755 | 17.667 | 0.10 |
| state=scrambled, kind=parity, count=64, weight=64, stim_mode=decomposed | 2.431 | 41.421 | 0.06 |
| state=scrambled, kind=parity, count=64, weight=64, stim_mode=native_pauli | 2.773 | 40.439 | 0.07 |
| state=scrambled, kind=parity, count=64, weight=1024, stim_mode=decomposed | 20.193 | 62.542 | 0.32 |
| state=scrambled, kind=parity, count=64, weight=1024, stim_mode=native_pauli | 17.258 | 50.724 | 0.34 |

## Interleaved circuits at 4,096 qubits

Times are total milliseconds; ratio >1 means Stim is faster.

| Parameters | Ours ms | Stim ms | Ours / Stim |
|---|---:|---:|---:|
| state=zero, kind=interleaved, count=128, layer=1, locality=random | 0.301 | 0.064 | 4.68 |
| state=zero, kind=interleaved, count=128, layer=1, locality=local | 0.052 | 0.026 | 2.05 |
| state=zero, kind=interleaved, count=128, layer=8, locality=random | 1.161 | 27.987 | 0.04 |
| state=zero, kind=interleaved, count=128, layer=8, locality=local | 1.301 | 45.354 | 0.03 |
| state=zero, kind=interleaved, count=128, layer=64, locality=random | 8.083 | 451.739 | 0.02 |
| state=zero, kind=interleaved, count=128, layer=64, locality=local | 5.038 | 390.894 | 0.01 |
| state=cluster, kind=interleaved, count=128, layer=1, locality=random | 10.985 | 1627.325 | 0.01 |
| state=cluster, kind=interleaved, count=128, layer=1, locality=local | 4.601 | 573.879 | 0.01 |
| state=cluster, kind=interleaved, count=128, layer=8, locality=random | 4.835 | 606.754 | 0.01 |
| state=cluster, kind=interleaved, count=128, layer=8, locality=local | 5.031 | 629.513 | 0.01 |
| state=cluster, kind=interleaved, count=128, layer=64, locality=random | 5.871 | 646.678 | 0.01 |
| state=cluster, kind=interleaved, count=128, layer=64, locality=local | 5.358 | 628.231 | 0.01 |
| state=scrambled, kind=interleaved, count=128, layer=1, locality=random | 61.991 | 580.279 | 0.11 |
| state=scrambled, kind=interleaved, count=128, layer=1, locality=local | 64.976 | 600.428 | 0.11 |
| state=scrambled, kind=interleaved, count=128, layer=8, locality=random | 62.316 | 650.842 | 0.10 |
| state=scrambled, kind=interleaved, count=128, layer=8, locality=local | 60.064 | 650.004 | 0.09 |
| state=scrambled, kind=interleaved, count=128, layer=64, locality=random | 64.659 | 691.194 | 0.09 |
| state=scrambled, kind=interleaved, count=128, layer=64, locality=local | 62.379 | 646.145 | 0.10 |

## 32 fresh-tableau shots at 1,024 qubits

Times are total milliseconds; ratio >1 means Stim is faster.

| Parameters | Ours ms | Stim ms | Ours / Stim |
|---|---:|---:|---:|
| state=cluster, shots=32 | 108.073 | 42.680 | 2.53 |
| state=scrambled, shots=32 | 760.336 | 575.723 | 1.32 |

## Largest completed isolated probes

These sizes are demonstrated under the measured workload and available RAM; they are not universal maximum qubit counts. Initialization and full-touch times, host snapshots, worker swap, failures and budget skips are in the raw reports.

| Mode | Backend | Qubits | Peak RSS GiB |
|---|---|---:|---:|
| gates | stabilizer | 155,648 | 11.32 |
| gates | stim | 141,312 | 11.29 |
| measurement | stabilizer | 156,672 | 11.47 |
| measurement | stim | 142,336 | 11.46 |

Native batches allow Stim to share conversion work. Sequential calls and first measurements have different costs. Deterministic measurement benefits from Stim's inverse tableau. Joint Pauli rows compare local Clifford decomposition either with the same decomposition or with Stim MPP, as labeled; those two modes should be interpreted separately.

See [summary.md](summary.md) for every case and complete methodology, [timings.csv](timings.csv) for numeric analysis, [memory.csv](memory.csv) for every isolated probe, and [results.json](results.json) for the full machine-readable run. Scope is the shared pure-state Clifford/Tableau backend; other algorithms require their own benchmarks.


Resume completed: 19 October 2026, 01:55:35 IST. All 59 remaining cases finished; the first 1,031 result records were verified unchanged using canonical JSON checksums. The full suite has 1,090 successful case records. Capacity probes retain their individual success/budget status in the raw reports.
