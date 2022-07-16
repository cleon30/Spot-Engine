<h1 align="center">Spot Engine</h1>

## Features
1. [x] Deposits
2. [x] Withdrawals
3. [X] Disputes
    1. [x] Deposit disputes
    2. [X] Withdrawal disputes
4. [x] Resolutions
5. [x] Chargebacks
    1. [x] Deposit chargebacks

## Description

Given a .csv with transactions from different users of a platform, this Spot-Engine program updates User Accounts, handles disputes and chargebacks and output the state of the clients accounts as .csv 


## Running
  1. `git clone https://github.com/cleon30/Spot-Engine.git`
  2. `cd Spot-Engine && cargo run -- account.csv < transactions.csv`
