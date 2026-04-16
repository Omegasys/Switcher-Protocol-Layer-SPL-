# SPL Double Ratchet Security Layer

## 1. Concept

The Double Ratchet Layer in SPL is inspired by the Signal Protocol’s Double Ratchet algorithm but adapted for networking rather than messaging.

Instead of securing messages between two endpoints only, SPL applies ratcheting concepts to:
- flows
- tunnels
- optionally hop-by-hop segments

---

## 2. Goals

- Forward secrecy across network flows
- Post-compromise recovery
- Continuous key evolution
- Resistance to long-term key exposure

---

## 3. Key Evolution Model

Each session maintains:
- Root key (initial trust anchor)
- Chain keys (forward evolution)
- Ephemeral keys (per packet batch or flow segment)

---

## 4. Ratcheting Process

On each update event:

1. Derive new chain key
2. Rotate encryption keys
3. Discard old keys
4. Generate new ephemeral session state

---

## 5. Application in SPL

Unlike messaging systems, SPL applies ratcheting to:

### Flow-level encryption
- Each TCP/UDP-like flow evolves independently

### Tunnel-level encryption
- VXLAN-like overlays rotate keys periodically

### Optional hop-level ratcheting
- Each intermediate node can participate in secure evolution (restricted mode)

---

## 6. Security Benefits

- Compromise of a node does not expose historical traffic
- Keys are never reused across long periods
- Replay attacks are mitigated by evolving state

---

## 7. Design Constraint

Must balance:
- security strength
- routing performance
- hardware acceleration limitations

---
