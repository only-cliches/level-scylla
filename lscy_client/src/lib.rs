use scylla::{IntoTypedRows, Session, SessionBuilder};
use std::error::Error;
use std::ops::Deref;
use rustc_hash::FxHashMap;
use bloom::BloomFilter;

struct LSCY_Data_Partition {
    pub cluster_key: [u8; 16],
    pub leve: u8,
    pub starting_key: [u8; 16],
    pub ending_key: [u8; 16],
    pub bloom: BloomFilter // also 16 bytes
}

struct LSCY_Session {
    scylla_session: Session,
    is_connected: bool,
    write_key: [u8; 16],
    data_partitions: Vec<Vec<LSCY_Data_Partition>>
}

impl Deref for LSCY_Session {
    type Target = Session;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.scylla_session
    }
}

impl LSCY_Session {

    pub async fn connect_compactor(url: &str) -> Result<(), Box<dyn Error>> {

        Ok(())
    }

    pub async fn create_table() -> Result<(), Box<dyn Error>> {

        Ok(())
    }

    pub async fn drop_table() -> Result<(), Box<dyn Error>> {

        Ok(())
    }

    pub async fn flush_table() -> Result<(), Box<dyn Error>> {

        Ok(())
    }

    pub async fn insert() -> Result<(), Box<dyn Error>> {

        Ok(())
    }

    pub async fn scan() -> Result<(), Box<dyn Error>> {

        Ok(())
    }

    pub async fn delete() -> Result<(), Box<dyn Error>> {

        Ok(())
    }

    pub async fn get() -> Result<(), Box<dyn Error>> {

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
