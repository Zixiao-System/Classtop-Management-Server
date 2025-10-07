const API_BASE = '/api'

const handleResponse = async (response) => {
  if (!response.ok) {
    const error = await response.json().catch(() => ({ detail: 'Request failed' }))
    throw new Error(error.detail || 'Request failed')
  }
  const json = await response.json()
  return json.data
}

// Statistics
export const fetchStats = async () => {
  const response = await fetch(`${API_BASE}/statistics`)
  return handleResponse(response)
}

export const fetchClientStats = async () => {
  const response = await fetch(`${API_BASE}/statistics/clients`)
  return handleResponse(response)
}

// Clients
export const fetchClients = async () => {
  const response = await fetch(`${API_BASE}/clients`)
  return handleResponse(response)
}

export const fetchClient = async (id) => {
  const response = await fetch(`${API_BASE}/clients/${id}`)
  return handleResponse(response)
}

export const createClient = async (data) => {
  const response = await fetch(`${API_BASE}/clients/register`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data)
  })
  return handleResponse(response)
}

export const removeClient = async (id) => {
  const response = await fetch(`${API_BASE}/clients/${id}`, {
    method: 'DELETE'
  })
  return handleResponse(response)
}

export const fetchClientCourses = async (id) => {
  const response = await fetch(`${API_BASE}/clients/${id}/courses`)
  return handleResponse(response)
}

export const fetchClientSchedule = async (id) => {
  const response = await fetch(`${API_BASE}/clients/${id}/schedule`)
  return handleResponse(response)
}
