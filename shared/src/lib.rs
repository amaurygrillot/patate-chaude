use std::io::{Read, Write};
use std::net::TcpStream;
use std::{str};
use serde::{Serialize, Deserialize};
use md5::Digest;
use rand::Rng;
use serde_json::Error;



pub fn receive(stream: &mut TcpStream, mut array: [u8; 4]) {
    stream.read( &mut array).unwrap();

    let size_message: u32 = u32::from_be_bytes(array);
    let size_message = size_message as usize;

    println!("{}", size_message);

    let mut vector = vec![0; size_message];

    stream.read(&mut vector).unwrap();

    let message_received = std::str::from_utf8(&*vector).unwrap();

    let welcome_serialized = serde_json::to_string(&message_received).unwrap();
    let welcome_serialized_without_backslash = welcome_serialized.replace("\\", "");


    let first_last_off: &str = &welcome_serialized_without_backslash[1..welcome_serialized_without_backslash.len() - 1];
    let message: Result<Message, _> = serde_json::from_str(&first_last_off);

    match message {
        Ok(m) => println!("message={m:?}"),
        Err(err) => println!("error={err:?}")
    }
}

pub fn send(stream: &mut TcpStream, message_to_send: Message) {
    let message_to_serialized = serde_json::to_string(&message_to_send);
    let message_to_serialized = message_to_serialized.unwrap();
    let serialized_message_length_to_u32 = (message_to_serialized.len()) as u32;

    stream.write_all(&serialized_message_length_to_u32.to_be_bytes()).unwrap();

    stream.write_all(&message_to_serialized.as_bytes()).expect("Broken Pipe");
}

const CHARSET: &[u8] = b"ABCDEF0123456789";
const PASSWORD_LEN: usize = 4;

pub fn solve() -> MD5HashCashOutput{

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
    return output


}


#[derive(Serialize, Deserialize, Debug)]
pub struct Welcome{
    version: i32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(PublicLeaderBoard),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    ChallengeAnswer(ChallengeAnswer),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicLeaderBoard(Vec<PublicPlayer>);

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Challenge {
    MD5HashCash(MD5HashCashInput),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResult {
    pub answer: ChallengeAnswer,
    pub next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult(BadResult),
    OK(Ok)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BadResult {
    used_time: f64,
    next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ok {
    used_time: f64,
    next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashInput {
    complexity: u32,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MD5HashCashOutput {
    seed: u64,
    hashcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedChallengeResult {
    name: String,
    value: ChallengeValue
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndOfGame {
    leader_board: PublicLeaderBoard
}
