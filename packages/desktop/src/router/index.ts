import { createRouter, createWebHistory } from 'vue-router'
import MainView from "@/views/MainView.vue";
import SettingsView from "@/views/SettingsView.vue";


const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: MainView,
    },
    {
      path: '/settings',
      name: 'settings',
      component: SettingsView
    }
  ]
})

export default router
