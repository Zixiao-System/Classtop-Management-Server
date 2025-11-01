import { auth } from './auth.js'

const API_BASE = '/api'

const handleResponse = async (response) => {
  // 401 未授权，清除 token 并重新登录
  if (response.status === 401) {
    auth.logout()
    window.location.href = '/#login'
    throw new Error('Unauthorized - Please login again')
  }

  // 429 限流
  if (response.status === 429) {
    throw new Error('Too many requests - Please try again later')
  }

  if (!response.ok) {
    const error = await response.json().catch(() => ({ detail: 'Request failed' }))
    throw new Error(error.detail || 'Request failed')
  }

  const json = await response.json()
  return json.data
}

// 创建带认证的 fetch 请求
const authedFetch = (url, options = {}) => {
  const token = auth.getToken()
  const headers = {
    ...options.headers,
  }

  if (token) {
    headers['Authorization'] = `Bearer ${token}`
  }

  return fetch(url, {
    ...options,
    headers
  })
}

// Statistics
export const fetchStats = async () => {
  const response = await authedFetch(`${API_BASE}/statistics`)
  return handleResponse(response)
}

export const fetchClientStats = async () => {
  const response = await authedFetch(`${API_BASE}/statistics/clients`)
  return handleResponse(response)
}

// Clients
export const fetchClients = async () => {
  const response = await authedFetch(`${API_BASE}/clients`)
  return handleResponse(response)
}

export const fetchClientsPaginated = async (page = 1, pageSize = 20) => {
  const response = await authedFetch(`${API_BASE}/clients/paginated?page=${page}&page_size=${pageSize}`)
  return handleResponse(response)
}

export const fetchClient = async (id) => {
  const response = await authedFetch(`${API_BASE}/clients/${id}`)
  return handleResponse(response)
}

export const createClient = async (data) => {
  const response = await authedFetch(`${API_BASE}/clients/register`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data)
  })
  return handleResponse(response)
}

export const removeClient = async (id) => {
  const response = await authedFetch(`${API_BASE}/clients/${id}`, {
    method: 'DELETE'
  })
  return handleResponse(response)
}

export const fetchClientCourses = async (id) => {
  const response = await authedFetch(`${API_BASE}/clients/${id}/courses`)
  return handleResponse(response)
}

export const fetchClientSchedule = async (id) => {
  const response = await authedFetch(`${API_BASE}/clients/${id}/schedule`)
  return handleResponse(response)
}
