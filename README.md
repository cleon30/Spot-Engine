<div align="center">
  <h1>
    <code>Spot Engine</code>
  </h1>
  <strong>Spot payment engine</sup>
  
  <sub>Built with 🦀 <a href="https://www.rust-lang.org" target="_blank">Rust</a> </sub>

</div>

## Description

Given a .csv with transactions from different users of a platform, this Spot-Engine program updates User Accounts, handles disputes and chargebacks and output the state of the clients accounts as .csv 

## Features
1. [x] Deposits
2. [x] Withdrawals
3. [X] Disputes
    1. [x] Deposit disputes
    2. [X] Withdrawal disputes
4. [x] Resolutions
5. [x] Chargebacks
    1. [x] Deposit chargebacks



## Running
  1. `git clone https://github.com/cleon30/Spot-Engine.git`
  2. `cd Spot-Engine`
  3. `cargo run -- account.csv < transactions.csv `

## Deposits Requirements

- Transaction ID must be unique
- Transaction Amount > 0.0001
- Client Account must not be frozen

## Withdrawal Requirements

- Transaction ID must be unique
- Transaction Amount > 0.0001 
- Available funds in User Account must be greater than Transaction Amount 
- Client Account must not be frozen

## Dispute Requirements

- Transaction ID must be in the Transactions History that our Engine has made
- Transaction Amount > 0.0001 
- Available funds in User Account must be greater than Transaction Amount 
- Client Account must not be frozen

### hello