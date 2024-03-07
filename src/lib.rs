pub mod utils;

use crate::utils::now_millis;

#[derive(Clone, Debug)]
pub struct Superflake {
    inc: u32,
    epoch: u64,
    node_id: u32,
    last_incremental_exhaustion: u64
}

#[derive(Clone, Debug)]
pub struct DecodedSuperflake {
    pub id: u64,
    pub inc: u64,
    pub epoch: u64,
    pub node_id: u32,
    pub timestamp: u64
}

const DEFAULT_EPOCH: u64 = 1_616_275_800_000; // March 20, 2021

impl Superflake {
    #[inline]
    pub fn gen(&mut self) -> u64 {
        self.gen_with_timestamp(now_millis())
    }

    pub fn new_with_node_id(node_id: u32, epoch: Option<u64>) -> Self {
        Self {
            inc: 0,
            node_id,
            last_incremental_exhaustion: 0,
            epoch: epoch.unwrap_or(DEFAULT_EPOCH)
        }
    }

    pub fn gen_with_timestamp(&mut self, timestamp: u64) -> u64 {
        if self.inc >= 4095 && timestamp == self.last_incremental_exhaustion {
            while now_millis() - timestamp < 1 {
                continue;
            }
        }

        let superflake = ((timestamp - self.epoch) << 22)
            | (u64::from(self.node_id) << 12)
            | u64::from(self.inc);

        self.inc = if self.inc >= 4095 { 0 } else { self.inc + 1 };

        if self.inc == 4095 {
            self.last_incremental_exhaustion = timestamp;
        }

        superflake
    }

    pub fn decode(&self, superflake: u64) -> DecodedSuperflake {
        let timestamp = (superflake >> 22) + self.epoch;
        let node_id = (superflake >> 12) & 0x3FF;
        let inc = superflake & 0xFFF;

        DecodedSuperflake {
            inc,
            timestamp,
            id: superflake,
            epoch: self.epoch,
            node_id: node_id as u32
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Superflake;

    #[test]
    fn gen_superflake() {
        let mut superflake: Superflake = 
            Superflake::new_with_node_id(1023, None);

        let id = superflake.gen();

        let decoded = superflake.decode(id);

        assert_eq!(decoded.id, id);
        assert_eq!(decoded.node_id, 1023);
    }

    #[test]
    fn gen_multiple_superflakes() {
        let mut superflake = Superflake::new_with_node_id(1023, None);

        let ids: Vec<u64> = (0..4096).map(|_| superflake.gen()).collect();
        let last_superflake = superflake.gen();

        for (sequence, &id) in ids.iter().enumerate() {
            let decoded = superflake.decode(id);

            assert_eq!(decoded.inc, sequence as u64);
        }

        let decoded = superflake.decode(last_superflake);
        assert_eq!(decoded.inc, 0);
    }
}