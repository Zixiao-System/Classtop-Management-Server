import { createRouter, createWebHashHistory } from 'vue-router'
import Main from '../Main.vue'
import DashboardView from '../pages/DashboardView.vue'
import ClientsView from '../pages/ClientsView.vue'
import DataView from '../pages/DataView.vue'
import LoginView from '../components/LoginView.vue'
import { auth } from '../auth.js'

const routes = [
  {
    path: '/login',
    name: 'Login',
    component: LoginView
  },
  {
    path: '/',
    component: Main,
    meta: { requiresAuth: true },
    children: [
      { path: '', name: 'Dashboard', component: DashboardView },
      { path: '/clients', name: 'Clients', component: ClientsView },
      { path: '/data', name: 'Data', component: DataView }
    ]
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

// 路由守卫
router.beforeEach((to, from, next) => {
  const isAuthenticated = auth.isAuthenticated()

  // 如果路由需要认证且用户未登录，重定向到登录页
  if (to.meta.requiresAuth && !isAuthenticated) {
    next('/login')
  }
  // 如果用户已登录且访问登录页，重定向到主页
  else if (to.path === '/login' && isAuthenticated) {
    next('/')
  }
  // 其他情况正常导航
  else {
    next()
  }
})

export default router