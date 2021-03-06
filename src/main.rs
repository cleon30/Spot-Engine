// Created by 0xCleon // 

// Dependencies & libraries //
use std::fs::File;
use std::error::Error;
use std::{io, process};
use std::collections::HashMap;
use serde::{Deserialize,Serialize};
use csv::{ReaderBuilder, StringRecord, Trim, Writer, ByteRecord};

// CSV Input Transaction structure //
#[derive(Debug, Deserialize,Serialize, Clone)]
struct Transaction {
    r#type: String,
    client: u16,
    tx: u32,
    amount: Option<f32>,
}

// AccountBalance Account of User with specific ID // 
#[derive(Debug, Serialize, PartialEq, Deserialize)]
struct AccountBalance {
    available: f32,
    held: f32,
    total:f32,
    locked:bool,
}

// Script of the Payment Engine //

fn payments_engine() -> Result<(), Box<dyn Error>> {
    let file_path = std::env::args().nth(1).expect("Unable to get args");
    let file = File::open(file_path)?;
    let mut users = HashMap::<u16, AccountBalance>::new(); // hashmap of {Client : Balance Account}
    let mut read = ReaderBuilder::new()                     // input of std:in
                                        .trim(Trim::All)    //whitespaces
                                        .flexible(true)     //dimensional flexible 
                                        .from_reader(file); 

    let mut transaction_history = HashMap::<u32, Transaction>::new();  // Hashmap of {Transaction ID: Transaction Info}
    let mut dispute_tickets = Vec::new();     // Vector with the tx of the disputed tickets
    let mut resolved_tickets = Vec::new();     // Vector with the tx of the resolved tickets
    let mut chargeback_tickets = Vec::new();    // Vector with the tx of the chargeback tickets
    for result in read.deserialize() {
       
        let transaction: Transaction = result?; // Adapt each row from .csv to Transaction structure
        let account = &users.get(&transaction.client); // Account Balance of the Client that made the transaction
        match transaction.amount{ // Amount of the Transaction => Some(f32) is related with Deposit & withdrawal
                                    // None is related with Dispute,Resolve and Chargeback
            Some(amount) =>{                
                if transaction.r#type == "deposit" // First case : Deposit as a transaction // 
                && amount>=0.0001                   // having a minimum amount of money to send(to avoid negative/0 value) // 
                && transaction_history.contains_key(&transaction.tx) == false{ // the tx must be unique
                    transaction_history.insert(transaction.tx,transaction.to_owned()); // add the tx to the history
                    match account{ 
                        None => users.insert(transaction.client,    // Case for new client!(No account found)
                                AccountBalance{
                                    available: amount,
                                    held: 0.0,              // Init with deposit funds
                                    total: amount, 
                                    locked: false,
                                }
                            ),
                        Some(spot_funds) =>{
                            if spot_funds.locked {      // Frozen Account ??????
                                eprintln!("\n??? Account {:?} has been frozen, deposits are currently not available.", transaction.client);              // Default Account
                                users.insert(transaction.client,
                                    AccountBalance{
                                        available: spot_funds.available ,
                                        held: spot_funds.held,
                                        total: spot_funds.total , 
                                        locked: spot_funds.locked,
                                    }
                                )
                            }else{              // Regular users, normal case 
                                
                                users.insert(transaction.client,
                                    AccountBalance{
                                        available: spot_funds.available + amount ,
                                        held: spot_funds.held,
                                        total: spot_funds.total + amount, 
                                        locked: spot_funds.locked,
                                    }
                                )
                            }
                        },
                    };
                    // Second type of transaction: Withdrawal //
                }else if transaction.r#type == "withdrawal"  
                && amount>=0.0001
                && transaction_history.contains_key(&transaction.tx) == false{   // the tx must be unique
                    if let Some(spot_funds) = account{
                        if amount<=spot_funds.available// The transaction amount must be less that the user has
                        && spot_funds.locked == false{ //??????????????????????????????
                            transaction_history.insert(transaction.tx,transaction.to_owned()); // add the tx to the history
                            users.insert(transaction.client,
                                AccountBalance{
                                    available: spot_funds.available - amount,    // Withdraw == delete amount from total and available
                                    held: spot_funds.held,
                                    total:spot_funds.total - amount, 
                                    locked:spot_funds.locked,
                                }
                            )
                        }else{
                            eprintln!("\n??? Account {:?} Withdrawal does not proceed.Account Frozen = {:?} | Amount to Withdraw: {:?} vs Amount Available: {:?}", transaction.client, spot_funds.locked, amount, spot_funds.available);
                            users.insert(transaction.client,
                                AccountBalance{
                                available: spot_funds.available,
                                held: spot_funds.held,
                                total: spot_funds.total, 
                                locked:spot_funds.locked,
                        
                                }
                            )
                        };
                    } 
            
                    
                };
            }
            None =>{  // None is related with Dispute, Resolve and Chargeback
                  // Third type of transaction: Dispute //
                if transaction.r#type == "dispute"
                {
                    let current_transaction_slot = &transaction_history.get(&transaction.tx);// Obtain the transaction info of the dispute tx
                    if let Some(x) = current_transaction_slot{// Obtain the struct values, None case has no sense
                        if let Some(transaction_quantity) = x.amount{   // transaction_quantity = amount $ from the tx we consulted
                            if x.client == transaction.client       // the client that made the dispute must be the same with who made the transaction
                            && dispute_tickets.contains(&transaction.tx)==false // None-repeated disputed
                            && (x.r#type == "deposit" || x.r#type == "withdrawal") {   // Only has sense with deposit & withdrawal

                                if let Some(spot_funds) = account{ // Account info to add $ to held
                                    if spot_funds.available>=transaction_quantity
                                    && spot_funds.locked == false{ // frozen account
                                    
                                    dispute_tickets.push(transaction.tx);// Adding this dispute ticket to the history
                                    users.insert(transaction.client,
                                        AccountBalance{
                                            available: spot_funds.available - transaction_quantity,
                                            held: spot_funds.held + transaction_quantity,
                                            total: spot_funds.held + spot_funds.available ,           // Basically we are deleting funds from available
                                            locked: spot_funds.locked,                        // to store them into held
                                        }
                                    );
                                    }else{
                                        eprintln!("\n??? Disputed from client {:?}  does not proceed. \n  - Status: Account Frozen = {:?} | Amount of Disputed transaction: {:?} vs Amount Available: {:?}", transaction.client, spot_funds.locked, transaction_quantity, spot_funds.available);
                                    } 
                                }
                            }else{
                                eprintln!("\n??? Disputed from Client {:?} does not meet requirements,please check your dispute status.\nClient is the same in tx and dispute? =  {:?} | Dispute ticket already exists? = {:?} | Transaction Type : {:?}", transaction.client, x.client==transaction.client, dispute_tickets.contains(&transaction.tx), x.r#type);
                            }
                        }
                    }else{
                        eprintln!("\n??? Seems like your tx {:?} does not exist in tx history", transaction.tx);
                    }
                // Fourth type of transaction: Resolve //
                }else if transaction.r#type == "resolve"
                {
                    let current_transaction_slot = &transaction_history.get(&transaction.tx);// Obtain the transaction info of the Resolve tx
                    if let Some(x) = current_transaction_slot{              // Obtain the struct values, None case has no sense
                        if let Some(transaction_quantity) = x.amount{       // transaction_quantity = amount $ from the tx we consulted
                            if x.client == transaction.client               // the client that made the resolve must be the same with who made the transaction
                            && dispute_tickets.contains(&transaction.tx)==true // Disputed ticket is mandatory before resolve
                            && resolved_tickets.contains(&transaction.tx)==false{ // Resolve ticket Non-Repeated
                                if let Some(spot_funds) = account{  
                                    if spot_funds.locked == false {                 // Account info to add $ to held
                                        resolved_tickets.push(transaction.tx);  // Adding this resolve ticket to the history
                                        users.insert(transaction.client,
                                            AccountBalance{
                                                available: spot_funds.available +transaction_quantity,
                                                held: spot_funds.held - transaction_quantity,    //Reversing the dispute action(returning the funds to available)
                                                total: spot_funds.total, 
                                                locked: spot_funds.locked,
                                            }
                                        );
                                    }else{
                                        eprintln!("\n??? Account {} is frozen, resolve is not available", x.client);
                                    }
                                }
                            }else{
                                eprintln!("\n??? Resolve from Client {:?} does not meet requirements,please check your dispute status.\nClient is the same in tx and dispute? =  {:?} | Dispute ticket previously made? = {:?} | Resolved ticked already exist? : {:?}", transaction.client, x.client==transaction.client, dispute_tickets.contains(&transaction.tx), resolved_tickets.contains(&transaction.tx));
                            }
                        }
                    }else{
                        eprintln!("\n??? Seems like your tx {:?} does not exist in tx history", transaction.tx);
                    }
                    // Last Type of Transaction: chargeback //
                }else if transaction.r#type == "chargeback" 
                {
                    let current_transaction_slot = &transaction_history.get(&transaction.tx); // Obtain the transaction info of the chargeback tx
                    if let Some(x) = current_transaction_slot{  // Obtain the struct values, None case has no sense
                        if let Some(transaction_quantity) = x.amount{   // transaction_quantity = amount $ from the tx we consulted
                            if x.client == transaction.client   // the client that made the chargeback must be the same with who made the transaction
                            && dispute_tickets.contains(&transaction.tx)==true  // Disputed ticket is mandatory before resolve
                            && chargeback_tickets.contains(&transaction.tx)==false // Chargeback ticket Non-Repeated
                            {   // chargebacks are only available for deposit disputes
                                if let Some(spot_funds) = account{
                                    if transaction_quantity<=spot_funds.held
                                    && spot_funds.locked == false{ // account must not be frozen 
                                        chargeback_tickets.push(transaction.tx); // Adding chargeback ticket to the history
                                        users.insert(transaction.client,
                                            AccountBalance{
                                                available: spot_funds.available,
                                                held: spot_funds.held - transaction_quantity,
                                                total: spot_funds.total - transaction_quantity, 
                                                locked: true,    
                                            }
                                        );
                                    }else{
                                        eprintln!("\n??? Your Account is frozen({:?}) or your Transaction Quantity({:?}) is greater than your Held funds({:?})", spot_funds.locked, transaction_quantity, spot_funds.held);
                                    }
                                }
                            }else{
                                eprintln!("\n??? Requirements not matched for proceding chargeback:{:?}", transaction.tx);
                            }
                        }else{
                            eprintln!("\n??? None value found for transaction consulted:{:?}", x.amount);
                        }
                    }else{
                        eprintln!("\n??? Transaction {:?} has not found", transaction.tx);
                    }
                } else{
                    eprintln!("\n??? Transaction Instruction {:?} does not exist", transaction.r#type);
                    }
                },
        }
    };
    // Write OUTPUT file ! // 
    let mut writer = Writer::from_writer(io::stdout()); // the result will be saved as accounts.csv
    writer.write_record(&["client", "available", "held", "total", "locked"])?;
    for (user, AccountBalance) in users { 
        writer.write_byte_record(&ByteRecord::from(
            vec![
            user.to_string(),
            format!("{:.4}", AccountBalance.available),
            format!("{:.4}", AccountBalance.held),
            format!("{:.4}", AccountBalance.total),
            AccountBalance.locked.to_string(),
        ]));
    };
    writer.flush()?;    
    Ok(()) 
}

fn main() {
    if let Err(err) = payments_engine(){
        eprintln!("{}", err);
        process::exit(1);
    }
}