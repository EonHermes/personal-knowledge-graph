import React, { useState } from 'react';
import styled from 'styled-components';
import GraphView from './components/GraphView';
import NoteList from './components/NoteList';
import NoteEditor from './components/NoteEditor';
import Sidebar from './components/Sidebar';

const Container = styled.div`
  display: flex;
  height: 100vh;
  background-color: #f5f7fa;
`;

const MainContent = styled.div`
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
`;

const Header = styled.header`
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 1rem 2rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
`;

const Title = styled.h1`
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
`;

const ContentArea = styled.div`
  flex: 1;
  display: flex;
  overflow: hidden;
`;

function App() {
  const [activeView, setActiveView] = useState<'graph' | 'list'>('graph');
  const [selectedNote, setSelectedNote] = useState<string | null>(null);
  const [isEditorOpen, setIsEditorOpen] = useState(false);

  return (
    <Container>
      <Sidebar 
        activeView={activeView}
        onViewChange={setActiveView}
        onNewNote={() => {
          setSelectedNote(null);
          setIsEditorOpen(true);
        }}
      />
      
      <MainContent>
        <Header>
          <Title>Personal Knowledge Graph</Title>
        </Header>
        
        <ContentArea>
          {activeView === 'graph' ? (
            <GraphView 
              onNodeSelect={(id) => {
                setSelectedNote(id);
                setIsEditorOpen(true);
              }}
            />
          ) : (
            <NoteList 
              onSelectNote={(id) => {
                setSelectedNote(id);
                setIsEditorOpen(true);
              }}
              onNewNote={() => {
                setSelectedNote(null);
                setIsEditorOpen(true);
              }}
            />
          )}
        </ContentArea>
      </MainContent>

      {isEditorOpen && (
        <NoteEditor 
          noteId={selectedNote}
          onClose={() => setIsEditorOpen(false)}
        />
      )}
    </Container>
  );
}

export default App;
