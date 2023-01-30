# VestaX Finance Liquid Staking SC


## Liquid Staking Workflow

The idea behind liquid staking is to enable people to stake without losing access to the liquidity of their tokens. This takes place through tokenization and issuance of on-chain representations of staked assets - derivative staked tokens - that are a claim on underlying staking positions.

### 1) User Side

There are 3 available actions for Users - stake EGLD (and receive vEGLD), unstake EGLD (send vEGLD), withdraw unbonded EGLD.

#### - Stake

> User A stakes 1 EGLD and receives vEGLD corresponding to 1 EGLD at the moment.
The vEGLD price is determined by the vEGLD/EGLD pool (virtual pool - vEGLD amount is total supply of vEGLD and EGLD amount is total staked amount of Liquidity Staking SC at the moment).
If the vEGLD/EGLD ratio is 1:1 and User A stakes 1 EGLD, he will receive 1 vEGLD. If the vEGLD/EGLD ratio is 1:2, he will receive 0.5 vEGLD.

#### - Unstake

> User A sends 1 vEGLD and unstake EGLD corresponding to 1 vEGLD at the moment. User A won’t receive unstaked EGLD immediately but he will be able to withdraw unstake EGLD after 10-11 days.
If the current vEGLD/EGLD ratio is 1:1 and User A sends 1 vEGLD, he will receive 1 EGLD after 11 days. If the current vEGLD/EGLD ratio is 1:2 and User A sends 1 vEGLD, he will receive 2 EGLD.

#### - Withdraw

> User A will be able to withdraw unstaked EGLD 11 days after unstaking.


### 2) Admin Side

There are 4 actions - delegate,redelegate rewards, undelegate and withdraw.

#### - Admin Delegate

> Admin chooses delegate providers and delegates Users’ EGLD into those selected delegate providers.

#### - Admin Redelegate Rewards

> Admin redelegates rewards and updates the main pool - vEGLD price will increase. Admin will take a fee (i.e, 10%)  for treasury and shareholders.
SC will mint vEGLD for 10% fee and minted vEGLD will be sent to the treasury wallet - Fee will be taken in vEGLD not EGLD. (It will increase the amount of staked EGLD and rewards.)


#### - Admin Undelegate

> Admin undelegates EGLD if Users unstaked EGLD.

#### - Admin Withdraw

> Admin withdraws unbonded EGLD from delegate providers 11 days after undelegation.

### 3) Pool

vEGLD/EGLD ratio will be updated every 24 hours - when an epoch is finished and rewards are generated. Admin will redelegate rewards and vEGLD price will increase.
During 24 hours, the vEGLD/EGLD ratio will not change because staking & unstaking will be done according to the current vEGLD/EGLD ratio. The staked amount can increase or decrease but the vEGLD/EGLD ratio will never change.



## SC Endpoints

### liquid_staking/admin.rs

```rust
#[endpoint(adminDelegate)]
fn  admin_delegate(
	&self,
	delegate_address: ManagedAddress,
	opt_amount: OptionalValue<BigUint>,
)
```

Owner or Admin delegates user-staked EGLD to a staking provider (delegate_address).
If delegation is successful, Prestake Pool will be decreased by delegated EGLD amount in the callback function.

```rust
#[endpoint(adminUndelegate)]
fn  admin_undelegate(
	&self,
	delegate_address: ManagedAddress,
	opt_amount: OptionalValue<BigUint>,
)
```

Owner or Admin undelegates user-unstaked EGLD from a staking provider (delegate_address).
If undelegation is successful, Preunstake Pool will be decreased and Unstaking Pool will be increased by undelegated EGLD amount in the callback function.

```rust
#[endpoint(adminWithdraw)]
fn  admin_withdraw(
	&self,
	delegate_address: ManagedAddress,
	withdraw_amount: BigUint,
)
```

Owner or Admin withdraws unbonded EGLD from a staking provider (delegate_address).
If withdrawl is successful, Unstaking Pool will be decreased and Unbonded Pool will be increased by withdrew EGLD amount in the callback function.

```rust
#[endpoint(adminRedelegateRewards)]
fn  admin_redelegate_rewards(
	&self,
	delegate_address: ManagedAddress,
	rewards_amount: BigUint,
	opt_without_fee: OptionalValue<bool>
)
```

Owner or Admin withdraws redelegates EGLD rewards from a staking provider (delegate_address).
If redelegation is successful, protocol fee is taken in the form of vEGLD in the callback function. New vEGLD is minted as protocol fee and it will be sent to the treasury wallet.

### liquid_staking/user.rs

```rust
#[payable("EGLD")]
#[endpoint]
fn  stake(&self)
```

User stakes EGLD, and new vEGLD corresponding to staked EGLD amount will be minted and will be sent to User.
If Auto-Delegate is enabled, staked EGLD will be directly delegated to a staking provider.

```rust
#[payable("*")]
#[endpoint]
fn  unstake(&self)
```

User unstakes vEGLD and those vEGLD will be burnt.
Admin will undelegate EGLD according to user-unstaked amount.

```rust
#[endpoint]
fn  withdraw(&self)
```

User withdraws EGLD. Unstaking packs 11 days (user-unstaking-period) after unstake can be withdrew.

### storages/common_storage.rs

This file includes storage mappers for common settings.

### storages/pool_storage.rs

This file includes storage mappers for all pools - Main Pools (EGLD, vEGLD), Prestake Pool, Preunstake Pool, Unstaking Pool, Unbonded Pool.

### amm.rs

`quoteVegld` and `quoteEgld` are querying price functions. vEGLD price is determined by ratio of `pool_vegld_amount` and `pool_egld_amount` storages.

### brand.rs

This file includes vEGLD token issuance and set-role functions.

### config.rs

This file includes functions for configuring main settings and common storage mappers.

### constant.rs

This file includes all constants.

### context.rs

This file includes all structs for function context and view functions.

### delegate_proxy.rs

This file includes endpoints of Elrond native staking provider SC.

### event.rs

This file includes all events.

### validation.rs

This file includes all require & checkup functions similar to Solidity modifier.

### view.rs

This file includes all view functions for frontend.


## Extra

### `auto_delegate_address` and `auto_undelegate_address` storages

These addresses are used in `adminDelegate` and `adminUndelegate` as default staking provider.

### `whitelisted_sp_addresses`

only whitelisted Staking Providers can participate in delegation & undelegation

## Gas Limit

### `adminDelegate`, `adminUndelegate`, `adminWithdraw`, `adminClaimRewards`

30_000_000

### `stake`, `unstake`, `withdraw`, `donate`

15_000_000