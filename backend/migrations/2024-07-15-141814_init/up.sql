-- Your SQL goes here

CREATE TABLE IF NOT EXISTS nodes (
  id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  name VARCHAR NOT NULL,
  startdue INTEGER DEFAULT NULL, 
  deadline INTEGER DEFAULT NULL, 
  notes TEXT NOT NULL DEFAULT "", 
  done BOOL NOT NULL DEFAULT FALSE, 
  started BOOL NOT NULL DEFAULT FALSE,
  parent_id INTEGER DEFAULT NULL,
  is_open BOOL NOT NULL DEFAULT FALSE,
  FOREIGN KEY (parent_id) REFERENCES nodes(id)
  ON DELETE CASCADE
);

CREATE TRIGGER start_done
BEFORE UPDATE of done ON nodes
WHEN NEW.done = 1 AND OLD.done IS NOT 1
BEGIN
UPDATE nodes
    SET started = 1
    WHERE id = NEW.id;
END;

CREATE TRIGGER propag_children_done
AFTER UPDATE OF done ON nodes
WHEN NEW.done = 1 AND OLD.done IS NOT 1
BEGIN
  UPDATE nodes
  SET done = 1
  WHERE id IN (
  WITH RECURSIVE descendants AS (
        SELECT id, parent_id FROM nodes WHERE id = OLD.id
        UNION ALL
        SELECT n.id, n.parent_id
        FROM nodes n
        JOIN descendants d ON n.parent_id = d.id
    )
    SELECT id FROM descendants
  );
END;

CREATE TRIGGER propag_ancestor_started
AFTER UPDATE ON nodes
WHEN (NEW.done = 1 AND OLD.done IS NOT 1) OR (NEW.started = 1 AND OLD.started IS NOT 1)
BEGIN
 UPDATE nodes
    SET started = 1
    WHERE id IN (
        WITH RECURSIVE ancestors AS (
            SELECT id, parent_id
            FROM nodes
            WHERE id = NEW.id

            UNION ALL

            SELECT n.id, n.parent_id
            FROM nodes n
            JOIN ancestors a ON n.id = a.parent_id
        )
        SELECT id FROM ancestors
    );
END;
