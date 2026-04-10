# Qalbaani 

### *Two hearts connected*

---

### 1. Project Description
This repository contains a strictly **educational and experimental** project. The primary objective is to study the integration between a native mobile interface and a high-performance server using the **Rust** programming language.

The application functions as a private, one-on-one chat platform, serving as a laboratory for practicing systems architecture, state management, and network communication.

---

### 2. System Architecture
The project utilizes a **monorepo** structure to facilitate synchronized versioning between the two development fronts:

* **/android**: Native frontend developed specifically for the **Android** platform using Android Studio. (Note: Currently, support is exclusive to Android).
* **/backend**: High-performance server developed in **Rust**, focused on memory safety and efficient concurrency.

---

### 3. Tech Stack
* **Mobile**: Kotlin / Android SDK
* **Server**: Rust (utilizing frameworks like Tokio / Axum or Actix-web)
* **Communication Protocol**: JSON via HTTP / WebSockets
* **Dependency Management**: Cargo (Rust) and Gradle (Android)

---

### 4. Development Status
> **Note**: This is a **learning project**.
> * The code is not intended