import React, { useEffect, useState } from 'react';
import styled from 'styled-components';
import { notesApi, Note } from '../services/api';

const Container = styled.div`
  flex: 1;
  padding: 1rem;
  overflow-y: auto;
`;

const Header = styled.div`
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
`;

const Title = styled.h2`
  margin: 0;
  color: #2d3748;
  font-size: 1.5rem;
`;

const SearchInput = styled.input`
  padding: 0.75rem 1rem;
  border: 2px solid #e2e8f0;
  border-radius: 8px;
  font-size: 1rem;
  width: 300px;
  transition: border-color 0.2s;

  &:focus {
    outline: none;
    border-color: #667eea;
  }
`;

const NoteCard = styled.div`
  background: white;
  border-radius: 12px;
  padding: 1.5rem;
  margin-bottom: 1rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }
`;

const NoteTitle = styled.h3`
  margin: 0 0 0.5rem 0;
  color: #2d3748;
  font-size: 1.1rem;
`;

const NoteContent = styled.p`
  margin: 0;
  color: #718096;
  font-size: 0.95rem;
  line-height: 1.5;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
`;

const NoteMeta = styled.div`
  margin-top: 0.75rem;
  display: flex;
  gap: 1rem;
  font-size: 0.85rem;
  color: #a0aec0;
`;

const TypeBadge = styled.span<{ type: string }>`
  padding: 0.25rem 0.75rem;
  border-radius: 20px;
  background: ${props => props.type === 'bookmark' ? '#f6e05e' : '#bee3f8'};
  color: ${props => props.type === 'bookmark' ? '#744210' : '#2b6cb0'};
  font-size: 0.75rem;
  font-weight: 600;
`;

const EmptyState = styled.div`
  text-align: center;
  padding: 3rem;
  color: #718096;
`;

interface NoteListProps {
  onSelectNote: (id: string) => void;
  onNewNote: () => void;
}

const NoteList: React.FC<NoteListProps> = ({ onSelectNote, onNewNote }) => {
  const [notes, setNotes] = useState<Note[]>([]);
  const [searchQuery, setSearchQuery] = useState('');
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    fetchNotes();
  }, []);

  const fetchNotes = async () => {
    try {
      const response = await notesApi.getAll();
      if (response.data.success) {
        setNotes(response.data.data);
      }
    } catch (err) {
      console.error('Error fetching notes:', err);
    } finally {
      setLoading(false);
    }
  };

  const filteredNotes = notes.filter(note => 
    note.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
    note.content.toLowerCase().includes(searchQuery.toLowerCase())
  );

  if (loading) {
    return <Container><EmptyState>Loading notes...</EmptyState></Container>;
  }

  return (
    <Container>
      <Header>
        <Title>Notes</Title>
        <SearchInput
          type="text"
          placeholder="Search notes..."
          value={searchQuery}
          onChange={(e) => setSearchQuery(e.target.value)}
        />
      </Header>

      {filteredNotes.length === 0 ? (
        <EmptyState>
          <p>No notes found. Create your first note!</p>
          <button onClick={onNewNote} style={{
            padding: '0.75rem 1.5rem',
            background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
            color: 'white',
            border: 'none',
            borderRadius: '8px',
            cursor: 'pointer',
            fontSize: '1rem',
          }}>
            + Create Note
          </button>
        </EmptyState>
      ) : (
        filteredNotes.map(note => (
          <NoteCard key={note.id} onClick={() => onSelectNote(note.id)}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'start' }}>
              <NoteTitle>{note.title}</NoteTitle>
              <TypeBadge type={note.note_type}>
                {note.note_type === 'bookmark' ? '🔗 Bookmark' : '📝 Note'}
              </TypeBadge>
            </div>
            <NoteContent>{note.content}</NoteContent>
            <NoteMeta>
              <span>Created: {new Date(note.created_at).toLocaleDateString()}</span>
              {note.url && <span>🔗 {new URL(note.url).hostname}</span>}
            </NoteMeta>
          </NoteCard>
        ))
      )}
    </Container>
  );
};

export default NoteList;
