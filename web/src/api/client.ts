const BASE_URL = ''

export class ApiError extends Error {
  constructor(public status: number, message: string) {
    super(message)
  }
}

export async function api<T = any>(
  path: string,
  options: RequestInit = {}
): Promise<T> {
  const token = localStorage.getItem('token')
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    ...(options.headers as Record<string, string> || {}),
  }
  if (token) {
    headers['Authorization'] = `Bearer ${token}`
  }

  const res = await fetch(`${BASE_URL}${path}`, {
    ...options,
    headers,
  })

  if (res.status === 401) {
    localStorage.removeItem('token')
    localStorage.removeItem('user')
    window.location.href = '/login'
    throw new ApiError(401, 'Unauthorized')
  }

  if (!res.ok) {
    const body = await res.json().catch(() => ({}))
    throw new ApiError(res.status, body.error || res.statusText)
  }

  // Handle empty responses
  const text = await res.text()
  if (!text) return {} as T
  return JSON.parse(text)
}
