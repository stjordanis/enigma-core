//sgx 
use sgx_types::{uint8_t, uint32_t};
use sgx_types::{sgx_enclave_id_t, sgx_status_t};
// general 
use rlp;
use enigma_tools_u;
use enigma_tools_u::attestation_service::service;
use enigma_tools_u::attestation_service::constants;
use failure::Error;
//web3
use web3;
use web3::futures::{Future, Stream};
use web3::contract::{Contract, Options};
use web3::types::{Address, U256, Bytes};
use rustc_hex::FromHex;
use web3::transports::Http;
use web3::Web3;

// files 
use std::fs::File;
use std::io::prelude::*;
use serde_json;
use serde_json::{Value};


pub struct EnigmaContract{

    pub web3 : Web3<Http>,
    pub contract : Contract<Http>, 
    pub account : Address,
    pub eloop : web3::transports::EventLoopHandle,
    
}


impl EnigmaContract{
    pub fn new(web3: Web3<Http>, eloop : web3::transports::EventLoopHandle ,address: &str, path: &str, account: &str) -> Self{
        let contract_address: Address = address
            .parse()
            .expect("unable to parse contract address");
        let contract = EnigmaContract::deployed(&web3, contract_address, path);

        let account: Address = account
            .parse()
            .expect("unable to parse account address");

        EnigmaContract { web3: web3, contract: contract, account : account , eloop : eloop}
     }
        /// Fetch the Enigma contract deployed on Ethereum using an HTTP Web3 provider
    pub fn deployed(web3: &Web3<Http>, address: Address, path: &str) -> Contract<Http> {
       
       let abi = EnigmaContract::load_abi(path);

       let contract = Contract::from_json(
           web3.eth(), 
           address, 
           abi.unwrap().as_bytes(),
         ).expect("unable to fetch the deployed contract on the Ethereum provider");

        contract
    }
    // given a path load EnigmaContract.json and extract the ABI
    pub fn load_abi(path: &str) -> Result<String,Error>{
               
       let mut f = File::open(path)
        .expect("file not found.");

       let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("canno't read file");

       let contract_data : Value = serde_json::from_str(&contents)
            .expect("unable to parse JSON built contract");

        let abi = serde_json::to_string(&contract_data["abi"])
            .expect("unable to find the abi key at the root of the JSON built contract");

        Ok(abi)
    }
    pub fn test_web3(&self){
        // get accounts 
        let accounts = self.web3.eth().accounts().wait().unwrap();
        println!("Accounts: {:?}", accounts);
    }

    pub fn register_as_worker(&self, signer : &String, report : &Vec<u8>, gas_limit: &String)->Result<(),Error>{
        // register 
        let signer_addr : Address = signer.parse().unwrap();
        let mut options = Options::default();
        let mut gas : U256 = U256::from_dec_str(gas_limit).unwrap();
        options.gas = Some(gas);
        // call the register function
        self.contract.call("register",(signer_addr,report.to_vec(),12,),self.account,options ).wait().expect("error registering to the enigma smart contract.");
        Ok(())
    }
}