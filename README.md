# **Overview**

Web-app steganography tool, that uses LSB technique to encode/decode PNG/JPEG/WAV files.

## **Tools**
Javascript/HTML/CSS/Bootstrap - Frontend\
Rust/WebAssembly - Encryption logic

## **Prerequisites**

* Node.js / npm
* JavaScript
* Optional: rustc, cargo, wasm-pack (if you want to modify encryption source files)

## **Installation**

1. Clone the repository and open it
```bash
git clone https://github.com/Tykion/invisibits.git
cd invisibits
```

2. Install dependencies
```bash
cd frontend
npm install
```

3. Start the local server and open it in any browser
```bash
npm run start
```
```bash
http://localhost:8000
```

***Note: No need to build and compile Rust files since they're already compiled and built in frontend/pkg directory.***

## **Interface usage**

### About: 
Web-app information
### Encode/Decode:
Encode and decode PNG/JPEG files
### WAV encode/WAV decode:
Encode and decode WAV files

## **Encryption logic used**

|Order|Data|Bits|
|:---:|:---:|:---:|
|1|Password Header|len	2 -> 00000010|
|2|Password|'1'	00110001|
|3|Password|'2'	00110010|
|4|Message Header|len 2 -> 00000010|
|5|Message|'H'	01001000|
|6|Message|'i'	01101001|

