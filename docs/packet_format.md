# SPL Packet Format Specification

## 1. Overview

SPL uses a flexible, extensible packet format designed for:
- Unified switching and routing
- AI-assisted forwarding metadata
- Built-in security context
- Overlay compatibility

## 2. Packet Structure
+------------------------------------------------------+
| SPL Header |
+------------------------------------------------------+
| Security Layer Header |
+------------------------------------------------------+
| Routing Metadata |
+------------------------------------------------------+
| AI Context Block |
+------------------------------------------------------+
| Payload |
+------------------------------------------------------+


---

## 3. SPL Header

Fields:
- Version
- Packet Type (data, control, telemetry)
- Network ID
- Flow ID
- TTL (hop limit)

---

## 4. Routing Metadata

Contains:
- Path vector (graph-based route hint)
- Next-hop suggestion
- Segment routing labels (optional)
- QoS class

---

## 5. AI Context Block

Used for adaptive routing systems:

- Congestion score
- Predicted latency
- Model confidence value
- Policy tag ID

This block may be ignored by legacy nodes.

---

## 6. Security Header

Includes:
- Session ID
- Key epoch (ratcheting version)
- Encryption scheme ID
- Integrity tag

---

## 7. Design Principle

SPL packets are:
- Forward-compatible
- Extensible via optional headers
- Optimized for programmable parsing engines (P4-like systems)
