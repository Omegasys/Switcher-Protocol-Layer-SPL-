# SPL AI Routing Model

## 1. Overview

SPL replaces static routing metrics with an AI-driven adaptive routing system that continuously learns network behavior.

---

## 2. Routing Problem Definition

The network is modeled as a graph:

- Nodes = routers/switches
- Edges = links with dynamic weights

Goal:
Minimize:
- latency
- congestion
- packet loss
- energy cost (optional)

---

## 3. AI Model Type

SPL supports hybrid models:

### 3.1 Reinforcement Learning Agent
- State: network graph snapshot
- Action: path selection
- Reward:
  - low latency (+)
  - packet loss (-)
  - congestion avoidance (+)

### 3.2 Supervised Prediction Layer
- Predicts:
  - link failure probability
  - congestion trends
  - traffic bursts

---

## 4. Routing Decision Flow

1. Collect telemetry snapshot
2. Encode network graph state
3. Run inference model
4. Generate candidate paths
5. Score paths using policy engine
6. Select optimal route

---

## 5. Exploration vs Exploitation

SPL uses adaptive balancing:
- Explore new routes under uncertainty
- Exploit known optimal paths under stability

---

## 6. Continuous Learning Loop

- Real-time telemetry feeds training buffer
- Model updates occur in rolling windows
- Policy updates propagate across nodes

---

## 7. Failure Handling

If AI is unavailable:
- fallback to deterministic routing (BGP/OSPF-like behavior)

---
