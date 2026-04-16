Switcher Protocol Layer (SPL) is an experimental networking architecture designed to unify and modernize how packets are switched, routed, secured, and optimized across distributed systems.

Traditional networks separate switching (L2), routing (L3), overlay tunneling, security, and control logic into distinct and often rigid layers. SPL replaces this fragmentation with a single adaptive protocol layer that treats the network as a dynamic graph of programmable nodes and links.

SPL integrates concepts from Ethernet switching, Internet routing protocols (OSPF, BGP, IS-IS), software-defined networking (SDN), and modern overlay technologies (VXLAN/EVPN-style systems). On top of this foundation, it introduces an AI-driven routing engine that continuously optimizes traffic flow using real-time telemetry, congestion prediction, and policy-based decision making.

Security is a first-class design goal. SPL incorporates a multi-layer cryptographic model, including session-based key evolution inspired by the Double Ratchet algorithm, alongside traditional transport and link encryption strategies.

The system is designed to be telemetry-first, programmable, and self-adaptive, enabling networks that can observe, learn, and reconfigure themselves in response to changing conditions.

Key Goals
Unify switching, routing, and overlay networking into a single protocol layer
Enable AI-driven adaptive routing and congestion control
Support programmable data and control planes
Provide multi-layer security with dynamic key evolution concepts
Maintain real-time telemetry feedback loops for closed-loop optimization
Enable hybrid distributed + centralized network control models
Core Inspirations
Ethernet (IEEE 802.3, VLANs)
OSPF, IS-IS, BGP
VXLAN / EVPN / MPLS
Software Defined Networking (SDN)
P4 programmable data planes
WireGuard / IPsec / MACsec
Signal Protocol Double Ratchet (conceptual security model)
Intent-Based Networking systems
Architecture Overview

SPL is structured into modular subsystems:

Switching Engine – L2 forwarding and MAC/VLAN management
Routing Core – graph-based path computation and segment routing
AI Layer – predictive traffic optimization and adaptive policies
Security Layer – session encryption and dynamic key evolution
Overlay Network – virtualized network tunnels and segmentation
Telemetry System – real-time network state and analytics
Control Plane – distributed and centralized coordination logic
Data Plane – packet processing and forwarding execution
Status

This project is in early-stage design and research. It is intended as an experimental architecture for exploring future network protocol designs that combine programmability, intelligence, and security.
