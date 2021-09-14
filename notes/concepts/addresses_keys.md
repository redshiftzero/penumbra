# Addresses and Keys

The key hierarchy is based on the [Zcash Sapling](https://github.com/zcash/zips/blob/main/protocol/sapling.pdf) design, which we briefly summarize here.

All addresses and keys are ultimately derived from a secret *spending key* $sk$, which is a 32-byte random number. From this *spending key* $sk$, we can derive:

* an expanded form of the spending key called the *expanded spending key* which has components used to derive *viewing keys* and the *proof authorizion key* as described below,
* *viewing keys* which allow the holder to identify but not spend notes associated with the *spending key*,
* *diversified payment addresses*, which can be shared in order to receive payments,
* a *proof authorization key*, which lets the holder spend notes associated with the *spending key*.

We describe each of these keys in more detail below.

## Expanded Spending Keys

The *expanded spending key* has three components:

* $ask$, the *spend authorization key* which is a scalar value
* $nsk$, the *nullifier private key* which is a scalar value
* $ovk$, the *outgoing viewing key* which is a 32-byte number

The scalars are derived by hashing $sk$ along with a value $t$ ($t=0$ for $ask$, $t=1$ for $nsk$, $t=2$ for $ovk$), then mapping to a scalar for the decaf377 curve.

TODO: Define $ToScalar$ function for decaf377
TODO: Confirm $PRF^{expand}_{sk}$ is unchanged from Zcash sapling (using Blake2b)

TK: once TODOs done, write out exactly these functions

TK: FMD flag key goes in here derived from $sk$?

## Viewing Keys

TK

## Addresses

Addresses in Penumbra are diversified payment addresses as in Zcash Sapling: for each *spending key*, there are many possible payment addresses. Each address consists of a *diversifier* $d$ as well as a *transmission key* $pk_d$.

## Proof Authorization Keys

TK

# Implementation Notes

Keys are implemented as described in section 4.2 of the [Sapling Protocol Specification](https://github.com/zcash/zips/blob/main/protocol/sapling.pdf) and [ZIP 32](https://zips.z.cash/zip-0032) with the following changes:

* `JubJub` is replaced by `decaf377`.
* In "Sapling diversifier derivation" in ZIP 32, all diversifiers $d_j$ are valid. The default diversifier for an extended key is $d_0$. Similarly, in section 4.2.2 of the specification, the `CheckDiversifier` function returns only $d$, again as all diversifiers are valid.
* TK
