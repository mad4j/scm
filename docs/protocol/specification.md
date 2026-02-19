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

> _To be defined._

### 3.1 System Components

> _To be defined._

### 3.2 Frontend Component

> _To be defined._

### 3.3 Backend Component

> _To be defined._

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
