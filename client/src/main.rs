use std::io::{Read, Write};
use std::net::TcpStream;
use std::{io, str};
use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use hashcash::{Stamp, check};


fn main() {
    let stream = std::net::TcpStream::connect("127.0.0.1:7878");
    match stream {
        Ok(mut stream) => {
            let hello = Message::Hello;
            send(&mut stream, hello);

            let subscribe = Message::Subscribe(Subscribe { name: "Lucas".parse().unwrap() });
            send(&mut stream, subscribe);

            let array = [0; 4];

            // welcome
            receive(&mut stream, array);

            // subscribe result
            receive(&mut stream, array);

            // leaderBoard
            receive(&mut stream, array);

            // challenge
            let challenge_result = Message::ChallengeResult(ChallengeResult { answer: ChallengeAnswer::MD5HashCash(MD5HashCashOutput { seed: 0, hashcode: "".to_string() }),  next_target: "".to_string() });
            send(&mut stream, challenge_result);

            // challenge result
            receive(&mut stream, array);

            // Round Summary
            receive(&mut stream, array);

            // leaderBoard
            receive(&mut stream, array);
        }
        Err(err) => panic!("Cannot connect: {err}")
    }
}

fn receive(stream: &mut TcpStream, mut array: [u8; 4]) {
    stream.read( &mut array).unwrap();

    let size_message: u32 = u32::from_be_bytes(array);
    let size_message = size_message as usize;

    println!("{}", size_message);

    let mut vector = vec![0; size_message];

    stream.read(&mut vector).unwrap();

    let message_received = std::str::from_utf8(&*vector).unwrap();

    let welcome_serialized = serde_json::to_string(&message_received).unwrap();
    let a = welcome_serialized.replace("\\", "");


    let first_last_off: &str = &a[1..a.len() - 1];
    let message: Result<Message, _> = serde_json::from_str(&first_last_off);

    match message {
        Ok(m) => println!("message={m:?}"),
        Err(err) => println!("error={err:?}")
    }
}

fn send(stream: &mut TcpStream, message_to_send: Message) {
    let message_to_serialized = serde_json::to_string(&message_to_send);
    let message_to_serialized = message_to_serialized.unwrap();
    let serialized_message_length_to_u32 = (message_to_serialized.len()) as u32;

    stream.write_all(&serialized_message_length_to_u32.to_be_bytes()).unwrap();

    stream.write_all(&message_to_serialized.as_bytes()).expect("Broken Pipe");
}

#[derive(Serialize, Deserialize, Debug)]
struct Welcome{
    version: i32
}

#[derive(Debug, Serialize, Deserialize)]
struct Subscribe {
    name: String
}

#[derive(Debug, Serialize, Deserialize)]
enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

#[derive(Debug, Serialize, Deserialize)]
enum SubscribeResult {
    Ok,
    Err(SubscribeError)
}

#[derive(Debug, Serialize, Deserialize)]
enum Message {
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
struct PublicLeaderBoard(Vec<PublicPlayer>);

#[derive(Debug, Serialize, Deserialize)]
struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

#[derive(Debug, Serialize, Deserialize)]
enum Challenge {
    MD5HashCash(MD5HashCashInput),
}

#[derive(Debug, Serialize, Deserialize)]
enum ChallengeAnswer {
    MD5HashCash(MD5HashCashOutput)
}

#[derive(Debug, Serialize, Deserialize)]
struct ChallengeResult {
    answer: ChallengeAnswer,
    next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
enum ChallengeValue {
    Unreachable,
    Timeout,
    BadResult(BadResult),
    OK(Ok)
}

#[derive(Debug, Serialize, Deserialize)]
struct BadResult {
    used_time: f64,
    next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Ok {
    used_time: f64,
    next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
struct MD5HashCashInput {
    complexity: u32,
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MD5HashCashOutput {
    seed: u64,
    hashcode: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RoundSummary {
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

#[derive(Debug, Serialize, Deserialize)]
struct ReportedChallengeResult {
    name: String,
    value: ChallengeValue
}

#[derive(Debug, Serialize, Deserialize)]
struct EndOfGame {
    leader_board: PublicLeaderBoard
}