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
    1. [x] Deposit resolutions
    2. [X] Withdrawal resolutions
5. [x] Chargebacks
    1. [x] Deposit chargebacks



## Running
  1. `git clone https://github.com/cleon30/Spot-Engine.git`
  2. `cd Spot-Engine`
  3. `cargo run -- account.csv < transactions.csv `

## Logic 

### **Deposits**

Requirements:

- Transaction ID must be unique
- Transaction Amount > 0.0001
- Client Account must not be frozen

```bash
AccountBalance{
                available: spot_funds.available + amount ,
                held: spot_funds.held,
                total: spot_funds.total + amount, 
                locked: spot_funds.locked,
                }
```

### **Withdrawal**

Requirements:

- Transaction ID must be unique
- Transaction Amount > 0.0001 
- Available funds in User Account must be greater than Transaction Amount 
- Client Account must not be frozen

### **Dispute**

Requirements:

- Transaction ID must be in the Transactions History that our Engine has made.
- Only 1 dispute per tx.
- The client from the Transactions History must be the same as the client who is creating a Dispute.
- Client Account must not be frozen.

#### ***Dispute Deposit***

Requirements:

- Available funds in User Account must be greater than Transaction Amount of dispute
    
When you are actually creating a dispute of a Deposit transaction, your transactions amount dispute will decrease your available funds with that amount. Also, increasing the amount to held.

#### ***Dispute Withdrawal***

When you are actually creating a dispute of a Withdrawal transaction, you will be adding the transaction amount to the held funds, also increasing total amount. These funds will not be on available until resolving the dispute.

In this case I had the dilemma working with Dispute Withdrawal, because the idea of disputing a withdrawal is useful to prevent wrong withdrawals. So you make a dispute, you get the funds held but the funds are not available if resolve does not proceed.

### **Resolve**

Requirements:

- Transaction ID must be in the Transactions History that our Engine has made.
- Dispute of Transaction ID must be in the Dispute Tickets that our Engine has made.
- Only 1 Resolve for Dispute ticket.
- The client from the Transactions History must be the same as the client for who is resolving.
- Client Account must not be frozen

When you are actually receiving a resolve transaction, is indicating that the dispute has succeed and the held funds must be transfer to available funds.
 
 In this case I had the dilemma working with Resolving Deposit, because I was not sure if what is wanted is to reverse the transaction ID or just transfer the held funds to Available funds.


### **Chargeback**

Requirements:

- Transaction ID must be in the Transactions History that our Engine has made.
- Dispute of Transaction ID must be in the Dispute Tickets that our Engine has made.
- The client from the Transactions History must be the same as the client who is chargebacking.
- Client Account must not be frozen.
- Type of transaction in the tx History must be Deposit. 

When you are receiving a chargeback, it means that the funds on held and total must be decreased by the quantity of the deposit that the client user has made. The account will be instantly frozen.
 



