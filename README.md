
# HIDDB App

Welcome to the HIDDB App repository. This project is an persistant implementation of approximate nearest neighbor search using Hierarchical Navigable Small World graphs by Malkov et. al. (https://arxiv.org/abs/1603.09320).

## Table of Contents

- [About The Project](#about-the-project)
- [Built With](#built-with)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)
- [Acknowledgments](#acknowledgments)

## About The Project

This project extents the algorithm proposed by Malkov et. al. with persistence. [Rocksdb](https://rocksdb.org/) is used as key-value store.

### Built With

- [Rust](https://www.rust-lang.org/)

## Getting Started

To get a local copy up and running follow these simple steps.

### Prerequisites

- Rust
- Docker

### Installation

1. Clone the repo:
   ```sh
   git clone https://github.com/hiddb/app.git
   ```
2. Install dependencies and build the binary:
   ```sh
   cargo build
   ```

## Usage

Here are some examples of how to use the project:

1. Run the application:
   ```sh
   cargo run
   ```
## Contributing

Contributions are what make the open-source community such an amazing place to be learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the GNU GENERAL PUBLIC LICENSE Version 3, 29 June 2007. See `LICENSE.txt` for more information.

## Contact

Project Link: [https://github.com/hiddb/app](https://github.com/hiddb/app)

## Contributers

- [Benjamin Bolbrinker](https://github.com/benjaminbolbrinker)
- [Marc Erdmann](https://github.com/MarcErdmann)
