# Notes-API

An API that can be used with a frontend to create a note taking app.

## Tech Stack

**Server:** Axum Rust

**Database:** Sqlx, Sqlite

## Run Locally

Clone the project

```bash
  git clone https://github.com/that1Kid878/notes-api
```

Go to the project directory

```bash
  cd my-project
```

Compose docker

```bash
  docker-compose up --build
```

## API Reference

#### Get notes by username

```http
  GET /notes/user/{username}
```

#### Get note by note id

```http
  GET /notes/id/{id}
```

#### Create a new note

```http
  GET /notes/new
```

Body:

```
{
    "username": {string},
    "title": {string},
    "body": {string}
}
```

#### Edit an existing note

```http
  GET /notes/edit
```

Body:

```
{
    "id": {integer},
    "title": {string},
    "body": {string}
}
```

#### Delete an existing note

```http
  GET /notes/delete/{id}
```

## License

[MIT](https://choosealicense.com/licenses/mit/)
