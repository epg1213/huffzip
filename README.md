# HUFFZIP

File compression utility made with rust, using Huffman coding.

## Usage

Compression :
```bash
huffzip -i file.txt -o file.txt.hzip
```

Decompression :
```basj
huffzip -i file.txt.hzip -o file.txt -d
```

## Installation

```bash
cargo install --path .
```

## Testing

```bash
cargo test
```

