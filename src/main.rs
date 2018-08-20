// Copyright 2018 Aark Koduru
//
//!  Helix - Block chain for DNA storage and validation
//!
//!  Takes a string of DNA as input and when mined oututs
//!  a the blockchain
//!


extern crate sha2;
extern crate serde_json;
extern crate regex;
extern crate hex;
extern crate data_encoding;

#[macro_use]
extern crate serde_derive;

use data_encoding::HEXUPPER;
use std::io;
use std::process;
use std::io::Write;
use sha2::{Digest};
use std::str;
use regex::Regex;


#[derive(Serialize, Deserialize, Clone, Debug)]
struct Block {
    p_hash: String,
    id: String,
    dna_hash: String,
}
#[derive(Clone)]
struct Txn {
    id: String,
    dna_hash: String,
}
#[derive(Clone)]
struct Chain {
    blockchain: Vec<Block>,
    pending_txns: Vec<Txn>,
}

impl Chain {
    fn add_dna(&mut self, dna: String, uid: String){
        let buf1 = sha2::Sha256::digest(dna.trim().as_bytes());
        let dna_hash = HEXUPPER.encode(buf1.as_ref());

        let buf2 = sha2::Sha256::digest(uid.trim().as_bytes());
        let id = HEXUPPER.encode(buf2.as_ref());

        let txn = Txn{
                id,
                dna_hash,
        };
        self.pending_txns.push(txn);
    }
}

/// This function takes in the the blockchain
/// as an input and mines the first pending
/// transcation
fn mine_option(chain: &mut Chain) {
    if (chain.pending_txns.len() as i32) < 1{
        println!("Currently there are no pending transcations available to mine.");
        process::exit(0);
    }

    if(chain.blockchain.len() as i32) < 1 {
        let dna_hash = chain.pending_txns[0].dna_hash.clone();
        let p_block = "0000000000";
        let p_hash_buff = sha2::Sha256::digest(p_block.to_string().as_bytes());
        let p_hash = HEXUPPER.encode(p_hash_buff.as_ref());
        let id = chain.pending_txns[0].id.clone();

        let block = Block{
            p_hash,
            id,
            dna_hash,
        };
        chain.blockchain.push(block);
        chain.pending_txns.remove(0);
        println!("Block has been mined successfully");
        println!("{:?}", chain.blockchain);
        process::exit(0);
    }else{
        let dna_hash = chain.pending_txns[0].dna_hash.clone();
        for block in &chain.blockchain {
            if dna_hash == block.dna_hash {
                chain.pending_txns.remove(0);
                println!("You are trying to upload DNA which already exits on the blockchain");
                process::exit(0);
            }
        }
        let blockchain = chain.blockchain.clone();
        let p_block = blockchain.last();
        let p_block_json = serde_json::to_string(&p_block).unwrap();
        let p_block_hash_buff = sha2::Sha256::digest(p_block_json.as_bytes());
        let p_hash = HEXUPPER.encode(p_block_hash_buff.as_ref());
        let id = chain.pending_txns[0].id.clone();
        let block = Block{
            p_hash,
            id,
            dna_hash,
        };
        chain.blockchain.push(block);
        chain.pending_txns.remove(0);
        println!("Block has been mined successfully");
        println!("{:?}", chain.blockchain);
        process::exit(0);
    }

}

/// This function takes a string of DNA sequence
/// as an input and adds it to the list of pending
/// transcations
fn upload_option(chain: &mut Chain) {
    let mut dna = String::new();
    let mut uid = String::new();
    print!("Please enter the DNA seq: ");
    io::stdout().flush().expect("Flushing failed");
    io::stdin().read_line(&mut dna).expect("Failed To read Input");
    if Regex::new(r"[^ACTGactg]").unwrap().is_match(&dna.trim()){
        println!("You entered an invalid DNA");
        process::exit(0);
    }

    print!("Please enter your UID: ");
    io::stdout().flush().expect("Flushing failed");
    io::stdin().read_line(&mut uid).expect("Failed To read Input");

    chain.add_dna(dna, uid);

}

fn main() {

    let mut user_response = String::new();
    let blockchain = Vec::new();
    let pending_txns = Vec::new();

    let mut chain = Chain{
        blockchain,
        pending_txns,
    };

    println!("Welcome to helix!");

    loop{

        loop{
            print!("Would you like to upload DNA? ");
            io::stdout().flush().expect("Flushing failed");
            user_response.clear();
            io::stdin().read_line(&mut user_response).expect("Failed To read Input");
            match user_response.to_lowercase().trim() {
                "yes"|"y" => upload_option(&mut chain),
                "no"| "n" => break,
                _ => println!("Please enter yes/no"),
            }
        }

        loop{
            print!("Would you like to mine a block? ");
            io::stdout().flush().expect("Flushing failed");
            user_response.clear();
            io::stdin().read_line(&mut user_response).expect("Failed To read Input");
            match user_response.to_lowercase().trim() {
                "yes"|"y" => mine_option(&mut chain),
                "no"| "n" => break,
                _ => println!("Please enter yes/no"),
            }
        }

        print!("Would you like to exit? ");
        io::stdout().flush().expect("Flushing failed");
        user_response.clear();
        io::stdin().read_line(&mut user_response).expect("Failed To read Input");
        match user_response.to_lowercase().trim() {
            "yes"|"y" => break,
            "no"| "n" => continue,
            _ => println!("Please enter yes/no"),
        }

    }

}
