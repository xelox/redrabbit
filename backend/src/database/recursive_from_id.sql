WITH RECURSIVE open_tree AS (
  SELECT *
  FROM nodes
  WHERE id = ?

  UNION ALL

  SELECT t.*
  FROM nodes t
  JOIN open_tree ot ON t.parent_id = ot.id
  WHERE ot.is_open = 1
)
SELECT * FROM open_tree;
