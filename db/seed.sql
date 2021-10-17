INSERT INTO users (id, email) VALUES (1, 'anowell@gmail.com');

INSERT INTO groups (id, name, owner_id) VALUES (1, 'Team 1', 1);

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

