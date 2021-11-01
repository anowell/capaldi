INSERT INTO users (id, email) VALUES (1, 'anowell@gmail.com');

INSERT INTO groups (id, name, owner_id) VALUES (1, 'Applications', 1);
INSERT INTO groups (id, name, owner_id) VALUES (2, 'Platforms', 1);

INSERT INTO resource_roles (id, name) VALUES
    (1, 'Dev'),
    (2, 'Test'),
    (3, 'Manager');

INSERT INTO categories (id, name) VALUES
    (1, 'Feature Enhancements'), -- new customer value (even if customer is internal) - correlate with Component for customer value
    (2, 'Feature Defects'), -- after feature is "complete", otherwise it's part of the feature enhancement
    (3, 'Operational Overhead'),  -- overhead to keep things running, builds/release, manual testing, etc..
    (4, 'Managing Complexity / Tech Debt'), -- refactor, rewrite, rearchitect, simplify, deprecate, etc...
    (5, 'Reactive / Disruptive Work'), -- oncall, customer disruption, etc. Urgent work that prevents any of the above
    (6, 'Time Off');

INSERT INTO projects (id, name, category_id) VALUES
    (1, "Time Off", 6),
    (2, "Component Stewardship: Bugs", 2),
    (3, "Component Stewardship: Ops", 3),
    (4, "Component Stewardship: Tech Debt", 4),
    (5, "Ramp / Onboarding / Training", 3),
    (6, "On-Call / Escalation", 5);

INSERT INTO resources (id, group_id, name, role_id, is_fte) VALUES
    (1, 1, "Barbara Liskov", 1, true),
    (2, 1, "Alan Turin", 1, true),
    (3, 1, "Ada Lovelace", 1, true),
    (4, 1, "Yukihiro Matsumoto", 1, true),
    (5, 1, "Ken Thompson", 1, true),
    (6, 2, "Claude Shannon", 1, true),
    (7, 2, "Marvin Minsky", 1, true),
    (8, 2, "Allen Newell", 1, true),
    (9, 2, "Andrew Ng", 1, true),
    (10, 2, "Leslie Lamport", 1, true),
    (11, 2, "Edsger Dijkstra", 1, true);

INSERT INTO allocations (id, start_date, resource_id, project_id, component_id, percent) VALUES
    (1, "2021-11-01", 1, 1, 1, 50),
    (2, "2021-11-01", 1, 2, 2, 50),
    (3, "2021-11-01", 2, 3, 3, 50),
    (4, "2021-11-01", 2, 4, 1, 50),
    (5, "2021-11-01", 3, 1, 2, 50),
    (6, "2021-11-01", 3, 2, 3, 50),
    (7, "2021-11-01", 4, 3, 1, 50),
    (8, "2021-11-01", 4, 4, 2, 50);