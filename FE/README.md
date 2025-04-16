# Copilot Metrics Viewer Frontend

A Vue.js application for visualizing GitHub Copilot metrics and managing RBAC (Role-Based Access Control).

## Project Overview

This frontend application provides a user interface for:

- Visualizing GitHub Copilot usage metrics across teams and organizations
- Analyzing developer productivity and code quality metrics
- Tracking DORA (DevOps Research and Assessment) metrics
- Managing users, roles, and departments (RBAC)

## Project Structure

```
src/
├── api/                  # API endpoints and configurations
├── assets/              # Static assets (images, fonts, etc.)
├── common/              # Common utilities and shared code
├── components/          # Reusable UI components
├── config.ts           # Application configuration
├── layouts/            # Layout components
├── lib/                # Library code and utilities
├── main.ts            # Application entry point
├── model/             # Data models and interfaces
├── plugins/           # Vue plugins
├── router/            # Vue Router configuration
├── services/          # API and other services
├── store/             # Vuex store modules
├── types/             # TypeScript type definitions
├── utils/             # Utility functions
└── views/             # Page/screen components
```

## Tech Stack

- Vue 3 with TypeScript
- Vite as build tool
- Vue Router for routing
- Vuex for state management
- Vuetify 3 for UI components
- Chart.js for data visualization
- Tailwind CSS for styling
- Axios for API requests

## Getting Started

### Prerequisites

- Node.js 14.x or higher
- npm 7.x or higher

### Installation

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

### Development Commands

```bash
# Lint code
npm run lint

# Format code
npm run format
```

### Environment Configuration

Copy `.env.example` to `.env` and configure the following variables:

```env
VITE_API_URL=your_api_url
VITE_USE_MOCK_DATA=false
```

## Docker Support

The application can be containerized using Docker:

```bash
# Build Docker image
docker build -t copilot-metrics-viewer .

# Run container
docker run -p 8080:80 --env-file ./.env copilot-metrics-viewer
```

## Key Features

1. **Authentication & Authorization**

   - Role-based access control (RBAC)
   - Secure login/logout functionality
   - User session management

2. **Metrics Visualization**

   - GitHub Copilot usage metrics
   - DORA metrics dashboard
   - Developer productivity analytics
   - Language breakdown analysis
   - Seat utilization tracking

3. **Organization Management**
   - Team management
   - Department structure
   - User role assignments

## Development Guidelines

1. **Code Style**

   - Follow Vue.js style guide
   - Use TypeScript for type safety
   - Follow ESLint and Prettier configurations

2. **Git Workflow**

   - Create feature branches from `develop`
   - Use conventional commits
   - Submit PRs for review

3. **Testing**
   - Write unit tests for new components
   - Test API integrations
   - Ensure responsive design

## Documentation

Additional documentation can be found in:

- `NEXT_STEPS.md` - Future development plans
- `IMPLEMENTATION_SUMMARY.md` - Implementation details
- `VIEW_STRUCTURE_REFACTORING.md` - UI/UX improvements
- `SUPPORT.md` - Support information
- `SECURITY.md` - Security guidelines
- `LEGACY_README.md` - Legacy documentation and detailed metrics information

## License

This project is licensed under the terms specified in `LICENSE.txt`.

## Support

For support and questions, please refer to `SUPPORT.md` or create an issue in the repository.

## Implementation Status

### Completed Features

- Authentication flow with login/logout functionality
- Dashboard view with navigation to different metric pages
- DORA metrics visualization components
- Application layout and navigation structure
- Type-safe API service for backend communication

### In Progress

- Developer metrics visualization
- Organization and team views
- RBAC management interface
- Data analysis views
