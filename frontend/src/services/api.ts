import axios from 'axios';

const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:3000/api';

const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Note endpoints
export const notesApi = {
  getAll: () => api.get('/notes'),
  getById: (id: string) => api.get(`/notes/${id}`),
  create: (data: CreateNoteData) => api.post('/notes', data),
  update: (id: string, data: UpdateNoteData) => api.put(`/notes/${id}`, data),
  delete: (id: string) => api.delete(`/notes/${id}`),
  search: (query: string) => api.get(`/notes/search/${encodeURIComponent(query)}`),
};

// Tag endpoints
export const tagsApi = {
  getAll: () => api.get('/tags'),
  create: (data: CreateTagData) => api.post('/tags', data),
  getNoteTags: (noteId: string) => api.get(`/notes/${noteId}/tags`),
  addTagToNote: (noteId: string, tagId: string) => 
    api.post(`/notes/${noteId}/tags/${tagId}`),
  removeTagFromNote: (noteId: string, tagId: string) => 
    api.delete(`/notes/${noteId}/tags/${tagId}`),
};

// Connection endpoints
export const connectionsApi = {
  getAll: () => api.get('/connections'),
  create: (data: CreateConnectionData) => api.post('/connections', data),
  getNoteConnections: (noteId: string) => 
    api.get(`/notes/${noteId}/connections`),
  delete: (id: string) => api.delete(`/connections/${id}`),
};

// Graph endpoints
export const graphApi = {
  getData: () => api.get('/graph'),
  getSuggestions: (noteId: string) => 
    api.get(`/notes/${noteId}/suggestions`),
};

// Health check
export const healthApi = {
  check: () => api.get('/health'),
};

// Type definitions
export interface Note {
  id: string;
  title: string;
  content: string;
  note_type: string;
  url?: string;
  created_at: string;
  updated_at: string;
}

export interface Tag {
  id: string;
  name: string;
  created_at: string;
}

export interface Connection {
  id: string;
  source_note_id: string;
  target_note_id: string;
  connection_type: string;
  strength: number;
  created_at: string;
}

export interface CreateNoteData {
  title: string;
  content: string;
  note_type?: string;
  url?: string;
}

export interface UpdateNoteData {
  title?: string;
  content?: string;
  url?: string;
}

export interface CreateTagData {
  name: string;
}

export interface CreateConnectionData {
  source_note_id: string;
  target_note_id: string;
  connection_type?: string;
  strength?: number;
}

export interface GraphData {
  nodes: Note[];
  links: Connection[];
}
