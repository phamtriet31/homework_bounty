# homework_bounty

## Project Title
homework_bounty

## Project Description
HomeworkBounty is a Soroban smart contract that turns a teacher's homework assignment into an on-chain bounty. The teacher posts a problem with a deadline, students submit a hashed solution before time runs out, and the teacher picks one student as the winner. Every action — posting the bounty, submitting a solution, and awarding it — is authenticated via the participant's Stellar account, so authorship and ownership are verifiable on-chain. The contract focuses purely on storage and authorization logic; no real XLM is moved.

## Project Vision
Our long-term vision is to create a transparent, trustless homework and assignment layer for the Stellar education ecosystem. We want students to be recognized — and rewarded — for outstanding work without relying on centralized grading platforms, and we want teachers to publish assignments in a way that is publicly auditable, tamper-proof, and globally accessible. Eventually, the same contract pattern could power open homework markets, peer-graded study groups, and scholarship programs where the Stellar community itself funds the prizes.

## Key Features
- **Post a bounty** — any teacher can register a new homework problem with a unique `problem_id` and a unix-timestamp deadline.
- **Submit a solution** — any student (except the teacher) can submit a single `answer_hash` per bounty before the deadline.
- **Pick a winner** — only the teacher who posted the bounty can mark it as awarded, and the chosen winner must be a real submitter.
- **On-chain authorization** — every state-changing call requires `require_auth()` on the relevant Stellar account, so no one can post, submit, or award on someone else's behalf.
- **Cheap & fast** — the contract stores only tiny key/value entries (Symbol ids, u64 timestamps, Address placeholders), so it runs well within Soroban's minimal-fee model on Testnet.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** education dApp — see `contracts/homework_bounty/src/lib.rs` for the full homework_bounty business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** CDX75QRTAXBJPATICSVCYGJ4CJNKARMO4NOCAFWCBZS2AIEQWNYZ4DVL
- **Explorer template:** https://stellar.expert/explorer/testnet/tx/dd92fba5d28a0d9295b4c866cab15e507f8c22ef5dfffa0eafe6aed58a7fdcf6
- **Screenshot of deployed contract on Stellar Expert:**
![screenshot](https://ibb.co/mdbsWkf)

## Future Scope
- **Real XLM payouts** — extend `pick_winner` to call a token transfer so the bounty prize is paid out atomically with the award.
- **Multi-judge / DAO voting** — replace the single-teacher award step with weighted voting from a council of teachers or token holders.
- **On-chain answer verification** — integrate a verifiable computation (or oracle) so the contract can auto-verify objective answers instead of relying solely on the teacher's judgement.
- **Frontend dApp** — build a Freighter-connected React UI that lets teachers post bounties and lets students browse and submit solutions.
- **Reputation & leaderboards** — track each student's historical win count to surface a public reputation score for the best contributors.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `homework_bounty` (education)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
