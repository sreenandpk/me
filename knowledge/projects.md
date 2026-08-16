# Projects — Sreenand P K

---

## 1. CareStream

**Type:** Prototype / Demonstration Project  
**Subtitle:** Live Patient Health Monitoring System

### Purpose
A web platform that lets doctors and nurses monitor patients' vital signs — such as heart rate and oxygen levels — in real time, directly from their screen.

### Problem Solved
Traditional monitoring systems showed patient data only when someone manually checked it, causing dangerous delays in noticing critical changes in a patient's condition. There was also no way to control who could access sensitive patient information.

### Solution
Built a live data feed that streams vital signs from physical/simulated sensors directly to the doctors' dashboard — no manual refresh needed. Added an intelligent system using machine learning to automatically spot unusual readings and alert the right medical staff immediately. Secured the platform so only authorized doctors, nurses, and admins can access patient records. Deployed the system to the cloud.

### Technologies
- **Frontend:** Next.js, Vercel
- **Backend:** Django REST Framework
- **Database:** PostgreSQL (hosted on AWS RDS)
- **Cache / Broker:** Redis
- **Background Jobs:** Celery
- **Real-time:** WebSockets
- **Machine Learning:** Scikit-Learn (Isolation Forest for anomaly detection on vital sign data)
- **Infrastructure:** AWS ECS (backend), AWS RDS (PostgreSQL), Vercel (frontend)
- **IoT Integration:** Supports simulated and live ESP32/MAX30102 sensor data

### Architecture & Data Flow
Physical/simulated sensors → WebSocket stream → Django backend → PostgreSQL storage → Next.js dashboard (real-time). Scikit-Learn Isolation Forest analyzes incoming vital data and triggers alerts when anomalies are detected.

### Authentication & Security (IMPLEMENTED)
- Role-based access control (RBAC): Doctors, Nurses, Admins
- Only authorized roles can access patient records

### Machine Learning (IMPLEMENTED)
- Scikit-Learn Isolation Forest used for anomaly detection / analysis of patient vital data
- Can be publicly mentioned as part of the project's intelligent vital-data analysis

### Deployment (IMPLEMENTED)
- AWS ECS: Backend deployment
- AWS RDS: PostgreSQL database
- Vercel: Frontend deployment
- Note: CareStream is a prototype, not a production healthcare system

### Testing
- OMITTED — not confirmed for this project

### Metrics
- OMITTED — no confirmed real metrics

### Links
- **GitHub:** https://github.com/sreenandpk/carestream
- **Live URL:** https://care-stream.vercel.app/docs

---

## 2. E-Commerce Platform

**Subtitle:** Full Online Shopping Experience

### Purpose
A complete online store where customers can browse products, manage their cart and wishlist, place orders, and track purchases — while store admins manage inventory, categories, and orders from a dedicated dashboard.

### Problem Solved
Building a shopping platform that is fast, secure, and works seamlessly for both customers and admins. User accounts and sessions need to be safe, the product catalog needs to handle many items, and the experience needs to work on any device.

### User Functionality (IMPLEMENTED)
- Browse product catalog
- Manage cart and wishlist
- Place orders
- Track purchases
- Secure account and session management

### Admin Functionality (IMPLEMENTED)
- Manage inventory
- Manage categories
- Manage orders
- Dedicated admin panel — no code changes needed

### Authentication (IMPLEMENTED)
- JWT (JSON Web Tokens)

### Payment Integration
- OMITTED — not confirmed for this project

### Technologies
- **Frontend:** React.js, Bootstrap 5, Framer Motion, Vercel
- **Backend:** Django REST Framework
- **Database:** PostgreSQL
- **Auth:** JWT

### Deployment (IMPLEMENTED)
- Frontend: Vercel

### Testing
- OMITTED — not confirmed for this project

### Technical Challenges
- Complex state management for cart and user sessions
- Secure JWT-based authentication across frontend and backend
- Responsive UI that works on any device

