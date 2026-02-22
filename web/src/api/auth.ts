import { api } from './client'

export interface LoginResponse {
  access_token: string
  type: string
  user: UserInfo
}

export interface UserInfo {
  name: string
  email: string
  is_admin: boolean
  note: string
}

export function login(username: string, password: string): Promise<LoginResponse> {
  return api('/api/login', {
    method: 'POST',
    body: JSON.stringify({ username, password, id: '', uuid: '' }),
  })
}

export function logout(): Promise<void> {
  return api('/api/logout', { method: 'POST' })
}

export function getCurrentUser(): Promise<UserInfo> {
  return api('/api/currentUser')
}
