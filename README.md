# Personal Knowledge Graph ⚜️

A visual network graph application that helps you organize notes, bookmarks, and thoughts. The app automatically suggests connections between related concepts based on content similarity.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![React](https://img.shields.io/badge/react-18-blue.svg)
![TypeScript](https://img.shields.io/badge/typescript-5-blue.svg)

## Features

- 📝 **Create & Edit Notes** - Rich text editor for capturing your thoughts
- 🔗 **Bookmarks** - Save and organize web links with notes
- 🕸️ **Visual Graph View** - Interactive network visualization of your knowledge
- 🤖 **Smart Suggestions** - Auto-suggest connections based on content similarity
- 🏷️ **Tagging System** - Organize content with custom tags
- 🔍 **Search** - Full-text search across all notes
- 📊 **Dual Views** - Switch between graph and list views

## Tech Stack

### Backend
- **Rust** with Axum web framework
- **SQLite** for persistent storage
- **SQLx** for async database operations
- Clean architecture with separate layers (API, Services, Repositories)

### Frontend
- **React 18** with TypeScript
- **Vis-Network** for graph visualization
- **Styled Components** for theming
- **Axios** for API calls

## Architecture

```
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│   React     │────▶│    Axum      │────▶│   SQLite    │
│  Frontend   │     │   Backend    │     │  Database   │
└─────────────┘     └──────────────┘     └─────────────┘
                           │
                    ┌──────┴──────┐
                    │             │
               ┌────▼────┐  ┌────▼────┐
               │ Services│  │Repos    │
               └─────────┘  └─────────┘
```

## Quick Start

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Node.js 18+ ([Install Node.js](https://nodejs.org/))
- npm or yarn

### Backend Setup

```bash
cd backend

# Install dependencies
cargo build

# Set environment variables (optional)
export DATABASE_URL=sqlite:personal_knowledge_graph.db
export PORT=3000

# Run the server
cargo run
```

The backend will start on `http://localhost:3000`

### Frontend Setup

```bash
cd frontend

# Install dependencies
npm install

# Start development server
npm start
```

The frontend will open at `http://localhost:3001`

## API Endpoints

### Notes
- `POST /api/notes` - Create a new note
- `GET /api/notes` - List all notes
- `GET /api/notes/:id` - Get a specific note
- `PUT /api/notes/:id` - Update a note
- `DELETE /api/notes/:id` - Delete a note
- `GET /api/notes/search/:query` - Search notes

### Tags
- `POST /api/tags` - Create a new tag
- `GET /api/tags` - List all tags
- `GET /api/notes/:noteId/tags` - Get tags for a note
- `POST /api/notes/:noteId/tags/:tagId` - Add tag to note
- `DELETE /api/notes/:noteId/tags/:tagId` - Remove tag from note

### Connections
- `POST /api/connections` - Create a connection
- `GET /api/connections` - List all connections
- `GET /api/notes/:noteId/connections` - Get connections for a note
- `DELETE /api/connections/:id` - Delete a connection

### Graph & Suggestions
- `GET /api/graph` - Get full graph data (nodes + edges)
- `GET /api/notes/:noteId/suggestions` - Get suggested connections

## Usage Guide

### Creating Notes

1. Click the "+ New Note" button in the sidebar
2. Enter a title and content
3. Choose between "Note" or "Bookmark" type
4. For bookmarks, add a URL
5. Add tags to organize your content
6. Click "Save Note"

### Viewing the Graph

1. Select "Graph View" from the sidebar
2. Nodes represent your notes (blue) and bookmarks (purple)
3. Edges show connections between related items
4. Click on any node to edit it
5. The graph automatically arranges itself using physics simulation

### Finding Connections

The system automatically suggests connections based on:
- Word overlap in content
- Shared tags
- Content similarity (threshold > 15%)

Suggestions appear when you have fewer than 5 existing connections for a note.

### Searching

Use the search bar in the Note List view to find notes by:
- Title keywords
- Content keywords
- Tag names

## Development

### Running Tests

```bash
# Backend tests (requires libssl-dev)
cd backend
cargo test

# Frontend tests
cd frontend
npm test
```

### Building for Production

```bash
# Backend release build
cd backend
cargo build --release

# Frontend production build
cd frontend
npm run build
```

## Project Structure

```
personal-knowledge-graph/
├── backend/                 # Rust backend
│   ├── src/
│   │   ├── api/            # REST API handlers & routes
│   │   ├── db/             # Database layer
│   │   │   ├── repositories/
│   │   │   └── schema.rs   # Database migrations
│   │   ├── models/         # Data structures
│   │   ├── services/       # Business logic
│   │   └── main.rs
│   └── Cargo.toml
├── frontend/               # React frontend
│   ├── src/
│   │   ├── components/     # React components
│   │   ├── services/       # API client
│   │   └── App.tsx
│   └── package.json
└── README.md
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built with [Axum](https://github.com/tokio-rs/axum) - Ergonomic and modular web framework
- Graph visualization powered by [Vis-Network](https://visjs.github.io/vis-network/)
- Styled with [Styled Components](https://styled-components.com/)

---

Made with ❤️ using Rust & React
