import { useEffect, useState } from 'react';
import './App.css';

interface User {
  id: number;
  name: string;
  email: string;
}

const API_URL = 'http://localhost:3000/users';
const INIT_DB_URL = 'http://localhost:3000/init-db';

function App() {
  const [users, setUsers] = useState<User[]>([]);
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [editingId, setEditingId] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    fetchUsers();
  }, []);

  const fetchUsers = async () => {
    try {
      const response = await fetch(API_URL);
      if (response.ok) {
        const data = await response.json();
        setUsers(data);
        setError(null);
      } else {
        setError('Failed to fetch users. Database might not exist.');
      }
    } catch (error) {
      console.error('Error fetching users:', error);
      setError('Error fetching users. Is the backend running?');
    }
  };

  const handleInitDb = async () => {
    try {
      const response = await fetch(INIT_DB_URL, { method: 'POST' });
      if (response.ok) {
        alert('Database initialized successfully!');
        fetchUsers();
      } else {
        const msg = await response.text();
        alert(`Failed to initialize DB: ${msg}`);
      }
    } catch (error) {
      alert(`Error: ${error}`);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    const user = { name, email };

    try {
      if (editingId) {
        // Update
        const response = await fetch(`${API_URL}/${editingId}`, {
          method: 'PUT',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(user),
        });
        if (response.ok) {
           const updatedUser = await response.json();
           setUsers(users.map(u => u.id === editingId ? updatedUser : u));
           setEditingId(null);
        }
      } else {
        // Create
        const response = await fetch(API_URL, {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(user),
        });
        if (response.ok) {
          const newUser = await response.json();
          setUsers([...users, newUser]);
        }
      }
      setName('');
      setEmail('');
    } catch (error) {
      console.error('Error saving user:', error);
    }
  };

  const handleEdit = (user: User) => {
    setName(user.name);
    setEmail(user.email);
    setEditingId(user.id);
  };

  const handleDelete = async (id: number) => {
    if (!confirm('Are you sure?')) return;

    try {
      const response = await fetch(`${API_URL}/${id}`, {
        method: 'DELETE',
      });
      if (response.ok) {
        setUsers(users.filter(u => u.id !== id));
      }
    } catch (error) {
      console.error('Error deleting user:', error);
    }
  };

  const handleCancel = () => {
    setEditingId(null);
    setName('');
    setEmail('');
  }

  return (
    <div className="container">
      <div style={{display: 'flex', justifyContent: 'space-between', alignItems: 'center'}}>
          <h1>User Management</h1>
          <button onClick={handleInitDb} style={{backgroundColor: '#28a745', color: 'white'}}>
              Initialize Database
          </button>
      </div>
      
      {error && <div style={{color: 'red', marginBottom: '10px'}}>{error}</div>}

      <form onSubmit={handleSubmit} className="form-group">
        <input
          type="text"
          placeholder="Name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          required
        />
        <input
          type="email"
          placeholder="Email"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          required
        />
        <button type="submit">{editingId ? 'Update' : 'Add'} User</button>
        {editingId && <button type="button" onClick={handleCancel} style={{marginLeft: '10px', backgroundColor: '#666'}}>Cancel</button>}
      </form>

      <table>
        <thead>
          <tr>
            <th>ID</th>
            <th>Name</th>
            <th>Email</th>
            <th>Actions</th>
          </tr>
        </thead>
        <tbody>
          {users.map((user) => (
            <tr key={user.id}>
              <td>{user.id}</td>
              <td>{user.name}</td>
              <td>{user.email}</td>
              <td>
                <button
                  className="action-btn edit-btn"
                  onClick={() => handleEdit(user)}
                >
                  Edit
                </button>
                <button
                  className="action-btn delete-btn"
                  onClick={() => handleDelete(user.id)}
                >
                  Delete
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

export default App;
