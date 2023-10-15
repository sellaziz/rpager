# rpager

**rpager** is a CLI text pager written in RUST.

# Getting Started
## Installation

To install rpager, follow these steps:

Clone the repository or download the latest release:


```bash
git clone https://github.com/sellaziz/rpager.git
cd rpager
```

Build the executable:

```bash
cargo build
```

Optionally, install rpager system-wide:

```bash
cargo install --path .
```

## Usage

To use rpager to view a file:

```bash
rpager --filename filename.txt
```

Replace `filename.txt` with the path to the file you want to view.
Key Bindings
```
    Arrow keys, 'j', 'k': Navigate up and down.
    '/': Type a word to search.
    'n': Next occurrence of word to search.
    'N': Previous occurrence of word to search.
    'q': Quit rpager.
```
