# SPL Security Model

## 1. Overview

SPL implements a multi-layer security architecture combining:
- link encryption
- tunnel encryption
- session-based cryptographic evolution

---

## 2. Security Layers

### 2.1 Link Layer Security
- MACsec-like encryption
- Protects individual physical links

### 2.2 Tunnel Security
- WireGuard/IPsec-like encrypted overlays
- Protects multi-hop paths

### 2.3 Session Security
- Per-flow encryption context
- Dynamic key evolution (ratcheting model)

---

## 3. Identity Model

Each SPL node has:
- cryptographic identity keypair
- rotating session identifiers
- trust score (AI-influenced optional layer)

---

## 4. Threat Model

SPL assumes resistance against:
- passive eavesdropping
- packet injection
- replay attacks
- compromised intermediate nodes

---

## 5. Key Management

- Keys are rotated based on:
  - time intervals
  - packet volume
  - anomaly detection triggers

---

## 6. Integrity Protection

- All packets include authentication tags
- Optional per-hop validation in secure modes

---

## 7. Security Principle

Security is not a static layer—it is a **continuous evolving state machine** integrated into routing and forwarding decisions.

---
