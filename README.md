# SDD Navigator — Infrastructure

Kubernetes deployment infrastructure for the SDD Navigator stack: **Rust API**, **PostgreSQL**, and **Next.js frontend**.

![Architecture Diagram](assets/architecture.png)

## Architecture

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   Next.js    │────▶│   Rust API   │────▶│  PostgreSQL  │
│  (Frontend)  │     │  (Actix-web) │     │  (Bitnami)   │
└──────────────┘     └──────────────┘     └──────────────┘
     :3000                :8080                :5432
```

## Recent Updates & Fixes
- **Rust Toolchain**: Upgraded to `1.88-slim` and fixed `sqlx` compatibility issues.
- **Frontend Docker**: Added `.gitkeep` to `public/` directory to prevent build failures.
- **Local Dev**: Verified and tested local deployment using **Kind** and **Helm**.
- **CI/CD**: Fully linted and validated all infrastructure manifests (Helm & Ansible).

## Project Structure
```
sdd-navigator/
├── apps/                    # Application source code
│   ├── api/                 # Rust API (Actix-web + SQLx)
│   └── frontend/            # Next.js 14 frontend
├── helm/sdd-navigator/      # Helm umbrella chart
│   ├── charts/api/          # API sub-chart
│   ├── charts/frontend/     # Frontend sub-chart
│   ├── values-dev.yaml      # Dev overrides (optimized for Kind)
│   └── values-prod.yaml     # Prod overrides
├── ansible/                 # Ansible automation
│   ├── playbooks/           # deploy, setup-cluster, rollback
│   └── inventory/           # dev, prod hosts
└── .github/workflows/       # CI pipeline
```

## Local Development (Kind)

### 1. Prerequisites
- Docker Desktop
- `kind`, `kubectl`, `helm`

### 2. Build & Load Images
```powershell
# Build
docker build -t sdd-navigator/api:latest apps/api/
docker build -t sdd-navigator/frontend:latest apps/frontend/

# Load into Kind (use .tar archive if direct load fails on Windows)
docker save sdd-navigator/api:latest -o api.tar
kind load image-archive api.tar --name sdd-navigator
```

### 3. Deploy
```powershell
cd helm/sdd-navigator
helm upgrade --install sdd-navigator . -n sdd-navigator-dev -f values-dev.yaml --wait
```

### 4. Access
```powershell
# Frontend
kubectl port-forward -n sdd-navigator-dev svc/sdd-navigator-frontend 3000:3000
# API
kubectl port-forward -n sdd-navigator-dev svc/sdd-navigator-api 8080:8080
```

## CI Pipeline
The GitHub Actions workflow validates YAML, Helm charts, Ansible playbooks, and Dockerfiles on every push.

## Environments
| Parameter | Dev (Kind) | Prod |
|-----------|-----------|------|
| Namespace | `sdd-navigator-dev` | `sdd-navigator-prod` |
| Image Policy | `IfNotPresent` | `Always` |
| DB storage | 1Gi | 20Gi |
