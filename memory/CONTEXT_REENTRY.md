# Switcher Protocol Layer (SPL) — Context Reentry File

## 🧠 What this project is

Switcher Protocol Layer (SPL) is an experimental network architecture that unifies:

- Ethernet switching (Layer 2)
- Routing protocols (Layer 3)
- Overlay networking (VXLAN-like virtual networks)
- AI-driven adaptive routing
- Telemetry-first feedback loops
- Multi-layer security including Double Ratchet-inspired key evolution
- SDN-style control + programmable data plane

It is designed as a **graph-based, self-adapting network system** rather than a traditional layered stack.

---

## 🏗 Core Architecture

SPL is divided into six major subsystems:

### 1. Switching Layer
- MAC learning tables
- VLAN management
- L2 forwarding engine

### 2. Routing Layer
- Graph-based routing engine
- Segment routing model
- Congestion-aware path selection

### 3. Overlay Layer
- VXLAN-like encapsulation system
- Virtual networks (VNI-based isolation)
- Tunnel management system

### 4. AI Layer
- Reinforcement learning routing agent
- Policy engine for scoring paths
- Traffic prediction model
- Anomaly detection system

### 5. Security Layer
- Session-based encryption model
- Key rotation system
- Identity + trust scoring
- Double Ratchet-inspired key evolution

### 6. Control + Data Plane
- Intent-based control API
- Distributed consensus model
- Packet parsing and forwarding engine
- Hardware abstraction layer

---

## 📡 Telemetry System

SPL is telemetry-first:

- Flow-level monitoring
- Link utilization tracking
- Node health metrics
- Topology change detection

Telemetry feeds directly into:
- AI routing engine
- control plane policy decisions
- anomaly detection system

---

## 🌐 Overlay Model

- VXLAN-like virtual networks
- Tunnel abstraction layer
- Dynamic node membership
- Multi-tenant network isolation

---

## 🤖 AI Routing Model

Routing decisions are based on:

- latency
- congestion
- packet loss
- predicted traffic trends

Mechanisms:
- reinforcement learning router
- policy-based scoring engine
- adaptive exploration/exploitation balance

Fallback:
- deterministic graph routing (BFS/Dijkstra-style)

---

## 🔐 Security Model

SPL uses layered security:

- Link encryption (MACsec-like concept)
- Tunnel encryption (WireGuard/IPsec-like concept)
- Session-level encryption (flow-based keys)
- Identity system with trust scoring

### Double Ratchet Concept
- Keys evolve continuously
- Forward secrecy across flows
- Compromise resistance over time

---

## ⚙ Control Plane

- Intent-based networking API
- Policy dispatcher system
- Distributed consensus engine
- Centralized controller abstraction

Example intents:
- reduce latency
- minimize congestion
- increase reliability
- optimize cost

---

## 📦 Data Plane

- Packet parsing system
- Forwarding engine
- Rule execution engine
- Hardware abstraction layer

---

## 🧪 Tools & Simulation

- Network emulator (topology + delay simulation)
- Topology generator (random graph creation)
- Full system simulator
- Debug CLI interface

---

## ⚙ Config Profiles

- default.toml → balanced mode
- dev.toml → lightweight simulation mode
- secure.toml → full security + ratcheting
- ai_tuning.toml → AI behavior tuning

---

## 🚧 Current Status

SPL is currently in:
> Early-stage architectural + simulation implementation

It is not a production networking protocol, but a **research system for next-generation adaptive networking design**.

---

## 🔄 How to resume in a new chat

Paste this file and say:

> "Continue SPL from CONTEXT_REENTRY.md"

Then the system can reconstruct:
- architecture
- modules
- design intent
- current implementation state

---

## 🧠 Open design questions

- Should routing be fully AI-driven or hybrid deterministic?
- Should double ratchet operate per-flow or per-tunnel?
- Should control plane be fully decentralized or hybrid SDN?
- Should data plane support programmable packet logic (P4-like)?

---
