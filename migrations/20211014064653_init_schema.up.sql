CREATE TABLE users (
  id integer NOT NULL PRIMARY KEY,
  email varchar(255) NOT NULL,
  created_at timestamp NOT NULL DEFAULT current_timestamp,
  is_admin boolean NOT NULL DEFAULT false,

  -- soft delete
  is_deleted boolean NOT NULL DEFAULT false,
  deleted_at timestamp
);

CREATE UNIQUE INDEX idx_users_email_deleted
  ON users (email ASC, is_deleted ASC);

CREATE TABLE resources (
  id integer NOT NULL PRIMARY KEY,
  group_id integer NOT NULL,
  name text NOT NULL,
  role_id integer NOT NULL,
  is_fte boolean NOT NULL,

  -- soft delete
  is_deleted boolean NOT NULL DEFAULT false,
  deleted_at timestamp,

  FOREIGN KEY (group_id) REFERENCES groups (id),
  FOREIGN KEY (role_id) REFERENCES resource_roles (id)
);

CREATE TABLE resource_roles (
  id integer NOT NULL PRIMARY KEY,
  name text NOT NULL
);

CREATE UNIQUE INDEX idx_resources_deleted
  ON resources (is_deleted ASC);

CREATE TABLE projects (
  id integer NOT NULL PRIMARY KEY,
  name text NOT NULL,
  category_id integer NOT NULL,
  is_closed boolean NOT NULL DEFAULT false,

  -- consider moving this into a metadata json field
  jira varchar(60),
  release varchar(60),

  FOREIGN KEY (category_id) REFERENCES categories (id)
);

CREATE UNIQUE INDEX idx_projects_release
  ON projects (release ASC);

CREATE TABLE groups (
  id integer NOT NULL PRIMARY KEY,
  owner_id integer NOT NULL,
  name text NOT NULL,

  -- todo: consider adding 'parent_id integer' to nest groups
  FOREIGN KEY (owner_id) REFERENCES users (id)
);

CREATE TABLE categories (
  id integer NOT NULL PRIMARY KEY,
  name text NOT NULL
);

CREATE TABLE components (
  id integer NOT NULL PRIMARY KEY,
  name text NOT NULL
);

CREATE TABLE allocations (
  id integer NOT NULL PRIMARY KEY,
  start_date datetime NOT NULL,
  resource_id integer NOT NULL,
  project_id integer NOT NULL,
  component_id integer NOT NULL,
  percent integer NOT NULL,

  FOREIGN KEY (resource_id) REFERENCES resources (id),
  FOREIGN KEY (project_id) REFERENCES projects (id)
);

CREATE INDEX idx_allocations_start_resources
  ON allocations (start_date ASC, resource_id ASC);

CREATE INDEX idx_allocations_start_projects
  ON allocations (start_date ASC, project_id ASC);