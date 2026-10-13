"""Reproduce Fig. 2 of AG2004: seconds per measurement after beta*n*log2(n)
random unitary gates, for several beta.

    python examples/figure2.py                 # quick: n = 200..1600
    python examples/figure2.py --full          # the paper's n = 200..3200
    python examples/figure2.py --plot out.png  # also save a plot (needs matplotlib)
"""

import argparse

from stabilizer import time_measurements

BETAS = [0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2]


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--full", action="store_true", help="n up to 3200 as in the paper")
    ap.add_argument("--plot", metavar="FILE", help="save a plot to FILE")
    ap.add_argument("--seed", type=int, default=0)
    args = ap.parse_args()

    ns = list(range(200, (3200 if args.full else 1600) + 1, 200))
    results = {}
    print("n     " + "".join(f"beta={b:<6}" for b in BETAS))
    for n in ns:
        row = [time_measurements(n, b, seed=args.seed + n) for b in BETAS]
        for b, t in zip(BETAS, row):
            results.setdefault(b, []).append(t)
        print(f"{n:<6}" + "".join(f"{t * 1e6:9.1f}us " for t in row))

    if args.plot:
        import matplotlib.pyplot as plt

        fig, ax = plt.subplots(figsize=(6, 4))
        for b in reversed(BETAS):
            ax.plot(ns, results[b], marker="o", label=f"beta={b}")
        ax.set_xlabel("Number of qubits n")
        ax.set_ylabel("Seconds per measurement")
        ax.legend()
        fig.tight_layout()
        fig.savefig(args.plot)
        print(f"saved {args.plot}")


if __name__ == "__main__":
    main()
