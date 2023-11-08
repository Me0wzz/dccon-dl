use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Read, Write},
    path::Path,
};

use flate2::read::GzDecoder;
use http::{HeaderMap, HeaderValue};

use crate::constant::TEST_USER_AGENT;

pub async fn dccon_req_test_1() {
    let client = reqwest::Client::builder()
        .user_agent(TEST_USER_AGENT)
        .build()
        .expect("Failed to build Reqwest Client!");
    //req_header from dccon.dcinside.com/#{dccon_id}
    let mut req_header = HeaderMap::new();
    req_header.insert(
        reqwest::header::USER_AGENT,
        HeaderValue::from_static(TEST_USER_AGENT),
    );
    req_header.insert(
        "Accept",
        HeaderValue::from_static("image/avif,image/webp,*/*"),
    );
    req_header.insert(
        "Accept-Encoding",
        HeaderValue::from_static("gzip, deflate, br"),
    );
    req_header.insert(
        "Accept-Language",
        HeaderValue::from_static("en-US,en;q=0.5"),
    );
    req_header.insert("Host", HeaderValue::from_static("dcimg5.dcinside.com"));
    req_header.insert(
        "Referer",
        HeaderValue::from_static("https://dccon.dcinside.com/"),
    );
    req_header.insert("User-Agent", HeaderValue::from_static(TEST_USER_AGENT));
    // send 까진 200 뜨고 이미지를 bytes로
    let response = client.get("https://dcimg5.dcinside.com/dccon.php?no=62b5df2be09d3ca567b1c5bc12d46b394aa3b1058c6e4d0ca41648b651e2246ee4be8057de98a9d24e0029870c1d84c86f4076347186b097a4f0ffccc9572e4a76cafd3f6b")
    .headers(req_header).send().await.unwrap().bytes().await.unwrap();
    // reponse without headermap (403 status code)
    // let response2 = client.get("https://dcimg5.dcinside.com/dccon.php?no=62b5df2be09d3ca567b1c5bc12d46b394aa3b1058c6e4d0ca41648b651e2246ee4be8057de98a9d24e0029870c1d84c86f4076347186b097a4f0ffccc9572e4a76cafd3f6b").send().await.unwrap();
    //println!("{:?}", response);
    // 이미지 기본이 Gzip compressed 상태이기에 decode 해줄 필요 있음
    // 특정 뷰어에선 사용 가능 but, 대부분 뷰어에선 열리지도 않음
    //let mut decoded_response = String::new();
    //gzipdecoder.read_to_string(&mut decoded_response).unwrap();
    File::create("./image.gif.gz").unwrap();
    let mut dest = BufWriter::new(File::create("./image.gif").unwrap());
    let path = Path::new("./image.gif.gz");
    fs::write(path, response).unwrap();
    let mut gzipdecoder = GzDecoder::new(BufReader::new(File::open("./image.gif.gz").unwrap()));
    let mut buf = vec![];
    gzipdecoder.read_to_end(&mut buf).unwrap();
    dest.write_all(&buf).unwrap();
    /*
    200 OK 뜸 여기까지 ok
    println!("{}", response.status());
    */
}
