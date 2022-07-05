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
        let mut seed: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(20)
            .map(char::from)
            .collect();
        println!("seed : {}", seed);
        let mut digest = md5::compute(seed + &input.message);
        let mut zero_count = 0;
        for char in digest.to_vec()
        {
            print!("{}", char);
            number_of_loops += 1;
            zero_count += char.count_zeros();
            if zero_count >= input.complexity
            {
                println!("bonne string trouvée");
                continue_loop = false;
                break;
            }
        }
        println!("Number of 0 : {}, number of loops : {}", zero_count, number_of_loops);
        println!("complexity : {}", input.complexity);
        output.hashcode = digest.
    }


}