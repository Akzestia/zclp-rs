# Guidelines

1. [Introduction](#introduction)
2. [Project structure & architecture](#project-structure--architecture)
3. [Function signatures](#function-signatures)
4. [Function return types](#function-return-types)
5. [Data Structures](#data-structures)
6. [Usage of Assertations](#usage-of-assertations)
7. [Naming Conventions](#naming-conventions)
8. [CI & CD](#ci--cd)
9. [Code formatting](#code-formatting)
10. [PRs & commits](#prs--commits)
11. [AI Bs](#usage-of-ai)

### Introduction

zclp - is a network protocol based on IETF QUIC v1, customized for the needs of zurui application.
The main difference being addition of new frames, p2p though NAT functionality implementation etc.
Before going further I strongly recommend reading the following 1st: RFC 8446, 8999, 9000, 9001, 9002 and Describing
QUIC Protocol Data Units with Augmented Packet Header Diagrams.

> [!NOTE]
> zclp has 2 implementations written in c++ & rust.
> For the latest updates u should check the c++ one.

### Project structure & architecture

This project is a monolith repository, structured into subfolders where each represents
a distinct functionality or feature set. Each subfolder encapsulates a specific domain of the project, ensuring
modularity and maintainability.

```bash
zclp++
├── assets
├── build
├── client
├── docs
├── scylla
├── server
├── tests
├── tokio-cpp
├── zclp++
└── zclp_utils
```

### Function signatures

Functions that accept certain `Struct` as input & produce output of certain `type` must have the following signature

```cpp
ResultType name (const Struct& in, type*& out);
```

Functions that accept certain `type` as input & produce output of certain `Struct` must have one of the following
signatures

```cpp
ResultType name (const uint8_t* in, size_t in_len, Struct& out);
```

```cpp
ResultType name (const uint8_t* in, Struct& out);
```

### Function return types

All functions that require return type must return specifically defined `ResultType` for this function/group of
functions.</br>
Minimum `ResultType` implementation must look like the following

```cpp
struct ResultType {
    type success,
    type payload,
    ..
};
```

`type` of success field is either a `bool` or an `enum`, where `enum` can represent different error codes
including success.</br>
`type` of payload can contain any type, but most of the time it will be a pointer `uint8_t*`.</br>
In addition to `success` and `paylaod` fields other can be added if necessary.

```cpp
struct ResultType{
    type success,
    type payload,
    type flags,
    type warnings
    ..
};
```

### Data Structures
```
TODO
```

### Usage of Assertations

Code must only contain static assertations. Any non-static asserts must be excluded from release builds.

### Naming Conventions

Use snake keys for both variable names and functions.

```cpp
void process_udp_pack(uint8_t* packet, ssize_t len);
```

```cpp
Structs::Connection m_req_res_con;
```

Use `m_` for private fields.

```cpp
private:
    int m_socket_fd;
    struct sockaddr_in m_addr;
    uint16_t m_port;
    const int m_max_mtu;
    zclp_tls::zclp_tls_arena m_tls;
```

### CI & CD
```
TODO
```

### Code formatting
For c++ use .clang-format file provided by the project.
For rust just use rust fmt.

### PRs & commits
```
TODO
```

### Usage of Ai
Pls don't use this shit, It is ain't good enough for this shit xD.</br>
Like you can try, but don't ^_^.