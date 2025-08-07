use std::io::Read;
use transaction::{Transaction, Input, Output, Amount};
mod transaction;


fn read_compact_size(transaction_bytes: &mut &[u8]) -> u64 {
    let mut compact_size = [0_u8; 1];
    transaction_bytes.read(&mut compact_size).unwrap();

    match compact_size[0] {
        0..=252 => compact_size[0] as u64,
        253 => {
            let mut buffer = [0; 2];
            transaction_bytes.read(&mut buffer).unwrap();
            u16::from_le_bytes(buffer) as u64
        },
        254 => {
            let mut buffer = [0; 4];
            transaction_bytes.read(&mut buffer).unwrap();
            u32::from_le_bytes(buffer) as u64
        },
        255 => {
            let mut buffer = [0; 8];
            transaction_bytes.read(&mut buffer).unwrap();
            u64::from_le_bytes(buffer)
        }
    }
}

fn read_u32(transaction_bytes: &mut &[u8]) -> u32 {
   
    let mut buffer = [0; 4];
    transaction_bytes.read(&mut buffer).unwrap();
    u32::from_le_bytes(buffer)   
}

fn read_amount(transaction_bytes: &mut &[u8]) -> Amount {
    let mut buffer = [0; 8];
    transaction_bytes.read(&mut buffer).unwrap();
    Amount::from_sat(u64::from_le_bytes(buffer))
}

fn read_txid(transaction_bytes: &mut &[u8]) -> String{
    let mut buffer = [0; 32];
    transaction_bytes.read(&mut buffer).unwrap();
    buffer.reverse(); // Bitcoin transactions are in little-endian format
    hex::encode(buffer)
}

fn read_script(transaction_bytes: &mut &[u8]) -> String {
    let script_size = read_compact_size(transaction_bytes) as usize;
    let mut buffer = vec![0_u8; script_size];
    transaction_bytes.read(&mut buffer).unwrap();
    hex::encode(buffer)
}

fn main() {
    let transaction_hex ="01000000000101eb3ae38f27191aa5f3850dc9cad00492b88b72404f9da135698679268041c54a0100000000ffffffff02204e0000000000002251203b41daba4c9ace578369740f15e5ec880c28279ee7f51b07dca69c7061e07068f8240100000000001600147752c165ea7be772b2c0acb7f4d6047ae6f4768e0141cf5efe2d8ef13ed0af21d4f4cb82422d6252d70324f6f4576b727b7d918e521c00b51be739df2f899c49dc267c0ad280aca6dab0d2fa2b42a45182fc83e817130100000000";
    let transaction_bytes = hex::decode(transaction_hex).unwrap();
    let mut bytes_slice = transaction_bytes.as_slice();
    let version = read_u32(&mut bytes_slice);
    let input_count = read_compact_size(&mut bytes_slice);
    let mut inputs = vec![];

    for _ in 0..input_count {
        let txid = read_txid(&mut bytes_slice);
        let output_index =  read_u32(&mut bytes_slice);
        let script_sig = read_script(&mut bytes_slice);
        let sequence = read_u32(&mut bytes_slice);  

        inputs.push(Input {
            txid,
            output_index,
            script_sig,
            sequence,
        });

    }

    let output_count = read_compact_size(&mut bytes_slice);
    let mut outputs = vec![];

    for _ in 0..output_count {
        let amount = read_amount(&mut bytes_slice);
        let script_pubkey = read_script(&mut bytes_slice);

        outputs.push(Output {
            amount,
            script_pubkey,
        });
    }
    let transaction = Transaction {
        version,
        inputs,
        outputs,
    };

    //let json_inputs = serde_json::to_string_pretty(&inputs).unwrap();

    println!("Transaction: {}", serde_json::to_string_pretty(&transaction).unwrap());

    // println!("inputs: {:?}", json_inputs);
}


#[cfg(test)]

mod test {
    use super::read_compact_size;

    #[test]

    fn test_read_compact_size() {
        let mut bytes = Cursor::new(&[1_u8]);
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 1_64);

        let mut bytes = Cursor::new(&[253_u8, 0, 1]);
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 253_u64);

        let mut bytes = Cursor::new(&[254_u8, 0, 0, 1, 0]);
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 256_u64.pow(3));

        let mut bytes = Cursor::new(&[255_u8, 0, 0, 0, 0, 0, 0, 0, 1]);
        let count = read_compact_size(&mut bytes);
        assert_eq!(count, 256_u64.pow(7));

        let hex = "fd204e";
        let decoded = hex::decode(hex).unwrap();
        let mut bytes = decoded.as_slice();
        let count =  read_compact_size(&mut bytes);
        let expected_count = 20_000_u64;
        assert_eq!(count, expected_count);
    }
}