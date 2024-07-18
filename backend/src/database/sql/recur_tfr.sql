WITH RECURSIVE open_tree AS (
  SELECT *
  FROM nodes
  WHERE parent_id IS NULL

  UNION ALL

  SELECT t.*
  FROM nodes t
  JOIN open_tree ot ON t.parent_id = ot.id
)
SELECT * FROM open_tree;
