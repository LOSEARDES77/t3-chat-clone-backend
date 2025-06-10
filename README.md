# T3 Chat Clone Backend API

## API Routes

### Authentication

- `POST /api/auth/login` - User login
- `POST /api/auth/register` - User registration
- `POST /api/auth/logout` - User logout
- `GET /api/auth/me` - Get current user info

### Chat Management

- `GET /api/chats` - Get all user's chat sessions
- `POST /api/chats` - Create new chat session
- `GET /api/chats/:id` - Get specific chat session
- `DELETE /api/chats/:id` - Delete chat session
- `PUT /api/chats/:id` - Update chat session (rename, etc.)

### Messages

- `GET /api/chats/:id/messages` - Get chat history for a session
- `POST /api/chats/:id/messages` - Send new message to LLM
- `DELETE /api/messages/:id` - Delete specific message

### LLM Integration

- `POST /api/llm/chat` - Direct LLM chat endpoint
- `GET /api/llm/models` - Get available LLM models
- `POST /api/llm/stream` - Streaming chat response

### User Management

- `GET /api/users/profile` - Get user profile
- `PUT /api/users/profile` - Update user profile
- `GET /api/users/settings` - Get user preferences
- `PUT /api/users/settings` - Update user preferences
