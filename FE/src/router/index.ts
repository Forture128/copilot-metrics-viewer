import { createRouter, createWebHistory } from 'vue-router';
import MainComponent from '../components/MainComponent.vue';
import DoraDashboard from '../components/DoraDashboard.vue';

/**
 * Defines the application routes.
 * 
 * @note
 * - The root path ('/') redirects to '/copilot-module'.
 * - The '/copilot-module' path loads the `MainComponent`.
 * - The '/dora-module' path loads the `DoraDashboard`.
 */
const routes = [
  {
    path: '/',
    component: MainComponent,
    children: [
      {
        path: '',
        name: 'Home',
        component: MainComponent,
      },
      {
        path: 'dora-dashboard',
        name: 'DoraDashboard',
        component: DoraDashboard,
      },
    ],
  },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;