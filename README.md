
# High-Dimensional Database / HIDDB

This application is a Vector database optimized for similarity search for high-dimensional vectors. 
We offer an persistent implementation of approximate nearest neighbor search using Hierarchical Navigable Small World graphs by Malkov et. al. (https://arxiv.org/abs/1603.09320).

## Documentation

Please refer to the [documentation](https://docs.hiddb.bebo.tech/).

## Table of Contents

- [About The Project](#about-the-project)
- [Built With](#built-with)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Contact](#contact)
- [Acknowledgments](#acknowledgments)

## About The Project

This project extents the algorithm proposed by Malkov et. al. with persistence. [Rocksdb](https://rocksdb.org/) is used as key-value store.
The application is accessable via a RESTful API.

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
The application is now accessible via http://localhost:8080

You can interact with the application using the official [Python client](https://pypi.org/project/hiddb/), [Java Script client](https://www.npmjs.com/package/hiddb) or direct HTTP requests, e.g. using [curl](https://curl.se/).

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

Project Link: [https://github.com/hiddb/app](https://github.com/hiddb/hiddb)

## Acknowledgments

- [Benjamin Bolbrinker](https://github.com/benjaminbolbrinker)
- [Marc Erdmann](https://github.com/MarcErdmann)
