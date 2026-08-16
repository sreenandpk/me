# Engineering Approach — Sreenand P K

> This document describes observed engineering practices from Sreenand's projects.
> It distinguishes between confirmed practices and personal philosophy (which is not assumed).

---

## Project Planning

Sreenand's stated approach: "Understand the problem, plan the work, build it, and improve it as I go. Pay attention to the details, test what I build, and try to keep the final solution clear and easy to maintain."

---

## API Design

- Uses RESTful API design principles
- Implements APIs using FastAPI or Django REST Framework depending on the project
- Documents APIs using Swagger / OpenAPI
- Structures API routes with clear separation of concerns

*Observed across: CareStream, E-Commerce, Just Listen, Microservices Platform*

---

## Backend Application Structure

- Separates concerns across distinct service layers
- In FastAPI projects: uses async Python with SQLAlchemy 2.x, Alembic for migrations, Redis, Celery, Docker
- In Django projects: uses Django ORM, Django REST Framework, structured app layout
- In microservices: uses separate repositories per service with shared Docker network and shared PostgreSQL instance (schema-per-service ownership)

*Observed across: Just Listen (FastAPI), CareStream (Django), Microservices Platform (polyrepo)*

---

## Authentication

Sreenand implements multi-layered, production-oriented authentication:

- **JWT access tokens** for stateless API authentication
- **Refresh token sessions** with hashed tokens stored in the database
- **JTI (JWT ID) tracking** to prevent token reuse and replay attacks
- **HttpOnly cookies** for refresh token delivery (XSS protection)
- **Full refresh token rotation** on every use
- **RBAC (Role-Based Access Control)** for fine-grained access at the resource level

*Observed in: Just Listen (full auth system), CareStream (RBAC for Doctors/Nurses/Admins)*

---

## Database Design

- Uses PostgreSQL as the primary relational database
- Uses SQLAlchemy 2.x as the ORM in FastAPI projects
- Uses Alembic for schema migrations
- In microservices architecture: shared PostgreSQL instance with separate schemas owned by each service

*Observed across: All projects*

---

## Testing

- Uses **pytest** for unit and integration tests
- Uses **Locust** for performance and load testing
- Uses **security scanning** in CI pipelines
- Enforces testing through CI gates

*Observed in: Just Listen (confirmed pytest + Locust + security scanning)*

> Note: Testing in CareStream and E-Commerce was not confirmed — do not claim it.

---

## Security

- Security is considered from the start of development
- RBAC enforced at the API layer
- Refresh tokens hashed before storage (never stored in plain text)
- HttpOnly cookies prevent client-side JavaScript from accessing tokens
- JTI tracking ensures a refresh token cannot be replayed after rotation
- Access and refresh token separation

*Observed across: Just Listen (full auth), CareStream (RBAC)*

---

## Performance

- Uses Redis for caching frequently accessed data
- Uses Celery to offload slow/heavy tasks to background workers
- Uses async Python with FastAPI for non-blocking I/O
- Uses Locust for load testing to identify performance bottlenecks

*Observed across: Just Listen, CareStream, Microservices Platform*

---

## Docker

- Containerizes all application services
- In microservices projects: uses a shared Docker network so services can communicate
- Infrastructure services (PostgreSQL, Redis) run as Docker services within the network

*Observed across: Just Listen, Microservices Platform*

---

## Debugging

Personal debugging approach not confirmed — omitted.

---

## Git & Version Control

- Uses GitHub with professional branch-based workflows
- Uses pull requests and code reviews
- Branch protection rules enforced
- Uses GitHub Organizations for team/project management
- Uses GitHub Actions for CI/CD automation

*Observed from skills and project context*

---

## Code Review & Code Quality

- Ruff for linting and auto-formatting (Python)
- mypy for static type checking
- pre-commit hooks to enforce quality before commits
- CodeRabbit AI for automated code review assistance
- CI gates prevent merging code that fails quality checks

*Observed in: Just Listen*

---

## CI/CD

- GitHub Actions used for CI pipelines
- CI enforces: code quality (Ruff, mypy), tests (pytest), security scanning

*Observed in: Just Listen (confirmed CI/CD)*

> Note: CI/CD at Bridgeon was not confirmed — do not attribute personal tooling to Bridgeon.
