# Northwind API
This repository contains a **modular Rust backend** built as a Cargo workspace.
The goal is to implement a clean, scalable API using **domain modules** and shared **platform infrastructure**.
The API serves Northwind data.

The project is structured as a **modular monolith**, where each domain lives in its own crate and exposes its own HTTP router. The main API binary composes these routers into a single application.

---

# Workspace Structure

```
.
├── apps
│   └── api                # API binary crate (composition root)
│
├── crates
│   ├── customers          # Customer domain module
│   └── platform           # Shared infrastructure
│
├── Cargo.toml             # Workspace configuration
└── Cargo.lock
```

---

# Crates

## `apps/api`

The **API binary crate**.
Responsible for:

* Bootstrapping the application
* Composing routers from domain modules
* Starting the HTTP server

---

## `crates/customers`

Domain module for **Customers**.

Structure:

```
customers
├── api
│   ├── handlers.rs
│   └── router.rs
├── domain
├── repository
└── service
```

Layers:

* **api** – HTTP layer (handlers and routers)
* **service** – application/business logic
* **repository** – database access
* **domain** – domain models and rules

The module exposes an HTTP router that is mounted by the API binary.

---

## `crates/platform`

Shared infrastructure used by all domains.

Responsibilities:

* database configuration
* application errors
* middleware
* telemetry / tracing

Structure:

```
platform
├── database
├── errors
├── middleware
└── telemetry
```

---

# Development

## Build

```
cargo build
```

## Run API

```
cargo run -p api
```

The API will start on:

```
http://localhost:3000
```

Example endpoint:

```
GET /customers
```

---

# Workspace Dependencies

Shared dependencies are defined in the workspace root `Cargo.toml`:

```
[workspace.dependencies]
```

Crates reference them using:

```
dependency = { workspace = true }
```

---

# Debugging / Sharing Project Structure

The following command outputs the **entire project structure and relevant file contents** into a single file.

This is useful for debugging, sharing the project, or providing context when asking for help.

```
(
echo "PROJECT STRUCTURE"
echo "================="
tree -a -I "target|.git|node_modules"

echo "\nFILE CONTENTS"
echo "============="

find . \
-type f \
\( -name "*.rs" -o -name "*.toml" -o -name "*.sql" \) \
-not -path "*/target/*" \
-not -path "*/.git/*" \
| sort \
| while read file; do
    echo "\n===== $file ====="
    cat "$file"
done
) > rust_project_dump.txt
```

This generates:

```
rust_project_dump.txt
```

which contains:

* the folder structure
* the contents of all Rust, TOML, and SQL files

---

# Design Goals

* modular domain architecture
* clear separation of concerns
* reusable domain crates
* centralized infrastructure
* maintainable workspace dependency management

This structure allows the project to grow easily as more domains are added (e.g., `orders`, `products`, `auth`).

---

# Future Work

Planned additions include:

* database integration
* repository implementations
* application services
* structured logging / tracing
* middleware
* additional domain modules
