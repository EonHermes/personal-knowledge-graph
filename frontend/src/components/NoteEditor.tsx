import React, { useEffect, useState } from 'react';
import styled from 'styled-components';
import { notesApi, tagsApi, Note, Tag, CreateNoteData, UpdateNoteData } from '../services/api';

const Overlay = styled.div`
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
`;

const Container = styled.div`
  background: white;
  border-radius: 16px;
  width: 90%;
  max-width: 800px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
`;

const Header = styled.div`
  padding: 1.5rem 2rem;
  border-bottom: 1px solid #e2e8f0;
  display: flex;
  justify-content: space-between;
  align-items: center;
`;

const Title = styled.h2`
  margin: 0;
  color: #2d3748;
`;

const CloseButton = styled.button`
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: #718096;
  padding: 0.5rem;
  border-radius: 8px;

  &:hover {
    background: #f7fafc;
    color: #2d3748;
  }
`;

const Content = styled.div`
  padding: 2rem;
`;

const FormGroup = styled.div`
  margin-bottom: 1.5rem;
`;

const Label = styled.label`
  display: block;
  margin-bottom: 0.5rem;
  color: #4a5568;
  font-weight: 600;
`;

const Input = styled.input`
  width: 100%;
  padding: 0.75rem 1rem;
  border: 2px solid #e2e8f0;
  border-radius: 8px;
  font-size: 1rem;
  transition: border-color 0.2s;

  &:focus {
    outline: none;
    border-color: #667eea;
  }
`;

const TextArea = styled.textarea`
  width: 100%;
  padding: 0.75rem 1rem;
  border: 2px solid #e2e8f0;
  border-radius: 8px;
  font-size: 1rem;
  min-height: 200px;
  resize: vertical;
  font-family: inherit;

  &:focus {
    outline: none;
    border-color: #667eea;
  }
`;

const Select = styled.select`
  width: 100%;
  padding: 0.75rem 1rem;
  border: 2px solid #e2e8f0;
  border-radius: 8px;
  font-size: 1rem;
  background: white;

  &:focus {
    outline: none;
    border-color: #667eea;
  }
`;

const Button = styled.button<{ primary?: boolean }>`
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 8px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;

  ${props => props.primary ? 
    `background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
     color: white;` :
    `background: #e2e8f0;
     color: #4a5568;`
  }

  &:hover {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }
`;

const ButtonGroup = styled.div`
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 2rem;
`;

const TagsSection = styled.div`
  margin-top: 1.5rem;
`;

const TagList = styled.div`
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-top: 0.5rem;
`;

const TagChip = styled.span`
  padding: 0.25rem 0.75rem;
  background: #e9d8fd;
  color: #553c9a;
  border-radius: 16px;
  font-size: 0.85rem;
  display: flex;
  align-items: center;
  gap: 0.25rem;
`;

const TagInput = styled.input`
  padding: 0.5rem 0.75rem;
  border: 2px solid #e2e8f0;
  border-radius: 16px;
  font-size: 0.9rem;
  width: 150px;

  &:focus {
    outline: none;
    border-color: #667eea;
  }
