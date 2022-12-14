# Polkadot Blockchain Acadmey - Final Project

## Project Details

<br>

**Build a Quadratic Voting System:**

- Create a simple Identity pallet, or use the existing identity pallet.
- Create a voting system where users with identities can reserve an amount of token, which then weights their vote on a quadratic scale.
- Proposals should be a simple on chain text or hash, votes can be simply aye or nay.
- As a bonus, create a more complex proposal system where users can vote on multiple things at once, and have to consider how they want to distribute their votes across them.

## Project Steps

**Adding The Identity Pallet**

The Idenity pallet needed to be add as you need to link the persons vote with an idenity to make sure that they are voting twice. 

1. Clone Substrate Node Template
2. Add current Identity pallet to the runtime dependices
    - Add pallet dependicies to the runtime - Cargo.toml file
    - Identity Pallet information: [Source Code](https://github.com/paritytech/substrate/tree/master/frame/identity) & [Documentation](https://paritytech.github.io/substrate/master/pallet_identity/index.html)
    - Add pallet to the list of features in the cargo.toml file: "pallet-identity/std"
3. Configure the Identity pallet
    - [Confiduration Documents](https://paritytech.github.io/substrate/master/pallet_identity/pallet/trait.Config.html)
    - Or look at pallet [src code](https://github.com/paritytech/substrate/blob/master/frame/identity/src/lib.rs) under "pub trait Config"
4. Add configuration to runtime file
    - Open: runtime/src/lib.rs
    - Add impl block for identity pallet

```sh
impl pallet_identity::Config for Runtime {
 type Event = Event;
 type Currency = Balances;
 type BasicDeposit = ConstU128<100>;
 type FieldDeposit = ConstU128<5>;
 type SubAccountDeposit = ConstU128<1>;
 type MaxSubAccounts = ConstU32<16>;
 type MaxAdditionalFields = ConstU32<2>;
 type MaxRegistrars = ConstU32<32>;
 type Slashed = ();
 type ForceOrigin = frame_system::EnsureRoot<AccountId>;
 type RegistrarOrigin = frame_system::EnsureRoot<AccountId>;
 type WeightInfo = ();
   }

```

5. Add Identity pallet to the "construct_runtime!" macro

```sh
  Identity: pallet_identity,
```
6. In the end I added it to the mock.rs file. I orginally deleted the mock.rs file as I was following what we had done in the tutorial.

Notes: The entire Identity pallet was probably more than I needed and could of created a simpler one. 

<br>

**Bring in Balances Pallet**

Needed to bring in the balances palelt to have a token to vote with.

1. Bring in pallet to support tokens

- Add dependencies to Cargo.toml
- Add under dependencies and features sections
- Added to mock.rs file 

**Quadratic Voting Pallet**

1. Update name of template to "quadratic-voting" in repository
2. Removed the quadratic-voting/src/benchmarking.rs file - not needed for assignment
3. Updating quadratic-voting/src/lib.rs file to include logic for quadratic voting
4. Added vote.rs file to include vote data types (Don't know if I will end up using it in the end)
5. Added a quadratic-conviction.rs file - did not end up using this file in the end. Lefet notes at the top of the file
6. Time for referendum is in blockctime
7. Voting based of of the amount of tokens that they had. In the fututre I believe that an arbitray amount of tokens should be allotacted to each particiapnt to be used as 'voting credits'. This will be more applicable when there is more than one propoal being voted on at a time and the user need to allocate their votes across multiple items.

## Notes about the project

- I left notes at the top of each file for more specific notes about the file
- Did not worry about weights of the functions as I did not do any benchmarking
- I dod not finish everything and could not get the file to compile
- I need to spend more time with the substrate framwork, how to properly import pallet and set the attributes as I would like


## What I Learned
- It was nice to get hanfds on work with frame and not just following the tutorial
- I need to update my Rust skills! Spent a lot of time working on that
- Also to have a better plan of how I want to attack the problem. I tried too many things that didnt work by trying to rework existing pallets.  

## Build and Run the Project

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with non-persistent state:

```bash
./target/release/node-template --dev
```

Purge the development chain's state:

```bash
./target/release/node-template purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_BACKTRACE=1 ./target/release/node-template -ldebug --dev
```

> Development chain means that the state of our chain will be in a tmp folder while the nodes are
> running. Also, **alice** account will be authority and sudo account as declared in the
> [genesis state](https://github.com/substrate-developer-hub/substrate-node-template/blob/main/node/src/chain_spec.rs#L49).
> At the same time the following accounts will be pre-funded:
>
> - Alice
> - Bob
> - Alice//stash
> - Bob//stash

In case of being interested in maintaining the chain' state between runs a base path must be added
so the db can be stored in the provided folder instead of a temporal one. We could use this folder
to store different chain databases, as a different folder will be created per different chain that
is ran. The following commands shows how to use a newly created folder as our db base path.

```bash
// Create a folder to use as the db base path
$ mkdir my-chain-state

// Use of that folder to store the chain state
$ ./target/release/node-template --dev --base-path ./my-chain-state/

// Check the folder structure created inside the base path after running the chain
$ ls ./my-chain-state
chains
$ ls ./my-chain-state/chains/
dev
$ ls ./my-chain-state/chains/dev
db keystore network
```

### Connect with Polkadot-JS Apps Front-end

Once the node template is running locally, you can connect it with **Polkadot-JS Apps** front-end
to interact with your chain. [Click
here](https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944) connecting the Apps to your
local node template.
