# SDD Navigator — Infrastructure

Kubernetes deployment infrastructure for the SDD Navigator stack: **Rust API**, **PostgreSQL**, and **Next.js frontend**.

## Architecture

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   Next.js    │────▶│   Rust API   │────▶│  PostgreSQL  │
│  (Frontend)  │     │  (Actix-web) │     │  (Bitnami)   │
└──────────────┘     └──────────────┘     └──────────────┘
     :3000                :8080                :5432
```

## Project Structure

```
sdd-navigator/
├── apps/                    # Application source code
│   ├── api/                 # Rust API (Actix-web + SQLx)
│   └── frontend/            # Next.js 14 frontend
├── helm/sdd-navigator/      # Helm umbrella chart
│   ├── charts/api/          # API sub-chart
│   ├── charts/frontend/     # Frontend sub-chart
│   ├── values-dev.yaml      # Dev overrides
│   └── values-prod.yaml     # Prod overrides
├── ansible/                 # Ansible automation
│   ├── playbooks/           # deploy, setup-cluster, rollback
│   ├── roles/               # helm-deploy, k8s-namespace, k8s-prerequisites
│   ├── inventories/         # dev, prod
│   └── group_vars/          # Environment variables
└── .github/workflows/       # CI pipeline
```

## Quick Start

### Prerequisites

- `kubectl` configured with cluster access
- `helm` v3.x
- `ansible` 2.15+
- `docker` (for building images)

### 1. Build Docker images

```bash
# API
docker build -t sdd-navigator/api:latest apps/api/

# Frontend
docker build -t sdd-navigator/frontend:latest apps/frontend/
```

### 2. Setup cluster (first time)

```bash
cd ansible
ansible-playbook -i inventories/dev/hosts.yml playbooks/setup-cluster.yml
```

### 3. Deploy

```bash
cd ansible
ansible-playbook -i inventories/dev/hosts.yml playbooks/deploy.yml
```

### 4. Rollback (if needed)

```bash
cd ansible
ansible-playbook -i inventories/dev/hosts.yml playbooks/rollback.yml
```

### Manual Helm deploy (without Ansible)

```bash
cd helm/sdd-navigator
helm dependency update .
helm upgrade --install sdd-navigator . \
  --namespace sdd-navigator-dev \
  --create-namespace \
  -f values-dev.yaml \
  --wait
```

## CI Pipeline

The GitHub Actions workflow (`.github/workflows/ci-infra.yml`) runs on every push/PR to `main` and validates:

| Check | Tool | What it validates |
|-------|------|-------------------|
| YAML Lint | `yamllint` | All YAML syntax |
| Helm Lint | `helm lint` | Chart structure and values |
| Helm Template | `helm template` | Template rendering (dev + prod) |
| Ansible Lint | `ansible-lint` | Playbook best practices |
| Dockerfile Lint | `hadolint` | Dockerfile best practices |

## Environments

| Parameter | Dev | Prod |
|-----------|-----|------|
| Namespace | `sdd-navigator-dev` | `sdd-navigator-prod` |
| API replicas | 1 | 3 (HPA: 3-10) |
| Frontend replicas | 1 | 3 |
| Ingress | Disabled | Enabled + TLS |
| DB storage | 1Gi | 20Gi |
| Logging | `debug` | `info` |
