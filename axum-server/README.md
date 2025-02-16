```sql
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL,
    priority INTEGER NOT NULL,
    status TEXT NOT NULL,
    due_date TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    user_id INTEGER NOT NULL
);


INSERT INTO tasks (title, description, category, priority, status, due_date, user_id)
VALUES
    ('Task 1', 'This is task 1 description', 'work', 1, 'pending', NOW(), 1),
    ('Task 2', 'This is task 2 description', 'personal', 2, 'in_progress', NOW(), 2),
    ('Task 3', 'This is task 3 description', 'education', 3, 'completed', NOW(), 3),
    ('Task 4', 'This is task 4 description', 'health', 1, 'pending', NOW(), 4),
    ('Task 5', 'This is task 5 description', 'finance', 2, 'in_progress', NOW(), 1),
    ('Task 6', 'This is task 6 description', 'work', 3, 'completed', NOW(), 2),
    ('Task 7', 'This is task 7 description', 'personal', 1, 'pending', NOW(), 3),
    ('Task 8', 'This is task 8 description', 'education', 2, 'in_progress', NOW(), 4),
    ('Task 9', 'This is task 9 description', 'health', 3, 'completed', NOW(), 1),
    ('Task 10', 'This is task 10 description', 'finance', 1, 'pending', NOW(), 2);

```


```shell
curl -X GET "http://localhost:3000/api/tasks" \
    -H "Authorization: Bearer YOUR_TOKEN"


curl -X POST "http://localhost:3000/api/tasks" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer YOUR_TOKEN" \
    -d '{
        "title": "New Task",
        "description": "This is a test task",
        "category": "work",
        "priority": 1,
        "due_date": "2025-02-20T12:00:00Z"
    }'


curl -X GET "http://localhost:3000/api/tasks/1" \
    -H "Authorization: Bearer YOUR_TOKEN"


curl -X PUT "http://localhost:3000/api/tasks/1" \
    -H "Content-Type: application/json" \
    -H "Authorization: Bearer YOUR_TOKEN" \
    -d '{
        "title": "Updated Task",
        "description": "Updated task description",
        "priority": 2,
        "status": "completed"
    }'


curl -X DELETE "http://localhost:3000/api/tasks/1" \
    -H "Authorization: Bearer YOUR_TOKEN"


```