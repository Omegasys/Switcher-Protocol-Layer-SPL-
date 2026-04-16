# SPL Architecture Overview

Switcher Protocol Layer (SPL) is a unified networking architecture that merges switching, routing, overlay networking, security, and AI-driven optimization into a single adaptive protocol layer.

## 1. Design Philosophy

Traditional networks separate functionality into rigid layers:
- Layer 2: Switching (Ethernet)
- Layer 3: Routing (IP)
- Overlay: VXLAN/GENEVE tunnels
- Security: IPsec/TLS/MACsec
- Control: SDN/management systems

SPL collapses these into a **single programmable network model** based on a dynamic graph abstraction.

## 2. Core Model

The network is represented as:

- **Nodes**: switches, routers, or SPL agents
- **Edges**: physical or virtual links
- **Weights**: dynamic cost metrics (latency, congestion, trust, energy)

This graph is continuously updated using telemetry feedback.

## 3. System Layers

### 3.1 Data Plane
Responsible for:
- Packet forwarding
- Header parsing
- Rule execution
- Fast-path switching decisions

### 3.2 Control Plane
Responsible for:
- Topology management
- Routing computation
- Policy enforcement
- Network state synchronization

### 3.3 AI Plane
Responsible for:
- Traffic prediction
- Route optimization
- Anomaly detection
- Adaptive policy tuning

### 3.4 Security Plane
Responsible for:
- Encryption
- Key rotation
- Identity validation
- Secure session management

## 4. Key Innovation

Unlike traditional networks, SPL uses:
- Unified switching + routing engine
- Continuous learning loop
- Real-time topology adaptation
- Cryptographic state evolution

## 5. Execution Model

Packets are processed as:
1. Ingress parsing
2. Graph-based route selection
3. Policy + AI adjustment
4. Secure encapsulation
5. Forwarding execution

---
