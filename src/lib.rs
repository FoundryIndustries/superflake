pub mod utils;
pub mod superflake;

#[cfg(test)]
mod tests {
    use crate::superflake::Superflake;

    #[test]
    fn gen_superflake() {
        let mut superflake: Superflake = 
            Superflake::new_with_node_id(1023, None);

        let id = superflake.gen();

        let decoded = superflake.decode(id);

        assert_eq!(decoded.id, id);
        assert_eq!(decoded.node_id, 1023);

        println!("{:?}", decoded);
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