# VPN Project Architecture

This document outlines the architecture of the Rust-based VPN project.

## High-Level Overview

The project implements a simple Layer 3 VPN using a client-server architecture. It creates a secure tunnel between a client and a server, allowing the client to route its traffic through the server.

The core components are:
- **Client**: Initiates the connection and sends/receives traffic through the tunnel.
- **Server**: Listens for client connections, manages peers, and routes traffic.
- **Transport Protocol**: Uses UDP for communication between the client and server.
- **Tunneling**: Utilizes a TUN device to create a virtual network interface for capturing and injecting IP packets.
- **Cryptography**: Employs a handshake protocol for key exchange, with plans for encrypting tunnel traffic.

## Core Modules

The project is structured into several modules under the `src/core/` directory:

- `tun`: Handles the creation and management of the TUN virtual network interface. This is OS-specific, with the current implementation targeting macOS.
- `udp`: A simple wrapper around a non-blocking UDP socket for sending and receiving data.
- `protocol`: Defines the structure of control messages, such as handshake packets.
- `crypto`: Manages cryptographic operations, starting with key generation for the handshake.
- `echo`: A test module to demonstrate a simple echo tunnel.

## Handshake and Session Establishment

The connection is established using a custom handshake protocol. The goal of the handshake is for the client and server to exchange ephemeral public keys to perform a Diffie-Hellman key exchange (`X25519`) and establish a shared secret.

**Handshake Flow:**

```
Client                                     Server
  |                                            |
  | --- Handshake Initiation (Type 1) --->     |
  |  (contains client's ephemeral public key)  |
  |                                            |
  |     <--- Handshake Response (Type 2) ---   |
  |  (contains server's ephemeral public key)  |
  |                                            |
```

1.  **Client -> Server**: The client sends a `HandshakeInitiation` message. This packet is unencrypted and contains the client's ephemeral public key for the key exchange.
2.  **Server -> Client**: The server receives the initiation, stores the client's public key, and replies with a `HandshakeResponse` message containing its own ephemeral public key.

After this exchange, both client and server can compute a shared secret to be used for symmetric encryption of the tunnel traffic. (Note: The encryption of data packets is not yet implemented).

## Data Flow

Once the handshake is complete, the data flow for a packet sent from the client's machine to a remote host would be as follows:

**Outgoing Packet (Client -> Server):**

```
[App on Client] -> [OS Kernel] -> [TUN Device] -> [VPN Client] -> [UDP Socket] -> [Internet] -> [VPN Server]
```

1.  An application on the client's machine sends an IP packet.
2.  The OS's routing table directs this packet to the TUN interface.
3.  The VPN client reads the raw IP packet from the TUN device.
4.  The client encrypts the IP packet using the shared secret.
5.  The encrypted packet is wrapped in a UDP datagram and sent to the VPN server.

**Incoming Packet (Server -> Client):**

```
[VPN Server] -> [UDP Socket] -> [VPN Client] -> [TUN Device] -> [OS Kernel] -> [App on Client]
```

1.  The VPN client receives a UDP packet from the server.
2.  The client decrypts the packet's payload to get the original IP packet.
3.  The client writes the raw IP packet to the TUN device.
4.  The client's OS network stack processes the packet, and it is delivered to the destination application.

## Packet Structure

The protocol defines message types to distinguish between handshake and data packets.

- **Type 1: Handshake Initiation**: `[1:u8] [client_id:u32] [ephemeral_public_key:32*u8]`
- **Type 2: Handshake Response**: `[2:u8] [receiver_index:u32] [server_ephem_pub:32*u8]`
- **Type 4: Encrypted Data Packet**: (To be implemented)

This structure allows the server and client to manage the state of the connection and handle packets accordingly.
