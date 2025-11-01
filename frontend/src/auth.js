// Token 管理
const TOKEN_KEY = 'classtop_token'
const USER_KEY = 'classtop_user'

export const auth = {
  // 保存 token 和用户信息
  setToken(token) {
    localStorage.setItem(TOKEN_KEY, token)
  },

  getToken() {
    return localStorage.getItem(TOKEN_KEY)
  },

  removeToken() {
    localStorage.removeItem(TOKEN_KEY)
  },

  setUser(user) {
    localStorage.setItem(USER_KEY, JSON.stringify(user))
  },

  getUser() {
    const user = localStorage.getItem(USER_KEY)
    return user ? JSON.parse(user) : null
  },

  removeUser() {
    localStorage.removeItem(USER_KEY)
  },

  // 检查是否已登录
  isAuthenticated() {
    return !!this.getToken()
  },

  // 登出
  logout() {
    this.removeToken()
    this.removeUser()
  },

  // 登录
  async login(username, password) {
    const response = await fetch('/api/auth/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password })
    })

    if (!response.ok) {
      const error = await response.json().catch(() => ({ detail: 'Login failed' }))
      throw new Error(error.detail || 'Invalid credentials')
    }

    const data = await response.json()
    const { token, user } = data.data

    this.setToken(token)
    this.setUser(user)

    return user
  },

  // 注册
  async register(username, password, email = null) {
    const response = await fetch('/api/auth/register', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ username, password, email })
    })

    if (!response.ok) {
      const error = await response.json().catch(() => ({ detail: 'Registration failed' }))
      throw new Error(error.detail || 'Registration failed')
    }

    const data = await response.json()
    const { token, user } = data.data

    this.setToken(token)
    this.setUser(user)

    return user
  }
}
