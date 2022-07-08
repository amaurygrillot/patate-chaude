use std::io::{Read, Write};
use std::net::TcpStream;
use std::{io, str};
use shared::{Message, receive, send, Welcome, Subscribe, ChallengeResult, ChallengeAnswer, SubscribeError, MD5HashCashOutput, solve};

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
            let challenge_result = Message::ChallengeResult(ChallengeResult { answer: ChallengeAnswer::MD5HashCash(solve()),  next_target: "".to_string() });
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


