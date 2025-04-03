# View Structure Refactoring Tasks

## Overview

We need to refactor our current view structure to match the target structure outlined in the implementation summary. This will involve reorganizing existing views and creating necessary folder structures.

## Current Structure

```
src/
└── views/
    ├── LoginView.vue
    └── DashboardView.vue
```

## Target Structure

```
src/
└── views/
    ├── auth/
    │   └── LoginView.vue
    ├── dashboard/
    │   └── DashboardView.vue
    ├── metrics/
    │   ├── DeveloperMetricsView.vue
    │   ├── CollaborationQualityView.vue
    │   └── DeliveryInsightsView.vue (via DoraMetricsView.vue)
    └── rbac/
        ├── UsersView.vue
        ├── RolesView.vue
        └── DepartmentsView.vue
```

## Tasks

### 1. Create Folder Structure

- [x] Create `src/views/auth` directory
- [x] Create `src/views/dashboard` directory
- [x] Create `src/views/metrics` directory
- [x] Create `src/views/rbac` directory

### 2. Move Existing Views

- [x] Move `src/views/LoginView.vue` to `src/views/auth/LoginView.vue`
- [x] Move `src/views/DashboardView.vue` to `src/views/dashboard/DashboardView.vue`

### 3. Update Router References

- [x] Update router import paths for the moved views:
  - [x] Change `import LoginView from '../views/LoginView.vue'` to `import LoginView from '../views/auth/LoginView.vue'`
  - [x] Change `import DashboardView from '../views/DashboardView.vue'` to `import DashboardView from '../views/dashboard/DashboardView.vue'`

### 4. Create Layout Structure

- [x] Create `src/layouts` directory
- [x] Create `src/layouts/AppLayout.vue` for the main application layout
- [x] Create `src/layouts/AuthLayout.vue` for authentication pages

### 5. Refactor Components Directory

- [x] Create `src/components/common` directory for reusable UI components
- [x] Create `src/components/dashboard` directory for dashboard-specific components
- [x] Create `src/components/metrics` directory for metrics visualization components
- [x] Create `src/components/rbac` directory for RBAC management components

### 6. Handle DoraDashboard Component

- [x] Assess whether `DoraDashboard.vue` belongs in metrics views or components
- [x] Move to appropriate location based on its functionality and usage
- [x] Create a proper view component (`DoraMetricsView`) that uses the DoraDashboard component
- [x] Update the router to use the new view component

## Completed Actions

1. Created the new directory structure for views and components
2. Moved LoginView and DashboardView to their proper locations
3. Created layout components (AuthLayout and AppLayout)
4. Created DoraMetricsView and moved DoraDashboard to metrics/dora
5. Updated router to reference new file locations
6. Updated navigation links in the AppLayout
7. Updated DashboardView to navigate to the proper DoraMetricsView

## Remaining Tasks

1. Implement Developer Metrics and Collaboration Quality views
2. Create RBAC management views (Users, Roles, Departments)
3. Finalize and test routing for all views
4. Ensure all components correctly use the appropriate layouts
5. Add proper loading and error handling to all views

## Implementation Notes

1. When moving files, ensure imports and references are correctly updated
2. Test router navigation after refactoring to verify paths are correct
3. Consider implementing the layouts first to establish a consistent UI structure
4. Refactor one section at a time and verify functionality before proceeding to the next

## Benefits

This refactoring will:

1. Improve code organization and maintainability
2. Establish a scalable structure for future development
3. Provide clearer separation of concerns
4. Make it easier for new developers to understand the project architecture