### Links
- **GitHub:** https://github.com/sreenandpk/ecommerce
- **Live URL:** https://ecommerce-django-frontend-lhvj.vercel.app

---

## 3. Just Listen

**Status:** In progress / actively developed  
**Type:** Personal / Production-oriented backend project

### Purpose
Just Listen is a FastAPI-based backend application focused on disciplined decision-making and analysis. It is designed as a modern asynchronous backend system demonstrating production-oriented backend engineering practices.

### Engineering Focus
The project demonstrates real production-level backend practices including:
- Modern async Python architecture
- PostgreSQL with SQLAlchemy 2.x and Alembic
- Secure authentication with refresh-token sessions
- Redis caching / brokering
- Background job processing with Celery
- Docker containerization
- Comprehensive testing (pytest + Locust performance testing)
- Security scanning and CI enforcement
- Code quality tooling (Ruff, mypy, pre-commit)

### Backend Architecture (IMPLEMENTED)
- **Framework:** FastAPI (async Python)
- **Database:** PostgreSQL
- **ORM:** SQLAlchemy 2.x
- **Migrations:** Alembic
- **Cache / Broker:** Redis
- **Background Jobs:** Celery

### Authentication & Session Management (IMPLEMENTED)
- JWT access tokens
- Refresh-token sessions
- Hashed refresh tokens stored in the database
- JTI (JWT ID) tracking to prevent token reuse / replay attacks
- HttpOnly cookies for refresh tokens
- Full refresh-token rotation on every use

### Testing (IMPLEMENTED)
- pytest (unit / integration tests)
- Locust (performance / load testing)
- Security scanning in CI pipeline

### Code Quality (IMPLEMENTED)
- Ruff (linter and formatter)
- mypy (static type checking)
- pre-commit hooks
- CI enforcement of code quality gates

### Infrastructure (IMPLEMENTED)
- Docker containerization for all services

### Deployment
- No confirmed public live URL

### Links
- **GitHub:** Not publicly confirmed yet
- **Live URL:** None

---

## 4. Trading / Market Microservices Platform

**Status:** In active development  
**Type:** Personal / Architecture exploration project

### Purpose
A backend trading/market platform built using a multi-repository microservices architecture. Demonstrates service-oriented backend engineering with shared infrastructure, independent service deployment, and clean service boundary design.

### Architecture Overview (IMPLEMENTED)
- **Structure:** Polyrepo (multi-repository) — each service has its own repository
- **Communication:** All services run on a shared Docker network
- **Database:** Shared PostgreSQL instance; each service owns its own schema
- **Infrastructure:** Centralized in a dedicated infrastructure repository

### Implemented Services

#### infrastructure (IMPLEMENTED)
- PostgreSQL setup and schema management
- Redis setup
- Docker network configuration
- Shared environment for all services

#### authentication-service (IMPLEMENTED)
- Dedicated microservice for authentication
- Handles user identity and access

#### market-service (IMPLEMENTED)
- Dedicated microservice for market-related functionality

### Planned Services (NOT YET IMPLEMENTED)
The following are planned but not built:
- strategy-service
- risk-service
- analytics-service
- API gateway

### Infrastructure Details (IMPLEMENTED)
- **PostgreSQL:** Shared instance, separate schema per service
- **Redis:** Shared infrastructure layer
- **Celery:** Queues for background processing
- **Docker:** All services containerized with a shared Docker network

### Testing
- Not confirmed — omitted

### CI/CD
- Not confirmed — omitted

### Observability / Logging
- Not confirmed — omitted

### Security
- Not confirmed — omitted

### GitHub Repositories
- Not publicly confirmed yet — omitted

---

> **Note to RAG system:** Do NOT confuse project-specific technologies. For example: Scikit-Learn belongs only to CareStream. Locust belongs only to Just Listen. AWS ECS/RDS belongs only to CareStream. Do not cross-assign technologies between projects.
