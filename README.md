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
## Logic 

### Deposits Requirements

- Transaction ID must be unique
- Transaction Amount > 0.0001
- Client Account must not be frozen

### Withdrawal Requirements

* *Transaction ID must be unique*
* *Transaction Amount > 0.0001*
* *Available funds in User Account must be greater than Transaction Amount* 
* *Client Account must not be frozen*

### Dispute Requirements

- Transaction ID must be in the Transactions History that our Engine has made
- The client from the Transactions History must be the same as the client who is creating a Dispute
- Transaction Amount > 0.0001 
- Client Account must not be frozen

#### *Dispute Deposit*

- Available funds in User Account must be greater than Transaction Amount of dispute
    
When you are actually creating a dispute of a Deposit transaction, your transactions amount dispute will decrease your available funds with that amount. Also, increasing the amount to held.

#### *Dispute Withdrawal*

When you are actually creating a dispute of a Withdrawal transaction, you will be adding the transaction amount to the held funds, also increasing total amount. These funds will not be on available until resolving the dispute.

### Resolve

