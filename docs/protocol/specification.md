# SCM Protocol Specification

**Document Identifier**: SCM-SPEC-001
**Version**: 1.0
**Status**: Draft

---

## Status of This Memo

This document specifies the Simple Communication Middleware (SCM) protocol.
Distribution is unlimited.

---

## Abstract

This document specifies the Simple Communication Middleware (SCM) protocol, a
bidirectional Remote Procedure Call (RPC) protocol designed for communication
between a Frontend component implementing a Human-Machine Interface (HMI) and a
Backend component representing a generic apparatus.  The protocol defines four
command types: `configure`, `query`, and `execute` (initiated by the Frontend)
and `notify` (initiated by the Backend).  Messages are encoded using Protocol
Buffers and transmitted over a WebSocket connection.  The document also
specifies the Dispatcher component and its API for both Frontend and Backend
implementations.

---

## Table of Contents

1. [Introduction](#1-introduction)
2. [Terminology](#2-terminology)
3. [Operational Environment](#3-operational-environment)
4. [Protocol Overview](#4-protocol-overview)
5. [Message Encoding](#5-message-encoding)
6. [Transport Layer](#6-transport-layer)
7. [Commands](#7-commands)
   - 7.1 [configure](#71-configure)
   - 7.2 [query](#72-query)
   - 7.3 [execute](#73-execute)
   - 7.4 [notify](#74-notify)
8. [Dispatcher Component](#8-dispatcher-component)
   - 8.1 [Overview](#81-overview)
   - 8.2 [Frontend Dispatcher API](#82-frontend-dispatcher-api)
   - 8.3 [Backend Dispatcher API](#83-backend-dispatcher-api)
9. [Error Handling](#9-error-handling)
10. [Security Considerations](#10-security-considerations)
11. [Requirements Summary](#11-requirements-summary)
12. [References](#12-references)

---

## 1. Introduction

> _To be defined._

---

## 2. Terminology

> _To be defined._

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD",
"SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this
document are to be interpreted as described in BCP 14 \[RFC2119\] \[RFC8174\].

---

## 3. Operational Environment

The SCM protocol is designed for use in distributed systems composed of two
logically distinct components: a **Frontend Component** and a **Backend
Component**.  These two components are deployed independently and communicate
exclusively through SCM messages carried over a WebSocket connection.

The architecture follows the **Model-View-Controller (MVC)** design pattern:

- The Frontend Component implements the **View** layer, providing the
  Human-Machine Interface (HMI) that an operator uses to observe and control
  the system.
- The Backend Component implements the **Controller** and **Model** layers,
  encapsulating the business logic and the state of the controlled apparatus.

The strict separation between the two components means that neither component
makes assumptions about the internal implementation of the other; all
interaction is governed solely by the message contract defined in this
specification.

```
┌───────────────────────────────────────────────────────────────┐
│                    Frontend Component                         │
│                   (View — MVC Pattern)                        │
│                                                               │
│  ┌─────────────────────────────────────────────────────────┐  │
│  │  Human-Machine Interface (HMI)                          │  │
│  │  - Renders system state to the operator                 │  │
│  │  - Captures operator input (commands / configuration)   │  │
│  └───────────────────────┬─────────────────────────────────┘  │
│                          │  SCM Frontend Dispatcher API        │
│  ┌───────────────────────┴─────────────────────────────────┐  │
│  │  SCM Dispatcher (Frontend)                              │  │
│  └───────────────────────┬─────────────────────────────────┘  │
└─────────────────────────-┼────────────────────────────────────┘
                           │  WebSocket (SCM Protocol)
┌──────────────────────────┼────────────────────────────────────┐
│  ┌───────────────────────┴─────────────────────────────────┐  │
│  │  SCM Dispatcher (Backend)                               │  │
│  └───────────────────────┬─────────────────────────────────┘  │
│                          │  SCM Backend Dispatcher API         │
│  ┌───────────────────────┴─────────────────────────────────┐  │
│  │  Controller Layer                                       │  │
│  │  - Receives and validates requests from the Frontend    │  │
│  │  - Executes commands on the apparatus                   │  │
│  │  - Emits unsolicited notifications to the Frontend      │  │
│  └───────────────────────┬─────────────────────────────────┘  │
│                          │                                     │
│  ┌───────────────────────┴─────────────────────────────────┐  │
│  │  Model Layer                                            │  │
│  │  - Represents the state of the controlled apparatus     │  │
│  │  - Exposes apparatus parameters for query/configuration │  │
│  └─────────────────────────────────────────────────────────┘  │
│                    Backend Component                          │
│             (Controller + Model — MVC Pattern)                │
└───────────────────────────────────────────────────────────────┘
```

### 3.1 System Components

An SCM deployment consists of exactly two components connected by a single
WebSocket session:

| Component | MVC Role | Initiates | Receives |
|-----------|----------|-----------|---------|
| Frontend  | View | `configure`, `query`, `execute` | `notify`, responses |
| Backend   | Controller + Model | `notify` | `configure`, `query`, `execute` |

The Frontend is the **WebSocket client**; it opens the connection to the
Backend.  The Backend is the **WebSocket server**; it listens for incoming
connections and accepts exactly one Frontend session at a time (a Backend MAY
support multiple concurrent Frontend sessions depending on the deployment
scenario, but this specification does not require it).

All messages exchanged between the two components MUST conform to the encoding
rules defined in Section 5 and MUST be transported over the WebSocket
connection as specified in Section 6.

### 3.2 Frontend Component

The Frontend Component implements the **View** layer of the MVC pattern.  Its
primary responsibility is to present the current state of the apparatus to a
human operator and to relay operator actions to the Backend Component.

The Frontend Component:

- MUST establish the WebSocket connection to the Backend Component before
  sending any SCM messages.
- MUST use the SCM Dispatcher Frontend API (Section 8.2) as the sole interface
  for sending requests and receiving responses or notifications.
- MUST NOT implement business logic related to apparatus control; it SHALL
  delegate all such logic to the Backend Component by issuing `configure`,
  `query`, or `execute` requests.
- SHOULD maintain a local representation of the apparatus state, updated by
  the responses received from the Backend Component and by unsolicited
  `notify` messages.
- MAY be implemented as a web application, a desktop application, a mobile
  application, or any other software capable of establishing a WebSocket
  connection.

### 3.3 Backend Component

The Backend Component implements both the **Controller** and the **Model**
layers of the MVC pattern.  It is authoritative over the state of the
apparatus and is responsible for executing all control actions.

The Backend Component:

- MUST expose a WebSocket server endpoint that the Frontend Component can
  connect to, as specified in Section 6.3.
- MUST use the SCM Dispatcher Backend API (Section 8.3) as the sole interface
  for receiving requests and sending responses or notifications.
- MUST implement handlers for all request types it intends to support
  (`configure`, `query`, `execute`) and MUST respond to each request with the
  corresponding response message.
- MAY emit unsolicited `notify` messages at any time to inform the Frontend
  Component of state changes that were not triggered by a Frontend request
  (e.g., alarms, sensor readings, or autonomous state transitions).
- MUST NOT initiate `configure`, `query`, or `execute` requests; those message
  types are reserved for use by the Frontend Component.
- MAY be implemented as an embedded application, a server-side process, a
  microservice, or any other software capable of hosting a WebSocket server.

---

## 4. Protocol Overview

> _To be defined._

### 4.1 Communication Model

> _To be defined._

### 4.2 Command Summary

> _To be defined._

### 4.3 Request Correlation

> _To be defined._

---

## 5. Message Encoding

> _To be defined._

### 5.1 Protocol Buffers

> _To be defined._

### 5.2 Message Envelope

> _To be defined._

---

## 6. Transport Layer

> _To be defined._

### 6.1 WebSocket

> _To be defined._

### 6.2 Connection Management

> _To be defined._

### 6.3 WebSocket URL

> _To be defined._

### 6.4 Message Framing

> _To be defined._

---

## 7. Commands

> _To be defined._

### 7.1 configure

> _To be defined._

### 7.2 query

> _To be defined._

### 7.3 execute

> _To be defined._

### 7.4 notify

> _To be defined._

---

## 8. Dispatcher Component

> _To be defined._

### 8.1 Overview

> _To be defined._

### 8.2 Frontend Dispatcher API

> _To be defined._

### 8.3 Backend Dispatcher API

> _To be defined._

---

## 9. Error Handling

> _To be defined._

---

## 10. Security Considerations

> _To be defined._

---

## 11. Requirements Summary

> _To be defined._

---

## 12. References

- \[RFC2119\] Bradner, S., "Key words for use in RFCs to Indicate Requirement
  Levels", BCP 14, RFC 2119, March 1997.
- \[RFC8174\] Leiba, B., "Ambiguity of Uppercase vs Lowercase in RFC 2119 Key
  Words", BCP 14, RFC 8174, May 2017.
- \[RFC6455\] Fette, I. and A. Melnikov, "The WebSocket Protocol", RFC 6455,
  December 2011.
- \[PROTOBUF\] Google LLC, "Protocol Buffers Language Guide (proto3)",
  https://protobuf.dev/programming-guides/proto3/

---

_Document ID: SCM-SPEC-001 | Version: 1.0 | Status: Draft_
