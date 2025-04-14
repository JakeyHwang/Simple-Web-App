# E-commerce Admin Panel

A minimal e-commerce admin panel built with Rust (backend) and React (frontend).

## Features

- Secure admin authentication
- Product management (CRUD operations)
- Clean, minimal, and mobile-friendly UI
- Docker deployment ready

## Tech Stack

### Backend
- Rust with Actix-web framework
- PostgreSQL database
- JWT authentication
- Docker

### Frontend
- React
- Tailwind CSS
- Axios for API calls
- React Router for navigation

## Project Structure

```
.
├── backend/           # Rust backend
│   ├── src/
│   ├── Cargo.toml
│   └── Dockerfile
├── frontend/         # React frontend
│   ├── src/
│   ├── package.json
│   └── Dockerfile
└── docker-compose.yml
```

## Getting Started

### Prerequisites
- Rust (latest stable)
- Node.js (v16 or later)
- Docker and Docker Compose
- PostgreSQL

### Development Setup

1. Clone the repository
2. Start the backend:
   ```bash
   cd backend
   cargo run
   ```

3. Start the frontend:
   ```bash
   cd frontend
   npm install
   npm run dev
   ```

### Docker Deployment

```bash
docker-compose up --build
```

## API Documentation

### Authentication
- POST /api/auth/login - Admin login
- POST /api/auth/logout - Admin logout

### Products
- GET /api/products - List all products
- POST /api/products - Create new product
- PUT /api/products/:id - Update product
- DELETE /api/products/:id - Delete product

## License

MIT
