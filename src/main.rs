// Dependencies & libraries //
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
    let mut users = HashMap::<u16, AccountBalance>::new(); // hashmap of Client + Balance Account
    let mut read = ReaderBuilder::new()                     // input of std:in
                                        .trim(Trim::All)    //whitespaces
                                        .flexible(true)     //dimensional flexible 
                                        .from_reader(io::stdin()); 

    let mut transaction_record = HashMap::<u32, Transaction>::new();  // hashmap of all the Deposits & Withdrawals from everyone
    let mut dispute_tickets = Vec::new();     // Vector with the tx of the disputed tickets
    let mut resolved_tickets = Vec::new();     // Vector with the tx of the resolved tickets
    let mut chargeback_tickets = Vec::new();    // Vector with the tx of the chargeback tickets
    for result in read.deserialize() {
       
        let transaction: Transaction = result?; // Adapt each row from .csv to Transaction structure
        let account = &users.get(&transaction.client); // Account Balance of the Client that made the transaction
        match transaction.amount{ // Amount of the Transaction => Some(f32) is related with Deposit & withdrawal
                                    // None is related with Dispute,Resolve and Chargeback
            Some(amount) =>{                
                if transaction.r#type == "deposit" // First case : Withdrawal as a transaction // 
                && amount>=0.0001                   // having a minimum amount of money to send(to avoid negative/0 value) // 
                && transaction_record.contains_key(&transaction.tx) == false{ // the tx must be unique
                    transaction_record.insert(transaction.tx,transaction.to_owned()); // add the tx to the record
                    match account{ 
                        None => users.insert(transaction.client,    // Case for new client!(No account found)
                                AccountBalance{
                                    available: amount,
                                    held: 0.0,              // Init with deposit funds
                                    total: amount, 
                                    locked: false,
                                }),
                        Some(x) =>{
                            if x.locked {      // Frozen Account ❄️
                                users.insert(transaction.client,
                                    AccountBalance{
                                        available: x.available ,
                                        held: x.held,
                                        total: x.total , 
                                        locked: x.locked,
                                    })
                            }else{              // Default Account
                                users.insert(transaction.client,
                                    AccountBalance{
                                        available: x.available + amount ,
                                        held: x.held,
                                        total: x.total + amount, 
                                        locked: x.locked,
                                    })
                            }
                        },
                    };
                    // Second type of transaction: Withdrawal //
                    }else if transaction.r#type == "withdrawal"  
                    && amount>=0.0001
                    && transaction_record.contains_key(&transaction.tx) == false{   // the tx must be unique
                        if let Some(x) = account{
                            if amount<=x.available// The transaction amount must be less that the user has
                            && x.locked == false{ //❄️❄️❄️❄️❄️
                                transaction_record.insert(transaction.tx,transaction.to_owned()); // add the tx to the record
                                users.insert(transaction.client,
                                    AccountBalance{
                                        available: x.available - amount,    // Withdraw == delete amount from total and available
                                        held: x.held,
                                        total:x.total - amount, 
                                        locked:x.locked,
                                    })
                                }else{
                                    users.insert(transaction.client,
                                        AccountBalance{
                                            available: x.available,
                                            held: x.held,
                                            total: x.total, 
                                            locked:x.locked,
                            
                                        })
                                };
                            } 
                   
                        };
                    }
             None =>{  // None is related with Dispute, Resolve and Chargeback
                  // Third type of transaction: Dispute //
            if transaction.r#type == "dispute"
             {
                let current_transaction_slot = &transaction_record.get(&transaction.tx);// Obtain the transaction info of the dispute tx
                if let Some(x) = current_transaction_slot{// Obtain the struct values, None case has no sense
                    if let Some(transaction_quantity) = x.amount{   // transaction_quantity = amount $ from the tx we consulted
                        if x.client == transaction.client       // the client that made the dispute must be the same with who made the transaction
                        && dispute_tickets.contains(&transaction.tx)==false // None-repeated disputed
                        && x.r#type == "deposit"{   // Only has sense with deposit

                            if let Some(y) = account{ // Account info to add $ to held
                                if y.available>=transaction_quantity
                                && y.locked == false{
                                 // frozen account
                                dispute_tickets.push(transaction.tx);// Adding this dispute ticket to the record
                                    users.insert(transaction.client,
                                        AccountBalance{
                                            available: y.available - transaction_quantity,
                                            held: y.held + transaction_quantity,
                                            total: y.held + y.available ,           // Basically we are deleting funds from available
                                            locked:y.locked,                        // to store them into held
                                        });
                                    } 
                                }
                        }else if x.client == transaction.client 
                        && dispute_tickets.contains(&transaction.tx)==false
                        && x.r#type == "withdrawal"{                            // disputes for withdrawals
                            if let Some(y) = account{                           // Account info to add $ to held
                                if y.locked == false{
                                 // frozen account
                                dispute_tickets.push(transaction.tx);// Adding this dispute ticket to the record
                                    users.insert(transaction.client,
                                        AccountBalance{
                                            available: y.available,
                                            held: y.held + transaction_quantity,
                                            total: y.held + y.available + transaction_quantity,           // Basically we are deleting funds from available
                                            locked:y.locked,                        // to store them into held
                                        });
                                    } 
                                }

                        }
                }
                
                }
             // Fourth type of transaction: Resolve //
             }else if transaction.r#type == "resolve"
             {
                let current_transaction_slot = &transaction_record.get(&transaction.tx);// Obtain the transaction info of the Resolve tx
                if let Some(x) = current_transaction_slot{              // Obtain the struct values, None case has no sense
                    if let Some(transaction_quantity) = x.amount{       // transaction_quantity = amount $ from the tx we consulted
                        if x.client == transaction.client               // the client that made the resolve must be the same with who made the transaction
                        && dispute_tickets.contains(&transaction.tx)==true // Disputed ticket is mandatory before resolve
                        && resolved_tickets.contains(&transaction.tx)==false{ // Resolve ticket Non-Repeated
                            if let Some(y) = account{  
                                if y.locked == false {                 // Account info to add $ to held
                                    resolved_tickets.push(transaction.tx);  // Adding this resolve ticket to the record
                                        users.insert(transaction.client,
                                            AccountBalance{
                                                available: y.available +transaction_quantity,
                                                held: y.held - transaction_quantity,    //Reversing the dispute action(returning the funds to available)
                                                total:y.total, 
                                                locked:y.locked,
                                            
                                            });
                                    }
                                }
                        }
                }
                }
                // Last Type of Transaction: chargeback //
             }else if transaction.r#type == "chargeback" 
             {
                let current_transaction_slot = &transaction_record.get(&transaction.tx); // Obtain the transaction info of the chargeback tx
                if let Some(x) = current_transaction_slot{  // Obtain the struct values, None case has no sense
                    if let Some(transaction_quantity) = x.amount{   // transaction_quantity = amount $ from the tx we consulted
                    if x.client == transaction.client   // the client that made the chargeback must be the same with who made the transaction
                    && dispute_tickets.contains(&transaction.tx)==true  // Disputed ticket is mandatory before resolve
                    && chargeback_tickets.contains(&transaction.tx)==false // Chargeback ticket Non-Repeated
                    && x.r#type == "deposit"{   // chargebacks are only available for deposit disputes
                        if let Some(y) = account{
                            if transaction_quantity<=y.held
                            && y.locked == false{ // account must not be frozen 
                                chargeback_tickets.push(transaction.tx); // Adding chargeback ticket to the record
                                 users.insert(transaction.client,
                                     AccountBalance{
                                         available: y.available,
                                         held: y.held - transaction_quantity,
                                         total:y.total - transaction_quantity, 
                                         locked:true,    
                                     });
                            }else if y.locked == false      // account must not be frozen 
                            && transaction_quantity>y.held{          
                                chargeback_tickets.push(transaction.tx); // Adding chargeback ticket to the record
                                users.insert(transaction.client,
                                    AccountBalance{
                                        available: y.available ,
                                        held: y.held,
                                        total:y.total, 
                                        locked:true,            // frozen added
                                    }
                                );
                            }
                        }
                    }
                }
                }
             } else{
                 ()
        }
    },
    }
    };
    // Write OUTPUT file ! // 
    let mut writer = Writer::from_path("accounts.csv")?; // the result will be saved as accounts.csv
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
    // payments_engine();
}
