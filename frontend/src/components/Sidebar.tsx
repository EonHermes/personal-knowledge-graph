import React from 'react';
import styled from 'styled-components';

const Container = styled.aside`
  width: 250px;
  background: white;
  border-right: 1px solid #e2e8f0;
  display: flex;
  flex-direction: column;
  padding: 1rem;
`;

const Logo = styled.div`
  font-size: 1.25rem;
  font-weight: bold;
  color: #667eea;
  margin-bottom: 2rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
`;

const NavButton = styled.button<{ active?: boolean }>`
  width: 100%;
  padding: 0.75rem 1rem;
  margin-bottom: 0.5rem;
  border: none;
  background: ${props => props.active ? '#667eea' : 'transparent'};
  color: ${props => props.active ? 'white' : '#4a5568'};
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
  text-align: left;
  transition: all 0.2s;

  &:hover {
    background: ${props => props.active ? '#5a67d8' : '#f7fafc'};
  }
`;

const NewNoteButton = styled.button`
  margin-top: auto;
  padding: 1rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  font-size: 1rem;
  font-weight: 600;
  transition: transform 0.2s, box-shadow 0.2s;

  &:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
  }
`;

interface SidebarProps {
  activeView: 'graph' | 'list';
  onViewChange: (view: 'graph' | 'list') => void;
  onNewNote: () => void;
}

const Sidebar: React.FC<SidebarProps> = ({ 
  activeView, 
  onViewChange, 
  onNewNote 
}) => {
  return (
    <Container>
      <Logo>
        <span>⚜️</span>
        <span>PKG</span>
      </Logo>

      <nav>
        <NavButton 
          active={activeView === 'graph'}
          onClick={() => onViewChange('graph')}
        >
          📊 Graph View
        </NavButton>
        
        <NavButton 
          active={activeView === 'list'}
          onClick={() => onViewChange('list')}
        >
          📝 Note List
        </NavButton>
      </nav>

      <NewNoteButton onClick={onNewNote}>
        + New Note
      </NewNoteButton>
    </Container>
  );
};

export default Sidebar;
