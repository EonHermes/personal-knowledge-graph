import React, { useEffect, useRef, useState } from 'react';
import Network from 'react-vis-network';
import styled from 'styled-components';
import { graphApi, Note, Connection } from '../services/api';

const Container = styled.div`
  flex: 1;
  padding: 1rem;
  display: flex;
  flex-direction: column;
`;

const GraphContainer = styled.div`
  flex: 1;
  background: white;
  border-radius: 12px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.05);
  overflow: hidden;
`;

const Loading = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #718096;
  font-size: 1.1rem;
`;

const Error = styled.div`
  padding: 2rem;
  text-align: center;
  color: #e53e3e;
`;

interface GraphViewProps {
  onNodeSelect: (id: string) => void;
}

interface NodeData {
  id: string;
  label: string;
  title: string;
  group?: string;
}

interface EdgeData {
  id: string;
  from: string;
  to: string;
  arrows: string;
  color?: string;
  width?: number;
}

const GraphView: React.FC<GraphViewProps> = ({ onNodeSelect }) => {
  const [nodes, setNodes] = useState<NodeData[]>([]);
  const [edges, setEdges] = useState<EdgeData[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      try {
        setLoading(true);
        const response = await graphApi.getData();
        
        if (response.data.success) {
          const { nodes: notes, links: connections } = response.data.data;
          
          // Convert notes to network nodes
          const networkNodes: NodeData[] = notes.map((note: Note) => ({
            id: note.id,
            label: note.title.length > 30 
              ? note.title.substring(0, 30) + '...' 
              : note.title,
            title: `${note.title}\n\n${note.content}`,
            group: note.note_type === 'bookmark' ? 'bookmark' : 'note',
          }));

          // Convert connections to network edges
          const networkEdges: EdgeData[] = connections.map((conn: Connection) => ({
            id: conn.id,
            from: conn.source_note_id,
            to: conn.target_note_id,
            arrows: 'to',
            color: {
              color: '#a0aec0',
              highlight: '#667eea',
            },
            width: Math.max(1, conn.strength * 2),
          }));

          setNodes(networkNodes);
          setEdges(networkEdges);
        } else {
          setError('Failed to load graph data');
        }
      } catch (err) {
        console.error('Error fetching graph data:', err);
        setError('Failed to connect to server. Make sure the backend is running.');
      } finally {
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  const handleNodeClick = (event: any) => {
    if (event.nodes.length > 0) {
      onNodeSelect(event.nodes[0]);
    }
  };

  const options = {
    nodes: {
      shape: 'dot',
      size: 25,
      font: {
        size: 14,
        color: '#2d3748',
      },
      borderWidth: 2,
      shadow: true,
    },
    edges: {
      smooth: {
        type: 'continuous',
      },
    },
    groups: {
      note: {
        color: {
          background: '#667eea',
          border: '#5a67d8',
        },
      },
      bookmark: {
        color: {
          background: '#764ba2',
          border: '#6b46c1',
        },
      },
    },
    physics: {
      stabilization: false,
      barnesHut: {
        gravitationalConstant: -2000,
        springConstant: 0.04,
        springLength: 95,
      },
    },
    interaction: {
      hover: true,
      tooltipDelay: 200,
    },
  };

  if (loading) {
    return (
      <Container>
        <GraphContainer>
          <Loading>Loading graph...</Loading>
        </GraphContainer>
      </Container>
    );
  }

  if (error) {
    return (
      <Container>
        <GraphContainer>
          <Error>{error}</Error>
        </GraphContainer>
      </Container>
    );
  }

  return (
    <Container>
      <GraphContainer>
        <Network
          network={{ nodes, edges }}
          options={options}
          events={{ click: handleNodeClick }}
          height="100%"
        />
      </GraphContainer>
    </Container>
  );
};

export default GraphView;