`;

interface NoteEditorProps {
  noteId: string | null;
  onClose: () => void;
}

const NoteEditor: React.FC<NoteEditorProps> = ({ noteId, onClose }) => {
  const [title, setTitle] = useState('');
  const [content, setContent] = useState('');
  const [noteType, setNoteType] = useState<'note' | 'bookmark'>('note');
  const [url, setUrl] = useState('');
  const [tags, setTags] = useState<Tag[]>([]);
  const [newTag, setNewTag] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (noteId) {
      loadNote(noteId);
    }
  }, [noteId]);

  const loadNote = async (id: string) => {
    try {
      const response = await notesApi.getById(id);
      if (response.data.success) {
        const note = response.data.data;
        setTitle(note.title);
        setContent(note.content);
        setNoteType(note.note_type as 'note' | 'bookmark');
        setUrl(note.url || '');
        
        // Load tags for this note
        const tagsResponse = await tagsApi.getNoteTags(id);
        if (tagsResponse.data.success) {
          setTags(tagsResponse.data.data);
        }
      }
    } catch (err) {
      console.error('Error loading note:', err);
    }
  };

  const handleSave = async () => {
    if (!title.trim() || !content.trim()) {
      alert('Title and content are required');
      return;
    }

    setLoading(true);
    try {
      const data: CreateNoteData | UpdateNoteData = {
        title,
        content,
        note_type: noteType,
        url: noteType === 'bookmark' ? url : undefined,
      };

      if (noteId) {
        await notesApi.update(noteId, data);
      } else {
        const response = await notesApi.create(data as CreateNoteData);
        noteId = response.data.data.id;
      }

      onClose();
    } catch (err) {
      console.error('Error saving note:', err);
      alert('Failed to save note');
    } finally {
      setLoading(false);
    }
  };

  const handleAddTag = async () => {
    if (!newTag.trim() || !noteId) return;

    try {
      // First, create or get the tag
      let tagResponse = await tagsApi.create({ name: newTag.trim() });
      
      // If tag already exists (400 error), we need to find it
      if (!tagResponse.data.success) {
        const allTags = await tagsApi.getAll();
        if (allTags.data.success) {
          const existingTag = allTags.data.data.find((t: Tag) => t.name === newTag.trim());
          if (existingTag) {
            tagResponse = { data: { success: true, data: existingTag } };
          }
        }
      }

      if (tagResponse.data.success && noteId) {
        await tagsApi.addTagToNote(noteId, tagResponse.data.data.id);
        setTags([...tags, tagResponse.data.data]);
        setNewTag('');
      }
    } catch (err) {
      console.error('Error adding tag:', err);
    }
  };

  const handleRemoveTag = async (tagId: string) => {
    if (!noteId) return;

    try {
      await tagsApi.removeTagFromNote(noteId, tagId);
      setTags(tags.filter(t => t.id !== tagId));
    } catch (err) {
      console.error('Error removing tag:', err);
    }
  };

  return (
    <Overlay onClick={onClose}>
      <Container onClick={(e) => e.stopPropagation()}>
        <Header>
          <Title>{noteId ? 'Edit Note' : 'New Note'}</Title>
          <CloseButton onClick={onClose}>&times;</CloseButton>
        </Header>

        <Content>
          <FormGroup>
            <Label>Title</Label>
            <Input
              type="text"
              value={title}
              onChange={(e) => setTitle(e.target.value)}
              placeholder="Enter note title..."
            />
          </FormGroup>

          <FormGroup>
            <Label>Type</Label>
            <Select value={noteType} onChange={(e) => setNoteType(e.target.value as 'note' | 'bookmark')}>
              <option value="note">📝 Note</option>
              <option value="bookmark">🔗 Bookmark</option>
            </Select>
          </FormGroup>

          {noteType === 'bookmark' && (
            <FormGroup>
              <Label>URL</Label>
              <Input
                type="url"
                value={url}
                onChange={(e) => setUrl(e.target.value)}
                placeholder="https://example.com"
              />
            </FormGroup>
          )}

          <FormGroup>
            <Label>Content</Label>
            <TextArea
              value={content}
              onChange={(e) => setContent(e.target.value)}
              placeholder="Write your note content here..."
            />
          </FormGroup>

          {noteId && (
            <TagsSection>
              <Label>Tags</Label>
              <div style={{ display: 'flex', gap: '0.5rem' }}>
                <TagInput
                  type="text"
                  value={newTag}
                  onChange={(e) => setNewTag(e.target.value)}
                  onKeyPress={(e) => e.key === 'Enter' && (e.preventDefault(), handleAddTag())}
                  placeholder="Add tag..."
                />
                <Button onClick={handleAddTag}>Add</Button>
              </div>
              <TagList>
                {tags.map(tag => (
                  <TagChip key={tag.id}>
                    {tag.name}
                    <span 
                      style={{ cursor: 'pointer', marginLeft: '0.25rem' }}
                      onClick={() => handleRemoveTag(tag.id)}
                    >
                      ×
                    </span>
                  </TagChip>
                ))}
              </TagList>
            </TagsSection>
          )}

          <ButtonGroup>
            <Button onClick={onClose}>Cancel</Button>
            <Button primary onClick={handleSave} disabled={loading}>
              {loading ? 'Saving...' : 'Save Note'}
            </Button>
          </ButtonGroup>
        </Content>
      </Container>
    </Overlay>
  );
};

export default NoteEditor;
