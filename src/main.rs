use md5;
use rand::{distributions::Alphanumeric, Rng, RngCore}; // 0.8
struct MD5HashCashInput {
    // complexity in bits
    complexity: u32,
    // message to sign
    message: String,
}

struct MD5HashCashOutput {
    // Seed used to solve the challenge
    seed: u64,
    // hashcode found using seed + message
    hashcode: String,
}
const CHARSET: &[u8] = b"ABCDEF0123456789";
const PASSWORD_LEN: usize = 4;
fn main(){

    let mut rng = rand::thread_rng();
    let input: MD5HashCashInput = MD5HashCashInput{
        complexity: rng.gen_range(1..128),
        message: "hello".to_string()
    };
    let mut continue_loop = true;
    let mut number_of_loops = 0;
    let mut output: MD5HashCashOutput = MD5HashCashOutput
    {
        seed: 0,
        hashcode: "".to_string()
    };
    while continue_loop
    {
        let mut seed: String = (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        println!("seed : {}", seed);
        let mut digest = md5::compute(seed.clone() + &input.message);
        let mut zero_count = 0;
        for char in digest.to_vec()
        {
            number_of_loops += 1;
            zero_count += char.count_zeros();
        }
        println!("Number of 0 : {}, number of loops : {}", zero_count, number_of_loops);
        println!("complexity : {}", input.complexity);
        if zero_count >= input.complexity
        {
            println!("bonne string trouvée");
            output.hashcode = format!("{:X}", digest);
            output.seed = u64::from_str_radix(&seed, 16).unwrap_or_default();
            println!("hashcode final : {}", output.hashcode);
            println!("seed finale : {}", output.seed);
            continue_loop = false;
            break;
        }
        else
        {
            output.hashcode = "".to_string();
        }
    }


}