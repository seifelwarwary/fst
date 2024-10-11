# FST - Distributed Fault-Tolerant File Storage System (In-Progress)

FST is a **Distributed Fault-Tolerant File Storage System** designed for high performance and reliability. This system uses **Rust** for low-level system performance, **PostgreSQL** for metadata management, and is orchestrated using **Docker** and **Kubernetes** for seamless scaling and fault-tolerance. 

## Key Features (Planned)

- **Fault-tolerance** for node failures using replication and sharding.
- **Efficient load balancing** and storage management.
- **PostgreSQL integration** for fast metadata management.
- **Container orchestration** with **Kubernetes** for scalability and reliability.

## Current Status

At this stage, FST supports the basic functionality of saving and retrieving files by their filename.

### Technologies

- **Rust**: For system-level performance.
- **PostgreSQL**: Metadata management to ensure quick access to file records.
- **Docker**: Containerization of the service.
- **Kubernetes**: Handling distributed architecture and fault-tolerance.

## Installation

### Prerequisites

- **Docker** and **Kubernetes** setup for container orchestration.
- **Rust** installed on your machine.
- **PostgreSQL** for metadata storage.

### Steps

1. Clone the repository:
   ```bash
   git clone https://github.com/seifelwarwary/fst.git
   ```
   
2. Navigate to the project directory:
   ```bash
   cd fst
   ```
   
3. Build the Rust project:
   ```bash
   cargo build --release
   ```

4. Set up Docker and Kubernetes (Instructions to be added in future updates).

## Usage

Currently, FST supports simple file upload and retrieval via filename.

### API Endpoints

- **Upload a File**:
   - Endpoint: `/upload`
   - Method: `POST`
   - Description: Uploads a file to the server.
   - Example:
     ```
     curl -X POST -F "file=@/path/to/file" http://localhost:8080/upload
     ```

- **Retrieve a File**:
   - Endpoint: `/file/{filename}`
   - Method: `GET`
   - Description: Retrieves a file by its filename.
   - Example:
     ```
     curl -X GET http://localhost:8080/file/{filename} -o {output_filename}
     ```

## Future Enhancements

- Adding **sharding and replication** for fault tolerance.
- Enhancing the **load-balancing** mechanism for file access.
- Implementing metadata tracking using **PostgreSQL**.
- Full integration with **Kubernetes** for container orchestration.

## Contributing

Feel free to contribute by submitting issues or pull requests!