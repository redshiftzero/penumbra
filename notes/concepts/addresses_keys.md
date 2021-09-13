# Addresses and Keys

Note: this initial description describes the basic design without Fuzzy Message Detection, which will be added in a future iteration.

The basic key hierarchy is as in [Zcash Sapling](https://github.com/zcash/zips/blob/main/protocol/sapling.pdf), which we summarize here.

All addresses and keys are ultimately derived from a *spending key* $sk$. From the *spending key* $sk$, we derive:

* *viewing keys* which allow the holder to identify but not spend notes associated with the *spending key*,
* *diversified payment addresses*, which can be shared in order to receive payments,
* a *proof authorization key*, which lets the holder spend keys associated with the *spending key*.

## Viewing Keys

TK

## Addresses

Addresses in Penumbra are diversified payment addresses as in Zcash Sapling: for each *spending key*, there are many possible payment addresses. Each address consists of a *diversifier* $d$ as well as a *transmission key* $pk_d$.

## Proof Authorization Keys

TK

# Implementation Notes

Keys are implemented as described in section 4.2 of the [Sapling Protocol Specification](https://github.com/zcash/zips/blob/main/protocol/sapling.pdf) and [ZIP 32](https://zips.z.cash/zip-0032) with the following changes:

* `JubJub` is replaced by `BLS12-377`.
* In "Sapling diversifier derivation" in ZIP 32, all diversifiers $d_j$ are valid. The default diversifier for an extended key is $d_0$. Similarly, in section 4.2.2 of the specification, the `CheckDiversifier` function returns only $d$, again as all diversifiers are valid.
* TK
