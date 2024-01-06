# my-eth-tool

[![License](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://github.com/lazyfuhrer/my-eth-tool/blob/main/LICENSE)
[![Rust](https://github.com/lazyfuhrer/my-eth-tool/actions/workflows/rust.yml/badge.svg)](https://github.com/lazyfuhrer/my-eth-tool/actions/workflows/rust.yml)

## Overview

**my-eth-tool** is a command-line tool written in Rust for interacting with Ethereum on the Holesky network (p.s. - currently). It provides functionality for creating wallets, making transactions, and more.

## Features

- **Wallet Management:** Create, manage Ethereum wallets.
- **Transaction Handling:** Send and receive transactions on the Ethereum blockchain.
- **Secure Key Handling:** Utilizes the secp256k1 library for secure key operations.

## Installation

Ensure you have Rust installed. Then, you can install **my-eth-tool** using the following command:

```bash
cargo install my-eth-tool
```

## Usage

### 1. Create a Wallet

```bash
my-eth-tool create
```
- This command generates a new Ethereum wallet.

### 2. Send Ether to Another Address

```bash
my-eth-tool send --to-addr <TO-ADDRESS> --value <VALUE> --secret-key <KEY>
```
- ```<TO-ADDRESS>``` The recipient's Ethereum address.
- ```<VALUE>``` The amount of Ether to send.
- ```<KEY>``` Your secret key for signing the transaction.

### 3. Check Wallet Balance

```bash
my-eth-tool balance --address <ADDRESS>
```
- ```<ADDRESS>``` The Ethereum address to check the balance for.

### 4. Get the Latest Block Information

```bash
my-eth-tool block
```
- This command retrieves the latest block number.

For help,

```bash
  my-eth-tool --help
```
## License

This project is licensed under the terms of the [MIT License](LICENSE) or the [Apache License 2.0](LICENSE).

The MIT License is a permissive open source license that allows for free use, modification, and distribution of the software, while the Apache License 2.0 is a more detailed and strict license. You can choose the license that best fits your needs.

For more details, see the [LICENSE](LICENSE) file.
