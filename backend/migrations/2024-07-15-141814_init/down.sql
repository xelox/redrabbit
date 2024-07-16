-- This file should undo anything in `up.sql`

DROP TABLE nodes;

DROP TRIGGER IF EXISTS start_done;
DROP TRIGGER IF EXISTS propag_children_done;
DROP TRIGGER IF EXISTS propag_ancestor_started;
