use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use ethers::signers::{LocalWallet, Signer};
use std::sync::Arc;
use anyhow::{Context, Result};
use crate::blockchain::contract::DataStorage;
use crate::config::APP_CONFIG;

#[derive(Clone)]
pub struct BlockchainService {
    contract: DataStorage<SignerMiddleware<Provider<Http>, LocalWallet>>,
}

impl BlockchainService {
    pub async fn new() -> Result<Self> {
        let provider = Provider::<Http>::try_from(&APP_CONFIG.blockchain_rpc_url)
            .context("Failed to create provider")?;
        
        let wallet: LocalWallet = APP_CONFIG
            .admin_private_key
            .parse()
            .context("Failed to parse admin private key")?;
        
        let chain_id = provider.get_chainid().await?;
        let wallet = wallet.with_chain_id(chain_id.as_u64());
        
        let client = SignerMiddleware::new(provider, wallet);
        let client = Arc::new(client);
        
        let contract_address: Address = APP_CONFIG
            .data_storage_contract_address
            .parse()
            .context("Failed to parse contract address")?;
        
        let contract = DataStorage::new(contract_address, client);
        
        Ok(Self { contract })
    }
    
    /// Register a single student on the blockchain
    pub async fn register_student(
        &self,
        wallet_address: &str,
        student_code: &str,
        full_name: &str,
        email: &str,
    ) -> Result<U256> {
        let address: Address = wallet_address.parse()
            .context("Failed to parse wallet address")?;
        
        let tx = self.contract
            .register_student(
                address,
                student_code.to_string(),
                full_name.to_string(),
                email.to_string(),
            )
            .send()
            .await
            .context("Failed to send register student transaction")?
            .await
            .context("Failed to wait for transaction confirmation")?
            .ok_or_else(|| anyhow::anyhow!("Transaction receipt not found"))?;
        
        // Parse the event to get the student ID
        let logs = tx.logs;
        for log in logs {
            if let Ok(event) = self.contract.decode_event::<(U256, Address, String)>(
                "StudentRegistered",
                log.topics.clone(),
                log.data.clone(),
            ) {
                return Ok(event.0);
            }
        }
        
        Err(anyhow::anyhow!("StudentRegistered event not found in transaction"))
    }
    
    /// Register multiple students on the blockchain in batch
    pub async fn register_students_batch(
        &self,
        wallet_addresses: Vec<String>,
        student_codes: Vec<String>,
        full_names: Vec<String>,
        emails: Vec<String>,
    ) -> Result<()> {
        let addresses: Result<Vec<Address>> = wallet_addresses
            .iter()
            .map(|addr| addr.parse().context("Failed to parse wallet address"))
            .collect();
        
        let addresses = addresses?;
        
        let tx = self.contract
            .register_students_batch(addresses, student_codes, full_names, emails)
            .send()
            .await
            .context("Failed to send batch register transaction")?
            .await
            .context("Failed to wait for transaction confirmation")?;
        
        if tx.is_none() {
            return Err(anyhow::anyhow!("Transaction receipt not found"));
        }
        
        Ok(())
    }
    
    /// Assign role to a user on the blockchain
    pub async fn assign_role(&self, user_address: &str, role: u8) -> Result<()> {
        let address: Address = user_address.parse()
            .context("Failed to parse user address")?;
        
        let _tx = self.contract
            .assign_role(address, role)
            .send()
            .await
            .context("Failed to send assign role transaction")?
            .await
            .context("Failed to wait for transaction confirmation")?;
        
        Ok(())
    }
}

